use std::{
    sync::{Arc, RwLock},
    thread,
};

use crate::{
    data::{
        Block, BlockAttributes, BlockHash, Transaction, VerifiedBlock, VerifiedTransaction,
        WalletId, MAX_REWARD,
    },
    util::{deserialize_wallet_id, serialize_wallet_id},
};

use anyhow::{Context, Result};
use chrono::Utc;
use crossbeam::channel::{Receiver, Sender};
use crossbeam::{channel, select};
use log::*;
use rand::{thread_rng, Rng};
use rayon::{ThreadPool, ThreadPoolBuilder};
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct MiningServiceConfig {
    pub thread_count: usize,
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
            thread_count: 0,
            max_tx_per_block: 0,
            public_key: WalletId::of_genesis(),
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
    config: MiningServiceConfig,
    info_receiver: Receiver<MiningInfo>,
    block_sender: Sender<VerifiedBlock>,
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

    pub fn run(&mut self) {
        // TODO: your code here.
        unimplemented!()
    }

    // TODO: your code here.
}

