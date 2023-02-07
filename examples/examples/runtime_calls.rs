// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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
use subxt::{
    config::{
        polkadot::{
            Era,
            PlainTip,
            PolkadotExtrinsicParamsBuilder as Params,
        },
        SubstrateConfig,
    },
    tx::PairSigner,
    OnlineClient,
};

use codec::Decode;

#[subxt::subxt(runtime_metadata_url = "http://localhost:9933")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let api = OnlineClient::<SubstrateConfig>::new().await?;

    let api_tx = polkadot::runtime_api::Core::version();
    println!("RuntimeApi payload: {:?}", api_tx);

    let bytes = api.runtime_api().at(None).await?.call(api_tx).await?;
    println!("Result: {:?}", bytes);

    let result: polkadot::runtime_api::Core::version_target =
        Decode::decode(&mut &bytes[..])?;
    println!(
        "Result for polkadot::runtime_api::Core::version(): {:?}\n\n",
        result
    );

    let api_tx = polkadot::runtime_api::Metadata::metadata_versions();
    println!("RuntimeApi payload: {:?}", api_tx);

    let bytes = api.runtime_api().at(None).await?.call(api_tx).await?;
    println!("Result: {:?}", bytes);

    let result: polkadot::runtime_api::Metadata::metadata_versions_target =
        Decode::decode(&mut &bytes[..])?;
    println!(
        "Result for polkadot::runtime_api::Metadata::metadata_versions(): {:?}\n\n",
        result
    );

    let alice = AccountKeyring::Alice.to_account_id().into();
    let api_tx = polkadot::runtime_api::AccountNonceApi::account_nonce(alice);
    println!("RuntimeApi payload: {:?}", api_tx);

    let bytes = api.runtime_api().at(None).await?.call(api_tx).await?;
    println!("Result: {:?}", bytes);

    let result: polkadot::runtime_api::AccountNonceApi::account_nonce_target =
        Decode::decode(&mut &bytes[..])?;
    println!(
        "Result for polkadot::runtime_api::AccountNonceApi::account_nonce: {:?}\n\n",
        result
    );

    // Send from Alice to Bob.
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();
    let tx = polkadot::tx()
        .balances()
        .transfer(dest, 123_456_789_012_345);
    let _hash = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &signer)
        .await?
        .wait_for_finalized()
        .await?;

    // Check nonce again.
    let alice = AccountKeyring::Alice.to_account_id().into();
    let api_tx = polkadot::runtime_api::AccountNonceApi::account_nonce(alice);
    println!("RuntimeApi payload: {:?}", api_tx);

    let bytes = api.runtime_api().at(None).await?.call(api_tx).await?;
    println!("Result: {:?}", bytes);

    let result: polkadot::runtime_api::AccountNonceApi::account_nonce_target =
        Decode::decode(&mut &bytes[..])?;
    println!(
        "Result for polkadot::runtime_api::AccountNonceApi::account_nonce: {:?}\n\n",
        result
    );

    Ok(())
}
