mod gossip_service;
mod mining_service;
mod peer_service;

use gossip_service::{GossipService, GossipServiceConfig};
use mining_service::{MiningService, MiningServiceConfig};
use peer_service::{PeerService, PeerServiceConfig};

use anyhow::{Context, Result};
use crossbeam::channel;
use serde::{Deserialize, Serialize};

use std::thread;

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub peer_service: PeerServiceConfig,
    pub gossip_service: GossipServiceConfig,
    pub mining_service: MiningServiceConfig,
}

pub fn run_forever(config: Config) -> Result<()> {
    let (peer_event_sender, peer_event_receiver) = channel::bounded(1000);
    let (command_sender, command_receiver) = channel::bounded(1000);
    let (block_sender, block_receiver) = channel::bounded(1000);
    let (mining_info_sender, mining_info_receiver) = channel::bounded(1000);

    let mut peer_service =
        PeerService::new(config.peer_service, peer_event_sender, command_receiver)
            .context("failed to create peer service")?;

    let mut gossip_service = GossipService::new(
        config.gossip_service,
        peer_event_receiver,
        command_sender,
        block_receiver,
        mining_info_sender,
    );

    let mut mining_service =
        MiningService::new(config.mining_service, mining_info_receiver, block_sender);

    thread::spawn(move || {
        gossip_service.run();
        panic!("gossip service terminated");
    });

    thread::spawn(move || {
        mining_service.run();
        panic!("mining service terminated");
    });

    peer_service.run();
    panic!("peer service terminated");
}
