#![forbid(unsafe_code)]

mod network;
mod runtime;
mod scheduler;
mod timer;

pub use rio_macros::test;

pub use network::UdpSocket;
pub use runtime::{spawn, Runtime};
pub use timer::sleep;
