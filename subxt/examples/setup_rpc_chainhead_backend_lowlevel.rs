//! Example to utilize the low-level api of ChainHeadBackend rpc backend to subscribe to finalized blocks.

#![allow(missing_docs)]

use futures::StreamExt;
use subxt::backend::chain_head::{ChainHeadBackend, ChainHeadBackendBuilder};
use subxt::backend::rpc::RpcClient;
use subxt::{OnlineClient, PolkadotConfig};
use tokio::spawn;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let rpc = RpcClient::from_url("ws://localhost:9944".to_string()).await?;
    let (backend, mut driver) = ChainHeadBackendBuilder::default().build(rpc.clone());

    // Note: That it is required to poll the driver until it's done (i.e returns None)
    // to ensure that the backend is shutdown properly.
    spawn(async move {
        while let Some(res) = driver.next().await {
            if let Err(err) = res {
                tracing::debug!(target: "subxt", "chainHead backend error={err}");
            }
        }

        tracing::debug!(target: "subxt", "chainHead backend was closed");
    });

    let api = OnlineClient::from_backend::<ChainHeadBackend<PolkadotConfig>>(std::sync::Arc::new(
        backend,
    ))
    .await?;

    let mut blocks_sub = api.blocks().subscribe_finalized().await?.take(100);

    while let Some(block) = blocks_sub.next().await {
        let block = block?;

        let block_number = block.number();
        let block_hash = block.hash();

        println!("Block #{block_number} ({block_hash})");
    }

    Ok(())
}
