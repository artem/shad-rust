use crate::runtime::ContextManager;

use super::task::{Task, TaskId};

use futures::task::ArcWake;
use log::debug;
use rayon::{ThreadPool, ThreadPoolBuilder};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard, Weak},
    task::{Context, Poll},
};

////////////////////////////////////////////////////////////////////////////////

struct SharedState {
    // TODO: your code here.
    thread_pool: ThreadPool,
}

////////////////////////////////////////////////////////////////////////////////

pub struct MultiThreadScheduler {
    shared_state: Arc<SharedState>,
}

impl MultiThreadScheduler {
    pub fn new(context_manager: ContextManager, num_threads: usize) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn submit(&self, task: Task) {
        // TODO: your code here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Waker {
    shared_state: Weak<SharedState>,
    // TODO: your code here.
}

impl ArcWake for Waker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // TODO: your code here.
        unimplemented!()
    }
}
