#![forbid(unsafe_code)]

use crossbeam::channel::{self, Receiver, Sender};

use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    thread,
};

////////////////////////////////////////////////////////////////////////////////

pub struct ThreadPool {
    // TODO: your code here.
}

impl ThreadPool {
    pub fn new(thread_count: usize, queue_size: usize) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    // pub fn spawn(&self, task: ...) -> JoinHandle<...> {}

    pub fn shutdown(self) {
        // TODO: your code here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct JoinHandle<T> {
    // TODO: your code here.
}

#[derive(Debug)]
pub struct JoinError {}

impl<T> JoinHandle<T> {
    pub fn join(self) -> Result<T, JoinError> {
        // TODO: your code here.
        unimplemented!()
    }
}
