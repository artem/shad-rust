#![forbid(unsafe_code)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    sync::mpsc::{self, Sender},
    thread,
};

use rayon::prelude::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub struct Match {
    pub path: PathBuf,
    pub line: String,
    pub line_number: usize,
}

#[derive(Debug)]
pub struct Error {
    pub path: PathBuf,
    pub error: io::Error,
}

pub enum Event {
    Match(Match),
    Error(Error),
}

pub fn run<P: AsRef<Path>>(path: P, pattern: &str) -> Vec<Event> {
    // TODO: your code here.
    unimplemented!()
}

