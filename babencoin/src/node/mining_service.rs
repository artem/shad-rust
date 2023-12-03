use crate::{
    data::{
        Block, BlockAttributes, BlockHash, Transaction, VerifiedBlock, VerifiedTransaction,
        WalletId, MAX_REWARD,
    },
    util::{deserialize_wallet_id, serialize_wallet_id},
};

use anyhow::{Context, Result};
use chrono::Utc;
use futures::{stream, Stream, StreamExt};
use log::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tokio::{
    select,
    sync::mpsc::{Receiver, Sender},
};

use std::{
    pin::pin,
    sync::{
        mpsc::{self, SyncSender},
        Arc, RwLock,
    },
    thread,
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct MiningServiceConfig {
    pub mining_thread_count: usize,
    pub max_tx_per_block: usize,

    #[serde(
        serialize_with = "serialize_wallet_id",
        deserialize_with = "deserialize_wallet_id"
    )]
    pub public_key: WalletId,
}

impl Default for MiningServiceConfig {
    fn default() -> Self {
        Self {
            mining_thread_count: 0,
            max_tx_per_block: 0,
            public_key: WalletId::genesis(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct MiningInfo {
    pub block_index: u64,
    pub prev_hash: BlockHash,
    pub max_hash: BlockHash,
    pub transactions: Vec<VerifiedTransaction>,
}

pub struct MiningService {
    // TODO: your code here.
}

impl MiningService {
    pub fn new(
        config: MiningServiceConfig,
        info_receiver: Receiver<MiningInfo>,
        block_sender: Sender<VerifiedBlock>,
    ) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    pub async fn run(&mut self) -> Result<()> {
        // TODO: your code here.
        unimplemented!()
    }

    fn make_block_stream(
        receiver: mpsc::Receiver<VerifiedBlock>,
    ) -> impl Stream<Item = VerifiedBlock> {
        stream::unfold(receiver, |receiver| async {
            tokio::task::spawn_blocking(move || {
                receiver.recv().ok().map(move |block| (block, receiver))
            })
            .await
            .ok()
            .flatten()
        })
    }

    // TODO: your code here.
}

