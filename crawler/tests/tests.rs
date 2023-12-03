use crawler::{Config, Crawler, Page};

use log::debug;
use rand::{thread_rng, Rng};
use test_log::test;
use tokio::{runtime, time::sleep};

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

////////////////////////////////////////////////////////////////////////////////

async fn run_server(desc: &[(&str, &str)]) -> u16 {
    let port = thread_rng().gen_range(49152..=65535);
    let mut app = tide::new();
    for (url, body) in desc {
        app.at(&url).get({
            let body = str::replace(&body, "$port", &port.to_string());
            move |_| {
                let body = body.clone();
                async { Ok(body) }
            }
        });
    }
    run_app(app, port).await;
    port
}

async fn run_app(app: tide::Server<()>, port: u16) {
    thread::spawn(move || {
        let runtime = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime
            .block_on(app.listen(format!("127.0.0.1:{}", port)))
            .unwrap();
    });
    wait_server_up(format!("http://localhost:{}/", port)).await;
}

async fn wait_server_up(address: impl AsRef<str>) {
    for i in 0..30 {
        let res_req = reqwest::get(address.as_ref()).await;
        match res_req {
            Ok(_) => break,
            Err(err) => {
                debug!("wait request failed: {}", err);
                if i == 29 {
                    panic!("failed to wait for server readiness");
                }
                sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    }
    debug!("server is ready");
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

#[test(tokio::test)]
async fn test_simple() {
    let port = run_server(&[("/", "Hello, world!")]).await;
    let mut cr = Crawler::new(Config::default());
    let pages = recv_all(cr.run(format!("http://localhost:{}/", port))).await;

    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].url, format!("http://localhost:{}/", port));
    assert_eq!(pages[0].body, "Hello, world!");
}

#[test(tokio::test)]
async fn test_circular() {
    let port = run_server(&[
        ("/", "Hello, world!\n Go here: http://localhost:$port/foo"),
        ("/foo", "Foo!\n Go here: http://localhost:$port/"),
    ])
    .await;

    let mut cr = Crawler::new(Config::default());
    let pages = recv_all(cr.run(format!("http://localhost:{}/", port))).await;

    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].url, format!("http://localhost:{}/", port));
    assert_eq!(
        pages[0].body,
        format!("Hello, world!\n Go here: http://localhost:{}/foo", port)
    );
    assert_eq!(pages[1].url, format!("http://localhost:{}/foo", port));
    assert_eq!(
        pages[1].body,
        format!("Foo!\n Go here: http://localhost:{}/", port)
    );
}

#[test(tokio::test)]
async fn test_repeated() {
    let port = run_server(&[
        (
            "/",
            "Hi!\n Go here:
        http://localhost:$port/foo
        http://localhost:$port/foo
        http://localhost:$port/foo
        http://localhost:$port/foo",
        ),
        ("/foo", "Foo!\n"),
    ])
    .await;

    let mut cr = Crawler::new(Config::default());
    let pages = recv_all(cr.run(format!("http://localhost:{}/", port))).await;

    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].url, format!("http://localhost:{}/", port));
    assert_eq!(pages[1].url, format!("http://localhost:{}/foo", port));
}

#[test(tokio::test)]
async fn test_repeated_concurrently() {
    let port = run_server(&[
        (
            "/",
            "Hi!\n Go here:
        http://localhost:$port/foo
        http://localhost:$port/foo
        http://localhost:$port/foo
        http://localhost:$port/foo
        http://localhost:$port/foo
        http://localhost:$port/foo",
        ),
        ("/foo", "Foo!\n"),
    ])
    .await;

    let mut cr = Crawler::new(Config {
        concurrent_requests: Some(4),
    });
    let pages = recv_all(cr.run(format!("http://localhost:{}/", port))).await;

    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].url, format!("http://localhost:{}/", port));
    assert_eq!(pages[1].url, format!("http://localhost:{}/foo", port));
}

#[test(tokio::test)]
async fn test_concurrently_same_link() {
    let port = run_server(&[
        (
            "/",
            "Hi!\n Go here:
        http://localhost:$port/foo1
        http://localhost:$port/foo2
        http://localhost:$port/foo3
        http://localhost:$port/foo4",
        ),
        ("/foo1", "go to foo: http://localhost:$port/foo"),
        ("/foo2", "go to foo: http://localhost:$port/foo"),
        ("/foo3", "go to foo: http://localhost:$port/foo"),
        ("/foo4", "go to foo: http://localhost:$port/foo"),
        ("/foo", "Foo!\n"),
    ])
    .await;

    let mut cr = Crawler::new(Config {
        concurrent_requests: Some(4),
    });
    let pages = recv_all(cr.run(format!("http://localhost:{}/", port))).await;

    assert_eq!(pages.len(), 6);
    assert_eq!(pages[0].url, format!("http://localhost:{}/", port));
    assert!(pages
        .iter()
        .find(|page| page.url == format!("http://localhost:{}/foo", port))
        .is_some());
}

#[test(tokio::test)]
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
                sleep(std::time::Duration::from_millis(500)).await;
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

    run_app(app, port).await;

    let mut cr = Crawler::new(Config {
        concurrent_requests: Some(2),
    });
    let pages = recv_all(cr.run(format!("http://localhost:{}/", port))).await;
    assert_eq!(max_concurrency_counter.load(Ordering::SeqCst), 2);

    assert_eq!(pages.len(), 4);
}

#[test(tokio::test)]
async fn test_concurrency_1() {
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
                sleep(std::time::Duration::from_millis(500)).await;
                concurrency_counter.fetch_sub(1, Ordering::SeqCst);
                Ok(body)
            }
        }
    };

    let mut app = tide::new();
    app.at("/").get(make_handler(format!(
        "your link:
        * http://localhost:{0}/0",
        port
    )));

    app.at("/0").get(make_handler(format!(
        "Here are your links:
        * http://localhost:{0}/1
        * http://localhost:{0}/2
        * http://localhost:{0}/3",
        port
    )));
    app.at("/1").get(make_handler("Page #1".to_string()));
    app.at("/2").get(make_handler("Page #2".to_string()));
    app.at("/3").get(make_handler("Page #3".to_string()));

    run_app(app, port).await;

    let mut cr = Crawler::new(Config {
        concurrent_requests: Some(3),
    });
    let pages = recv_all(cr.run(format!("http://localhost:{}/", port))).await;
    assert_eq!(max_concurrency_counter.load(Ordering::SeqCst), 3);

    assert_eq!(pages.len(), 5);
}

#[test(tokio::test)]
async fn test_concurrency_2() {
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
                sleep(std::time::Duration::from_millis(500)).await;
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
    app.at("/3").get(make_handler(format!(
        "Page #3 with link:
        * http://localhost:{0}/3/0",
        port
    )));
    app.at("/3/0").get(make_handler(format!(
        "Page #3/0 with link:
        * http://localhost:{0}/3/00",
        port
    )));
    app.at("/3/00").get(make_handler(format!(
        "More links:
        * http://localhost:{0}/3/00/4
        * http://localhost:{0}/3/00/5
        * http://localhost:{0}/3/00/6
        * http://localhost:{0}/3/00/7
        * http://localhost:{0}/3/00/8",
        port
    )));
    app.at("/3/4").get(make_handler("Page #4".to_string()));
    app.at("/3/5").get(make_handler("Page #5".to_string()));
    app.at("/3/6").get(make_handler("Page #6".to_string()));
    app.at("/3/7").get(make_handler("Page #7".to_string()));
    app.at("/3/8").get(make_handler("Page #8".to_string()));

    run_app(app, port).await;

    let mut cr = Crawler::new(Config {
        concurrent_requests: Some(2),
    });
    let pages = recv_all(cr.run(format!("http://localhost:{}/", port))).await;
    assert_eq!(max_concurrency_counter.load(Ordering::SeqCst), 2);

    assert_eq!(pages.len(), 11);
}
