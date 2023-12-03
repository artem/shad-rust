#![forbid(unsafe_code)]

use std::{any::type_name, env, process::exit, str::FromStr};

fn parse_or_exit<T: FromStr>(s: &str) -> T {
    match s.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Failed to parse {} from string '{}'", type_name::<T>(), s);
            exit(1);
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        eprintln!("Expected exactly 3 arguments");
        exit(1);
    }

    let width = parse_or_exit::<usize>(&args[1]);
    let height = parse_or_exit::<usize>(&args[2]);
    let vacancy = parse_or_exit::<f64>(&args[3]);

    let prob = perc::evaluate_probability(width, height, vacancy);
    println!("{}", prob);
}
