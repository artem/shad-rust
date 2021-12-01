#![forbid(unsafe_code)]

use futures::future::select_all;
use linkify::{LinkFinder, LinkKind};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use std::collections::HashSet;

#[derive(Clone, Default)]
pub struct Config {
    pub concurrent_requests: Option<usize>,
}

pub struct Page {
    pub url: String,
    pub body: String,
}

pub struct Crawler {
    config: Config,
}

impl Crawler {
    pub fn new(config: Config) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn run(&mut self, site: String) -> Receiver<Page> {
        // TODO: your code here.
        unimplemented!()
    }

    // TODO: your code here.
}
