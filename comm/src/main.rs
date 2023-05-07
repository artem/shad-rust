#![forbid(unsafe_code)]

use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_lines(args: &Vec<String>) -> HashSet<String> {
    let mut lines = HashSet::new();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.insert(line.unwrap());
    }
    lines
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut lines = read_lines(&args);

    let file = File::open(&args[2]).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_uw = &line.unwrap();
        if lines.remove(line_uw) {
            println!("{}", line_uw);
        }
    }
}
