#![forbid(unsafe_code)]

mod runtime;
mod scheduler;
mod timer;

#[cfg(feature = "net")]
mod network;

////////////////////////////////////////////////////////////////////////////////

pub use rio_macros::test;

pub use runtime::{runtime_id, spawn, Runtime};
pub use timer::sleep;

#[cfg(feature = "net")]
pub use network::UdpSocket;
