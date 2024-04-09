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

use crate::{test_context, utils::node_runtime};
use codec::Compact;
use subxt::{client::OnlineClient, config::SubstrateConfig};
use subxt_metadata::Metadata;

type Client = OnlineClient<SubstrateConfig>;

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

#[subxt_test]
async fn light_client_testing() -> Result<(), subxt::Error> {
    tracing_subscriber::fmt::init();

    let ctx = test_context().await;
    let api = ctx.client();

    non_finalized_headers_subscription(&api).await?;
    finalized_headers_subscription(&api).await?;
    runtime_api_call(&api).await?;
    storage_plain_lookup(&api).await?;
    dynamic_constant_query(&api).await?;
    dynamic_events(&api).await?;

    Ok(())
}
