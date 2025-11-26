//! Example to utilize the ChainHeadBackend rpc backend to subscribe to finalized blocks.

#![allow(missing_docs)]

use futures::StreamExt;
use subxt::backend::chain_head::{ChainHeadBackend, ChainHeadBackendBuilder};
use subxt::backend::rpc::RpcClient;
use subxt::{OnlineClient, PolkadotConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let rpc = RpcClient::from_url("ws://localhost:9944".to_string()).await?;
    let backend: ChainHeadBackend<PolkadotConfig> =
        ChainHeadBackendBuilder::default().build_with_background_driver(rpc.clone());
    let api = OnlineClient::from_backend(std::sync::Arc::new(backend)).await?;

    let mut blocks_sub = api.blocks().subscribe_finalized().await?.take(100);

    while let Some(block) = blocks_sub.next().await {
        let block = block?;

        let block_number = block.number();
        let block_hash = block.hash();

        println!("Block #{block_number} ({block_hash})");
    }

    Ok(())
}
