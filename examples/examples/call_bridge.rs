// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! ```bash
//! ./target/release/subxt metadata --url ws://localhost:9946 > artifacts/bridge_metadata.scale
//! ```

use sp_keyring::AccountKeyring;
use subxt::{
    extrinsic::{
        BridgeHubExtrinsicParams,
        BridgeHubExtrinsicParamsBuilder,
    },
    ClientBuilder,
    DefaultConfig,
    PairSigner,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/bridge_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    // max_size 50 is pallet's config T::StringMaxLength in runtime, so only 50 will pass

    let result = call_set_name("123456", 51, 1).await;
    assert!(result.is_err());
    eprintln!("Result failed as expected: {:?}", result);

    let result = call_set_name("123456", 50, 51).await;
    assert!(result.is_err());
    eprintln!("Result failed as expected: {:?}", result);

    let result = call_set_name("12345678910", 50, 1).await;
    assert!(result.is_err());
    eprintln!("Result failed as expected: {:?}", result);

    let result = call_set_name("123456789", 50, 1).await;
    assert!(result.is_err());
    eprintln!("Result failed as expected: {:?}", result);

    assert!(call_set_name("1234567", 50, 1).await.is_ok());
    assert!(call_set_name("123", 50, 1).await.is_ok());

    println!("Hurraaa, test passed");
    Ok(())
}

/// This is the highest level approach to using this API. We use `wait_for_finalized_success`
/// to wait for the transaction to make it into a finalized block, and also ensure that the
/// transaction was successful according to the associated events.
async fn call_set_name(
    data: &str,
    max_size: u32,
    priority: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let params = BridgeHubExtrinsicParamsBuilder::new(max_size, priority);

    let api = ClientBuilder::new()
        .set_url("ws://127.0.0.1:9946")
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, BridgeHubExtrinsicParams<DefaultConfig>>>();

    let data = data.as_bytes().to_vec();
    let data =
        polkadot::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec::<u8>(data);

    let result = api
        .tx()
        .bridge_hub_sample()
        .set_name(data)?
        .sign_and_submit_then_watch(&signer, params)
        .await?
        .wait_for_finalized_success()
        .await?;

    for (idx, event) in result.iter().enumerate() {
        println!("Bridge event ({}): {:?}", idx, event);
    }

    let success_event = result
        .find_first::<polkadot::system::events::ExtrinsicSuccess>()
        .expect("decode error");

    if let Some(event) = success_event {
        println!("Bridge success: {:?}", event);
    } else {
        panic!("Failed to call bridge: {:?}", result);
    }

    Ok(())
}
