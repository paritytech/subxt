// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot 0.9.18-4542a603cc-aarch64-macos.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.18/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{
    sp_core::{
        sr25519,
        Pair,
    },
    sp_runtime::AccountId32,
    ClientBuilder,
    DefaultConfig,
    PolkadotExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let era = api.storage().staking().active_era(None).await?.unwrap();
    println!(
        "Staking active era: index: {:?}, start: {:?}",
        era.index, era.start
    );

    let alice_id = AccountKeyring::Alice.to_account_id();
    println!("  Alice account id:        {:?}", alice_id);

    // Get Alice' Stash account ID
    let alice_stash_id: AccountId32 = sr25519::Pair::from_string("//Alice//stash", None)
        .expect("Could not obtain stash signer pair")
        .public()
        .into();
    println!("  Alice//stash account id: {:?}", alice_stash_id);

    // Map from all locked "stash" accounts to the controller account.
    let controller_acc = api
        .storage()
        .staking()
        .bonded(&alice_stash_id, None)
        .await?
        .unwrap();
    println!("    account controlled by: {:?}", controller_acc);

    let era_result = api
        .storage()
        .staking()
        .eras_reward_points(&era.index, None)
        .await?;
    println!("Era reward points: {:?}", era_result);

    Ok(())
}
