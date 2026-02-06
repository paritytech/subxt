//! This example demonstrates using the newer ChainHead RPC methods
//! for subscribing to blocks.

use subxt::{config::PolkadotConfig, config::RpcConfigFor};
use subxt_rpcs::{client::RpcClient, methods::ChainHeadRpcMethods};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an RPC client
    let rpc_client = RpcClient::from_url("wss://rpc.polkadot.io").await?;

    // Create ChainHead RPC methods using RpcConfigFor<PolkadotConfig>
    let rpc = ChainHeadRpcMethods::<RpcConfigFor<PolkadotConfig>>::new(rpc_client);

    println!("Subscribing to finalized blocks...");

    // Subscribe to finalized blocks using the chainHead API
    let mut block_sub = rpc.chainhead_v1_follow(true).await?;

    // Process the first few events
    let mut count = 0;
    while let Some(event) = block_sub.next().await {
        match event {
            Ok(event) => {
                println!("Received event: {:?}", event);
                count += 1;
                if count >= 3 {
                    println!("Received 3 events, stopping...");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error receiving event: {}", e);
                break;
            }
        }
    }

    Ok(())
}
