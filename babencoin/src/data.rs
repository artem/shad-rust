use crate::util::{
    deserialize_base64, deserialize_base64_fixed, deserialize_utc, deserialize_wallet_id,
    parse_pkcs8_public, serialize_base64, serialize_utc, serialize_wallet_id,
};

use anyhow::{bail, Context, Result};
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::{DateTime, TimeZone, Utc};
use rsa::{padding::PaddingScheme, PublicKey, PublicKeyParts, RSAPrivateKey, RSAPublicKey};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};

use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

////////////////////////////////////////////////////////////////////////////////

pub const GENESIS_TIMESTAMP: i64 = 1626002428;
pub const MAX_REWARD: u64 = 1000;
pub const HASH_LEN: usize = 64;

pub type BlockHash = [u8; HASH_LEN];
pub type TransactionHash = [u8; HASH_LEN];

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Eq)]
pub struct WalletId {
    pub public_key: RSAPublicKey,
}

impl WalletId {
    pub fn genesis() -> Self {
        parse_pkcs8_public(include_str!("../data/genesis.crt"))
            .unwrap()
            .into()
    }
}

impl Hash for WalletId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.public_key.n().hash(state);
        self.public_key.e().hash(state);
    }
}

impl PartialEq for WalletId {
    fn eq(&self, other: &Self) -> bool {
        self.public_key.n() == other.public_key.n() && self.public_key.e() == other.public_key.e()
    }
}

impl From<RSAPublicKey> for WalletId {
    fn from(public_key: RSAPublicKey) -> Self {
        Self { public_key }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
pub enum PeerMessage {
    Block(Box<Block>),
    Transaction(Box<Transaction>),
    Request {
        #[serde(
            serialize_with = "serialize_base64",
            deserialize_with = "deserialize_base64_fixed::<'_, _, HASH_LEN>"
        )]
        block_hash: BlockHash,
    },
}

impl PeerMessage {
    pub fn verified(self) -> Result<VerifiedPeerMessage> {
        match self {
            Self::Block(block) => Ok(VerifiedPeerMessage::Block(Box::new(block.verified()?))),
            Self::Transaction(tx) => Ok(VerifiedPeerMessage::Transaction(Box::new(tx.verified()?))),
            Self::Request { block_hash } => Ok(VerifiedPeerMessage::Request { block_hash }),
        }
    }
}

impl From<VerifiedPeerMessage> for PeerMessage {
    fn from(other: VerifiedPeerMessage) -> Self {
        match other {
            VerifiedPeerMessage::Block(block) => PeerMessage::Block(Box::new((*block).into())),
            VerifiedPeerMessage::Transaction(tx) => {
                PeerMessage::Transaction(Box::new((*tx).into()))
            }
            VerifiedPeerMessage::Request { block_hash } => PeerMessage::Request { block_hash },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum VerifiedPeerMessage {
    Block(Box<VerifiedBlock>),
    Transaction(Box<VerifiedTransaction>),
    Request { block_hash: BlockHash },
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockAttributes {
    pub index: u64,
    pub reward: u64,
    pub nonce: u64,

    #[serde(serialize_with = "serialize_utc", deserialize_with = "deserialize_utc")]
    pub timestamp: DateTime<Utc>,

    #[serde(
        serialize_with = "serialize_wallet_id",
        deserialize_with = "deserialize_wallet_id"
    )]
    pub issuer: WalletId,

    #[serde(
        serialize_with = "serialize_base64",
        deserialize_with = "deserialize_base64_fixed::<'_, _, HASH_LEN>"
    )]
    pub max_hash: BlockHash,

    #[serde(
        serialize_with = "serialize_base64",
        deserialize_with = "deserialize_base64_fixed::<'_, _, HASH_LEN>"
    )]
    pub prev_hash: BlockHash,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    #[serde(flatten)]
    pub attrs: BlockAttributes,
    pub transactions: Vec<Transaction>,
}

impl Deref for Block {
    type Target = BlockAttributes;

    fn deref(&self) -> &Self::Target {
        &self.attrs
    }
}

impl DerefMut for Block {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attrs
    }
}

impl Block {
    pub fn genesis() -> Block {
        Block {
            attrs: BlockAttributes {
                index: 0,
                timestamp: Utc.timestamp_opt(GENESIS_TIMESTAMP, 0).unwrap(),
                reward: 0,
                nonce: 0,
                issuer: WalletId::genesis(),
                max_hash: [255u8; HASH_LEN],
                prev_hash: [0u8; HASH_LEN],
            },
            transactions: vec![],
        }
    }

    pub fn compute_hash(&self) -> BlockHash {
        Self::compute_hash_inner(
            &self.attrs,
            self.transactions.iter().map(|tx| tx.compute_hash()),
        )
    }

    pub fn verified(self) -> Result<VerifiedBlock> {
        if self.timestamp.timestamp() < GENESIS_TIMESTAMP {
            bail!("block timestamp is less than genesis timestamp");
        }
        if self.timestamp > Utc::now() {
            bail!("block timestamp is greater than now");
        }
        if self.reward > MAX_REWARD {
            bail!("block reward is greater than max reward");
        }
        if self.index == 0 && self != Self::genesis() {
            bail!("block index is 0, but not the genesis block");
        }
        if self.index == 1 && self.prev_hash != VerifiedBlock::genesis().hash {
            bail!("block index is 1, but prev_hash != genesis");
        }

        let mut transactions = Vec::with_capacity(self.transactions.len());
        for tx in self.transactions.into_iter() {
            transactions.push(tx.verified().context("transaction verification failed")?);
        }

        let hash = Self::compute_hash_inner(&self.attrs, transactions.iter().map(|tx| *tx.hash()));
        if hash > self.attrs.max_hash {
            bail!("block hash is greater than max_hash");
        }

        Ok(VerifiedBlock {
            attrs: self.attrs,
            transactions,
            hash,
        })
    }

    fn compute_hash_inner(
        attrs: &BlockAttributes,
        transaction_hashes: impl IntoIterator<Item = TransactionHash>,
    ) -> BlockHash {
        let mut hasher = Sha3_512::new();
        hasher.write_u64::<LittleEndian>(attrs.index).unwrap();
        hasher
            .write_i64::<LittleEndian>(attrs.timestamp.timestamp())
            .unwrap();
        hasher.write_u64::<LittleEndian>(attrs.reward).unwrap();
        hasher.write_u64::<LittleEndian>(attrs.nonce).unwrap();
        hasher.update(attrs.issuer.public_key.n().to_bytes_le());
        hasher.update(attrs.issuer.public_key.e().to_bytes_le());
        hasher.update(attrs.max_hash);
        hasher.update(attrs.prev_hash);
        for tx_hash in transaction_hashes.into_iter() {
            hasher.update(tx_hash);
        }

        let digest = hasher.finalize();
        assert_eq!(digest.len(), HASH_LEN);

        let mut hash = [0u8; HASH_LEN];
        hash.copy_from_slice(&digest);
        hash
    }
}

impl From<VerifiedBlock> for Block {
    fn from(other: VerifiedBlock) -> Self {
        Self {
            attrs: other.attrs,
            transactions: other.transactions.into_iter().map(|tx| tx.into()).collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedBlock {
    attrs: BlockAttributes,
    transactions: Vec<VerifiedTransaction>,
    hash: BlockHash,
}

impl Deref for VerifiedBlock {
    type Target = BlockAttributes;

    fn deref(&self) -> &Self::Target {
        &self.attrs
    }
}

impl VerifiedBlock {
    pub fn genesis() -> VerifiedBlock {
        Block::genesis().verified().unwrap()
    }

    pub fn hash(&self) -> &BlockHash {
        &self.hash
    }

    pub fn transactions(&self) -> &[VerifiedTransaction] {
        &self.transactions
    }

    pub fn to_block(&self) -> Block {
        Block {
            attrs: self.attrs.clone(),
            transactions: self
                .transactions
                .iter()
                .map(|tx| (tx as &Transaction).clone())
                .collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub amount: u64,
    pub fee: u64,
    pub comment: String,

    #[serde(
        serialize_with = "serialize_wallet_id",
        deserialize_with = "deserialize_wallet_id"
    )]
    pub sender: WalletId,

    #[serde(
        serialize_with = "serialize_wallet_id",
        deserialize_with = "deserialize_wallet_id"
    )]
    pub receiver: WalletId,

    #[serde(
        serialize_with = "serialize_base64",
        deserialize_with = "deserialize_base64"
    )]
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn verified(self) -> Result<VerifiedTransaction> {
        let hash = self.compute_hash();

        self.sender.public_key.verify(
            PaddingScheme::PKCS1v15Sign { hash: None },
            &hash,
            &self.signature,
        )?;

        Ok(VerifiedTransaction { inner: self, hash })
    }

    pub fn compute_hash(&self) -> TransactionHash {
        let mut hasher = Sha3_512::new();
        hasher.write_u64::<LittleEndian>(self.amount).unwrap();
        hasher.write_u64::<LittleEndian>(self.fee).unwrap();
        hasher.update(self.comment.as_bytes());
        hasher.update(self.sender.public_key.n().to_bytes_le());
        hasher.update(self.sender.public_key.e().to_bytes_le());
        hasher.update(self.receiver.public_key.n().to_bytes_le());
        hasher.update(self.receiver.public_key.e().to_bytes_le());

        let digest = hasher.finalize();
        assert_eq!(digest.len(), HASH_LEN);

        let mut hash = [0u8; HASH_LEN];
        for (i, &byte) in digest.iter().enumerate() {
            hash[i] = byte;
        }

        hash
    }
}

impl From<VerifiedTransaction> for Transaction {
    fn from(other: VerifiedTransaction) -> Self {
        other.inner
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedTransaction {
    inner: Transaction,
    hash: TransactionHash,
}

impl Deref for VerifiedTransaction {
    type Target = Transaction;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl VerifiedTransaction {
    pub fn hash(&self) -> &TransactionHash {
        &self.hash
    }

    pub fn sign(
        sender: &RSAPrivateKey,
        receiver: WalletId,
        amount: u64,
        fee: u64,
        comment: String,
    ) -> Result<VerifiedTransaction> {
        let mut transaction = Transaction {
            sender: sender.to_public_key().into(),
            signature: vec![],
            receiver,
            amount,
            fee,
            comment,
        };

        let hash = transaction.compute_hash();
        transaction.signature = sender.sign(PaddingScheme::PKCS1v15Sign { hash: None }, &hash)?;

        Ok(VerifiedTransaction {
            inner: transaction,
            hash,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse_pkcs8_private;

    #[test]
    fn test_genesis() {
        VerifiedBlock::genesis();
        Block::genesis().verified().unwrap();
    }

    #[test]
    fn test_transaction_sign() {
        let priv_key = parse_pkcs8_private(include_str!("../data/test.pem")).unwrap();
        let genesis_key = Block::genesis().issuer.clone();
        let tx = VerifiedTransaction::sign(&priv_key, genesis_key, 100, 5, "ping".into()).unwrap();
        (&tx as &Transaction).clone().verified().unwrap();
    }

    #[test]
    fn test_block_json() {
        let block: Block = serde_json::from_str(include_str!("../data/test_block.json")).unwrap();
        let verified = block.verified().unwrap();

        let priv_key = parse_pkcs8_private(include_str!("../data/test.pem")).unwrap();
        let genesis = VerifiedBlock::genesis();

        assert_eq!(
            verified,
            Block {
                attrs: BlockAttributes {
                    index: 1,
                    reward: MAX_REWARD,
                    nonce: 27532,
                    timestamp: Utc.timestamp_opt(1626003028, 0).unwrap(),
                    issuer: priv_key.to_public_key().into(),
                    max_hash: [255u8; HASH_LEN],
                    prev_hash: *genesis.hash(),
                },
                transactions: vec![VerifiedTransaction::sign(
                    &priv_key,
                    genesis.issuer.clone(),
                    500,
                    30,
                    "hi".into()
                )
                .unwrap()
                .into(),],
            }
            .verified()
            .unwrap()
        );
    }
}
