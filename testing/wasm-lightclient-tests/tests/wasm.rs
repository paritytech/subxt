#![cfg(target_arch = "wasm32")]

use futures_util::StreamExt;
use subxt::{client::OnlineClient, config::PolkadotConfig, lightclient::LightClient};
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// Run the tests by calling:
//
// ```text
// wasm-pack test --firefox --headless
// ```
//
// Use the following to enable logs:
// ```
//  console_error_panic_hook::set_once();
//  tracing_wasm::set_as_global_default();
// ```

#[wasm_bindgen_test]
async fn light_client_works() {
    let api = connect_to_rpc_node().await;
    tracing_wasm::set_as_global_default();
    tracing::info!("Subscribe to latest finalized blocks: ");

    // Light clients can send Stop events during syncing, which may cause blocks to be missed.
    // Example log output shows a block being handed back ~14 seconds after this `begin_time`,
    // and then no followup block until ~288 seconds after this `begin_time`. 
    //
    // Between these two occurrences:
    // 1. Smoldot's GrandPa warp syncing completes, and
    // 2. Smoldot sends a "stop" event.
    //
    // This leads to use getting a DisconnectedWillReconnect error back, which we ignore here,
    // though we would like to find a better way to wait until synchronization is complete before
    // starting to use the light client. This is a particular issue in WASM and doesn't seem to be
    // an issue natively.
    //
    // TODO: Work out how to address this better.
    let begin_time = web_time::Instant::now();
    let blocks_sub = api
        .stream_blocks()
        .await
        .expect("Cannot subscribe to finalized blocks")
        .filter_map(|block| async {
            match block {
                Ok(b) => {
                    let block_number = b.number();
                    let block_received_at_ms = begin_time.elapsed().as_millis();
                    tracing::info!("Block {block_number} received at instant {block_received_at_ms}ms");
                    Some(b)
                },
                Err(e) => {
                    let err: subxt::Error = e.into();
                    if err.is_disconnected_will_reconnect() {
                        tracing::warn!("Light client reconnecting, some blocks may have been missed");
                        None
                    } else {
                        panic!("Block error: {err}");
                    }
                }
            }
        })
        .take(3);

    futures_util::pin_mut!(blocks_sub);

    // For each block, print information about it:
    while let Some(block) = blocks_sub.next().await {
        let block_number = block.header().number;
        let block_hash = block.hash();

        tracing::info!("Block #{block_number}:");
        tracing::info!("  Hash: {block_hash}");
    }
}

/// We connect to an RPC node because the light client can struggle to sync in
/// time to a new local node for some reason. Because this can be brittle (eg RPC nodes can
/// go down or have network issues), we try a few RPC nodes until we find one that works.
async fn connect_to_rpc_node() -> OnlineClient<PolkadotConfig> {
    let rpc_node_urls = [
        "wss://rpc.polkadot.io",
        "wss://1rpc.io/dot",
        "wss://polkadot-public-rpc.blockops.network/ws",
    ];

    async fn do_connect(
        url: &str,
    ) -> Result<OnlineClient<PolkadotConfig>, Box<dyn std::error::Error + Send + Sync + 'static>>
    {
        let chainspec = subxt::utils::fetch_chainspec_from_rpc_node(url).await?;
        let (_lc, rpc) = LightClient::relay_chain(chainspec.get())?;
        let config = PolkadotConfig::new();
        let api = OnlineClient::from_rpc_client(config, rpc).await?;
        Ok(api)
    }

    for url in rpc_node_urls {
        let res = do_connect(url).await;

        match res {
            Ok(api) => return api,
            Err(e) => tracing::warn!("Error connecting to RPC node {url}: {e}"),
        }
    }

    panic!("Could not connect to any RPC node")
}
