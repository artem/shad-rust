use futures::{channel::oneshot, future::poll_fn};
use test_log::test;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::Poll,
    thread,
};

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_simple() {
    let runtime = rio::Runtime::new_current_thread();
    let flag = runtime.block_on(async { true });
    assert!(flag);
}

#[test]
fn test_side_effect() {
    let flag = Arc::new(AtomicBool::new(false));
    {
        let runtime = rio::Runtime::new_current_thread();
        runtime.block_on({
            let flag = flag.clone();
            async move { flag.store(true, Ordering::Relaxed) }
        });
    }
    assert!(flag.load(Ordering::Relaxed));
}

#[test]
fn test_ping_pong() {
    let runtime = rio::Runtime::new_current_thread();

    let (sender_one, receiver_one) = oneshot::channel::<i32>();
    let (sender_two, receiver_two) = oneshot::channel::<i32>();
    runtime.spawn(async move {
        assert_eq!(receiver_one.await.unwrap(), 1);
        sender_two.send(2).unwrap();
    });

    let res = runtime.block_on(async move {
        sender_one.send(1).unwrap();
        receiver_two.await.unwrap()
    });
    assert_eq!(res, 2);
}

#[test]
fn test_threaded_ping_pong() {
    let (sender_one, receiver_one) = oneshot::channel::<i32>();
    let (sender_two, receiver_two) = oneshot::channel::<i32>();

    thread::spawn(move || {
        let runtime = rio::Runtime::new_current_thread();
        runtime.block_on(async move {
            assert_eq!(receiver_one.await.unwrap(), 1);
            sender_two.send(2).unwrap();
        });
    });

    let runtime = rio::Runtime::new_current_thread();
    let res = runtime.block_on(async move {
        sender_one.send(1).unwrap();
        receiver_two.await.unwrap()
    });
    assert_eq!(res, 2);
}

#[test]
fn test_global_spawn() {
    let runtime = rio::Runtime::new_current_thread();

    let (sender_one, receiver_one) = oneshot::channel::<i32>();
    let (sender_two, receiver_two) = oneshot::channel::<i32>();

    let res = runtime.block_on(async move {
        rio::spawn(async move {
            assert_eq!(receiver_one.await.unwrap(), 1);
            sender_two.send(2).unwrap();
        });
        sender_one.send(1).unwrap();
        receiver_two.await.unwrap()
    });
    assert_eq!(res, 2);
}

#[rio::test]
async fn test_wake_nonexistent_task() {
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
}
