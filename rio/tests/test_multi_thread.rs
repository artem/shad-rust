#![cfg(feature = "rt-multi-thread")]

use futures::{
    channel::{mpsc, oneshot},
    join,
    task::AtomicWaker,
    StreamExt,
};
use test_log::test;

use std::{
    future::poll_fn,
    sync::{
        self,
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    task::Poll,
    thread,
    time::Duration,
};

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_spawn() {
    let runtime = rio::Runtime::new_multi_thread(1);
    let (sender, receiver) = sync::mpsc::channel();
    runtime.spawn(async move {
        sender.send(()).unwrap();
    });
    receiver.recv_timeout(Duration::from_secs(1)).unwrap();
}

#[test]
fn test_thread_count() {
    for thread_count in [1, 2, 4, 8, 11] {
        let runtime = rio::Runtime::new_multi_thread(thread_count);
        let (sender, receiver) = sync::mpsc::channel();
        let barrier = Arc::new(sync::Barrier::new(thread_count + 1));

        for _ in 0..thread_count {
            let barrier = barrier.clone();
            let sender = sender.clone();
            runtime.spawn(async move {
                sender.send(()).unwrap();
                barrier.wait();
            });
        }

        for _ in 0..thread_count {
            receiver.recv_timeout(Duration::from_secs(1)).unwrap();
        }

        runtime.spawn(async move {
            sender.send(()).unwrap();
        });

        receiver
            .recv_timeout(Duration::from_millis(100))
            .unwrap_err();
        barrier.wait();

        receiver.recv_timeout(Duration::from_secs(1)).unwrap();
    }
}

#[test]
fn test_context_block_on() {
    let runtime = rio::Runtime::new_multi_thread(2);
    let barrier = Arc::new(sync::Barrier::new(2));
    let make_task = move || {
        let barrier = barrier.clone();
        async move {
            barrier.wait();
        }
    };
    runtime.block_on(async move {
        let first = rio::spawn(make_task());
        let second = rio::spawn(make_task());
        let (first_result, second_result) = join!(first, second);
        first_result.unwrap();
        second_result.unwrap();
    });
}

#[test]
fn test_context_spawn() {
    let runtime = rio::Runtime::new_multi_thread(2);
    let (sender, receiver) = sync::mpsc::channel();
    let barrier = Arc::new(sync::Barrier::new(2));
    let make_task = move || {
        let barrier = barrier.clone();
        async move {
            barrier.wait();
        }
    };

    runtime.spawn(async move {
        let first = rio::spawn(make_task());
        let second = rio::spawn(make_task());
        let (first_result, second_result) = join!(first, second);
        first_result.unwrap();
        second_result.unwrap();
        sender.send(()).unwrap();
    });

    receiver.recv_timeout(Duration::from_secs(1)).unwrap();
}

#[test]
fn test_context_multiple_runtimes() {
    let runtime_one = rio::Runtime::new_multi_thread(1);
    let runtime_two = rio::Runtime::new_multi_thread(2);

    let root_barrier = Arc::new(sync::Barrier::new(2));
    let make_root_task = move |expected_runtime_id| {
        let root_barrier = root_barrier.clone();
        async move {
            root_barrier.wait();
            rio::spawn(async move {
                root_barrier.wait();
                rio::spawn(async move {
                    assert_eq!(rio::runtime_id(), expected_runtime_id);
                })
                .await
                .unwrap();
            })
            .await
        }
    };

    let handle = runtime_one.spawn(make_root_task(runtime_one.id()));
    runtime_two
        .block_on(make_root_task(runtime_two.id()))
        .unwrap();
    runtime_one.block_on(handle).unwrap().unwrap();
}

#[test]
fn test_many_tasks() {
    let runtime = rio::Runtime::new_multi_thread(2);

    let mut oneshot_senders = vec![];
    let (mpsc_sender, mut mpsc_receiver) = mpsc::unbounded();

    for _ in 0..100 {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel::<usize>();
        oneshot_senders.push(oneshot_sender);
        let mpsc_sender = mpsc_sender.clone();
        runtime.spawn(async move {
            let value = oneshot_receiver.await.unwrap();
            mpsc_sender.unbounded_send(value).unwrap();
        });
    }
    drop(mpsc_sender);

    runtime.block_on(async move {
        for (i, oneshot_sender) in oneshot_senders.into_iter().enumerate().rev() {
            oneshot_sender.send(i).unwrap();
        }
        let mut values = vec![false; 100];
        while let Some(value) = mpsc_receiver.next().await {
            assert!(!values[value]);
            values[value] = true;
        }
        assert!(values.into_iter().all(|v| v));
    });
}

#[test]
fn test_many_tasks_two_runtimes() {
    let runtime_one = rio::Runtime::new_multi_thread(2);
    let runtime_two = rio::Runtime::new_multi_thread(2);

    let mut oneshot_senders = vec![];
    let (mpsc_sender, mut mpsc_receiver) = mpsc::unbounded();

    for _ in 0..100 {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel::<usize>();
        oneshot_senders.push(oneshot_sender);
        let mpsc_sender = mpsc_sender.clone();
        runtime_one.spawn(async move {
            let value = oneshot_receiver.await.unwrap();
            mpsc_sender.unbounded_send(value).unwrap();
        });
    }
    drop(mpsc_sender);

    runtime_two.block_on(async move {
        for (i, oneshot_sender) in oneshot_senders.into_iter().enumerate().rev() {
            oneshot_sender.send(i).unwrap();
        }
        let mut values = vec![false; 100];
        while let Some(value) = mpsc_receiver.next().await {
            assert!(!values[value]);
            values[value] = true;
        }
        assert!(values.into_iter().all(|v| v));
    });
}

#[test]
fn test_wake_nonexistent_task() {
    let runtime = rio::Runtime::new_multi_thread(2);
    runtime.block_on(async {
        let waker = rio::spawn(poll_fn(|cx| Poll::Ready(cx.waker().clone())))
            .await
            .unwrap();

        let mut handles = vec![];
        for _ in 0..10 {
            waker.wake_by_ref();
            handles.push(rio::spawn(async move { 42 }));
        }

        for h in handles {
            assert_eq!(h.await.unwrap(), 42);
        }
    });
}

#[test]
fn test_poll_once() {
    let runtime = rio::Runtime::new_multi_thread(2);
    let poll_count = Arc::new(AtomicU64::new(0));
    let (polled_sender, polled_receiver) = sync::mpsc::channel::<()>();

    runtime.spawn(poll_fn({
        let poll_count = poll_count.clone();
        move |_| {
            poll_count.fetch_add(1, Ordering::Relaxed);
            polled_sender.send(()).unwrap();
            Poll::<()>::Pending
        }
    }));

    polled_receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();

    let (sender, receiver) = oneshot::channel();
    let handle = runtime.spawn(async move {
        receiver.await.unwrap();
    });

    thread::sleep(Duration::from_millis(10));
    sender.send(()).unwrap();
    runtime.block_on(handle).unwrap();

    assert_eq!(poll_count.load(Ordering::Relaxed), 1);
}

#[test]
fn test_poll_once_multiple_wakes() {
    let runtime = rio::Runtime::new_multi_thread(2);
    let poll_count = Arc::new(AtomicU64::new(0));
    let (polled_sender, polled_receiver) = sync::mpsc::channel::<()>();
    let waker = Arc::new(AtomicWaker::new());

    runtime.spawn(poll_fn({
        let poll_count = poll_count.clone();
        let waker = waker.clone();
        move |cx| {
            waker.register(cx.waker());
            poll_count.fetch_add(1, Ordering::Relaxed);
            polled_sender.send(()).unwrap();
            Poll::<()>::Pending
        }
    }));

    polled_receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();

    let barrier = Arc::new(sync::Barrier::new(3));
    for _ in 0..2 {
        runtime.spawn({
            let barrier = barrier.clone();
            async move {
                barrier.wait();
            }
        });
    }

    for _ in 0..10 {
        waker.wake();
    }
    barrier.wait();

    polled_receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();

    let (sender, receiver) = oneshot::channel();
    let handle = runtime.spawn(async move {
        receiver.await.unwrap();
    });

    thread::sleep(Duration::from_millis(10));
    sender.send(()).unwrap();
    runtime.block_on(handle).unwrap();

    assert_eq!(poll_count.load(Ordering::Relaxed), 2);
}

#[test]
fn test_concurrent_wake() {
    let runtime = rio::Runtime::new_multi_thread(2);
    let poll_count = Arc::new(AtomicU64::new(0));
    let (polled_sender, polled_receiver) = sync::mpsc::channel::<()>();
    let (resume_sender, resume_receiver) = sync::mpsc::channel::<()>();
    let waker = Arc::new(AtomicWaker::new());

    runtime.spawn(poll_fn({
        let poll_count = poll_count.clone();
        let waker = waker.clone();
        move |cx| {
            let prev_poll_count = poll_count.fetch_add(1, Ordering::Relaxed);
            waker.register(cx.waker());
            polled_sender.send(()).unwrap();
            if prev_poll_count == 0 {
                resume_receiver.recv().unwrap();
            }
            Poll::<()>::Pending
        }
    }));

    polled_receiver.recv().unwrap();
    waker.wake();

    let (sender, receiver) = oneshot::channel();
    let handle = runtime.spawn(async move {
        receiver.await.unwrap();
    });

    thread::sleep(Duration::from_millis(10));
    sender.send(()).unwrap();
    runtime.block_on(handle).unwrap();

    resume_sender.send(()).unwrap();

    polled_receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();
    assert_eq!(poll_count.load(Ordering::Relaxed), 2);
}
