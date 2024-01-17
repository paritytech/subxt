// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Light Client Initialization and Testing
//!
//! The initialization process of the light client can be slow, especially when
//! it needs to synchronize with a local running node for each individual
//! #[tokio::test] in subxt. To optimize this process, a subset of tests is
//! exposed to ensure the light client remains functional over time. Currently,
//! these tests are placed under an unstable feature flag.
//!
//! Ideally, we would place the light client initialization in a shared static
//! using `OnceCell`. However, during the initialization, tokio::spawn is used
//! to multiplex between subxt requests and node responses. The #[tokio::test]
//! macro internally creates a new Runtime for each individual test. This means
//! that only the first test, which spawns the substrate binary and synchronizes
//! the light client, would have access to the background task. The cleanup process
//! would destroy the spawned background task, preventing subsequent tests from
//! accessing it.
//!
//! To address this issue, we can consider creating a slim proc-macro that
//! transforms the #[tokio::test] into a plain #[test] and runs all the tests
//! on a shared tokio runtime. This approach would allow multiple tests to share
//! the same background task, ensuring consistent access to the light client.
//!
//! For more context see: https://github.com/tokio-rs/tokio/issues/2374.
//!

use crate::utils::node_runtime;
use codec::Compact;
use subxt::{
    client::{LightClient, LightClientBuilder, OnlineClientT},
    config::PolkadotConfig,
};
use subxt_metadata::Metadata;

type Client = LightClient<PolkadotConfig>;

// Check that we can subscribe to non-finalized blocks.
async fn non_finalized_headers_subscription(api: &Client) -> Result<(), subxt::Error> {
    let mut sub = api.blocks().subscribe_best().await?;

    let _block = sub.next().await.unwrap()?;
    let _block = sub.next().await.unwrap()?;
    let _block = sub.next().await.unwrap()?;

    Ok(())
}

// Check that we can subscribe to finalized blocks.
async fn finalized_headers_subscription(api: &Client) -> Result<(), subxt::Error> {
    let mut sub = api.blocks().subscribe_finalized().await?;
    let header = sub.next().await.unwrap()?;
    let finalized_hash = api
        .backend()
        .latest_finalized_block_ref()
        .await
        .unwrap()
        .hash();

    assert_eq!(header.hash(), finalized_hash);

    let _block = sub.next().await.unwrap()?;
    let _block = sub.next().await.unwrap()?;
    let _block = sub.next().await.unwrap()?;

    Ok(())
}

// Check that we can subscribe to non-finalized blocks.
async fn runtime_api_call(api: &Client) -> Result<(), subxt::Error> {
    let mut sub = api.blocks().subscribe_best().await?;

    let block = sub.next().await.unwrap()?;
    let rt = block.runtime_api().await?;

    // get metadata via state_call. if it decodes ok, it's probably all good.
    let _ = rt
        .call_raw::<(Compact<u32>, Metadata)>("Metadata_metadata", None)
        .await?;

    Ok(())
}

// Lookup for the `Timestamp::now` plain storage entry.
async fn storage_plain_lookup(api: &Client) -> Result<(), subxt::Error> {
    let addr = node_runtime::storage().timestamp().now();
    let entry = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&addr)
        .await?;

    assert!(entry > 0);

    Ok(())
}

// Make a dynamic constant query for `System::BlockLenght`.
async fn dynamic_constant_query(api: &Client) -> Result<(), subxt::Error> {
    let constant_query = subxt::dynamic::constant("System", "BlockLength");
    let _value = api.constants().at(&constant_query)?;

    Ok(())
}

// Fetch a few all events from the latest block and decode them dynamically.
async fn dynamic_events(api: &Client) -> Result<(), subxt::Error> {
    let events = api.events().at_latest().await?;

    for event in events.iter() {
        let _event = event?;
    }

    Ok(())
}

/// Fetch the chainSpec from the given url.
async fn fetch_spec_from_url(url: &str) -> serde_json::Value {
    // use jsonrpsee::core::client::ClientT;
    // let client = jsonrpsee_helpers::client(url.as_ref()).await?;

    pub use jsonrpsee::{
        client_transport::ws::{self, EitherStream, Url, WsTransportClientBuilder},
        core::client::{Client, ClientT},
    };

    let url = Url::parse(url).expect("Failed to parse url");
    let (sender, receiver) = WsTransportClientBuilder::default()
        .build(url)
        .await
        .expect("Failed to connect to the node");

    let client = Client::builder()
        .max_buffer_capacity_per_subscription(4096)
        .build_with_tokio(sender, receiver);

    client
        .request("sync_state_genSyncSpec", jsonrpsee::rpc_params![true])
        .await
        .expect("Failed to fetch the chainSpec")
}

#[tokio::test]
async fn light_client_testing() -> Result<(), subxt::Error> {
    let chain_spec = fetch_spec_from_url("wss://rpc.polkadot.io:443").await;

    // Sleep 8 seconds to ensure that a new block is produced for the substrate test node.
    // Although the block production should be 6 seconds, the CI environment is sometimes slow.
    // This is a temporary workaround for: https://github.com/smol-dot/smoldot/issues/1562.
    tokio::time::sleep(std::time::Duration::from_secs(8)).await;

    let api: LightClient<PolkadotConfig> = LightClientBuilder::new()
        .build(&chain_spec.to_string())
        .await?;

    non_finalized_headers_subscription(&api).await?;
    finalized_headers_subscription(&api).await?;
    runtime_api_call(&api).await?;
    storage_plain_lookup(&api).await?;
    dynamic_constant_query(&api).await?;
    dynamic_events(&api).await?;

    Ok(())
}
