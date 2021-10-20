use tempdir::TempDir;

use std::{
    fs, io,
    path::{Component, Path},
};

use fswalk::{Handle, Walker};

////////////////////////////////////////////////////////////////////////////////

type TreeDesc = &'static [(&'static str, &'static [u8])];

fn make_tree(desc: TreeDesc) -> io::Result<TempDir> {
    let tmp_dir = TempDir::new("fswalk")?;
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
fn test_simple() {
    let tree_desc: TreeDesc = &[
        ("foo/bar/baz/hello", b"hello, world!"),
        ("foo/bar/tmp/", b""),
        ("foo/baz/bar/offense", b"here you are, filthy peasant!"),
        ("foo/baz/bar/empty/", b""),
        ("alpha/beta/gamma/hey", b"hey there!"),
        (
            "alpha/beta/gamma/trololo",
            b"trololololololololololo, ohohohoho",
        ),
        ("alpha/beta/binary", &[232, 100, 212, 123, 244, 244, 250]),
        (
            "martin/luther/king",
            b"The time is always right to do what is right.",
        ),
    ];

    let tmp_dir = make_tree(tree_desc).unwrap();

    let mut byte_count = 0;

    {
        let mut walker = Walker::new();

        walker.add_callback(|handle| match handle {
            Handle::Dir(dir_handle) => dir_handle.descend(),
            Handle::File(file_handle) => file_handle.read(),
            Handle::Content { content, file_path } => {
                let file_path_components = file_path.components().collect::<Vec<_>>();
                for (path_str, expected_content) in tree_desc {
                    let desc_components = Path::new(path_str).components().collect::<Vec<_>>();
                    if file_path_components[file_path_components.len() - desc_components.len()..]
                        == desc_components
                    {
                        assert_eq!(expected_content, content);
                        byte_count += content.len();
                        return;
                    }
                }
                panic!("descriptor not found: {}", file_path.to_str().unwrap());
            }
        });

        walker.walk(tmp_dir.path()).unwrap();
    }

    let expected_byte_count = tree_desc.iter().map(|(_, data)| data.len()).sum();
    assert_eq!(byte_count, expected_byte_count);
}

#[test]
fn test_two_handlers() {
    let tree_desc: TreeDesc = &[
        ("aloha/ahaha/aaa", b"hello, world!"),
        ("gamma/bravo", b"bravo"),
        ("beta/alpha", b"alpha"),
        ("beta/tiffany", b"beta_tiffany"),
        ("glee/yikes", b"glee_yikes"),
        ("glee/abra/cadabra", b"glee_abra_cadabra"),
        ("glee/baby/boom", b"glee_baby_boom"),
        ("echo", b"echo"),
    ];

    let tmp_dir = make_tree(tree_desc).unwrap();

    let mut a_count = 0;
    let mut b_count = 0;

    fn make_callback<'a>(
        counter: &'a mut usize,
        forbidden_prefix: &'static str,
    ) -> impl FnMut(&mut Handle) + 'a {
        move |handle: &mut Handle| {
            let path_to_check = match handle {
                Handle::Dir(dir_handle) => dir_handle.path().parent().unwrap().to_owned(),
                Handle::File(file_handle) => file_handle.path().parent().unwrap().to_owned(),
                Handle::Content { file_path, .. } => file_path.to_owned(),
            };
            for comp in path_to_check.components() {
                match comp {
                    Component::Normal(path) => {
                        assert!(!path.to_str().unwrap().starts_with(forbidden_prefix))
                    }
                    _ => (),
                }
            }

            let is_file_ok = |path: &Path| {
                !path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(forbidden_prefix)
            };

            match handle {
                Handle::Dir(dir_handle) => {
                    if is_file_ok(dir_handle.path()) {
                        dir_handle.descend();
                    }
                }
                Handle::File(file_handle) => {
                    if is_file_ok(file_handle.path()) {
                        file_handle.read();
                    }
                }
                Handle::Content { content, .. } => *counter += content.len(),
            }
        }
    }

    {
        let mut walker = Walker::new();
        walker.add_callback(make_callback(&mut a_count, "a"));
        walker.add_callback(make_callback(&mut b_count, "b"));
        walker.walk(tmp_dir.path()).unwrap();
    }

    assert_eq!(a_count, 45);
    assert_eq!(b_count, 44);
}

#[test]
fn test_empty() {
    let tree_desc: TreeDesc = &[("foo/bar/baz", b"hello, world!")];
    let tmp_dir = make_tree(tree_desc).unwrap();

    assert!(Walker::new().walk(tmp_dir).is_ok());
    assert!(Walker::new().walk("sadf/asfdasdfasdf/asdfasdf/sd").is_ok());
}

#[test]
fn test_error() {
    let mut walker = Walker::new();
    walker.add_callback(|_| ());
    assert!(walker.walk("oiuabsas/sapdigu/aspgdh").is_err());
}
