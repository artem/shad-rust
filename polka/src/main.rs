#![forbid(unsafe_code)]

use std::io::{stdin, stdout, BufRead, Write};

fn print_values(values: &[polka::Value]) {
    print!("[");
    if let Some(value) = values.first() {
        print!("{}", value);
        for value in &values[1..] {
            print!(", {}", value);
        }
    }
    println!("]");
}

fn main() {
    print!("> ");
    stdout().flush().unwrap();

    let mut inter = polka::Interpreter::new();
    for line in stdin().lock().lines() {
        inter.eval(&line.unwrap());
        print_values(inter.stack());
        print!("> ");
        stdout().flush().unwrap();
    }
}
