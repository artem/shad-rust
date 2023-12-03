#![allow(dead_code)]

use babencoin::{
    data::{Block, BlockHash, PeerMessage, HASH_LEN},
    node,
};

use anyhow::{bail, Context, Result};
use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;
use rsa::{algorithms::generate_multi_prime_key, RSAPrivateKey, RSAPublicKey};

use std::{
    fs,
    io::{self, ErrorKind, Read, Write},
    net::{SocketAddr, TcpStream},
    path::{Path, PathBuf},
    process::{Child, Command},
    thread,
    time::{Duration, Instant},
};

////////////////////////////////////////////////////////////////////////////////

const NODE_BINARY_PATH: &str = "../target/debug/babencoin";
const TEST_ARTIFACTS_PATH: &str = "./test_artifacts";
const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(3);

////////////////////////////////////////////////////////////////////////////////

macro_rules! test_env {
    ($name:literal) => {
        $crate::helpers::Env::new($name, ::babencoin::node::Config::default())
    };
    ($name:literal, $config:ident) => {
        $crate::helpers::Env::new($name, $config)
    };
}

pub struct Env {
    name: &'static str,
    node: Child,
    addr: SocketAddr,
    log_file_path: PathBuf,
}

impl Drop for Env {
    fn drop(&mut self) {
        self.node.kill().ok();
        if thread::panicking() {
            eprintln!("=== BEGIN LOGS OF TEST '{}' ===", self.name);
            eprintln!("{}", fs::read_to_string(&self.log_file_path).unwrap());
            eprintln!("=== END LOGS OF TEST '{}' ===", self.name);
        }
    }
}

impl Env {
    pub fn new(name: &'static str, mut config: node::Config) -> Self {
        let port = Seeder::from(name)
            .make_rng::<ChaCha20Rng>()
            .gen_range(49152..65436)
            + ChaCha20Rng::from_entropy().gen_range(0..100);
        let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
        config.peer_app.service.listen_address = Some(addr.to_string());

        let dir_suffix = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };
        let dir = Path::new(TEST_ARTIFACTS_PATH).join(format!("{}_{}", name, dir_suffix));
        fs::create_dir_all(&dir).unwrap();

        let config_path = dir.join("config.yaml");
        let mut config_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&config_path)
            .unwrap();
        config_file
            .write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
            .unwrap();
        config_file.flush().unwrap();

        let log_file_path = dir.join("stderr").to_owned();
        let _ = std::fs::remove_file(&log_file_path);
        let log_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&log_file_path)
            .unwrap();

        let node = Command::new(NODE_BINARY_PATH)
            .args(&["-c", config_path.to_str().unwrap()])
            .env("RUST_BACKTRACE", "1")
            .stderr(log_file)
            .spawn()
            .unwrap();

        let env = Self {
            name,
            node,
            addr,
            log_file_path,
        };

        Self::wait_for_liveness(&addr);

        env
    }

    fn wait_for_liveness(addr: &SocketAddr) {
        let interval = Duration::from_millis(100);
        for _ in 0..100 {
            thread::sleep(interval);
            if let Ok(mut conn) = TcpStream::connect_timeout(&addr, interval) {
                sync(&mut conn).unwrap();
                return;
            }
        }
        panic!("failed to wait for node liveness");
    }

    pub fn connect_to_node(&self) -> io::Result<TcpStream> {
        let conn = TcpStream::connect(&self.addr)?;
        conn.set_read_timeout(Some(DEFAULT_READ_TIMEOUT)).unwrap();
        Ok(conn)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn send_message(conn: &mut TcpStream, message: PeerMessage) -> io::Result<()> {
    conn.write_all(serde_json::to_string(&message).unwrap().as_bytes())?;
    conn.write_all(b"\0")
}

pub fn recv_message(conn: &mut TcpStream) -> Result<PeerMessage> {
    fn is_interrupted(res: &io::Result<u8>) -> bool {
        if let Err(err) = res {
            if err.kind() == ErrorKind::Interrupted {
                return true;
            }
        }
        false
    }

    // NB: extremely inefficient, but simple.
    let mut buffer = vec![];
    for mb_byte in conn.bytes() {
        if is_interrupted(&mb_byte) {
            continue;
        }

        let byte = mb_byte.context("failed to read stream")?;
        if byte != 0 {
            buffer.push(byte);
            continue;
        }

        let data_str = std::str::from_utf8(&buffer).context("message is not a valid utf-8")?;
        let msg: PeerMessage =
            serde_json::from_str(data_str).context("failed to deserialize message")?;
        return Ok(msg);
    }
    bail!("stream ended unexpectedly");
}

////////////////////////////////////////////////////////////////////////////////

// Make sure that all the previous messages have been processed by gossip service.
// To do that, we send a random block, request it back and wait for response.
pub fn sync(conn: &mut TcpStream) -> Result<()> {
    let block = random_block(10);

    send_message(conn, PeerMessage::Block(Box::new(block.clone())))?;
    send_message(
        conn,
        PeerMessage::Request {
            block_hash: block.compute_hash(),
        },
    )?;

    wait_for_message(conn, 3, |msg| match msg {
        PeerMessage::Block(recv_block) => **recv_block == block,
        _ => false,
    })?;

    Ok(())
}

pub fn wait_for_message<F: FnMut(&PeerMessage) -> bool>(
    conn: &mut TcpStream,
    wait_time_secs: u64,
    mut pred: F,
) -> Result<PeerMessage> {
    conn.set_read_timeout(Some(Duration::from_secs(wait_time_secs)))
        .unwrap();

    let start_ts = Instant::now();
    loop {
        let mb_msg = recv_message(conn);
        if start_ts.elapsed().as_secs() >= wait_time_secs {
            bail!("expected message is not received");
        }

        let msg = mb_msg.context("failed to receive message")?;
        if pred(&msg) {
            return Ok(msg);
        }
    }
}

// Make sure that gossip service does not have a message in queue
// that matches given predicate.
pub fn ensure_absence<F: FnMut(&PeerMessage) -> bool>(
    conn: &mut TcpStream,
    mut pred: F,
) -> Result<()> {
    let block = random_block(15);

    send_message(conn, PeerMessage::Block(Box::new(block.clone())))?;
    send_message(
        conn,
        PeerMessage::Request {
            block_hash: block.compute_hash(),
        },
    )?;

    loop {
        let msg = recv_message(conn).context("failed to receive message")?;
        if pred(&msg) {
            bail!("received a message that matches predicate");
        }
        if let PeerMessage::Block(recv_block) = msg {
            if *recv_block == block {
                return Ok(());
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn random_hash() -> BlockHash {
    let mut rng = thread_rng();
    let mut hash = [0u8; HASH_LEN];
    for i in 0..hash.len() {
        hash[i] = rng.gen();
    }
    hash
}

pub fn random_block(index: u64) -> Block {
    let mut block = Block::genesis();
    block.prev_hash = random_hash();
    block.index = index;
    block.timestamp = block
        .timestamp
        .checked_add_signed(chrono::Duration::minutes(10 * index as i64))
        .unwrap();
    block
}

pub fn generate_private_key() -> RSAPrivateKey {
    generate_multi_prime_key(&mut thread_rng(), 32, 1024).unwrap()
}

pub fn generate_public_key() -> RSAPublicKey {
    generate_private_key().into()
}
