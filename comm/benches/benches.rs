use std::{
    io::{self, Write},
    iter,
    process::Command,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{distributions::Alphanumeric, seq::SliceRandom, thread_rng, Rng};
use tempfile::{NamedTempFile, TempPath};

const RUST_BINARY_PATH: &str = "./target/release/comm";
const CPP_BINARY_PATH: &str = "./target/release/comm_cpp";

fn run_comm(path: &str, first: &[String], second: &[String]) -> Vec<String> {
    fn create_tempfile(data: &[String]) -> io::Result<TempPath> {
        let (mut file, path) = NamedTempFile::new()?.into_parts();
        for line in data {
            file.write_all(line.as_bytes())?;
            file.write(b"\n")?;
        }
        file.flush()?;
        Ok(path)
    }

    let first_path = create_tempfile(first).expect("failed to create temp file");
    let second_path = create_tempfile(second).expect("failed to create temp file");
    let output = Command::new(path)
        .args(&[first_path, second_path])
        .output()
        .expect("failed to call comm");

    assert!(output.status.success(), "comm process failed");

    let mut result: Vec<String> = String::from_utf8(output.stdout)
        .expect("comm result is not a valid utf-8")
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    result.pop(); // remove empty string

    result
}

fn generate_input(
    common: usize,
    left_unique: usize,
    right_unique: usize,
) -> (Vec<String>, Vec<String>) {
    fn random_string() -> String {
        thread_rng()
            .sample_iter(Alphanumeric)
            .take(64)
            .map(char::from)
            .collect()
    }

    let common_lines: Vec<_> = iter::repeat_with(random_string).take(common).collect();

    let mut left_lines: Vec<_> = iter::repeat_with(random_string)
        .take(left_unique)
        .chain(common_lines.iter().cloned())
        .collect();
    left_lines.shuffle(&mut thread_rng());

    let mut right_lines: Vec<_> = iter::repeat_with(random_string)
        .take(right_unique)
        .chain(common_lines.into_iter())
        .collect();
    right_lines.shuffle(&mut thread_rng());

    (left_lines, right_lines)
}

fn bench_50k_50k(c: &mut Criterion) {
    let mut group = c.benchmark_group("50k_50k");
    group
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(10));

    let (first, second) = generate_input(50_000, 50_000, 50_000);
    group.bench_function("rust", |b| {
        b.iter(|| black_box(run_comm(RUST_BINARY_PATH, &first, &second)))
    });
    group.bench_function("cpp", |b| {
        b.iter(|| black_box(run_comm(CPP_BINARY_PATH, &first, &second)))
    });
}

fn bench_0_100k(c: &mut Criterion) {
    let mut group = c.benchmark_group("0_100k");
    group
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(10));

    let (first, second) = generate_input(0, 100_000, 100_000);
    group.bench_function("rust", |b| {
        b.iter(|| black_box(run_comm(RUST_BINARY_PATH, &first, &second)))
    });
    group.bench_function("cpp", |b| {
        b.iter(|| black_box(run_comm(CPP_BINARY_PATH, &first, &second)))
    });
}

criterion_group!(benches, bench_50k_50k, bench_0_100k);
criterion_main!(benches);
