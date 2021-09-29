#![forbid(unsafe_code)]

use std::io::{copy, BufRead, Read, Write};

use anyhow::{bail, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use bit_reader::BitReader;
use deflate::{CompressionType, DeflateReader};
use gzip::{CompressionMethod, GzipReader};
use huffman_coding::{decode_litlen_distance_trees, DistanceToken, HuffmanCoding, LitLenToken};
use log::*;
use tracking_writer::TrackingWriter;

mod bit_reader;
mod deflate;
mod gzip;
mod huffman_coding;
mod tracking_writer;

////////////////////////////////////////////////////////////////////////////////

pub fn compress<R: BufRead, W: Write>(_input: R, _output: W) -> Result<()> {
    // NB: you are not required to implement compression.
    // But if you do, you can get extra score for it :)
    unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////

pub fn decompress<R: BufRead, W: Write>(input: R, mut output: W) -> Result<()> {
    // TODO: your code here.
    unimplemented!()
}

