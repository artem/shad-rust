use std::{
    collections::HashSet,
    io::{self, Write},
    process::Command,
};

use pretty_assertions::assert_eq;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use tempfile::{NamedTempFile, TempPath};

const BINARY_PATH: &str = if cfg!(debug_assertions) {
    "../target/debug/comm"
} else {
    "../target/release/comm"
};

fn run_comm(first: &[&str], second: &[&str]) -> Vec<String> {
    fn create_tempfile(data: &[&str]) -> io::Result<TempPath> {
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
    let output = Command::new(BINARY_PATH)
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

fn check(first: &[&str], second: &[&str], expected_output: &[&str]) {
    let mut output = run_comm(first, second);
    output.sort();
    let mut expected: Vec<_> = expected_output.iter().map(|s| s.to_string()).collect();
    expected.sort();

    if output != expected {
        eprintln!(">>> FIRST FILE:");
        for line in first {
            eprintln!("{}", line)
        }
        eprintln!(">>> SECOND FILE:");
        for line in second {
            eprintln!("{}", line)
        }

        assert_eq!(output, expected, "wrong answer (expected green, got red)");
    }
}

#[test]
fn test_simple() {
    check(&["foo"], &["foo"], &["foo"]);
    check(&["foo", "bar"], &["bar", "baz"], &["bar"]);
    check(
        &["apple", "orange", "potato"],
        &["pear", "orange", "banana"],
        &["orange"],
    );
    check(&[], &[], &[]);
    check(&[""], &[""], &[""]);
    check(&["", ""], &["", ""], &[""]);
}

#[test]
fn test_random() {
    fn make_random_lines(rng: &mut StdRng) -> Vec<&'static str> {
        const TOKENS: &[&str] = &[
            "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India",
            "Juliett", "Kilo", "Lima", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo",
            "Sierra", "Tango", "Uniform", "Victor", "Whiskey", "X-ray", "Yankee", "Zulu",
        ];

        let mut lines = vec![];
        for _ in 0..rng.gen_range(0..TOKENS.len()) {
            lines.push(*TOKENS.choose(rng).unwrap());
        }
        lines.shuffle(rng);
        lines
    }

    let mut rng = StdRng::seed_from_u64(13254252323);
    for _ in 0..1000 {
        let first = make_random_lines(&mut rng);
        let second = make_random_lines(&mut rng);
        let answer: Vec<&str> = first
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
            .intersection(&second.iter().cloned().collect())
            .cloned()
            .collect();
        check(&first, &second, &answer);
    }
}
