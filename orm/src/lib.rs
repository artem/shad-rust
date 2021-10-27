#![forbid(unsafe_code)]

mod connection;
mod error;
mod transaction;

pub mod data;
pub mod object;
pub mod storage;

pub use connection::Connection;
pub use data::ObjectId;
pub use error::{Error, Result};
pub use object::Object;
pub use transaction::{ObjectState, Transaction, Tx};

pub use orm_derive::Object;
