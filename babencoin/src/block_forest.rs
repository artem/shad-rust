use crate::data::{
    BlockHash, TransactionHash, VerifiedBlock, VerifiedTransaction, WalletId, HASH_LEN,
};

use anyhow::{bail, Context, Result};
use chrono::Duration;
use log::debug;
use num_bigint::BigUint;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
};

////////////////////////////////////////////////////////////////////////////////

pub const EPOCH_SIZE: usize = 16;
pub const TARGET_BLOCK_MINING_TIME_SECONDS: u64 = 10;

////////////////////////////////////////////////////////////////////////////////

pub struct BlockForest {
    head: Arc<VerifiedBlock>,
    blocks: HashMap<BlockHash, Arc<VerifiedBlock>>,
    children_hashes: HashMap<BlockHash, Vec<BlockHash>>,
    bad_block_hashes: HashSet<BlockHash>,
    unknown_block_hashes: HashSet<BlockHash>,
    balance_snapshots: HashMap<BlockHash, HashMap<WalletId, u64>>,
    pending_transactions: HashMap<TransactionHash, VerifiedTransaction>,
    pending_snapshot: HashMap<WalletId, u64>,
}

impl Default for BlockForest {
    fn default() -> Self {
        let genesis = Arc::new(VerifiedBlock::genesis());

        let mut blocks = HashMap::new();
        blocks.insert(*genesis.hash(), genesis.clone());

        let mut balance_snapshots = HashMap::new();
        balance_snapshots.insert(*genesis.hash(), HashMap::new());

        Self {
            head: genesis,
            blocks,
            children_hashes: HashMap::new(),
            bad_block_hashes: HashSet::new(),
            unknown_block_hashes: HashSet::new(),
            balance_snapshots,
            pending_transactions: HashMap::new(),
            pending_snapshot: HashMap::new(),
        }
    }
}

impl BlockForest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn head(&self) -> &Arc<VerifiedBlock> {
        &self.head
    }

    pub fn unknown_block_hashes(&self) -> &HashSet<BlockHash> {
        &self.unknown_block_hashes
    }

    pub fn pending_transactions(&self) -> &HashMap<TransactionHash, VerifiedTransaction> {
        &self.pending_transactions
    }

    pub fn find_block(&self, hash: &BlockHash) -> Option<&Arc<VerifiedBlock>> {
        self.blocks.get(hash)
    }

    pub fn next_max_hash(&self) -> BlockHash {
        let next_index = self.head.index + 1;
        if next_index % EPOCH_SIZE as u64 > 0 {
            return self.head.max_hash;
        };

        let mut prev_epoch = self.get_ancestors(&self.head, EPOCH_SIZE - 1);
        prev_epoch.reverse();
        prev_epoch.push(&self.head);

        assert_eq!(prev_epoch.len(), EPOCH_SIZE);
        self.compute_epoch_max_hash(&prev_epoch)
    }

    pub fn add_block(&mut self, block: VerifiedBlock) -> Result<()> {
        if self.bad_block_hashes.contains(block.hash()) {
            bail!("block {} is known to be bad", base64::encode(block.hash()));
        }
        if self.bad_block_hashes.contains(&block.prev_hash) {
            self.mark_bad_block(block.hash());
            bail!(
                "block {} parent is known to be bad",
                base64::encode(block.hash())
            );
        }

        if self.blocks.contains_key(block.hash()) {
            return Ok(());
        }

        self.unknown_block_hashes.remove(block.hash());

        let block_arc = Arc::new(block.clone());
        self.blocks.insert(*block.hash(), block_arc.clone());
        self.children_hashes
            .entry(block.prev_hash)
            .and_modify(|children| children.push(*block.hash()))
            .or_insert_with(|| vec![*block.hash()]);
        if !self.blocks.contains_key(&block.prev_hash) {
            self.unknown_block_hashes.insert(block.prev_hash);
        }

        self.validate_new_block(&block)?;

        if self.is_block_connected_to_genesis(block.hash()) {
            self.validate_transaction_balances(block.hash())?;

            let head_candidate = self.find_head_candidate(&block_arc);
            if head_candidate.index > self.head.index {
                let new_head = head_candidate.clone();
                self.switch_head_to(new_head);
            }
        }

        Ok(())
    }

    pub fn add_transaction(&mut self, tx: VerifiedTransaction) -> Result<()> {
        if self.pending_transactions.contains_key(tx.hash()) {
            return Ok(());
        }

        Self::try_apply_tx_to_snapshot(&tx, &mut self.pending_snapshot)?;
        self.pending_transactions.insert(*tx.hash(), tx);
        Ok(())
    }

    fn mark_bad_block(&mut self, root_hash: &BlockHash) {
        let root_block = &self.blocks[root_hash];
        if root_block.index > 0 {
            let parent_hash = self.blocks[root_hash].prev_hash;
            self.children_hashes
                .get_mut(&parent_hash)
                .unwrap()
                .retain(|hash| hash != root_hash);
        }

        let mut stack = vec![*root_hash];
        while let Some(hash) = stack.pop() {
            self.blocks.remove(&hash);
            self.bad_block_hashes.insert(hash);
            if let Some(children_hashes) = self.children_hashes.remove(&hash) {
                stack.extend(children_hashes);
            }
        }
    }

    fn validate_new_block(&mut self, block: &VerifiedBlock) -> Result<()> {
        if let Err(err) = self.validate_block(block) {
            self.mark_bad_block(block.hash());
            bail!(
                "block {} context validation failed: {}",
                base64::encode(block.hash()),
                err
            );
        }

        let mut stack = vec![*block.hash()];
        let mut bad_children = vec![];

        // Validate all descendants down to 2 * EPOCH_SIZE generations.
        while let Some(hash) = stack.pop() {
            let children_hashes = match self.children_hashes.get(&hash) {
                Some(h) => h,
                None => continue,
            };

            for child_hash in children_hashes {
                let child_block = &self.blocks[child_hash];
                match self.validate_block(child_block) {
                    Ok(()) => {
                        if child_block.index - block.index < (2 * EPOCH_SIZE) as u64 {
                            stack.push(*child_hash);
                        }
                    }
                    Err(err) => {
                        debug!(
                            "block {} context validation failed: {}",
                            base64::encode(child_hash),
                            err
                        );
                        bad_children.push(*child_hash);
                    }
                }
            }
        }

        for bad_hash in bad_children {
            self.mark_bad_block(&bad_hash);
        }

        Ok(())
    }

    fn validate_block(&self, block: &VerifiedBlock) -> Result<()> {
        if let Some(prev) = self.find_block(&block.prev_hash) {
            let expected_index = prev.index + 1;
            if block.index != expected_index {
                bail!(
                    "wrong block id: expected {}, got {}",
                    expected_index,
                    block.index
                );
            }

            if block.timestamp <= prev.timestamp {
                bail!(
                    "block timestamp <= parent timestamp (block ts: {}, parent ts: {})",
                    block.timestamp,
                    prev.timestamp,
                );
            }

            if block.index % EPOCH_SIZE as u64 > 0 && prev.max_hash != block.max_hash {
                bail!(
                    "wrong max_hash: expected {:?}, got {:?}",
                    prev.max_hash,
                    block.max_hash,
                );
            }
        }

        if let Some(expected_max_hash) = self.compute_max_hash(block) {
            if block.max_hash != expected_max_hash {
                bail!(
                    "wrong max_hash: expected {:?}, got {:?}",
                    expected_max_hash,
                    block.max_hash
                );
            }
        }

        Ok(())
    }

    fn compute_max_hash(&self, block: &VerifiedBlock) -> Option<BlockHash> {
        if block.index % EPOCH_SIZE as u64 > 0 {
            let parent = self.blocks.get(&block.prev_hash)?;
            Some(parent.max_hash)
        } else {
            let mut prev_epoch = self.get_ancestors(block, EPOCH_SIZE);
            if prev_epoch.len() != EPOCH_SIZE {
                return None;
            }
            prev_epoch.reverse();
            Some(self.compute_epoch_max_hash(&prev_epoch))
        }
    }

    fn get_ancestors(&self, block: &VerifiedBlock, limit: usize) -> Vec<&VerifiedBlock> {
        let mut ancestors = Vec::with_capacity(limit);
        let mut hash = block.prev_hash;
        while let Some(ancestor) = self.find_block(&hash) {
            if ancestors.len() == limit {
                break;
            }
            ancestors.push(ancestor as &VerifiedBlock);
            hash = ancestor.prev_hash;
        }
        ancestors
    }

    fn compute_epoch_max_hash(&self, epoch: &[&VerifiedBlock]) -> BlockHash {
        assert_eq!(epoch.len(), EPOCH_SIZE);
        let epoch_id = epoch[0].index / 16;
        assert_eq!(epoch[0].index, epoch_id * EPOCH_SIZE as u64);
        assert_eq!(
            epoch.last().unwrap().index,
            (epoch_id + 1) * EPOCH_SIZE as u64 - 1
        );

        let avg_duration = {
            let mut sum_duration = Duration::zero();
            for (prev, cur) in epoch.iter().zip(epoch.iter().skip(1)) {
                assert_eq!(prev.max_hash, cur.max_hash);

                let delta = cur.timestamp - prev.timestamp;
                assert!(delta > Duration::zero());

                sum_duration = sum_duration
                    .checked_add(&delta)
                    .expect("duration add overflow");
            }
            sum_duration / (epoch.len() - 1) as i32
        };

        let old_max_hash = BigUint::from_bytes_be(&epoch[0].max_hash);
        let factor = (avg_duration.num_seconds() as f64 / TARGET_BLOCK_MINING_TIME_SECONDS as f64)
            .max(0.001)
            .min(1000.);

        let max_hash = if factor > 1. {
            old_max_hash * factor.round() as u64
        } else {
            old_max_hash / (1. / factor).round() as u64
        };

        let bytes = max_hash.to_bytes_be();
        let prefix_size = bytes.len().saturating_sub(HASH_LEN);
        let leading_zeros = HASH_LEN.saturating_sub(bytes.len());

        if bytes.iter().take(prefix_size).any(|b| *b > 0) {
            [255u8; HASH_LEN]
        } else {
            let mut result = [0u8; HASH_LEN];
            for (i, byte) in (leading_zeros..HASH_LEN).zip(bytes.into_iter().skip(prefix_size)) {
                result[i] = byte;
            }
            result
        }
    }

    fn is_block_connected_to_genesis(&self, hash: &BlockHash) -> bool {
        let genesis_hash = *VerifiedBlock::genesis().hash();
        let mut last_hash = *hash;
        while last_hash != genesis_hash {
            if let Some(parent) = self.blocks.get(&last_hash) {
                last_hash = parent.prev_hash;
            } else {
                return false;
            }
        }
        true
    }

    fn validate_transaction_balances(&mut self, hash: &BlockHash) -> Result<()> {
        if self.balance_snapshots.contains_key(hash) {
            return Ok(());
        }

        let mut root_block = &self.blocks[hash];
        while !self.balance_snapshots.contains_key(&root_block.prev_hash) {
            root_block = &self.blocks[&root_block.prev_hash];
        }

        let mut bad_block_hashes = vec![];
        let mut queue: VecDeque<_> = vec![root_block].into();
        'next_block: while let Some(block) = queue.pop_back() {
            let mut snapshot = self.balance_snapshots[&block.prev_hash].clone();

            if let Err(err) = Self::try_apply_issuer_reward_to_snapshot(block, &mut snapshot) {
                debug!(
                    "failed to apply issuer reward: {:#} (block {})",
                    err,
                    base64::encode(block.hash()),
                );
                bad_block_hashes.push(*block.hash());
                continue 'next_block;
            }

            for tx in block.transactions() {
                if let Err(err) = Self::try_apply_tx_to_snapshot(tx, &mut snapshot) {
                    debug!(
                        "failed to apply block transactions: {:#} (block {}, tx {})",
                        err,
                        base64::encode(block.hash()),
                        base64::encode(tx.hash()),
                    );
                    bad_block_hashes.push(*block.hash());
                    continue 'next_block;
                }
            }

            self.balance_snapshots.insert(*block.hash(), snapshot);

            if let Some(children_hashes) = self.children_hashes.get(block.hash()) {
                for child_hash in children_hashes {
                    queue.push_back(&self.blocks[child_hash]);
                }
            }
        }

        for hash in bad_block_hashes.iter() {
            self.mark_bad_block(hash);
        }

        if bad_block_hashes.iter().any(|h| h == hash) {
            bail!("block transactions are invalid");
        }

        Ok(())
    }

    fn find_head_candidate<'a>(&'a self, root: &'a Arc<VerifiedBlock>) -> &'a Arc<VerifiedBlock> {
        let mut stack = vec![root];
        let mut best = root;
        while let Some(block) = stack.pop() {
            let children_hashes = match self.children_hashes.get(block.hash()) {
                Some(h) => h,
                None => continue,
            };

            for child_hash in children_hashes {
                if let Some(child_block) = self.blocks.get(child_hash) {
                    if child_block.index > best.index
                        || child_block.index == best.index && child_block.timestamp < best.timestamp
                    {
                        best = child_block;
                    }
                    stack.push(child_block);
                }
            }
        }
        best
    }

    fn switch_head_to(&mut self, new_head: Arc<VerifiedBlock>) {
        let lca = self.find_lca(&self.head, &new_head);

        let new_branch_tx_hashes: HashSet<_> = self
            .list_transactions(&new_head, lca)
            .into_iter()
            .map(|tx| *tx.hash())
            .collect();

        let old_branch_txs = self.list_transactions(&self.head, lca);

        let mut new_pending_transactions = HashMap::new();
        let mut new_snapshot = self.balance_snapshots.get(new_head.hash()).unwrap().clone();
        for tx in old_branch_txs
            .iter()
            .chain(self.pending_transactions.values())
        {
            if new_branch_tx_hashes.contains(tx.hash())
                || new_pending_transactions.contains_key(tx.hash())
            {
                continue;
            }

            if let Err(err) = Self::try_apply_tx_to_snapshot(tx, &mut new_snapshot) {
                debug!(
                    "discarding transaction {}: {:#}",
                    base64::encode(tx.hash()),
                    err,
                );
            } else {
                new_pending_transactions.insert(*tx.hash(), tx.clone());
            }
        }

        self.head = new_head;
        self.pending_transactions = new_pending_transactions;
        self.pending_snapshot = new_snapshot;
    }

    fn find_lca<'a>(
        &'a self,
        mut first: &'a Arc<VerifiedBlock>,
        mut second: &'a Arc<VerifiedBlock>,
    ) -> &'a Arc<VerifiedBlock> {
        while first.hash() != second.hash() {
            if first.index >= second.index {
                first = &self.blocks[&first.prev_hash];
            }
            if first.index < second.index {
                second = &self.blocks[&second.prev_hash];
            }
        }

        first
    }

    fn try_apply_issuer_reward_to_snapshot(
        block: &VerifiedBlock,
        snapshot: &mut HashMap<WalletId, u64>,
    ) -> Result<()> {
        let mut reward = block.reward;
        for tx in block.transactions() {
            reward = reward
                .checked_add(tx.fee)
                .context("reward + fees overflows u64")?;
        }

        let old_balance = *snapshot.get(&block.issuer).unwrap_or(&0);
        let new_balance = old_balance
            .checked_add(reward)
            .context("issuer balance overflows u64")?;

        if new_balance > 0 {
            snapshot.insert(block.issuer.clone(), new_balance);
        } else {
            snapshot.remove(&block.issuer);
        }

        Ok(())
    }

    fn try_apply_tx_to_snapshot(
        tx: &VerifiedTransaction,
        snapshot: &mut HashMap<WalletId, u64>,
    ) -> Result<()> {
        let old_sender_balance = *snapshot.get(&tx.sender).unwrap_or(&0);
        let new_sender_balance = old_sender_balance
            .checked_sub(tx.amount)
            .and_then(|value| value.checked_sub(tx.fee))
            .context("sender has insufficient funds")?;

        let old_receiver_balance = *snapshot.get(&tx.receiver).unwrap_or(&0);
        let new_receiver_balance = old_receiver_balance
            .checked_add(tx.amount)
            .context("receiver balance overflows u64")?;

        for (key, value) in [
            (tx.sender.clone(), new_sender_balance),
            (tx.receiver.clone(), new_receiver_balance),
        ] {
            if value > 0 {
                snapshot.insert(key, value);
            } else {
                snapshot.remove(&key);
            }
        }

        Ok(())
    }

    fn list_transactions(
        &self,
        inclusive_from: &Arc<VerifiedBlock>,
        exclusive_to: &Arc<VerifiedBlock>,
    ) -> Vec<VerifiedTransaction> {
        let mut transactions = vec![];
        let mut block = inclusive_from;
        while block.hash() != exclusive_to.hash() {
            transactions.extend(block.transactions().iter().cloned());
            block = &self.blocks[&block.prev_hash];
        }
        transactions
    }
}
