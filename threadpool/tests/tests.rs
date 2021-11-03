use threadpool::ThreadPool;

use crossbeam::channel;

use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
    time::Duration,
};

#[test]
fn test_simple() {
    for thread_count in 1..5 {
        let pool = ThreadPool::new(thread_count, 1000);
        let counter = Arc::new(AtomicI32::new(0));

        let handles = (0..100)
            .map(|_| {
                let counter = counter.clone();
                pool.spawn(move || counter.fetch_add(1, Ordering::Relaxed))
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }
}

#[test]
fn test_thread_count() {
    let pool = ThreadPool::new(5, 1000);
    let (start_sender, start_receiver) = channel::unbounded::<()>();
    let (finish_sender, finish_receiver) = channel::unbounded::<()>();

    for _ in 0..10 {
        let start_sender = start_sender.clone();
        let finish_receiver = finish_receiver.clone();
        pool.spawn(move || {
            start_sender.send(()).ok();
            finish_receiver.recv().ok();
        });
    }

    for _ in 0..5 {
        start_receiver.recv_timeout(Duration::from_secs(1)).unwrap();
    }

    for _ in 0..5 {
        let res = start_receiver.recv_timeout(Duration::from_millis(100));
        assert!(res.is_err());
        finish_sender.send(()).unwrap();
        start_receiver.recv_timeout(Duration::from_secs(1)).unwrap();
    }
}

#[test]
fn test_panic() {
    let pool = ThreadPool::new(1, 1000);
    let panic_handle = pool.spawn(|| panic!("something is wrong"));
    assert!(panic_handle.join().is_err());

    let res = pool.spawn(|| 3 + 8).join().unwrap();
    assert_eq!(res, 11);
}

#[test]
fn test_shutdown() {
    let pool = ThreadPool::new(1, 1000);
    let counter = Arc::new(AtomicI32::new(0));
    for _ in 0..1000 {
        let counter = counter.clone();
        pool.spawn(move || counter.fetch_add(1, Ordering::Relaxed));
    }

    pool.shutdown();
    assert_eq!(counter.load(Ordering::Relaxed), 1000);
}
