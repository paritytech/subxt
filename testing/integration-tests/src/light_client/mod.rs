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
use subxt::{client::OnlineClient, config::PolkadotConfig, lightclient::LightClient};
use subxt_metadata::Metadata;

type Client = OnlineClient<PolkadotConfig>;

// Check that we can subscribe to non-finalized blocks.
async fn non_finalized_headers_subscription(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();

    println!("In non_finalized_headers_subscription");
    let mut sub = api.blocks().subscribe_best().await?;

    let _block = sub.next().await.unwrap()?;
    println!("First block took {:?}", now.elapsed());

    let _block = sub.next().await.unwrap()?;
    println!("Second block took {:?}", now.elapsed());

    let _block = sub.next().await.unwrap()?;
    println!("Third block took {:?}\n", now.elapsed());

    Ok(())
}

// Check that we can subscribe to finalized blocks.
async fn finalized_headers_subscription(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();

    println!("In finalized_headers_subscription");

    let mut sub = api.blocks().subscribe_finalized().await?;
    let header = sub.next().await.unwrap()?;
    println!("First block took {:?}", now.elapsed());

    let finalized_hash = api
        .backend()
        .latest_finalized_block_ref()
        .await
        .unwrap()
        .hash();

    println!(
        "Finalized hash: {:?} took {:?}",
        finalized_hash,
        now.elapsed()
    );

    assert_eq!(header.hash(), finalized_hash);
    println!("Check progress {:?}", now.elapsed());

    let _block = sub.next().await.unwrap()?;
    println!("Second block took {:?}", now.elapsed());
    let _block = sub.next().await.unwrap()?;
    println!("Third block took {:?}", now.elapsed());
    let _block = sub.next().await.unwrap()?;
    println!("Fourth block took {:?}\n", now.elapsed());

    Ok(())
}

// Check that we can subscribe to non-finalized blocks.
async fn runtime_api_call(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();
    println!("In runtime_api_call");

    let mut sub = api.blocks().subscribe_best().await?;

    let block = sub.next().await.unwrap()?;
    println!("First block took {:?}", now.elapsed());
    let rt = block.runtime_api().await?;

    // get metadata via state_call. if it decodes ok, it's probably all good.
    let _ = rt
        .call_raw::<(Compact<u32>, Metadata)>("Metadata_metadata", None)
        .await?;

    println!("Made runtime API call in {:?}\n", now.elapsed());

    Ok(())
}

// Lookup for the `Timestamp::now` plain storage entry.
async fn storage_plain_lookup(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();
    println!("In storage_plain_lookup");

    let addr = node_runtime::storage().timestamp().now();
    let entry = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&addr)
        .await?;

    println!("Storage lookup took {:?}\n", now.elapsed());

    assert!(entry > 0);

    Ok(())
}

// Make a dynamic constant query for `System::BlockLenght`.
async fn dynamic_constant_query(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();
    println!("In dynamic_constant_query");

    let constant_query = subxt::dynamic::constant("System", "BlockLength");
    let _value = api.constants().at(&constant_query)?;

    println!("Dynamic constant query took {:?}\n", now.elapsed());

    Ok(())
}

// Fetch a few all events from the latest block and decode them dynamically.
async fn dynamic_events(api: &Client) -> Result<(), subxt::Error> {
    let now = std::time::Instant::now();
    println!("In dynamic_events");

    let events = api.events().at_latest().await?;

    for event in events.iter() {
        let _event = event?;

        println!("Event decoding took {:?}", now.elapsed());
    }

    println!("Dynamic events took {:?}\n", now.elapsed());

    Ok(())
}

#[tokio::test]
async fn light_client_testing() -> Result<(), subxt::Error> {
    tracing_subscriber::fmt::init();
    let now = std::time::Instant::now();

    let chainspec = subxt::utils::fetch_chainspec_from_rpc_node("wss://rpc.polkadot.io:443")
        .await
        .unwrap();
    let (_lc, rpc) = LightClient::relay_chain(chainspec.get())?;
    let api = Client::from_rpc_client(rpc).await?;

    println!("Light client initialization took {:?}\n", now.elapsed());

    // non_finalized_headers_subscription(&api).await?;
    finalized_headers_subscription(&api).await?;
    // runtime_api_call(&api).await?;
    // storage_plain_lookup(&api).await?;
    // dynamic_constant_query(&api).await?;
    // dynamic_events(&api).await?;

    Ok(())
}
