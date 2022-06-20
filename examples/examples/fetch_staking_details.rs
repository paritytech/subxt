// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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
