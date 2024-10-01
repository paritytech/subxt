//! Example to utilize the ChainHeadBackend to subscribe to finalized blocks.

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

    // Run for at most 100 blocks and print a bunch of information about it.
    //
    // The subscription is automatically re-started when the RPC client has reconnected.
    // You can test that by stopping the polkadot node and restarting it.
    let mut blocks_sub = api.blocks().subscribe_finalized().await?.take(100);

    while let Some(block) = blocks_sub.next().await {
        let block = match block {
            Ok(b) => b,
            Err(e) => {
                // This can only happen on the legacy backend and the unstable backend
                // will handle this internally.
                if e.is_disconnected_will_reconnect() {
                    println!("The RPC connection was lost and we may have missed a few blocks");
                    continue;
                }

                return Err(e.into());
            }
        };

        let block_number = block.number();
        let block_hash = block.hash();

        println!("Block #{block_number} ({block_hash})");
    }

    Ok(())
}
