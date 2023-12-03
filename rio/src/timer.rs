use crate::runtime::RuntimeHandle;

use futures::task::AtomicWaker;

use std::{
    collections::BinaryHeap,
    future::poll_fn,
    ops::Add,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::Poll,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

////////////////////////////////////////////////////////////////////////////////

pub async fn sleep(duration: Duration) {
    let timestamp = Instant::now().add(duration);
    let entry = RuntimeHandle::current()
        .state()
        .timer_handle
        .add_entry(timestamp);
    poll_fn(move |cx| {
        // NB: it is important to register waker before checking current time
        // to avoid race condition.
        entry.waker.register(cx.waker());
        if Instant::now() >= timestamp {
            return Poll::Ready(());
        }
        Poll::Pending
    })
    .await
}

////////////////////////////////////////////////////////////////////////////////

struct TimerEntry {
    timestamp: Instant,
    waker: AtomicWaker,
}

impl PartialEq for TimerEntry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for TimerEntry {}

impl Ord for TimerEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

////////////////////////////////////////////////////////////////////////////////

type TimerEntryHeap = BinaryHeap<Arc<TimerEntry>>;

pub struct TimerDriver {
    entries: Arc<Mutex<TimerEntryHeap>>,
    halt: Arc<AtomicBool>,
}

impl TimerDriver {
    pub fn start() -> TimerHandle {
        let entries = Arc::new(Mutex::new(BinaryHeap::new()));
        let halt = Arc::new(AtomicBool::new(false));
        let join_handle = thread::spawn({
            let entries = entries.clone();
            let halt = halt.clone();
            move || {
                TimerDriver { entries, halt }.run();
            }
        });
        TimerHandle {
            entries,
            halt,
            join_handle: Some(join_handle),
        }
    }

    fn run(&self) {
        while !self.halt.load(Ordering::Relaxed) {
            let Some(event) = self.entries.lock().unwrap().pop() else {
                thread::park();
                continue;
            };
            if event.timestamp <= Instant::now() {
                event.waker.wake();
            } else {
                let park_duration = event.timestamp.duration_since(Instant::now());
                self.entries.lock().unwrap().push(event);
                thread::park_timeout(park_duration);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct TimerHandle {
    entries: Arc<Mutex<TimerEntryHeap>>,
    halt: Arc<AtomicBool>,
    join_handle: Option<JoinHandle<()>>,
}

impl TimerHandle {
    fn add_entry(&self, timestamp: Instant) -> Arc<TimerEntry> {
        let entry = Arc::new(TimerEntry {
            timestamp,
            waker: Default::default(),
        });
        self.entries.lock().unwrap().push(entry.clone());
        self.join_handle.as_ref().unwrap().thread().unpark();
        entry
    }
}

impl Drop for TimerHandle {
    fn drop(&mut self) {
        self.halt.store(true, Ordering::Relaxed);
        let join_handle = self.join_handle.take().unwrap();
        join_handle.thread().unpark();
        join_handle.join().expect("failed to join timer thread");
    }
}
