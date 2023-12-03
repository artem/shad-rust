use std::net::SocketAddr;

use futures::channel::oneshot;
use log::debug;
use test_log::test;

use rio::UdpSocket;

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
        rio::Runtime::default().block_on(async move {
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
        rio::Runtime::default().block_on(async move {
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

// Some inefficient implementations might read the socket
// on every .poll(), which is still correct.
// So this test is disabled.
////////////////////////////////////////////////////////////////////////////////

/*
struct CountingWaker {
    waker: Waker,
    call_count: Arc<AtomicUsize>,
}

impl ArcWake for CountingWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.call_count.fetch_add(1, Ordering::Relaxed);
        arc_self.waker.wake_by_ref();
    }
}

impl CountingWaker {
    async fn from_current() -> (Waker, Arc<AtomicUsize>) {
        let waker = poll_fn(|cx| Poll::Ready(cx.waker().clone())).await;
        let call_count = Arc::new(AtomicUsize::new(0));
        let counting_waker = futures::task::waker(Arc::new(CountingWaker {
            waker,
            call_count: call_count.clone(),
        }));
        (counting_waker, call_count)
    }
}

#[rio::test]
async fn test_waker_call_count() {
    let socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let address = socket.local_addr().unwrap();

    let mut buf = [0u8; 5];
    let mut call_counters = vec![];
    {
        let recv_future = socket.recv(&mut buf);
        pin_mut!(recv_future);

        for _ in 0..4 {
            let (counting_waker, call_count) = CountingWaker::from_current().await;
            call_counters.push(call_count);

            let mut context = Context::from_waker(&counting_waker);
            let _ = recv_future.poll_unpin(&mut context);
        }

        let (counting_waker, call_count) = CountingWaker::from_current().await;
        call_counters.push(call_count);

        let mut context = Context::from_waker(&counting_waker);
        let _ = recv_future.poll_unpin(&mut context);

        let sending_socket = UdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
        sending_socket.send_to(b"hello", address).await.unwrap();

        while recv_future.poll_unpin(&mut context).is_pending() {}
    }
    assert_eq!(&buf, b"hello");

    for call_count in call_counters.iter().take(call_counters.len() - 1) {
        assert_eq!(call_count.load(Ordering::Relaxed), 0);
    }
    assert_eq!(call_counters.last().unwrap().load(Ordering::Relaxed), 1);
}
*/
