use std::collections::VecDeque;
use std::io::{self, Write};

use anyhow::{bail, Result};
use byteorder::WriteBytesExt;
use crc::{crc32, Hasher32};

////////////////////////////////////////////////////////////////////////////////

const HISTORY_SIZE: usize = 32768;

pub struct TrackingWriter<T> {
    inner: T,
    // TODO: your code here.
}

impl<T: Write> Write for TrackingWriter<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // TODO: your code here.
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        // TODO: your code here.
        unimplemented!()
    }
}

impl<T: Write> TrackingWriter<T> {
    pub fn new(inner: T) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    /// Write a sequence of `len` bytes written `dist` bytes ago.
    pub fn write_previous(&mut self, dist: usize, len: usize) -> Result<()> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn byte_count(&self) -> usize {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn crc32(&self) -> u32 {
        // TODO: your code here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() -> Result<()> {
        let mut buf: &mut [u8] = &mut [0u8; 10];
        let mut writer = TrackingWriter::new(&mut buf);

        assert_eq!(writer.write(&[1, 2, 3, 4])?, 4);
        assert_eq!(writer.byte_count(), 4);
        assert_eq!(writer.crc32(), 3057449933);

        assert_eq!(writer.write(&[4, 8, 15, 16, 23])?, 5);
        assert_eq!(writer.byte_count(), 9);
        assert_eq!(writer.crc32(), 3948347807);

        assert_eq!(writer.write(&[0, 0, 123])?, 1);
        assert_eq!(writer.byte_count(), 10);
        assert_eq!(writer.crc32(), 2992191065);

        assert_eq!(writer.write(&[42, 124, 234, 27])?, 0);
        assert_eq!(writer.byte_count(), 10);
        assert_eq!(writer.crc32(), 2992191065);

        Ok(())
    }

    #[test]
    fn write_previous() -> Result<()> {
        let mut buf: &mut [u8] = &mut [0u8; 512];
        let mut writer = TrackingWriter::new(&mut buf);

        for i in 0..=255 {
            writer.write_u8(i)?;
        }

        writer.write_previous(192, 128)?;
        assert_eq!(writer.byte_count(), 384);
        assert_eq!(writer.crc32(), 2611529849);

        assert!(writer.write_previous(10000, 20).is_err());
        assert_eq!(writer.byte_count(), 384);
        assert_eq!(writer.crc32(), 2611529849);

        assert!(writer.write_previous(256, 256).is_err());
        assert_eq!(writer.byte_count(), 512);
        assert_eq!(writer.crc32(), 2733545866);

        assert!(writer.write_previous(1, 1).is_err());
        assert_eq!(writer.byte_count(), 512);
        assert_eq!(writer.crc32(), 2733545866);

        Ok(())
    }
}
