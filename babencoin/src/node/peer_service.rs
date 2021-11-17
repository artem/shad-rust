use crate::data::{PeerMessage, VerifiedPeerMessage};

use anyhow::{bail, Context, Result};
use crossbeam::channel::{self, Receiver, Sender};
use log::*;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fmt::{self, Display},
    io::{self, BufReader, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

////////////////////////////////////////////////////////////////////////////////

const BUF_SIZE: usize = 65536;

pub type SessionId = u64;

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Serialize, Deserialize)]
pub struct PeerServiceConfig {
    #[serde(with = "humantime_serde")]
    pub dial_cooldown: Duration,
    pub dial_addresses: Vec<String>,
    pub listen_address: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PeerEvent {
    pub session_id: SessionId,
    pub event_kind: PeerEventKind,
}

#[derive(Debug, Clone)]
pub enum PeerEventKind {
    Connected,
    Disconnected,
    NewMessage(VerifiedPeerMessage),
}

#[derive(Debug, Clone)]
pub struct PeerCommand {
    pub session_id: SessionId,
    pub command_kind: PeerCommandKind,
}

#[derive(Debug, Clone)]
pub enum PeerCommandKind {
    SendMessage(VerifiedPeerMessage),
    Drop,
}

////////////////////////////////////////////////////////////////////////////////

pub struct PeerService {
    config: PeerServiceConfig,
    peer_event_sender: Sender<PeerEvent>,
    command_receiver: Receiver<PeerCommand>,
    // TODO: your code here.
}

impl PeerService {
    pub fn new(
        config: PeerServiceConfig,
        peer_event_sender: Sender<PeerEvent>,
        command_receiver: Receiver<PeerCommand>,
    ) -> Result<Self> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn run(&mut self) {
        // TODO: your code here.
        unimplemented!()
    }

    // TODO: your code here.
}

