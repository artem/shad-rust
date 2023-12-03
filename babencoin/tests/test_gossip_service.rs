#[macro_use]
mod helpers;

use core::time;

use helpers::{
    ensure_absence, generate_private_key, generate_public_key, random_block, send_message, sync,
    wait_for_message,
};

use babencoin::{
    data::{Block, PeerMessage, Transaction, VerifiedBlock, VerifiedTransaction},
    node,
};

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_block_request() {
    let env = test_env!("test_block_request");
    let mut conn = env.connect_to_node().unwrap();

    wait_for_message(&mut conn, 10, |msg| match msg {
        PeerMessage::Block(block) => **block == Block::genesis(),
        _ => false,
    })
    .unwrap();

    send_message(
        &mut conn,
        PeerMessage::Request {
            block_hash: *VerifiedBlock::genesis().hash(),
        },
    )
    .unwrap();

    wait_for_message(&mut conn, 10, |msg| match msg {
        PeerMessage::Block(block) => **block == Block::genesis(),
        _ => false,
    })
    .unwrap();

    let block = random_block(25);
    send_message(&mut conn, PeerMessage::Block(Box::new(block.clone()))).unwrap();

    wait_for_message(&mut conn, 10, |msg| match msg {
        PeerMessage::Request { block_hash } => block_hash == &block.attrs.prev_hash,
        _ => false,
    })
    .unwrap();

    let hash = block.compute_hash();
    send_message(&mut conn, PeerMessage::Request { block_hash: hash }).unwrap();

    wait_for_message(&mut conn, 10, |msg| match msg {
        PeerMessage::Block(recv_block) => **recv_block == block,
        _ => false,
    })
    .unwrap();
}

#[test]
fn test_tx_send() {
    let env = test_env!("test_tx_send");

    let key = generate_private_key();
    let tx =
        VerifiedTransaction::sign(&key, generate_public_key().into(), 0, 0, "Test".into()).unwrap();

    let mut conn_one = env.connect_to_node().unwrap();
    send_message(
        &mut conn_one,
        PeerMessage::Transaction(Box::new(tx.clone().into())),
    )
    .unwrap();
    sync(&mut conn_one).unwrap();
    drop(conn_one);

    let mut conn_two = env.connect_to_node().unwrap();
    wait_for_message(&mut conn_two, 10, |msg| match msg {
        PeerMessage::Transaction(recv_tx) => &recv_tx as &Transaction == &tx as &Transaction,
        _ => false,
    })
    .unwrap();
}

#[test]
fn test_tx_discard() {
    let env = test_env!("test_tx_discard");

    let key = generate_private_key();
    let tx = VerifiedTransaction::sign(&key, generate_public_key().into(), 100, 100, "Test".into())
        .unwrap();

    let mut conn_one = env.connect_to_node().unwrap();
    send_message(
        &mut conn_one,
        PeerMessage::Transaction(Box::new(tx.clone().into())),
    )
    .unwrap();
    sync(&mut conn_one).unwrap();
    drop(conn_one);

    let mut conn_two = env.connect_to_node().unwrap();
    ensure_absence(&mut conn_two, |msg| match msg {
        PeerMessage::Transaction(recv_tx) => &recv_tx as &Transaction == &tx as &Transaction,
        _ => false,
    })
    .unwrap();
}

#[test]
fn test_head_advance() {
    let env = test_env!("test_head_advance");

    let mut block = random_block(1);
    block.attrs.prev_hash = Block::genesis().compute_hash();

    let mut conn_one = env.connect_to_node().unwrap();
    send_message(&mut conn_one, PeerMessage::Block(Box::new(block.clone()))).unwrap();
    send_message(
        &mut conn_one,
        PeerMessage::Request {
            block_hash: block.compute_hash(),
        },
    )
    .unwrap();

    wait_for_message(&mut conn_one, 10, |msg| match msg {
        PeerMessage::Block(recv_block) => **recv_block == block,
        _ => false,
    })
    .unwrap();

    let mut conn_two = env.connect_to_node().unwrap();
    wait_for_message(&mut conn_two, 10, |msg| match msg {
        PeerMessage::Block(recv_block) => **recv_block == block,
        _ => false,
    })
    .unwrap();
}

#[test]
fn test_head_switch() {
    let env = test_env!("test_head_switch");

    let tx_one = VerifiedTransaction::sign(
        &generate_private_key(),
        generate_public_key().into(),
        0,
        0,
        "Test".into(),
    )
    .unwrap();

    let tx_two = VerifiedTransaction::sign(
        &generate_private_key(),
        generate_public_key().into(),
        0,
        0,
        "Test".into(),
    )
    .unwrap();

    let mut block_one = random_block(1);
    block_one.attrs.prev_hash = Block::genesis().compute_hash();
    block_one.transactions.push(tx_one.clone().into());

    let mut block_two = random_block(1);
    block_two.attrs.prev_hash = Block::genesis().compute_hash();
    block_two.transactions.push(tx_two.clone().into());

    let mut conn_one = env.connect_to_node().unwrap();
    send_message(
        &mut conn_one,
        PeerMessage::Transaction(Box::new(tx_one.clone().into())),
    )
    .unwrap();
    send_message(
        &mut conn_one,
        PeerMessage::Transaction(Box::new(tx_two.clone().into())),
    )
    .unwrap();
    send_message(
        &mut conn_one,
        PeerMessage::Block(Box::new(block_one.clone())),
    )
    .unwrap();
    send_message(
        &mut conn_one,
        PeerMessage::Block(Box::new(block_two.clone())),
    )
    .unwrap();
    sync(&mut conn_one).unwrap();

    let mut conn_two = env.connect_to_node().unwrap();
    wait_for_message(&mut conn_two, 10, |msg| match msg {
        PeerMessage::Block(block) => **block == block_one,
        _ => false,
    })
    .unwrap();
    wait_for_message(&mut conn_two, 10, |msg| match msg {
        PeerMessage::Transaction(tx) => &tx as &Transaction == &tx_two as &Transaction,
        _ => false,
    })
    .unwrap();
    drop(conn_two);

    let mut block_three = random_block(2);
    block_three.attrs.prev_hash = block_two.compute_hash();

    send_message(
        &mut conn_one,
        PeerMessage::Block(Box::new(block_three.clone())),
    )
    .unwrap();
    sync(&mut conn_one).unwrap();

    let mut conn_three = env.connect_to_node().unwrap();
    wait_for_message(&mut conn_three, 10, |msg| match msg {
        PeerMessage::Block(block) => **block == block_three,
        _ => false,
    })
    .unwrap();
    wait_for_message(&mut conn_three, 10, |msg| match msg {
        PeerMessage::Transaction(tx) => &tx as &Transaction == &tx_one as &Transaction,
        _ => false,
    })
    .unwrap();
}

#[test]
fn test_no_bad_block_memoization() {
    let env = test_env!("test_no_bad_block_memoization");
    let mut conn = env.connect_to_node().unwrap();

    let ok_block = random_block(25);
    let ok_block_hash = ok_block.compute_hash();

    let mut bad_block = random_block(75);
    bad_block.attrs.prev_hash = ok_block_hash;
    let bad_block_hash = bad_block.compute_hash();

    send_message(&mut conn, PeerMessage::Block(Box::new(bad_block))).unwrap();

    wait_for_message(&mut conn, 10, |msg| match msg {
        PeerMessage::Request { block_hash } => block_hash == &ok_block_hash,
        _ => false,
    })
    .unwrap();

    send_message(&mut conn, PeerMessage::Block(Box::new(ok_block))).unwrap();
    send_message(
        &mut conn,
        PeerMessage::Request {
            block_hash: bad_block_hash,
        },
    )
    .unwrap();

    ensure_absence(&mut conn, |msg| match msg {
        PeerMessage::Block(block) => {
            let hash = block.compute_hash();
            hash == bad_block_hash || hash == ok_block_hash
        }
        _ => false,
    })
    .unwrap();
}

#[test]
fn test_eager_requests() {
    let mut config = node::Config::default();
    config.gossip_app.service.eager_requests_interval = time::Duration::from_millis(200);

    let env = test_env!("test_eager_requests", config);

    let mut block_one = random_block(1);
    block_one.attrs.prev_hash = Block::genesis().compute_hash();

    let mut block_two = random_block(2);
    block_two.attrs.prev_hash = block_one.compute_hash();

    let mut conn_one = env.connect_to_node().unwrap();
    send_message(
        &mut conn_one,
        PeerMessage::Block(Box::new(block_two.clone())),
    )
    .unwrap();
    drop(conn_one);

    let mut conn_two = env.connect_to_node().unwrap();
    for _ in 0..5 {
        wait_for_message(&mut conn_two, 10, |msg| match msg {
            PeerMessage::Request { block_hash } => block_hash == &block_two.attrs.prev_hash,
            _ => false,
        })
        .unwrap();
    }

    send_message(
        &mut conn_two,
        PeerMessage::Block(Box::new(block_one.clone())),
    )
    .unwrap();
    sync(&mut conn_two).unwrap();
    drop(conn_two);

    let mut conn_three = env.connect_to_node().unwrap();
    ensure_absence(&mut conn_three, |msg| match msg {
        PeerMessage::Request { block_hash } => block_hash == &block_two.attrs.prev_hash,
        _ => false,
    })
    .unwrap();
}
