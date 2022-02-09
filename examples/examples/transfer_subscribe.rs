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

//! To run this example, a local polkadot node should be running.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.11/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
    PairSigner,
};
use futures::StreamExt;
use std::time::Duration;

#[subxt::subxt(runtime_metadata_path = "examples/polkadot_metadata.scale")]
pub mod polkadot {}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Subscribe to any events that occur:
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();
    let mut event_sub = api
        .events()
        .subscribe()
        .await?;

    // While this subscription is active, balance transfers are made somewhere:
    async_std::task::spawn(async {
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        let api = ClientBuilder::new()
            .build()
            .await
            .unwrap()
            .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

        // Make small balance transfers in a loop:
        loop {
            api.tx()
                .balances()
                .transfer(AccountKeyring::Bob.to_account_id().into(), 10_000)
                .sign_and_submit(&signer)
                .await
                .unwrap();
            async_std::task::sleep(Duration::from_secs(10)).await;
        }
    });

    // Our subscription will see the events emitted as a result of this:
    while let Some(events) = event_sub.next().await {
        let events = events?;

        // Find the first transfer event, ignoring any others:
        let transfer_event = events.find_first_event::<polkadot::balances::events::Transfer>()?;

        if let Some(ev) = transfer_event {
            println!("Balance transfer success in block {:?}: value: {:?}", events.block_hash(), ev.2);
        } else {
            println!("No balance transfer event found in block {:?}", events.block_hash());
        }
    }

    Ok(())
}
