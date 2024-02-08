//! Example to utilize the `reconnecting rpc client` in subxt
//! which hidden behind behind `--feature unstable-reconnecting-rpc-client`
//!
//! To utilize full logs from the RPC client use:
//! `RUST_LOG="jsonrpsee=trace,reconnecting_jsonrpsee_ws_client=trace"`

#![allow(missing_docs)]

use std::time::Duration;

use subxt::backend::rpc::reconnecting_rpc_client::{Client, ExponentialBackoff, PingConfig};
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
    let rpc = Client::builder()
        // Reconnect with exponential backoff
        //
        // This API is "iterator-like" so one could limit it to only
        // reconnect x times and then quit.
        .retry_policy(ExponentialBackoff::from_millis(100).max_delay(Duration::from_secs(10)))
        // Send period WebSocket pings/pongs every 6th second and if it's not ACK:ed in 30 seconds
        // then disconnect.
        //
        // This is just a way to ensure that the connection isn't idle if no message is sent that often
        .enable_ws_ping(
            PingConfig::new()
                .ping_interval(Duration::from_secs(6))
                .inactive_limit(Duration::from_secs(30)),
        )
        // There are other configurations as well that can be found here:
        // <https://docs.rs/reconnecting-jsonrpsee-ws-client/latest/reconnecting_jsonrpsee_ws_client/struct.ClientBuilder.html>
        .build("ws://localhost:9944".to_string())
        .await?;

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
