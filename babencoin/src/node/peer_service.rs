use crate::data::{PeerMessage, VerifiedPeerMessage};

use anyhow::{bail, Context, Result};
use futures::{
    stream::{self, FuturesUnordered},
    FutureExt, Stream, StreamExt,
};
use log::*;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{
        tcp::{ReadHalf, WriteHalf},
        TcpListener, TcpStream,
    },
    select,
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};

use std::{
    collections::HashMap,
    pin::pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
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
    // TODO: your code here.
}

impl PeerService {
    pub fn new(
        config: PeerServiceConfig,
        peer_event_sender: Sender<PeerEvent>,
        command_receiver: Receiver<PeerCommand>,
    ) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    pub async fn run(&mut self) -> Result<()> {
        // TODO: your code here.
        unimplemented!()
    }

    // TODO: your code here.
}

////////////////////////////////////////////////////////////////////////////////

struct MessageReader<'a> {
    inner: ReadHalf<'a>,
    buffer: Box<[u8; BUF_SIZE]>,
    len: usize,
}

impl<'a> MessageReader<'a> {
    fn new(inner: ReadHalf<'a>) -> Self {
        Self {
            inner,
            buffer: Box::new([0u8; BUF_SIZE]),
            len: 0,
        }
    }

    fn into_stream(self) -> impl Stream<Item = Result<VerifiedPeerMessage>> + 'a {
        stream::unfold(self, |mut reader| async {
            match reader.next_message().await {
                Ok(msg) => Some((Ok(msg), reader)),
                Err(err) => Some((Err(err), reader)),
            }
        })
    }

    async fn next_message(&mut self) -> Result<VerifiedPeerMessage> {
        if let Some(msg) = self.try_parse_message()? {
            return Ok(msg);
        }
        while self.len < BUF_SIZE {
            let bytes_read = self.inner.read(&mut self.buffer[self.len..]).await?;
            if bytes_read == 0 {
                bail!("peer has disconnected");
            }
            self.len += bytes_read;
            if let Some(msg) = self.try_parse_message()? {
                return Ok(msg);
            }
        }
        bail!("message is larger than {} bytes", BUF_SIZE);
    }

    fn try_parse_message(&mut self) -> Result<Option<VerifiedPeerMessage>> {
        let Some(zero_pos) = self.buffer[..self.len].iter().position(|b| *b == 0) else {
            return Ok(None);
        };
        let data_str = std::str::from_utf8(&self.buffer[..zero_pos])
            .context("message is not a valid utf-8")?;
        let msg: PeerMessage =
            serde_json::from_str(data_str).context("failed to deserialize message")?;
        let verified_msg = msg.verified().context("message verification failed")?;

        self.buffer[..self.len].rotate_left(zero_pos + 1);
        self.len -= zero_pos + 1;

        Ok(Some(verified_msg))
    }
}
