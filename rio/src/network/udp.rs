use super::driver::ReadinessKind;

use crate::runtime::RuntimeHandle;

use futures::future::poll_fn;
use log::debug;
use mio::Token;

use std::{
    io::{self, ErrorKind},
    net::SocketAddr,
    os::fd::{AsRawFd, RawFd},
    task::Poll,
};

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

impl AsRawFd for UdpSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}
