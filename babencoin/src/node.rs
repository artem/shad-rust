mod gossip_service;
mod mining_service;
mod peer_service;

use gossip_service::{GossipService, GossipServiceConfig};
use log::error;
use mining_service::{MiningService, MiningServiceConfig};
use peer_service::{PeerService, PeerServiceConfig};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tokio::{
    select,
    sync::{mpsc::channel, oneshot},
    task::JoinHandle,
};

use std::future::Future;

////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub peer_app: AppConfig<PeerServiceConfig>,
    pub gossip_app: AppConfig<GossipServiceConfig>,
    pub mining_app: AppConfig<MiningServiceConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            peer_app: AppConfig::<PeerServiceConfig> {
                thread_count: 2,
                service: Default::default(),
            },
            gossip_app: AppConfig::<GossipServiceConfig> {
                thread_count: 1,
                service: Default::default(),
            },
            mining_app: AppConfig::<MiningServiceConfig> {
                thread_count: 1,
                service: Default::default(),
            },
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct AppConfig<T> {
    pub thread_count: usize,
    pub service: T,
}

pub async fn run(config: Config) -> Result<()> {
    let (peer_event_sender, peer_event_receiver) = channel(1000);
    let (command_sender, command_receiver) = channel(1000);
    let (block_sender, block_receiver) = channel(1000);
    let (mining_info_sender, mining_info_receiver) = channel(1000);

    let mut peer_service =
        PeerService::new(config.peer_app.service, peer_event_sender, command_receiver);
    let mut peer_service_handle = start_runtime(config.peer_app.thread_count, async move {
        peer_service.run().await
    });

    let mut gossip_service = GossipService::new(
        config.gossip_app.service,
        peer_event_receiver,
        command_sender,
        block_receiver,
        mining_info_sender,
    );
    let mut gossip_service_handle = start_runtime(config.gossip_app.thread_count, async move {
        gossip_service.run().await
    });

    let mut mining_service = MiningService::new(
        config.mining_app.service,
        mining_info_receiver,
        block_sender,
    );
    let mut mining_service_handle = start_runtime(config.mining_app.thread_count, async move {
        mining_service.run().await
    });

    select! {
        result = &mut peer_service_handle => {
            error!("peer service terminated: {:?}", result);
        }
        result = &mut gossip_service_handle => {
            error!("gossip service terminated: {:?}", result);
        }
        result = &mut mining_service_handle => {
            error!("mining service terminated: {:?}", result);
        }
    }

    let handles = [
        peer_service_handle,
        gossip_service_handle,
        mining_service_handle,
    ];
    for handle in handles.iter() {
        handle.abort();
    }

    bail!("node terminated");
}

fn start_runtime<F>(thread_count: usize, future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send,
{
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(thread_count)
        .build()
        .expect("failed to build a Tokio runtime");
    let (sender, receiver) = oneshot::channel();
    let handle = runtime.spawn(async move {
        let res = future.await;
        let _ = sender.send(());
        res
    });
    std::thread::spawn(move || runtime.block_on(receiver));
    handle
}
