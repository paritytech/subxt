// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

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
