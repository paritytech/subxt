#![allow(missing_docs)]

use std::time::Duration;

use reconnecting_jsonrpsee_ws_client::{Client, ExponentialBackoff, PingConfig};
use subxt::backend::rpc::RpcClient;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client with with a reconnecting RPC client.
    let rpc = Client::builder()
        // Reconnect with exponential backoff.
        .retry_policy(ExponentialBackoff::from_millis(100))
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

    // When the connection is lost, a error RpcError::DisconnectWillReconnect is emitted on the stream.
    // in such scenarios the error will be seen here.
    //
    // In such scenario it's possible that messages are lost when reconnecting
    // and if that's acceptable you may just ignore that error message.
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Find a Transfer event and print it.
    let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    }

    println!("rpc client reconnected: {}", rpc.reconnect_count());

    Ok(())
}
