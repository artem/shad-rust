use std::{
    io::{self, BufWriter, Write},
    iter,
    process::Command,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{distributions::Alphanumeric, seq::SliceRandom, thread_rng, Rng};
use tempfile::{NamedTempFile, TempPath};

const RUST_BINARY_PATH: &str = "../target/release/comm";
const CPP_BINARY_PATH: &str = "../target/release/comm_cpp";

fn create_tempfile(data: &[String]) -> io::Result<TempPath> {
    let (file, path) = NamedTempFile::new()?.into_parts();
    let mut writer = BufWriter::new(file);
    for line in data {
        writer.write_all(line.as_bytes())?;
        writer.write(b"\n")?;
    }
    writer.flush()?;
    Ok(path)
}

fn create_tempfiles(first: &[String], second: &[String]) -> io::Result<(TempPath, TempPath)> {
    Ok((create_tempfile(first)?, create_tempfile(second)?))
}

fn run_comm(path: &str, first: &TempPath, second: &TempPath) {
    let output = Command::new(path)
        .args(&[first, second])
        .output()
        .expect("failed to call comm");

    assert!(output.status.success(), "comm process failed");
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
    let (first_path, second_path) =
        create_tempfiles(&first, &second).expect("failed to create tempfiles");

    group.bench_function("rust", |b| {
        b.iter(|| black_box(run_comm(RUST_BINARY_PATH, &first_path, &second_path)))
    });
    group.bench_function("cpp", |b| {
        b.iter(|| black_box(run_comm(CPP_BINARY_PATH, &first_path, &second_path)))
    });
}

fn bench_0_100k(c: &mut Criterion) {
    let mut group = c.benchmark_group("0_100k");
    group
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(10));

    let (first, second) = generate_input(0, 100_000, 100_000);
    let (first_path, second_path) =
        create_tempfiles(&first, &second).expect("failed to create tempfiles");

    group.bench_function("rust", |b| {
        b.iter(|| black_box(run_comm(RUST_BINARY_PATH, &first_path, &second_path)))
    });
    group.bench_function("cpp", |b| {
        b.iter(|| black_box(run_comm(CPP_BINARY_PATH, &first_path, &second_path)))
    });
}

criterion_group!(benches, bench_50k_50k, bench_0_100k);
criterion_main!(benches);
