//! This example shows how to define a custom RpcConfig for chains
//! that might have different types than the standard configuration.
//!
//! For most Substrate-based chains, you can use `RpcConfigFor<PolkadotConfig>`
//! as shown in the legacy_rpcs example. This example shows how to create
//! a completely custom configuration if needed.

#![allow(missing_docs)]

use subxt_rpcs::{RpcConfig, client::RpcClient, methods::LegacyRpcMethods};

// Define a custom config for your chain
pub enum MyCustomConfig {}

impl RpcConfig for MyCustomConfig {
    // For this example, we'll use the same types as Polkadot.
    // In a real custom chain, you might have different types.
    type Header = subxt::config::substrate::SubstrateHeader<subxt::config::substrate::H256>;
    type Hash = subxt::config::substrate::H256;
    type AccountId = subxt::config::substrate::AccountId32;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an RPC client
    let rpc_client = RpcClient::from_url("wss://rpc.polkadot.io").await?;

    // Use our custom config
    let rpc = LegacyRpcMethods::<MyCustomConfig>::new(rpc_client);

    // Make RPC calls as usual
    let chain_name = rpc.system_chain().await?;
    println!("Connected to chain: {}", chain_name);

    let finalized_hash = rpc.chain_get_finalized_head().await?;
    println!("Latest finalized block: {:?}", finalized_hash);

    // Get a header using our custom header type
    let header = rpc.chain_get_header(Some(finalized_hash)).await?;
    if let Some(header) = header {
        println!("Block number: {}", header.number);
        println!("Parent hash: {:?}", header.parent_hash);
    }

    println!("\nThis example demonstrates that you can define your own RpcConfig");
    println!("to match the types used by your specific blockchain.");
    println!("However, for most Substrate chains, using RpcConfigFor<PolkadotConfig> is simpler!");

    Ok(())
}
