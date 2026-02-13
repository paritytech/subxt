//! This example demonstrates setting up `subxt-rpcs` and using the legacy RPC methods
//! for subscribing to Substrate blocks.

use subxt::{config::PolkadotConfig, config::RpcConfigFor};
use subxt_rpcs::{client::RpcClient, methods::LegacyRpcMethods};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an RPC client from a URL. This connects to a Polkadot node.
    // You can replace this with any Substrate-based node URL.
    let rpc_client = RpcClient::from_url("wss://rpc.polkadot.io").await?;

    // Create RPC methods using RpcConfigFor<PolkadotConfig>.
    // This tells the RPC methods to use the same types as PolkadotConfig.
    let rpc = LegacyRpcMethods::<RpcConfigFor<PolkadotConfig>>::new(rpc_client);

    // Now we can make RPC calls! Let's get the chain name:
    let chain_name = rpc.system_chain().await?;
    println!("Connected to chain: {}", chain_name);

    // Get the latest finalized block hash:
    let finalized_hash = rpc.chain_get_finalized_head().await?;
    println!("Latest finalized block hash: {:?}", finalized_hash);

    // Get the block header for that hash:
    let header = rpc.chain_get_header(Some(finalized_hash)).await?;
    if let Some(header) = header {
        println!("Block number: {}", header.number);
        println!("Parent hash: {:?}", header.parent_hash);
    }

    // Get the runtime version:
    let runtime_version = rpc.state_get_runtime_version(None).await?;
    println!("Runtime spec version: {}", runtime_version.spec_version);
    println!(
        "Runtime transaction version: {}",
        runtime_version.transaction_version
    );

    Ok(())
}
