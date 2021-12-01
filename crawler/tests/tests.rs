use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crawler::{Config, Crawler, Page};

use rand::{thread_rng, Rng};

////////////////////////////////////////////////////////////////////////////////

async fn run_server(desc: &[(&str, &str)]) -> ServerHandle {
    let port = thread_rng().gen_range(49152..=65535);

    let mut app = tide::new();
    for (url, body) in desc {
        app.at(url).get({
            let body = str::replace(body, "$port", &port.to_string());
            move |_| {
                let body = body.clone();
                async { Ok(body) }
            }
        });
    }

    let handle = tokio::spawn(app.listen(format!("127.0.0.1:{}", port)));

    for i in 0..30 {
        let res_req = reqwest::get(format!("http://127.0.0.1:{}{}", port, desc[0].0)).await;
        match res_req {
            Ok(_) => break,
            Err(_) => {
                if i == 29 {
                    panic!("failed to wait for server readiness");
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    ServerHandle { port, handle }
}

struct ServerHandle {
    port: u16,
    handle: tokio::task::JoinHandle<std::io::Result<()>>,
}

impl Drop for ServerHandle {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

////////////////////////////////////////////////////////////////////////////////

async fn recv_all(mut receiver: tokio::sync::mpsc::Receiver<Page>) -> Vec<Page> {
    let mut pages = vec![];
    while let Some(page) = receiver.recv().await {
        pages.push(page);
    }
    pages
}

////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn test_simple() {
    let server = run_server(&[("/", "Hello, world!")]).await;
    let mut cr = Crawler::new(Config::default());
    let pages = recv_all(cr.run(format!("http://localhost:{}/", server.port))).await;

    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].url, format!("http://localhost:{}/", server.port));
    assert_eq!(pages[0].body, "Hello, world!");
}

#[tokio::test]
async fn test_circular() {
    let server = run_server(&[
        ("/", "Hello, world!\n Go here: http://localhost:$port/foo"),
        ("/foo", "Foo!\n Go here: http://localhost:$port/"),
    ])
    .await;

    let mut cr = Crawler::new(Config::default());
    let pages = recv_all(cr.run(format!("http://localhost:{}/", server.port))).await;

    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].url, format!("http://localhost:{}/", server.port));
    assert_eq!(
        pages[0].body,
        format!(
            "Hello, world!\n Go here: http://localhost:{}/foo",
            server.port
        )
    );
    assert_eq!(
        pages[1].url,
        format!("http://localhost:{}/foo", server.port)
    );
    assert_eq!(
        pages[1].body,
        format!("Foo!\n Go here: http://localhost:{}/", server.port)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrency() {
    let port = thread_rng().gen_range(49152..=65535);

    let concurrency_counter = Arc::new(AtomicUsize::new(0));
    let max_concurrency_counter = Arc::new(AtomicUsize::new(0));

    let make_handler = |body: String| {
        let concurrency_counter = concurrency_counter.clone();
        let max_concurrency_counter = max_concurrency_counter.clone();
        move |_| {
            let body = body.clone();
            let concurrency_counter = concurrency_counter.clone();
            let max_concurrency_counter = max_concurrency_counter.clone();
            async move {
                let last = concurrency_counter.fetch_add(1, Ordering::SeqCst);
                max_concurrency_counter.fetch_max(last + 1, Ordering::SeqCst);
                std::thread::sleep(std::time::Duration::from_millis(500));
                concurrency_counter.fetch_sub(1, Ordering::SeqCst);
                Ok(body)
            }
        }
    };

    let mut app = tide::new();
    app.at("/").get(make_handler(format!(
        "Here are your links:
        * http://localhost:{0}/1
        * http://localhost:{0}/2
        * http://localhost:{0}/3",
        port
    )));
    app.at("/1").get(make_handler("Page #1".to_string()));
    app.at("/2").get(make_handler("Page #2".to_string()));
    app.at("/3").get(make_handler("Page #3".to_string()));

    let handle = tokio::spawn(app.listen(format!("127.0.0.1:{}", port)));
    let _server_handle = ServerHandle { port, handle };

    let mut cr = Crawler::new(Config {
        concurrent_requests: Some(2),
    });
    recv_all(cr.run(format!("http://localhost:{}/", port))).await;
    assert_eq!(max_concurrency_counter.load(Ordering::SeqCst), 2);
}
