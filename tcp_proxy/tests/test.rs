use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command};
use std::str;
use std::thread;
use std::time;

const BINARY_PATH: &str = if cfg!(debug_assertions) {
    "../target/debug/tcp_proxy"
} else {
    "../target/release/tcp_proxy"
};

fn start_proxy() -> (TcpListener, Child, String) {
    let mut rng = rand::thread_rng();
    let port = rng.gen_range(40000..49151);

    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let port_str = format!("{}", port);
    let server_str = format!("127.0.0.1:{}", server.local_addr().unwrap().port());

    let proxy_proc = Command::new(BINARY_PATH)
        .args(&["-p", &port_str, "-d", &server_str])
        .spawn()
        .unwrap();
    thread::sleep(time::Duration::from_millis(10));
    (server, proxy_proc, format!("127.0.0.1:{}", port))
}

#[test]
fn test_ping_pong() {
    let (server, mut proxy, proxy_addr) = start_proxy();

    let mut client = TcpStream::connect(proxy_addr).unwrap();

    let client_thread = thread::spawn(move || {
        let msg = "ping";
        client.write_all(msg.as_bytes()).unwrap();
        let mut read_buffer: [u8; 4] = [0; 4];
        client.read_exact(&mut read_buffer).unwrap();

        assert_eq!(str::from_utf8(&read_buffer).unwrap(), "pong");
    });

    let server_thread = thread::spawn(move || {
        let mut connection = server.accept().unwrap();
        let mut read_buffer: [u8; 4] = [0; 4];
        connection.0.read_exact(&mut read_buffer).unwrap();
        assert_eq!(str::from_utf8(&read_buffer).unwrap(), "ping");

        let msg = "pong";
        connection.0.write_all(msg.as_bytes()).unwrap();
    });

    client_thread.join().unwrap();
    server_thread.join().unwrap();
    proxy.kill().unwrap();
}

#[test]
fn test_pong() {
    let (server, mut proxy, proxy_addr) = start_proxy();
    let mut client = TcpStream::connect(proxy_addr).unwrap();

    let client_thread = thread::spawn(move || {
        let mut read_buffer: [u8; 4] = [0; 4];
        client.read_exact(&mut read_buffer).unwrap();

        assert_eq!(str::from_utf8(&read_buffer).unwrap(), "pong");
    });

    let server_thread = thread::spawn(move || {
        let mut connection = server.accept().unwrap();
        let msg = "pong";
        connection.0.write_all(msg.as_bytes()).unwrap();
    });

    client_thread.join().unwrap();
    server_thread.join().unwrap();
    proxy.kill().unwrap();
}

#[test]
fn test_ping() {
    let (server, mut proxy, proxy_addr) = start_proxy();
    let mut client = TcpStream::connect(proxy_addr).unwrap();

    let client_thread = thread::spawn(move || {
        let msg = "ping";
        client.write_all(msg.as_bytes()).unwrap();
    });

    let server_thread = thread::spawn(move || {
        let mut connection = server.accept().unwrap();
        let mut read_buffer: [u8; 4] = [0; 4];
        connection.0.read_exact(&mut read_buffer).unwrap();
        assert_eq!(str::from_utf8(&read_buffer).unwrap(), "ping");
    });

    client_thread.join().unwrap();
    server_thread.join().unwrap();
    proxy.kill().unwrap();
}

#[test]
fn test_large_string_two_way() {
    let (server, mut proxy, proxy_addr) = start_proxy();
    let mut client = TcpStream::connect(proxy_addr).unwrap();

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10000)
        .map(char::from)
        .collect();

    let client_read_string = rand_string.clone();
    let server_write_string = rand_string.clone();
    let sercer_read_string = rand_string.clone();

    let mut client_read = client.try_clone().unwrap();

    let client_write_thread = thread::spawn(move || {
        client_read.write_all(rand_string.as_bytes()).unwrap();
    });

    let client_read_thread = thread::spawn(move || {
        let mut read_buffer: [u8; 10000] = [0; 10000];
        client.read_exact(&mut read_buffer).unwrap();
        assert_eq!(str::from_utf8(&read_buffer).unwrap(), client_read_string);
    });

    let mut server_write = server.accept().unwrap().0;
    let mut server_read = server_write.try_clone().unwrap();

    let server_write_thread = thread::spawn(move || {
        server_write
            .write_all(server_write_string.as_bytes())
            .unwrap();
    });

    let server_read_thread = thread::spawn(move || {
        let mut read_buffer: [u8; 10000] = [0; 10000];
        server_read.read_exact(&mut read_buffer).unwrap();
        assert_eq!(str::from_utf8(&read_buffer).unwrap(), sercer_read_string);
    });

    client_write_thread.join().unwrap();
    client_read_thread.join().unwrap();
    server_write_thread.join().unwrap();
    server_read_thread.join().unwrap();
    proxy.kill().unwrap();
}

#[test]
fn test_two_clients() {
    let (server, mut proxy, proxy_addr) = start_proxy();
    let mut client = TcpStream::connect(&proxy_addr).unwrap();
    let mut client_b = TcpStream::connect(proxy_addr).unwrap();
    let client_a_thread = thread::spawn(move || {
        let msg = "ping";
        client.write_all(msg.as_bytes()).unwrap();
        let mut read_buffer: [u8; 4] = [0; 4];
        client.read_exact(&mut read_buffer).unwrap();

        assert_eq!(str::from_utf8(&read_buffer).unwrap(), "pong");
    });

    let client_b_thread = thread::spawn(move || {
        let msg = "ping";
        client_b.write_all(msg.as_bytes()).unwrap();
        let mut read_buffer: [u8; 4] = [0; 4];
        client_b.read_exact(&mut read_buffer).unwrap();

        assert_eq!(str::from_utf8(&read_buffer).unwrap(), "pong");
    });

    let server_thread = thread::spawn(move || {
        let mut read_buffer: [u8; 4] = [0; 4];

        let mut connection = server.accept().unwrap();
        connection.0.read_exact(&mut read_buffer).unwrap();
        assert_eq!(str::from_utf8(&read_buffer).unwrap(), "ping");

        let mut connection_2 = server.accept().unwrap();
        connection_2.0.read_exact(&mut read_buffer).unwrap();
        assert_eq!(str::from_utf8(&read_buffer).unwrap(), "ping");

        let msg = "pong";
        connection.0.write_all(msg.as_bytes()).unwrap();
        connection_2.0.write_all(msg.as_bytes()).unwrap();
    });

    client_a_thread.join().unwrap();
    client_b_thread.join().unwrap();
    server_thread.join().unwrap();
    proxy.kill().unwrap();
}
