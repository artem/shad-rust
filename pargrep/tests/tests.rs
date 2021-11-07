use tempdir::TempDir;

use std::{
    fs,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
    time::{Duration, Instant},
};

////////////////////////////////////////////////////////////////////////////////

type TreeDesc = &'static [(&'static str, &'static [u8])];

fn make_tree(desc: TreeDesc) -> io::Result<TempDir> {
    let tmp_dir = TempDir::new("pargrep")?;
    for (path_str, data) in desc.iter() {
        if path_str.ends_with("/") {
            let dir_path = tmp_dir.path().join(path_str.strip_suffix("/").unwrap());
            fs::create_dir_all(&dir_path)?;
        } else {
            if let Some(dir_component) = Path::new(path_str).parent() {
                fs::create_dir_all(tmp_dir.path().join(dir_component))?;
            }
            fs::write(tmp_dir.path().join(path_str), data)?;
        }
    }

    Ok(tmp_dir)
}

////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_file() {
    let tmp_dir = TempDir::new("pargrep").unwrap();
    let path = tmp_dir.path().join("sonnet");
    fs::write(
        &path,
        b"From fairest creatures we desire increase,\n\
		That thereby beauty's rose might never die,\n\
		But as the riper should by time decrease,\n\
		His tender heir mught bear his memeory:\n\
		But thou, contracted to thine own bright eyes,\n\
		Feed'st thy light'st flame with self-substantial fuel,\n\
		Making a famine where abundance lies,\n\
		Thyself thy foe, to thy sweet self too cruel.\n\
		Thou that art now the world's fresh ornament\n\
		And only herald to the gaudy spring,\n\
		Within thine own bud buriest thy content\n\
		And, tender churl, makest waste in niggarding.\n\
		Pity the world, or else this glutton be,\n\
		To eat the world's due, by the grave and thee.\n",
    )
    .unwrap();

    let events = pargrep::run(&path, "thy");
    let mut matches = events
        .into_iter()
        .map(|ev| match ev {
            pargrep::Event::Match(m) => m,
            pargrep::Event::Error(err) => panic!("unexpected error: {:?}", err),
        })
        .collect::<Vec<_>>();

    assert_eq!(matches.len(), 3);

    matches.sort_by_key(|m| m.line_number);
    assert_eq!(
        matches,
        vec![
            pargrep::Match {
                path: path.to_path_buf(),
                line: "Feed'st thy light'st flame with self-substantial fuel,".into(),
                line_number: 6,
            },
            pargrep::Match {
                path: path.to_path_buf(),
                line: "Thyself thy foe, to thy sweet self too cruel.".into(),
                line_number: 8,
            },
            pargrep::Match {
                path: path.to_path_buf(),
                line: "Within thine own bud buriest thy content".into(),
                line_number: 11,
            },
        ]
    );
}

#[test]
fn test_tree() {
    let tree_desc: TreeDesc = &[
        (
            "foo/bar/baz/hello",
            b"hello, world!\nlooking for a substring?",
        ),
        ("foo/bar/tmp/", b""),
        (
            "foo/baz/bar/offense",
            b"substring\nhere you are, filthy peasant!",
        ),
        ("foo/baz/bar/empty/", b""),
        ("alpha/beta/gamma/hey", b"hey there! I have a substring"),
        (
            "alpha/beta/gamma/trololo",
            b"trololololololololololo, ohohohoho",
        ),
        (
            "martin/luther/king",
            b"The time is always right to do what is right.\nYour substring is here.",
        ),
    ];

    let tmp_dir = make_tree(tree_desc).unwrap();

    let events = pargrep::run(tmp_dir.path(), "substring");
    let mut matches = events
        .into_iter()
        .map(|ev| match ev {
            pargrep::Event::Match(m) => m,
            pargrep::Event::Error(err) => panic!("unexpected error: {:?}", err),
        })
        .collect::<Vec<_>>();

    assert_eq!(matches.len(), 4);

    matches.sort_by(|m1, m2| m1.path.cmp(&m2.path));
    let expected_matches = &[
        ("alpha/beta/gamma/hey", 1, "hey there! I have a substring"),
        ("foo/bar/baz/hello", 2, "looking for a substring?"),
        ("foo/baz/bar/offense", 1, "substring"),
        ("martin/luther/king", 2, "Your substring is here."),
    ];
    for (expected, actual) in expected_matches.iter().zip(matches) {
        assert!(actual.path.ends_with(expected.0));
        assert_eq!(actual.line_number, expected.1);
        assert_eq!(actual.line, expected.2);
    }
}

#[test]
fn test_error() {
    let path = "/sad/sdg/sdg/j/re/jta/rh/wethw/rt";
    let events = pargrep::run(path, "foo");

    assert_eq!(events.len(), 1);
    match &events[0] {
        pargrep::Event::Match(m) => panic!("unexpected match: {:?}", m),
        pargrep::Event::Error(error) => {
            assert_eq!(error.path.to_str().unwrap(), path);
        }
    }
}

#[test]
#[cfg(not(debug_assertions))]
fn test_performance() {
    let tmp_dir = TempDir::new("pargrep").unwrap();

    for i in 0..16 {
        let path = tmp_dir.path().join(i.to_string());
        write_haystack(&path, 4 * 1024, 1024, 16 << i);
    }

    let mut single_durations = vec![];
    let mut par_durations = vec![];
    for _ in 0..11 {
        single_durations.push(time(|| single_run(tmp_dir.path(), "abacaba")));
        par_durations.push(time(|| pargrep::run(tmp_dir.path(), "abacaba")));
    }

    single_durations.sort();
    par_durations.sort();
    assert!(par_durations[6] < single_durations[6]);
}

#[allow(unused)]
fn time<R, F: FnOnce() -> R>(func: F) -> Duration {
    let now = Instant::now();
    func();
    now.elapsed()
}

#[allow(unused)]
fn write_haystack(path: &Path, lines: usize, line_len: usize, needle_pos: usize) {
    let line = vec!["ab"; line_len / 2].join("");
    let mut writer = BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap(),
    );
    for i in 0..lines {
        if i == needle_pos {
            writer.write(b"abacaba\n").unwrap();
        } else {
            write!(writer, "{}\n", line).unwrap();
        }
    }
    writer.flush().unwrap();
}

#[allow(unused)]
fn single_run(path: &Path, pattern: &str) -> Vec<pargrep::Event> {
    let mut events = vec![];
    for mb_entry in path.read_dir().unwrap() {
        let path = mb_entry.unwrap().path();
        let reader = BufReader::new(fs::File::open(&path).unwrap());
        for (i, mb_line) in reader.lines().enumerate() {
            let line = mb_line.unwrap();
            if line.contains(pattern) {
                events.push(pargrep::Event::Match(pargrep::Match {
                    path: path.clone(),
                    line,
                    line_number: i + 1,
                }));
            }
        }
    }
    events
}
