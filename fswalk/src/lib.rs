#![forbid(unsafe_code)]

use std::{fs, io, path::Path};

////////////////////////////////////////////////////////////////////////////////

type Callback<'a> = dyn FnMut(&mut Handle) + 'a;

#[derive(Default)]
pub struct Walker<'a> {
    callbacks: Vec<Box<Callback<'a>>>,
}

impl<'a> Walker<'a> {
    pub fn new() -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Handle) + 'a,
    {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn walk<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        // TODO: your code here.
        unimplemented!()
    }

    // TODO: your code here.
}

////////////////////////////////////////////////////////////////////////////////

pub enum Handle<'a> {
    Dir(DirHandle<'a>),
    File(FileHandle<'a>),
    Content {
        file_path: &'a Path,
        content: &'a [u8],
    },
}

pub struct DirHandle<'a> {
    path: &'a Path,
    // TODO: your code here.
}

impl<'a> DirHandle<'a> {
    pub fn descend(&mut self) {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn path(&self) -> &Path {
        // TODO: your code here.
        unimplemented!()
    }
}

pub struct FileHandle<'a> {
    path: &'a Path,
    // TODO: your code here.
}

impl<'a> FileHandle<'a> {
    pub fn read(&mut self) {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn path(&self) -> &Path {
        // TODO: your code here.
        unimplemented!()
    }
}
