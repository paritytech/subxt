// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot v0.9.28-9ffe6e9e3da.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.28/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::dynamic::Value;
use subxt::{config::PolkadotConfig, OnlineClient};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_tiny.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // In the first part of the example calls are made using the static generated code
    // and as a result the returned values are strongly typed.

    // Create a runtime API payload that calls into
    // `Core_version` function.
    let runtime_api_call = polkadot::apis().core().version();

    // Submit the runtime API call.
    let version = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await;
    println!("Core_version: {:?}", version);

    // Show the supported metadata versions of the node.
    // Calls into `Metadata_metadata_versions` runtime function.
    let runtime_api_call = polkadot::apis().metadata().metadata_versions();

    // Submit the runtime API call.
    let versions = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await?;
    println!("Metadata_metadata_versions: {:?}", versions);

    // Create a runtime API payload that calls into
    // `AccountNonceApi_account_nonce` function.
    let account = AccountKeyring::Alice.to_account_id().into();
    let runtime_api_call = polkadot::apis().account_nonce_api().account_nonce(account);

    // Submit the runtime API call.
    let nonce = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await;
    println!("AccountNonceApi_account_nonce for Alice: {:?}", nonce);

    // Dynamic calls.
    let runtime_api_call = subxt::dynamic::runtime_api_call(
        "Metadata_metadata_versions",
        Vec::<Value<()>>::new(),
        None,
    );
    let versions = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await?;
    println!(
        " dynamic Metadata_metadata_versions: {:#?}",
        versions.to_value()
    );

    Ok(())
}
