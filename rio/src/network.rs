use std::{
    collections::HashMap,
    io::{self, ErrorKind},
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        Arc, Mutex, MutexGuard, Weak,
    },
    task::Poll,
    thread::{self, JoinHandle},
    time::Duration,
};

use futures::{future::poll_fn, task::AtomicWaker};

use log::debug;
use mio::{
    event::{Event, Source},
    Events, Token,
};

use crate::runtime::RuntimeHandle;

////////////////////////////////////////////////////////////////////////////////

pub struct UdpSocket {
    inner: mio::net::UdpSocket,
    runtime: RuntimeHandle,
    // TODO: your code here.
}

impl UdpSocket {
    pub fn bind(addr: SocketAddr) -> io::Result<UdpSocket> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn connect(&self, addr: SocketAddr) -> io::Result<()> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        // TODO: your code here.
        unimplemented!()
    }

    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        // TODO: your code here.
        unimplemented!()
    }

    pub async fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        // TODO: your code here.
        unimplemented!()
    }

    pub async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        // TODO: your code here.
        unimplemented!()
    }

    pub async fn send_to(&self, buf: &[u8], addr: SocketAddr) -> io::Result<usize> {
        // TODO: your code here.
        unimplemented!()
    }

    // TODO: your code here.
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        // TODO: your code here.
        unimplemented!()
    }
}

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
