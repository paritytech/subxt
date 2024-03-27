//! Example to utilize the `reconnecting rpc client` in subxt
//! which hidden behind behind `--feature unstable-reconnecting-rpc-client`
//!
//! To utilize full logs from the RPC client use:
//! `RUST_LOG="jsonrpsee=trace,reconnecting_jsonrpsee_ws_client=trace"`

#![allow(missing_docs)]

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use subxt::backend::rpc::reconnecting_rpc_client::{CallRetryPolicy, Client, RetryPolicy};
use subxt::backend::rpc::RpcClient;
use subxt::config::Header;
use subxt::error::{Error, RpcError};
use subxt::{OnlineClient, PolkadotConfig};
use tokio_retry::strategy::{jitter, ExponentialBackoff};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

async fn retryable_rpc_call<T, A>(mut retry_future: A) -> Result<T, Error>
where
    A: tokio_retry::Action<Item = T, Error = Error>,
{
    let retry_strategy = ExponentialBackoff::from_millis(10)
        .map(jitter) // add jitter to delays
        .take(10); // limit to 10 retries

    tokio_retry::RetryIf::spawn(
        retry_strategy,
        || retry_future.run(),
        |err: &Error| err.is_disconnected_will_reconnect(),
    )
    .await
}

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

    let api: OnlineClient<PolkadotConfig> =
        OnlineClient::from_rpc_client(RpcClient::new(rpc.clone())).await?;

    // Example how to retry a rpc call.
    let mut blocks_sub =
        retryable_rpc_call(|| api.backend().stream_finalized_block_headers()).await?;

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
