#[macro_use]
mod helpers;

use std::collections::HashSet;

use helpers::{
    generate_private_key, generate_public_key, recv_message, send_message, wait_for_message,
};

use babencoin::{
    block_forest::{BlockForest, EPOCH_SIZE, TARGET_BLOCK_MINING_TIME_SECONDS},
    data::{Block, BlockAttributes, PeerMessage, VerifiedTransaction, HASH_LEN},
    node,
};

use chrono::Duration;

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_genesis_mining() {
    let mut config = node::Config::default();
    config.mining_app.service.mining_thread_count = 1;
    config.mining_app.service.public_key = generate_public_key().into();

    let env = test_env!("test_genesis_mining", config);
    let mut conn = env.connect_to_node().unwrap();

    let mut block_forest = BlockForest::new();

    let msg = wait_for_message(&mut conn, 10, |msg| match msg {
        PeerMessage::Block(block) => block.index >= 5 as u64,
        _ => false,
    })
    .unwrap();

    let mut last_block = match msg {
        PeerMessage::Block(block) => block.verified().unwrap(),
        _ => unreachable!(),
    };
    block_forest.add_block(last_block.clone()).unwrap();

    while last_block.index > 0 {
        send_message(
            &mut conn,
            PeerMessage::Request {
                block_hash: last_block.prev_hash,
            },
        )
        .unwrap();

        wait_for_message(&mut conn, 10, |msg| match msg {
            PeerMessage::Block(block) => {
                let verified = block.clone().verified().unwrap();
                if verified.hash() == &last_block.prev_hash {
                    block_forest.add_block(verified.clone()).unwrap();
                    last_block = verified;
                    true
                } else {
                    false
                }
            }
            _ => false,
        })
        .unwrap();
    }
}

#[test]
fn test_no_repeated_parents() {
    let mut config = node::Config::default();
    config.mining_app.service.mining_thread_count = 1;
    config.mining_app.service.public_key = generate_public_key().into();

    let env = test_env!("test_no_repeated_parents", config);
    let mut conn = env.connect_to_node().unwrap();

    let mut seen_prev_hashes = HashSet::new();
    while seen_prev_hashes.len() < 5 {
        match recv_message(&mut conn).unwrap() {
            PeerMessage::Block(block) => assert!(seen_prev_hashes.insert(block.prev_hash)),
            _ => (),
        }
    }
}

#[test]
fn test_mining_difficulty() {
    let mut blocks = vec![Block::genesis()];
    for i in 1..3 * EPOCH_SIZE {
        let prev_block = blocks.last().unwrap().clone();
        let time_delta_seconds = if i < 2 * EPOCH_SIZE {
            TARGET_BLOCK_MINING_TIME_SECONDS
        } else {
            TARGET_BLOCK_MINING_TIME_SECONDS / 2
        };

        blocks.push(Block {
            attrs: BlockAttributes {
                index: i as u64,
                reward: 0,
                nonce: 0,
                timestamp: prev_block
                    .timestamp
                    .checked_add_signed(Duration::seconds(time_delta_seconds as i64))
                    .unwrap(),
                issuer: generate_public_key().into(),
                max_hash: [255; HASH_LEN],
                prev_hash: prev_block.compute_hash(),
            },
            transactions: vec![],
        });
    }

    let mut config = node::Config::default();
    config.mining_app.service.mining_thread_count = 1;
    config.mining_app.service.public_key = generate_public_key().into();

    let env = test_env!("test_minig_difficulty", config);
    let mut conn = env.connect_to_node().unwrap();

    for block in blocks.iter().skip(1).rev() {
        send_message(&mut conn, PeerMessage::Block(Box::new(block.clone()))).unwrap();
    }

    let expected_prev_hash = blocks.last().unwrap().compute_hash();
    let mut expected_max_hash = [255; HASH_LEN];
    expected_max_hash[0] = 127;

    wait_for_message(&mut conn, 15, |msg| match msg {
        PeerMessage::Block(block) => {
            if block.prev_hash == expected_prev_hash {
                assert_eq!(block.index, blocks.len() as u64);
                assert_eq!(block.max_hash, expected_max_hash);
                block.clone().verified().unwrap();
                true
            } else {
                false
            }
        }
        _ => false,
    })
    .unwrap();
}

#[test]
fn test_max_tx_per_block() {
    let transactions = (0..4)
        .map(|i| {
            VerifiedTransaction::sign(
                &generate_private_key(),
                generate_public_key().into(),
                0,
                0,
                format!("tx #{}", i),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    let mut config = node::Config::default();
    config.mining_app.service.mining_thread_count = 1;
    config.mining_app.service.max_tx_per_block = 2;
    config.mining_app.service.public_key = generate_public_key().into();

    let env = test_env!("test_max_tx_per_block", config);
    let mut conn = env.connect_to_node().unwrap();

    for tx in transactions.iter() {
        send_message(
            &mut conn,
            PeerMessage::Transaction(Box::new(tx.clone().into())),
        )
        .unwrap();
    }

    let mut received_transactions = vec![];
    wait_for_message(&mut conn, 15, |msg| match msg {
        PeerMessage::Block(block) => {
            let verified = block.clone().verified().unwrap();
            assert!(verified.transactions().len() <= 2);
            received_transactions.extend(verified.transactions().iter().cloned());
            received_transactions.len() >= transactions.len()
        }
        _ => false,
    })
    .unwrap();

    received_transactions.sort_by_key(|tx| tx.comment.clone());

    // NB: for easier debugging.
    assert_eq!(transactions.len(), received_transactions.len());
    for (expected_tx, got_tx) in transactions.iter().zip(received_transactions.iter()) {
        assert_eq!(expected_tx, got_tx);
    }
}
