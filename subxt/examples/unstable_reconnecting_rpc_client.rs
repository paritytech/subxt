//! Example to utilize the `reconnecting rpc client` in subxt
//! which hidden behind behind `--feature unstable-reconnecting-rpc-client`
//!
//! To utilize full logs from the RPC client use:
//! `RUST_LOG="jsonrpsee=trace,reconnecting_jsonrpsee_ws_client=trace"`

#![allow(missing_docs)]

use std::sync::Arc;
use std::time::Duration;

use futures::StreamExt;
use subxt::backend::retry::RetryBackend;
use subxt::backend::rpc::reconnecting_rpc_client::{Client, RetryPolicy};
use subxt::backend::rpc::RpcClient;
use subxt::backend::unstable::{UnstableBackend, UnstableBackendDriver};
use subxt::config::Header;
use subxt::error::{Error, RpcError};
use subxt::{OnlineClient, PolkadotConfig};

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

    let retry_backend = RetryBackend::from(backend.boxed_dyn_backend());

    let api: OnlineClient<PolkadotConfig> =
        OnlineClient::from_backend(Arc::new(retry_backend)).await?;

    // The retry-able rpc backend will re-run this until it's succesful.
    // It's also possible to run custom retry_logic withot the retry-backend
    //
    // Then you can use `subxt::backend::utils::retry` or `subxt::backend::utils::retry_with_strategy`
    // to retry rpc calls or write your own retry logic.
    let mut blocks_sub = api.backend().stream_finalized_block_headers().await?;

    // For each block, print a bunch of information about it:
    while let Some(block) = blocks_sub.next().await {
        let header = match block {
            Ok((header, _)) => header,
            Err(Error::Rpc(RpcError::DisconnectedWillReconnect(err))) => {
                println!("{err}");
                blocks_sub = blocks_sub.resubscribe().await?;
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        };

        let block_number = header.number;
        let block_hash = header.hash();

        println!("Block #{block_number} ({block_hash})");
    }

    println!("RPC client reconnected `{}` times", rpc.reconnect_count());

    Ok(())
}

async fn drive_rpc_backend(mut driver: UnstableBackendDriver<PolkadotConfig>) {
    while let Some(val) = driver.next().await {
        if let Err(e) = val {
            if e.is_disconnected_will_reconnect() {
                eprintln!("Unstable backend was disconnecting; reconnecting");
                continue;
            } else {
                eprintln!("Error driving unstable backend: {e}; terminating client");
            }
        }
    }
}
