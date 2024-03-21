//! Example to utilize the `reconnecting rpc client` in subxt
//! which hidden behind behind `--feature unstable-reconnecting-rpc-client`
//!
//! To utilize full logs from the RPC client use:
//! `RUST_LOG="jsonrpsee=trace,reconnecting_jsonrpsee_ws_client=trace"`

#![allow(missing_docs)]

use std::sync::Arc;
use std::time::Duration;

use subxt::backend::rpc::reconnecting_rpc_client::{CallRetryPolicy, Client, RetryPolicy};
use subxt::backend::rpc::RpcClient;
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
            // Just an example how to override the default retry policy for individual RPC calls.
            .retry_policy_for_method("foo", CallRetryPolicy::Retry)
            .build("ws://localhost:9944".to_string())
            .await?,
    );

    let api: OnlineClient<PolkadotConfig> =
        OnlineClient::from_rpc_client(RpcClient::new(rpc.clone())).await?;

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;

    // For each block, print a bunch of information about it:
    while let Some(block) = blocks_sub.next().await {
        let block = match block {
            Ok(b) => b,
            Err(Error::Rpc(RpcError::DisconnectedWillReconnect(err))) => {
                println!("{err}");
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        };

        let block_number = block.header().number;
        let block_hash = block.hash();

        println!("Block #{block_number} ({block_hash})");
    }

    println!("RPC client reconnected `{}` times", rpc.reconnect_count());

    Ok(())
}
