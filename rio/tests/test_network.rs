#![cfg(feature = "net")]

use rio::UdpSocket;

use futures::{channel::oneshot, task::ArcWake, FutureExt};
use log::debug;
use test_log::test;

use std::{
    future::{poll_fn, Future},
    net::SocketAddr,
    ops::Deref,
    os::fd::AsRawFd,
    pin::pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    task::{Context, Poll, Waker},
    time::{Duration, Instant},
};

////////////////////////////////////////////////////////////////////////////////

async fn expect_data_from(s: &UdpSocket, expected_data: &[u8], expected_peer: SocketAddr) {
    let mut buf = vec![0u8; expected_data.len()];
    let (len, remote_addr) = s.recv_from(&mut buf).await.unwrap();
    assert_eq!(len, expected_data.len());
    assert_eq!(remote_addr, expected_peer);
    assert_eq!(buf, expected_data);
}

////////////////////////////////////////////////////////////////////////////////

#[rio::test]
async fn test_simple_ping_pong() {
    let first_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let first_address = first_socket.local_addr().unwrap();
    let second_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let second_address = second_socket.local_addr().unwrap();

    first_socket
        .send_to(b"PING!", second_address)
        .await
        .unwrap();
    debug!("sent ping");

    expect_data_from(&second_socket, b"PING!", first_address).await;
    debug!("received ping");

    second_socket
        .send_to(b"PONG!", first_address)
        .await
        .unwrap();
    debug!("sent pong");

    expect_data_from(&first_socket, b"PONG!", second_address).await;
    debug!("received pong");
}

#[rio::test]
async fn test_concurrent_ping_pong() {
    let first_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let first_address = first_socket.local_addr().unwrap();
    let second_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let second_address = second_socket.local_addr().unwrap();

    let first_handle = rio::spawn(async move {
        for i in 0..5 {
            debug!("sending ping #{}", i);
            first_socket
                .send_to(format!("ping #{}", i).as_bytes(), second_address)
                .await
                .unwrap();

            debug!("sent ping #{}, expecting pong", i);

            expect_data_from(
                &first_socket,
                format!("pong #{}", i).as_bytes(),
                second_address,
            )
            .await;
            debug!("got pong #{}", i);
        }
    });

    let second_handle = rio::spawn(async move {
        for i in 0..5 {
            debug!("expecting ping #{}", i);
            expect_data_from(
                &second_socket,
                format!("ping #{}", i).as_bytes(),
                first_address,
            )
            .await;

            debug!("got ping #{}, sending pong", i);

            second_socket
                .send_to(format!("pong #{}", i).as_bytes(), first_address)
                .await
                .unwrap();
            debug!("sent pong #{}", i);
        }
    });

    first_handle.await.unwrap();
    second_handle.await.unwrap();
}

#[test]
fn test_parallel_ping_pong() {
    let (first_addr_sender, first_addr_receiver) = oneshot::channel();
    let (second_addr_sender, second_addr_receiver) = oneshot::channel();

    let first_handle = std::thread::spawn(move || {
        rio::Runtime::new_current_thread().block_on(async move {
            let socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
            first_addr_sender
                .send(socket.local_addr().unwrap())
                .unwrap();
            let remote_address = second_addr_receiver.await.unwrap();

            for i in 0..5 {
                socket
                    .send_to(format!("ping #{}", i).as_bytes(), remote_address)
                    .await
                    .unwrap();

                expect_data_from(&socket, format!("pong #{}", i).as_bytes(), remote_address).await;
            }
        });
    });

    let second_handle = std::thread::spawn(move || {
        rio::Runtime::new_current_thread().block_on(async move {
            let socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
            second_addr_sender
                .send(socket.local_addr().unwrap())
                .unwrap();
            let remote_address = first_addr_receiver.await.unwrap();

            for i in 0..5 {
                expect_data_from(&socket, format!("ping #{}", i).as_bytes(), remote_address).await;

                socket
                    .send_to(format!("pong #{}", i).as_bytes(), remote_address)
                    .await
                    .unwrap();
            }
        });
    });

    first_handle.join().unwrap();
    second_handle.join().unwrap();
}

#[rio::test]
async fn test_fan_in_ping_pong() {
    let server_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let server_address = server_socket.local_addr().unwrap();

    let handles = (0..3)
        .map(|i| {
            rio::spawn({
                let server_address = server_address.clone();
                async move {
                    let socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
                    debug!("sending ping (id {})", i);
                    socket.send_to(b"ping", server_address).await.unwrap();
                    debug!("sent ping, expecting pong (id {})", i);
                    expect_data_from(&socket, b"pong", server_address).await;
                    debug!("got pong (id {})", i)
                }
            })
        })
        .collect::<Vec<_>>();

    for i in 0..handles.len() {
        let mut buf = [0u8; 4];
        let (len, remote_address) = server_socket.recv_from(&mut buf).await.unwrap();
        assert_eq!(len, 4);
        assert_eq!(&buf, b"ping");
        debug!("got ping #{} (server)", i);
        server_socket
            .send_to(b"pong", remote_address)
            .await
            .unwrap();
        debug!("sent pong #{} (server)", i);
    }

    debug!("server finished");

    for (i, h) in handles.into_iter().enumerate() {
        h.await.unwrap();
        debug!("joined handle #{}", i);
    }
}

#[rio::test]
async fn test_runtime_switch() {
    let sender_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let receiver_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let receiver_address = receiver_socket.local_addr().unwrap();

    let handle = std::thread::spawn(move || {
        rio::Runtime::new_current_thread().block_on(async move {
            sender_socket
                .send_to(b"ping", receiver_address)
                .await
                .unwrap();
        })
    });
    handle.join().unwrap();

    let mut buf = [0; 4];
    receiver_socket.recv(&mut buf).await.unwrap();
    assert_eq!(&buf, b"ping");
}

#[rio::test]
async fn test_no_unnecessary_syscalls() {
    let socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();

    // NB: correct implementation should never do a syscall unless read readiness
    // event is received, so it will never notice that we closed the file descriptor.
    nix::unistd::close(socket.as_raw_fd()).unwrap();

    {
        let mut buf = [0; 4];
        let mut recv_future = pin!(socket.recv(&mut buf));
        {
            let waker = poll_fn(|cx| Poll::Ready(cx.waker().clone())).await;
            let mut context = Context::from_waker(&waker);
            for _ in 0..10 {
                assert!(recv_future.poll_unpin(&mut context).is_pending());
            }
        }
    }

    // NB: leak socket to prevent Drop from trying to close fd again.
    std::mem::forget(socket);
}

////////////////////////////////////////////////////////////////////////////////

struct CountingWaker {
    waker: Waker,
    call_count: Arc<AtomicUsize>,
}

impl ArcWake for CountingWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        debug!("waking counting waker #{:#x}", arc_self.id());
        arc_self.call_count.fetch_add(1, Ordering::Relaxed);
        arc_self.waker.wake_by_ref();
    }
}

impl CountingWaker {
    pub async fn from_current() -> Arc<CountingWaker> {
        let waker = poll_fn(|cx| Poll::Ready(cx.waker().clone())).await;
        let call_count = Arc::new(AtomicUsize::new(0));
        let counting_waker = Arc::new(Self { waker, call_count });
        debug!("created counting waker #{:#x}", counting_waker.id());
        counting_waker
    }

    pub fn poll_future<T: Future + Unpin>(self: &Arc<Self>, future: &mut T) -> Poll<T::Output> {
        let waker = futures::task::waker(self.clone());
        let mut context = Context::from_waker(&waker);
        future.poll_unpin(&mut context)
    }

    pub fn call_count(&self) -> usize {
        self.call_count.load(Ordering::Relaxed)
    }

    pub fn id(self: &Arc<Self>) -> usize {
        self.deref() as *const Self as usize
    }
}

fn wait_for(mut predicate: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        if predicate() {
            return;
        }
    }
    panic!("wait_for failed");
}

////////////////////////////////////////////////////////////////////////////////

#[rio::test]
async fn test_only_last_waker_called() {
    let socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let address = socket.local_addr().unwrap();

    let mut buf = [0u8; 5];
    let mut wakers = vec![];

    {
        let mut recv_future = pin!(socket.recv(&mut buf));

        for _ in 0..4 {
            let waker = CountingWaker::from_current().await;
            assert!(waker.poll_future(&mut recv_future).is_pending());
            assert_eq!(waker.call_count(), 0);
            wakers.push(waker);
        }

        let sending_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
        sending_socket.send_to(b"hello", address).await.unwrap();

        let last_waker = wakers.last().unwrap();
        wait_for(|| last_waker.call_count() > 0);
        assert!(last_waker.poll_future(&mut recv_future).is_ready());
    }

    assert_eq!(&buf, b"hello");

    for waker in wakers.iter().take(wakers.len() - 1) {
        assert_eq!(waker.call_count(), 0);
    }
    assert_eq!(wakers.last().unwrap().call_count(), 1);
}

#[rio::test]
async fn test_both_futures_woken() {
    let socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let address = socket.local_addr().unwrap();

    let mut buf_one = [0u8; 4];
    let mut buf_two = [0u8; 4];

    let waker_one = CountingWaker::from_current().await;
    let waker_two = CountingWaker::from_current().await;

    {
        let mut recv_future_one = pin!(socket.recv(&mut buf_one));
        assert!(waker_one.poll_future(&mut recv_future_one).is_pending());
        assert_eq!(waker_one.call_count(), 0);

        let mut recv_future_two = pin!(socket.recv(&mut buf_two));
        assert!(waker_two.poll_future(&mut recv_future_two).is_pending());
        assert_eq!(waker_two.call_count(), 0);

        let sending_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
        sending_socket.send_to(b"ping", address).await.unwrap();

        wait_for(|| waker_one.call_count() > 0 && waker_two.call_count() > 0);

        assert!(waker_one.poll_future(&mut recv_future_one).is_ready());
        assert!(waker_two.poll_future(&mut recv_future_two).is_pending());
    }

    assert!(&buf_one == b"ping");
    assert!(buf_two == [0; 4]);
    assert_eq!(waker_one.call_count(), 1);
    assert_eq!(waker_two.call_count(), 1);
}

#[rio::test]
async fn test_writes_dont_wake_reads() {
    let socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let receiver_address = UdpSocket::bind("127.0.0.1:0".parse().unwrap())
        .unwrap()
        .local_addr()
        .unwrap();

    let mut buf = [0u8; 4];
    let recv_waker = CountingWaker::from_current().await;
    let mut recv_future = pin!(socket.recv(&mut buf));
    assert!(recv_waker.poll_future(&mut recv_future).is_pending());

    let datagram = vec![0; 1024];
    let mut bytes_written = 0;
    while bytes_written < 64 * 1024 * 1024 {
        bytes_written += socket.send_to(&datagram, receiver_address).await.unwrap();
    }

    assert_eq!(recv_waker.call_count(), 0);
}

////////////////////////////////////////////////////////////////////////////////

#[rio::test]
async fn stress_test_one_way() {
    const N_RETRIES: usize = 1000;
    const N_MESSAGES: usize = 100;

    for _ in 0..N_RETRIES {
        let receiver_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
        let receiver_address = receiver_socket.local_addr().unwrap();

        std::thread::spawn(move || {
            rio::Runtime::new_current_thread().block_on(async move {
                let sender_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
                for i in 0..N_MESSAGES {
                    sender_socket
                        .send_to(b"ping", receiver_address)
                        .await
                        .unwrap();
                    debug!("sent message #{}", i);
                }
            });
        });

        for i in 0..N_MESSAGES {
            let mut buf = [0; 4];
            assert_eq!(receiver_socket.recv(&mut buf).await.unwrap(), 4);
            assert_eq!(&buf, b"ping");
            debug!("received message #{}", i);
        }
    }
}

#[rio::test]
async fn stress_test_ping_pong() {
    const N_RETRIES: usize = 1000;
    const N_MESSAGES: usize = 100;

    for _ in 0..N_RETRIES {
        let receiver_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
        let receiver_address = receiver_socket.local_addr().unwrap();

        std::thread::spawn(move || {
            rio::Runtime::new_current_thread().block_on(async move {
                let sender_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
                for _ in 0..N_MESSAGES {
                    assert_eq!(
                        sender_socket
                            .send_to(b"ping", receiver_address)
                            .await
                            .unwrap(),
                        4
                    );

                    let mut buf = [0; 4];
                    assert_eq!(sender_socket.recv(&mut buf).await.unwrap(), 4);
                    assert_eq!(&buf, b"pong");
                }
            });
        });

        for _ in 0..N_MESSAGES {
            let mut buf = [0; 4];
            let (_, addr) = receiver_socket.recv_from(&mut buf).await.unwrap();
            assert_eq!(&buf, b"ping");

            assert_eq!(receiver_socket.send_to(b"pong", addr).await.unwrap(), 4);
        }
    }
}
