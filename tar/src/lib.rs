#![forbid(unsafe_code)]

pub struct TarFile<'a> {
    pub header: TarHeader<'a>,
    pub data: &'a [u8],
}

pub struct TarHeader<'a> {
    pub name: &'a [u8],
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub data_size: usize,
}

pub fn parse_tar(mut tar: &[u8]) -> Vec<TarFile> {
    // TODO: your code here.
    unimplemented!()
}

