#![forbid(unsafe_code)]

pub mod proto {
    tonic::include_proto!("chat_proto");
}

pub mod client;
pub mod common;
pub mod server;

pub use client::Client;
pub use common::*;
pub use server::serve;
