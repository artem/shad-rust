use tar::parse_tar;

use pretty_assertions::assert_eq;

////////////////////////////////////////////////////////////////////////////////

const BLOCK_SIZE: usize = 512;

#[derive(Default)]
struct TarBuilder {
    data: Vec<u8>,
}

#[derive(Default)]
struct TarFile<'a> {
    path: &'a [u8],
    mode: u32,
    uid: u32,
    gid: u32,
    data: &'a [u8],
}

impl TarBuilder {
    fn add(self, file: TarFile) -> Self {
        self.write_raw_header(file.path, file.mode, file.uid, file.gid, file.data.len())
            .write_raw_data(file.data)
    }

    fn write_raw_header(
        mut self,
        path: &[u8],
        mode: u32,
        uid: u32,
        gid: u32,
        data_size: usize,
    ) -> Self {
        let mut block = [0; BLOCK_SIZE];

        assert!(path.len() < 100);
        (&mut block[..path.len()]).copy_from_slice(&path);

        let write_number = |slice: &mut [u8], number: u32| {
            let s = format!("{:o}", number);
            assert!(s.len() < slice.len());
            let (begin, end) = (slice.len() - s.len() - 1, slice.len() - 1);
            (&mut slice[..begin]).fill(b'0');
            (&mut slice[begin..end]).copy_from_slice(s.as_bytes());
        };

        write_number(&mut block[100..108], mode);
        write_number(&mut block[108..116], uid);
        write_number(&mut block[116..124], gid);
        write_number(&mut block[124..136], data_size as u32);

        self.data.extend(block);
        self
    }

    fn write_raw_data(mut self, data: &[u8]) -> Self {
        self.data.extend(data);
        self.data.resize(
            BLOCK_SIZE * ((self.data.len() + BLOCK_SIZE - 1) / BLOCK_SIZE),
            0,
        );
        self
    }

    fn build(mut self) -> Vec<u8> {
        self.data.resize(self.data.len() + 2 * BLOCK_SIZE, 0);
        self.data
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_empty_tar() {
    let data = TarBuilder::default().build();
    let files = parse_tar(&data);
    assert_eq!(files.len(), 0);
}

#[test]
fn test_empty_file() {
    let data = TarBuilder::default().add(TarFile::default()).build();

    let files = parse_tar(&data);
    assert_eq!(files.len(), 1);

    let header = &files[0].header;
    assert_eq!(header.name, b"");
    assert_eq!(header.mode, 0);
    assert_eq!(header.uid, 0);
    assert_eq!(header.gid, 0);
    assert_eq!(files[0].data, b"");
}

#[test]
fn test_one_file() {
    let data = TarBuilder::default()
        .add(TarFile {
            path: b"my_dir/my_file",
            mode: 0o644,
            uid: 123,
            gid: 345,
            data: b"hello, world!",
        })
        .build();
    assert_eq!(data.len(), 2048);

    let files = parse_tar(&data);
    assert_eq!(files.len(), 1);

    let header = &files[0].header;
    assert_eq!(header.name, b"my_dir/my_file");
    assert_eq!(header.mode, 0o644);
    assert_eq!(header.uid, 123);
    assert_eq!(header.gid, 345);
    assert_eq!(files[0].data, b"hello, world!");
}

#[test]
fn test_three_files() {
    let data = TarBuilder::default()
        .add(TarFile {
            path: b"foo/one",
            mode: 0o644,
            uid: 123,
            gid: 345,
            data: b"hello, world!",
        })
        .add(TarFile {
            path: b"foo/two",
            mode: 0o755,
            uid: 794,
            gid: 945,
            data: b"A man is but a product of his thoughts. What he thinks he becomes.",
        })
        .add(TarFile {
            path: b"bar/baz",
            mode: 0o123,
            uid: 124252,
            gid: 345354,
            data: b"Education is the most powerful weapon which you can use to change the world.",
        })
        .build();

    let files = parse_tar(&data);
    assert_eq!(files.len(), 3);

    let header = &files[0].header;
    assert_eq!(header.name, b"foo/one");
    assert_eq!(header.mode, 0o644);
    assert_eq!(header.uid, 123);
    assert_eq!(header.gid, 345);
    assert_eq!(files[0].data, b"hello, world!");

    let header = &files[1].header;
    assert_eq!(header.name, b"foo/two");
    assert_eq!(header.mode, 0o755);
    assert_eq!(header.uid, 794);
    assert_eq!(header.gid, 945);
    assert_eq!(
        files[1].data,
        b"A man is but a product of his thoughts. What he thinks he becomes."
    );

    let header = &files[2].header;
    assert_eq!(header.name, b"bar/baz");
    assert_eq!(header.mode, 0o123);
    assert_eq!(header.uid, 124252);
    assert_eq!(header.gid, 345354);
    assert_eq!(
        files[2].data,
        b"Education is the most powerful weapon which you can use to change the world."
    );
}

#[test]
fn test_non_utf8() {
    let data = TarBuilder::default()
        .add(TarFile {
            path: b"Hello \xF0\x90\x80World",
            data: b"\0\0\0\0",
            ..Default::default()
        })
        .build();

    let files = parse_tar(&data);
    assert_eq!(files.len(), 1);

    let header = &files[0].header;
    assert_eq!(header.name, b"Hello \xF0\x90\x80World");
    assert_eq!(files[0].data, b"\0\0\0\0");
}

#[test]
fn test_big_file() {
    let file = vec![b'1'; 10 * 1024 * 1024];
    let data = TarBuilder::default()
        .add(TarFile {
            path: b"file/big",
            data: &file,
            ..Default::default()
        })
        .add(TarFile {
            path: b"file/small",
            data: b"just a small file",
            ..Default::default()
        })
        .build();

    let files = parse_tar(&data);
    assert_eq!(files.len(), 2);

    let header = &files[0].header;
    assert_eq!(header.name, b"file/big");
    assert_eq!(files[0].data, file);

    let header = &files[1].header;
    assert_eq!(header.name, b"file/small");
    assert_eq!(files[1].data, b"just a small file");
}

#[test]
#[should_panic]
fn test_eof_header() {
    let data = [0u8; 1023];
    parse_tar(&data);
}

#[test]
#[should_panic]
fn test_eof_data() {
    let data = TarBuilder::default()
        .write_raw_header(b"example", 0, 0, 0, 256000)
        .write_raw_data(b"I AM HUGE!!!!")
        .build();
    parse_tar(&data);
}

#[test]
#[should_panic]
fn test_path_null_terminator() {
    let mut data = TarBuilder::default()
        .add(TarFile {
            path: b"",
            data: b"my precious data",
            ..Default::default()
        })
        .build();
    (&mut data[..100]).copy_from_slice(&vec![b'a', 100]);
    parse_tar(&data);
}

#[test]
fn test_invalid_size() {
    for raw_size in [
        b"512343\xF0\x90\x8032" as &[u8],
        b"\xd0\xbfp\xd0\xb8\xd1\x84\xd0\xba\xd0\xb8",
        b"12345678901",
        b"123456712345", // overwrite null terminator
    ] {
        let mut data = TarBuilder::default()
            .add(TarFile {
                path: b"example",
                data: b"foobar",
                ..Default::default()
            })
            .build();
        (&mut data[124..124 + raw_size.len()]).copy_from_slice(raw_size);
        let result = std::panic::catch_unwind(|| parse_tar(&data));
        assert!(result.is_err(), "should panic, case {:?}", raw_size);
    }
}
