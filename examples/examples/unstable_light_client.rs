//! This is a comprehensive example that utilizes the Light Client
//! for a subset of subxt requests.
//!
//! Includes:
//!  - Subscribes to all finalized blocks using the old RPC method
//!  - Subscribes to the head of the chain using the new `chainHead` RPC method
//!  - Dynamically query constants
//!  - Dynamically decode the events of the latest block
//!  - Various RPC calls to ensure proper shape of the response.
//!
//! # Note
//!
//! This feature is experimental and things might break without notice.

use futures::StreamExt;
use std::sync::Arc;
use subxt::{
    rpc::{types::FollowEvent, LightClient},
    OnlineClient, PolkadotConfig,
};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a light client from the provided chain spec.
    // Note: this connects to the live polkadot chain.
    let light_client = LightClient::new(include_str!("../../artifacts/polkadot_spec.json"))?;
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(Arc::new(light_client)).await?;

    // Subscribe to the latest 3 finalized blocks.
    {
        println!("Subscribe to latest finalized blocks: ");

        let mut blocks_sub = api.blocks().subscribe_finalized().await?.take(3);
        // For each block, print a bunch of information about it:
        while let Some(block) = blocks_sub.next().await {
            let block = block?;

            let block_number = block.header().number;
            let block_hash = block.hash();

            println!("Block #{block_number}:");
            println!("  Hash: {block_hash}");
        }
    }

    // Subscribe to a few events from the head of the chain.
    {
        println!("\nSubscribe to latest finalized blocks with `chainHead`: ");

        let blocks = api.rpc().chainhead_unstable_follow(false).await?;
        let sub_id = blocks
            .subscription_id()
            .expect("RPC provides a valid subscription id; qed")
            .to_owned();

        let mut blocks = blocks.take(5);
        while let Some(event) = blocks.next().await {
            let event = event?;

            println!("chainHead_follow event: {event:?}");

            // Fetch the body, header and storage of the best blocks only.
            let FollowEvent::BestBlockChanged(best_block) = event else {
                // Note: for production use-cases, users need to call `chainhead_unstable_unpin`.
                continue
            };
            let hash = best_block.best_block_hash;

            // Fetch the block's header.
            let header = api
                .rpc()
                .chainhead_unstable_header(sub_id.clone(), hash)
                .await?;
            if let Some(header) = header {
                println!("  chainHead_header: {header}");
            } else {
                println!("  chainHead_header: Header not in memory for {hash}");
            }
        }
    }

    // A dynamic query to obtain some contant:
    {
        println!("\nDynamic queries for constants: ");

        let constant_query = subxt::dynamic::constant("System", "BlockLength");

        // Obtain the value:
        let value = api.constants().at(&constant_query)?;

        println!("Constant bytes: {:?}", value.encoded());
        println!("Constant value: {}", value.to_value()?);
    }

    // Get events for the latest block:
    {
        println!("\nDinamically decode events: ");

        let events = api.events().at_latest().await?;

        for event in events.iter() {
            let event = event?;

            let pallet = event.pallet_name();
            let variant = event.variant_name();
            let field_values = event.field_values()?;

            println!("{pallet}::{variant}: {field_values}");
        }
    }

    // Build a dynamic storage query to access account information.
    {
        println!("\nStorag query to fetch `total_issuance`: ");

        let addr = polkadot::storage().balances().total_issuance();
        let total_issuance = api
            .storage()
            .at_latest()
            .await?
            .fetch_or_default(&addr)
            .await?;

        println!("Total issuance: {total_issuance}");
    }

    {
        println!("\nVarious RPC calls: ");

        let system_chain = api.rpc().system_chain().await?;
        println!("System chain: {system_chain}");

        let system_name = api.rpc().system_name().await?;
        println!("System name: {system_name}");

        let finalized_hash = api.rpc().finalized_head().await?;
        println!("Finalized hash {finalized_hash:?}");
    }

    Ok(())
}
