use futures::task::AtomicWaker;
use log::debug;
use mio::{
    event::{Event, Source},
    Events, Token,
};

use std::{
    collections::HashMap,
    io,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        Arc, Mutex, MutexGuard, Weak,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

////////////////////////////////////////////////////////////////////////////////

pub struct NetworkDriver {
    poll: mio::Poll,
    halt: Arc<AtomicBool>,
    // TODO: your code here.
}

impl NetworkDriver {
    pub fn start() -> NetworkHandle {
        // TODO: your code here.
        unimplemented!()
    }

    // TODO: your code here.
}

////////////////////////////////////////////////////////////////////////////////

pub struct NetworkHandle {
    registry: mio::Registry,
    // TODO: your code here.
}

impl NetworkHandle {
    // TODO: your code here.
}

impl Drop for NetworkHandle {
    fn drop(&mut self) {
        // TODO: your code here.
        unimplemented!()
    }
}
