//! Example to utilize the `reconnecting rpc client` in subxt
//! which hidden behind behind `--feature reconnecting-rpc-client`
//!
//! To utilize full logs from the RPC client use:
//! `RUST_LOG="jsonrpsee=trace,reconnecting_jsonrpsee_ws_client=trace"`

#![allow(missing_docs)]

use std::time::Duration;

use subxt::backend::rpc::reconnecting_rpc_client::{Client, ExponentialBackoff, PingConfig};
use subxt::backend::rpc::RpcClient;
use subxt::error::{Error, RpcError};
use subxt::{tx::TxStatus, OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

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

    // Build a balance transfer extrinsic.
    let dest = dev::bob().public_key().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer_allow_death(dest, 10_000);

    // Submit the balance transfer extrinsic from Alice, and wait for it to be successful
    // and in a finalized block. We get back the extrinsic events if all is well.
    let from = dev::alice();

    let mut balance_transfer_progress = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
        .await?;

    // When the connection is lost, a error RpcError::DisconnectWillReconnect is emitted on the stream.
    // In this example we just ignore it and print the why it was disconnected.
    //
    // If you want to quit on reconnects just use `balance_transfer_progress.wait_for_finalized_success().await`
    // and remove the while loop below.
    while let Some(status) = balance_transfer_progress.next().await {
        match status {
            // It's finalized in a block!
            Ok(TxStatus::InFinalizedBlock(in_block)) => {
                // grab the events and fail if no ExtrinsicSuccess event seen:
                let events = in_block.wait_for_success().await?;
                // We can look for events (this uses the static interface; we can also iterate
                // over them and dynamically decode them):
                let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;

                if transfer_event.is_some() {
                    println!("Balance transfer success");
                } else {
                    println!("Failed to find Balances::Transfer Event");
                }
            }
            // Just log any other status we encounter:
            //
            // In this example we emit some important status handling for
            // here such as Dropped, Invalid etc....
            Ok(_) => {
                println!("New status");
            }
            // In this example we just ignore when reconnections occurs
            // but it's technically possible that we can lose
            // messages on the subscription such as `InFinalizedBlock`
            // when reconnecting.
            Err(Error::Rpc(RpcError::DisconnectedWillReconnect(e))) => {
                println!("{e}");
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    println!("RPC client reconnected `{}` times", rpc.reconnect_count());

    Ok(())
}
