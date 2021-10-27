use crate::{storage::StorageTransaction, Result, Transaction};

use std::path::Path;

////////////////////////////////////////////////////////////////////////////////

trait StorageConnection {
    fn new_transaction(&mut self) -> Result<Box<dyn StorageTransaction + '_>>;
}

impl StorageConnection for rusqlite::Connection {
    fn new_transaction(&mut self) -> Result<Box<dyn StorageTransaction + '_>> {
        Ok(Box::new(self.transaction()?))
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Connection {
    inner: Box<dyn StorageConnection>,
}

impl Connection {
    pub fn open_sqlite_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            inner: Box::new(rusqlite::Connection::open(path)?),
        })
    }

    pub fn open_in_memory() -> Result<Self> {
        Ok(Self {
            inner: Box::new(rusqlite::Connection::open_in_memory()?),
        })
    }

    pub fn new_transaction(&mut self) -> Result<Transaction<'_>> {
        Ok(Transaction::new(self.inner.new_transaction()?))
    }
}
