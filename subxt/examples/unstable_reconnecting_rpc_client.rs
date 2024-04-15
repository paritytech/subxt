//! Example to utilize the `reconnecting rpc client` in subxt
//! which hidden behind behind `--feature unstable-reconnecting-rpc-client`
//!
//! To utilize full logs from the RPC client use:
//! `RUST_LOG="jsonrpsee=trace,reconnecting_jsonrpsee_ws_client=trace"`

#![allow(missing_docs)]

use std::sync::Arc;
use std::time::Duration;

use futures::StreamExt;
use subxt::backend::rpc::reconnecting_rpc_client::{Client, RetryPolicy};
use subxt::backend::rpc::RpcClient;
use subxt::backend::unstable::{UnstableBackend, UnstableBackendDriver};
use subxt::config::Header;
use subxt::error::{Error, RpcError};
use subxt::tx::TxStatus;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a new client with with a reconnecting RPC client.
    let rpc = Arc::new(
        Client::builder()
            // Reconnect with exponential backoff
            .retry_policy_for_reconnect(
                RetryPolicy::exponential(Duration::from_millis(100))
                    .with_max_delay(Duration::from_secs(10))
                    .with_max_retries(usize::MAX),
            )
            .build("ws://localhost:9944".to_string())
            .await?,
    );

    let (backend, driver) = UnstableBackend::builder().build(RpcClient::new(rpc.clone()));

    // The unstable backend needs driving:
    tokio::spawn(drive_rpc_backend(driver));

    let api: OnlineClient<PolkadotConfig> = OnlineClient::from_backend(Arc::new(backend)).await?;

    subscribe_to_a_few_blocks(&api).await?;
    submit_retry_transaction(&api).await?;

    println!("RPC client reconnected `{}` times", rpc.reconnect_count());

    Ok(())
}

async fn drive_rpc_backend(mut driver: UnstableBackendDriver<PolkadotConfig>) {
    while let Some(val) = driver.next().await {
        if let Err(e) = val {
            eprintln!("Error driving unstable backend: {e}; terminating client");
        }
    }
}

// The retry-able rpc backend will re-run this until it's succesful.
// It's also possible to run custom retry_logic without the retry-backend
//
// Then you can use `subxt::backend::utils::retry` or `subxt::backend::utils::retry_with_strategy`
// to retry rpc calls or write your own retry logic.
async fn subscribe_to_a_few_blocks(api: &OnlineClient<PolkadotConfig>) -> Result<(), Error> {
    let mut blocks_sub = api.blocks().subscribe_finalized().await?.take(10);

    // For each block, print a bunch of information about it:
    // This is automatically re-start when the unstable backend is re-started.
    while let Some(block) = blocks_sub.next().await {
        let block = match block {
            Ok(b) => b,
            // This is re-started automatically when it reconnects.
            Err(e) => {
                return Err(e.into());
            }
        };

        let block_number = block.number();
        let block_hash = block.hash();

        println!("Block #{block_number} ({block_hash})");
    }

    Ok(())
}

// Submit the balance transfer extrinsic from Alice and wait for it to be successful
// and in a finalized block.
//
// If the balance transfer failed because the RPC connection was closed, it's retried.
//
// This differ from the blocks API and this is not re-started on reconnect.
// Build a balance transfer extrinsic.
async fn submit_retry_transaction(api: &OnlineClient<PolkadotConfig>) -> Result<(), Error> {
    let dest = dev::bob().public_key().into();
    let tx = polkadot::tx().balances().transfer_allow_death(dest, 10_000);

    let mut tx_status = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &dev::alice())
        .await?;

    while let Some(status) = tx_status.next().await {
        match status {
            // It's finalized in a block!
            Ok(TxStatus::InFinalizedBlock(in_block)) => {
                let events = in_block.wait_for_success().await?;
                let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;

                if transfer_event.is_some() {
                    println!("Balance transfer success");
                } else {
                    println!("Failed to find Balances::Transfer Event");
                }
                return Ok(());
            }
            // Just log any other status we encounter:
            //
            // In this example we emit some important status handling for
            // here such as Dropped, Invalid etc....
            Ok(_) => {
                println!("New status");
            }
            Err(err) => {
                if err.is_disconnected_will_reconnect() {
                    // This is not a good idea but just an example
                    // how to re-submit transaction if the connection was lost.
                    tx_status = api
                        .tx()
                        .sign_and_submit_then_watch_default(&tx, &dev::alice())
                        .await?;
                } else {
                    return Err(err.into());
                }
            }
        }
    }

    Ok(())
}
