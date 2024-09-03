//! Example to utilize the `reconnecting rpc client` in subxt
//! which hidden behind behind `--feature unstable-reconnecting-rpc-client`
//!
//! To utilize full logs from the RPC client use:
//! `RUST_LOG="jsonrpsee=trace,subxt-reconnecting-rpc-client=trace"`

#![allow(missing_docs)]

use std::time::Duration;

use futures::StreamExt;
use subxt::backend::rpc::reconnecting_rpc_client::{ExponentialBackoff, RpcClient};
use subxt::{OnlineClient, PolkadotConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a new client with with a reconnecting RPC client.
    let rpc = RpcClient::builder()
        // Reconnect with exponential backoff
        //
        // This API is "iterator-like" and we use `take` to limit the number of retries.
        .retry_policy(
            ExponentialBackoff::from_millis(100)
                .max_delay(Duration::from_secs(10))
                .take(3),
        )
        // There are other configurations as well that can be found at [`reconnecting_rpc_client::ClientBuilder`].
        .build("ws://localhost:9944".to_string())
        .await?;

    // If you want to use the unstable backend with the reconnecting RPC client, you can do so like this:
    //
    // ```
    // use subxt::backend::unstable::UnstableBackend;
    // use subxt::OnlineClient;
    //
    // let (backend, mut driver) = UnstableBackend::builder().build(RpcClient::new(rpc.clone()));
    // tokio::spawn(async move {
    //     while let Some(val) = driver.next().await {
    //         if let Err(e) = val {
    //             eprintln!("Error driving unstable backend: {e}; terminating client");
    //        }
    //    }
    // });
    // let api: OnlineClient<PolkadotConfig> = OnlineClient::from_backend(Arc::new(backend)).await?;
    // ```

    let api: OnlineClient<PolkadotConfig> = OnlineClient::from_rpc_client(rpc.clone()).await?;

    // Run for at most 100 blocks and print a bunch of information about it.
    //
    // The subscription is automatically re-started when the RPC client has reconnected.
    // You can test that by stopping the polkadot node and restarting it.
    let mut blocks_sub = api.blocks().subscribe_finalized().await?.take(100);

    while let Some(block) = blocks_sub.next().await {
        let block = match block {
            Ok(b) => b,
            Err(e) => {
                // This can only happen on the legacy backend and the unstable backend
                // will handle this internally.
                if e.is_disconnected_will_reconnect() {
                    println!("The RPC connection was lost and we may have missed a few blocks");
                    continue;
                }

                return Err(e.into());
            }
        };

        let block_number = block.number();
        let block_hash = block.hash();

        println!("Block #{block_number} ({block_hash})");
    }

    Ok(())
}
