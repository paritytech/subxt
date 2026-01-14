// Copyright 2019-2025 Parity Technologies (UK) Ltd.
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
use std::sync::Arc;
use subxt::dynamic::Value;
use subxt::{
    client::OnlineClient, config::PolkadotConfig, lightclient::LightClient, metadata::Metadata,
    rpcs::RpcClient,
};

type Client = OnlineClient<PolkadotConfig>;

/// The Polkadot chainspec.
const POLKADOT_SPEC: &str = include_str!("../../../artifacts/demo_chain_specs/polkadot.json");

// Create a light client using the backend configured in our test features.
async fn create_client() -> OnlineClient<PolkadotConfig> {
    let (_lc, rpc) = LightClient::relay_chain(POLKADOT_SPEC)
        .expect("Should be able to run LightClient::relay_chain");

    #[cfg(default_backend)]
    let client = {
        let backend = subxt::backend::CombinedBackend::builder()
            .build_with_background_driver(RpcClient::new(rpc))
            .await
            .expect("Should be able to build background driver for CombinedBackend");
        OnlineClient::<PolkadotConfig>::from_backend(Arc::new(backend)).await
    };
    #[cfg(legacy_backend)]
    let client = {
        let backend = subxt::backend::LegacyBackend::builder().build(RpcClient::new(rpc));
        OnlineClient::<PolkadotConfig>::from_backend(Arc::new(backend)).await
    };
    #[cfg(chainhead_backend)]
    let client = {
        let backend = subxt::backend::ChainHeadBackend::builder()
            .build_with_background_driver(RpcClient::new(rpc));
        OnlineClient::<PolkadotConfig>::from_backend(Arc::new(backend)).await
    };

    client.expect("Should be able to create client")
}

// Check that we can subscribe to non-finalized blocks.
async fn non_finalized_headers_subscription(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();

    tracing::trace!("Check non_finalized_headers_subscription");
    let mut sub = api.stream_best_blocks().await?;

    let _block = sub.next().await.unwrap()?;
    tracing::trace!("First block took {:?}", now.elapsed());

    let _block = sub.next().await.unwrap()?;
    tracing::trace!("Second block took {:?}", now.elapsed());

    let _block = sub.next().await.unwrap()?;
    tracing::trace!("Third block took {:?}", now.elapsed());

    Ok(())
}

// Check that we can subscribe to finalized blocks.
async fn finalized_headers_subscription(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();

    tracing::trace!("Check finalized_headers_subscription");

    let mut sub = api.stream_blocks().await?;
    let header = sub.next().await.unwrap()?;
    tracing::trace!("First block took {:?}", now.elapsed());

    let finalized_hash = api.at_current_block().await?.block_hash();

    tracing::trace!(
        "Finalized hash: {:?} took {:?}",
        finalized_hash,
        now.elapsed()
    );

    assert_eq!(header.hash(), finalized_hash);
    tracing::trace!("Check progress {:?}", now.elapsed());

    let _block = sub.next().await.unwrap()?;
    tracing::trace!("Second block took {:?}", now.elapsed());
    let _block = sub.next().await.unwrap()?;
    tracing::trace!("Third block took {:?}", now.elapsed());
    let _block = sub.next().await.unwrap()?;
    tracing::trace!("Fourth block took {:?}\n", now.elapsed());

    Ok(())
}

// Check that we can subscribe to non-finalized blocks.
async fn runtime_api_call(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();
    tracing::trace!("Check runtime_api_call");

    let mut sub = api.stream_best_blocks().await?;

    let block = sub.next().await.unwrap()?;
    tracing::trace!("First block took {:?}", now.elapsed());

    // get metadata via state_call. if it decodes ok, it's probably all good.
    let result_bytes = block
        .at()
        .await?
        .runtime_apis()
        .call_raw("Metadata_metadata", None)
        .await?;
    let (_, _meta): (Compact<u32>, Metadata) = codec::Decode::decode(&mut &*result_bytes)?;

    tracing::trace!("Made runtime API call in {:?}\n", now.elapsed());

    Ok(())
}

// Lookup for the `Timestamp::now` plain storage entry.
async fn storage_plain_lookup(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();
    tracing::trace!("Check storage_plain_lookup");

    let addr = node_runtime::storage().timestamp().now();
    let entry = api
        .at_current_block()
        .await?
        .storage()
        .fetch(addr, ())
        .await?
        .decode()?;

    tracing::trace!("Storage lookup took {:?}\n", now.elapsed());

    assert!(entry > 0);

    Ok(())
}

// Make a dynamic constant query for `System::BlockLength`.
async fn dynamic_constant_query(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();
    tracing::trace!("Check dynamic_constant_query");

    let constant_query = subxt::dynamic::constant::<Value>("System", "BlockLength");
    let _value = api
        .at_current_block()
        .await?
        .constants()
        .entry(&constant_query)?;

    tracing::trace!("Dynamic constant query took {:?}\n", now.elapsed());

    Ok(())
}

// Fetch a few all events from the latest block and decode them dynamically.
async fn dynamic_events(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();
    tracing::trace!("Check dynamic_events");

    let at_block = api.at_current_block().await?;
    let events = at_block.events().fetch().await?;

    for event in events.iter() {
        let _event = event?;

        tracing::trace!("Event decoding took {:?}", now.elapsed());
    }

    tracing::trace!("Dynamic events took {:?}\n", now.elapsed());

    Ok(())
}

#[tokio::test]
async fn light_client_tests() {
    // Note: This code fetches the chainspec from the Polkadot public RPC node.
    // This is not recommended for production use, as it may be slow and unreliable.
    // However, this can come in handy for testing purposes.
    //
    // let chainspec = subxt::utils::fetch_chainspec_from_rpc_node("wss://rpc.polkadot.io:443")
    //     .await
    //     .unwrap();
    // let chain_config = chainspec.get();

    tracing::trace!("Init light client");
    let now = std::time::Instant::now();

    // We only do this once for all tests because it's quite expensive.
    let api = create_client().await;
    tracing::trace!("Light client initialization took {:?}", now.elapsed());

    non_finalized_headers_subscription(&api)
        .await
        .expect("non_finalized_headers_subscription should pass");
    finalized_headers_subscription(&api)
        .await
        .expect("finalized_headers_subscription should pass");
    runtime_api_call(&api)
        .await
        .expect("runtime_api_call should pass");
    storage_plain_lookup(&api)
        .await
        .expect("storage_plain_lookup should pass");
    dynamic_constant_query(&api)
        .await
        .expect("dynamic_constant_query should pass");
    dynamic_events(&api)
        .await
        .expect("dynamic_events should pass");

    tracing::trace!("Light complete testing took {:?}", now.elapsed());
}
