//! This is a comprehensive example that utilizes the Light Client
//! for all subxt requests.
//!
//! Includes:
//!  - submitting a transaction to a local development chain
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
use sp_keyring::AccountKeyring;
use std::sync::Arc;
use subxt::dynamic::At;
use subxt::dynamic::Value;
use subxt::rpc::types::FollowEvent;
use subxt::rpc::LightClient;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("Creating light client");
    // Create a light client from the provided chain spec.
    let light_client = LightClient::new(include_str!("../../artifacts/dev_spec.json"))?;
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(Arc::new(light_client)).await?;
    println!("Done with creating light client");

    // Build a balance transfer extrinsic.
    // {
    //     let dest = AccountKeyring::Bob.to_account_id().into();
    //     let balance_transfer_tx = polkadot::tx().balances().transfer(dest, 10_000);

    //     // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    //     // and in a finalized block. We get back the extrinsic events if all is well.
    //     let from = PairSigner::new(AccountKeyring::Alice.pair());
    //     let events = api
    //         .tx()
    //         .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
    //         .await?
    //         .wait_for_finalized_success()
    //         .await?;

    //     // Find a Transfer event and print it.
    //     let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    //     if let Some(event) = transfer_event {
    //         println!("Balance transfer success: {event:?}");
    //     }
    // }

    // Subscribe to the latest 3 finalized blocks.
    {
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
        let blocks = api.rpc().chainhead_unstable_follow(false).await?;
        let sub_id = blocks
            .subscription_id()
            .expect("RPC provides a valid subscription id; qed")
            .to_owned();

        let mut blocks = blocks.take(10);
        while let Some(event) = blocks.next().await {
            let event = event?;

            println!("chainHead_follow event: {event:?}");

            // Fetch the body, header and storage of the best blocks only.
            let FollowEvent::BestBlockChanged(best_block) = event else {
                continue
            };
            let hash = best_block.best_block_hash;

            // Subscribe to fetch the block's body.
            let mut sub = api
                .rpc()
                .chainhead_unstable_body(sub_id.clone(), hash)
                .await?
                .take(5);

            while let Some(event) = sub.next().await {
                let event = event?;

                println!("  chainHead_body event: {event:?}");
            }

            // Fetch the block's header.
            // let header = api
            //     .rpc()
            //     .chainhead_unstable_header(sub_id.clone(), hash)
            //     .await?;
            // let header = header.expect("RPC must have this header in memory; qed");

            // println!("  chainHead_header: {header}");

            // Make a storage query.
            let account_id: AccountId32 = AccountKeyring::Alice.to_account_id().into();
            let addr = polkadot::storage().system().account(account_id);
            let addr_bytes = api.storage().address_bytes(&addr).unwrap();

            let mut sub = api
                .rpc()
                .chainhead_unstable_storage(sub_id.clone(), hash, &addr_bytes, None)
                .await?
                .take(5);

            while let Some(event) = sub.next().await {
                let event = event?;

                println!("  chainHead_storage event: {event:?}");
            }
        }
    }

    // A dynamic query to obtain some contant:
    {
        let constant_query = subxt::dynamic::constant("System", "BlockLength");

        // Obtain the value:
        let value = api.constants().at(&constant_query)?;

        println!("Constant bytes: {:?}", value.encoded());
        println!("Constant value: {}", value.to_value()?);
    }

    // Get events for the latest block:
    {
        let events = api.events().at_latest().await?;

        // We can dynamically decode events:
        println!("Dynamic event details:");
        for event in events.iter() {
            let event = event?;

            let pallet = event.pallet_name();
            let variant = event.variant_name();
            let field_values = event.field_values()?;

            println!("{pallet}::{variant}: {field_values}");
        }
    }

    // Create a dynamically runtime API payload that calls the
    // `AccountNonceApi_account_nonce` function.
    {
        let account = AccountKeyring::Alice.to_account_id();
        let runtime_api_call = subxt::dynamic::runtime_api_call(
            "AccountNonceApi_account_nonce",
            vec![Value::from_bytes(account)],
        );

        // Submit the call to get back a result.
        let nonce = api
            .runtime_api()
            .at_latest()
            .await?
            .call(runtime_api_call)
            .await?;

        println!("Account nonce: {:#?}", nonce.to_value());
    }

    // Build a dynamic storage query to access account information.
    {
        let account = AccountKeyring::Alice.to_account_id();
        let storage_query =
            subxt::dynamic::storage("System", "Account", vec![Value::from_bytes(account)]);

        // Use that query to `fetch` a result. Because the query is dynamic, we don't know what the result
        // type will be either, and so we get a type back that can be decoded into a dynamic Value type.
        let result = api
            .storage()
            .at_latest()
            .await?
            .fetch(&storage_query)
            .await?;
        let value = result.unwrap().to_value()?;

        println!("Alice has free balance: {:?}", value.at("data").at("free"));
    }

    {
        let system_chain = api.rpc().system_chain().await?;
        println!("System chain: {system_chain}");

        let system_name = api.rpc().system_name().await?;
        println!("System name: {system_name}");

        let finalized_hash = api.rpc().finalized_head().await?;
        println!("Finalized hash {finalized_hash:?}");
    }

    Ok(())
}
