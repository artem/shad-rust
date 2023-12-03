use thiserror::Error;

use crate::{Address, Nibble, OpCode, Operation, Word};

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown opcode: {0}")]
    UnknownOpCode(OpCode),
    #[error("operation is not supported: {0:?}")]
    UnsupportedOperation(Operation),
    #[error("stack underflow")]
    StackUnderflow,
    #[error("stack overflow")]
    StackOverflow,
    #[error("invalid key: {0:#04x}")]
    InvalidKey(Word),
    #[error("invalid sprite: address {0}, size {1}")]
    InvalidSprite(Address, Nibble),
    #[error("the interpreter has crashed and is now unrecoverable")]
    Crashed,
}

pub type Result<T> = std::result::Result<T, Error>;
