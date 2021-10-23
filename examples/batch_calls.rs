// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

//! To run this example, a local polkadot node should be running.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.11/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, PairSigner};

#[subxt::subxt(runtime_metadata_path = "examples/polkadot_metadata.scale")]
pub mod polkadot {}

type Call = polkadot::runtime_types::polkadot_runtime::Call;
type BalancesCall = polkadot::runtime_types::pallet_balances::pallet::Call;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let alice = PairSigner::new(AccountKeyring::Alice.pair());
    let bob = AccountKeyring::Bob.to_account_id().into();
    let charlie = AccountKeyring::Charlie.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<polkadot::DefaultConfig>>();

    let calls = vec![
        Call::Balances(BalancesCall::transfer {
            dest: bob,
            value: 10_000,
        }),
        Call::Balances(BalancesCall::transfer {
            dest: charlie,
            value: 5_000,
        }),
    ];

    let result = api
        .tx()
        .utility()
        .batch(calls)
        .sign_and_submit_then_watch(&alice)
        .await
        .unwrap();

    if let Some(event) =
        result.find_event::<polkadot::utility::events::BatchCompleted>()?
    {
        println!("Batch success: value: {:?}", event);
    } else {
        println!("Failed to find Utility::BatchCompleted Event");
    }
    Ok(())
}
