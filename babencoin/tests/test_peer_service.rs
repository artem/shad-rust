#[macro_use]
mod helpers;

use helpers::send_message;

use babencoin::{
    data::{Block, PeerMessage, Transaction, VerifiedBlock, VerifiedTransaction, MAX_REWARD},
    node,
    util::parse_pkcs8_private,
};

use std::{
    io::{Read, Write},
    net::TcpListener,
    thread::sleep,
    time::Duration,
};

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_simple() {
    let env = test_env!("test_simple");
    let mut conn = env.connect_to_node().unwrap();

    send_message(&mut conn, PeerMessage::Block(Box::new(Block::genesis()))).unwrap();

    let genesis_key = Block::genesis().attrs.issuer;
    let priv_key = parse_pkcs8_private(include_str!("../data/test.pem")).unwrap();
    let tx = VerifiedTransaction::sign(&priv_key, genesis_key, 100, 10, "comment".into()).unwrap();
    send_message(&mut conn, PeerMessage::Transaction(Box::new(tx.into()))).unwrap();

    send_message(
        &mut conn,
        PeerMessage::Request {
            block_hash: *VerifiedBlock::genesis().hash(),
        },
    )
    .unwrap();

    // Check that connection is still alive.
    // NB: for some reason, this check may hang forever (even though there is a socket timeout set),
    // so disable it for now.
    /*
        let mut data = vec![];
        let error_kind = conn.read_to_end(&mut data).unwrap_err().kind();
        assert!(matches!(
            error_kind,
            ErrorKind::WouldBlock | ErrorKind::TimedOut
        ));
    */
}

#[test]
fn test_invalid_messages() {
    let invalid_block = {
        let mut block = Block::genesis();
        block.attrs.index = 10;
        block.attrs.reward = MAX_REWARD + 1;
        block
    };

    let genesis_key = Block::genesis().attrs.issuer;

    let invalid_tx = Transaction {
        amount: 1000,
        fee: 30,
        comment: "foo".into(),
        sender: genesis_key.clone(),
        receiver: genesis_key,
        signature: vec![0; 64],
    };

    let cases: &[(&str, String)] = &[
        ("invalid_json", "{\"index\": 10]".into()),
        ("invalid_message", "{\"hello\": \"world\"}".into()),
        (
            "invalid_block",
            serde_json::to_string(&invalid_block).unwrap(),
        ),
        ("invalid_tx", serde_json::to_string(&invalid_tx).unwrap()),
    ];

    let env = test_env!("test_invalid_messages");
    for (name, data) in cases {
        let mut conn = env.connect_to_node().unwrap();

        conn.write_all(&data.as_bytes()).unwrap();
        conn.write_all(b"\0").unwrap();
        let mut buf = vec![];

        if conn.read_to_end(&mut buf).is_err() {
            panic!("node didn't drop connection in case '{}'", name);
        }
    }
}

#[test]
fn test_huge_message() {
    let env = test_env!("test_huge_message");
    let mut conn = env.connect_to_node().unwrap();

    for i in 0..10 {
        if let Err(_) = conn.write_all(&vec![b'{'; 65536]) {
            break;
        }
        if i == 9 {
            panic!("node didn't drop connection");
        }
        sleep(Duration::from_millis(100));
    }
}

#[test]
fn test_dial() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();

    let mut config = node::Config::default();
    config.peer_app.service.dial_addresses = vec![listener.local_addr().unwrap().to_string()];
    let _env = test_env!("test_dial", config);

    for _ in 0..3 {
        listener.accept().unwrap();
    }
}
