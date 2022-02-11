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

//! To run this example, a local polkadot node should be running. Example verified against polkadot 0.9.13-82616422d0-aarch64-macos.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.13/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use futures::{
    future,
    stream,
    StreamExt,
};
use sp_keyring::AccountKeyring;
use std::time::Duration;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
    PairSigner,
};

#[subxt::subxt(runtime_metadata_path = "examples/polkadot_metadata.scale")]
pub mod polkadot {}

/// Subscribe to all events, and then manually look through them and
/// pluck out the events that we care about.
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Subscribe to any events that occur:
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    // Subscribe to just balance transfer events, making use of `flat_map` and
    // `filter_map` from the StreamExt trait to filter everything else out.
    let mut transfer_events = api
        .events()
        .subscribe()
        .await?
        // Ignore errors returning events:
        .filter_map(|events| future::ready(events.ok()))
        // Map events to just the one we care about:
        .flat_map(|events| {
            let transfer_events = events
                .find::<polkadot::balances::events::Transfer>()
                .collect::<Vec<_>>();
            stream::iter(transfer_events)
        });

    // While this subscription is active, we imagine some balance transfers are made somewhere else:
    async_std::task::spawn(async {
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        let api = ClientBuilder::new()
            .build()
            .await
            .unwrap()
            .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

        // Make small balance transfers from Alice to Bob in a loop:
        loop {
            api.tx()
                .balances()
                .transfer(AccountKeyring::Bob.to_account_id().into(), 1_000_000_000)
                .sign_and_submit(&signer)
                .await
                .unwrap();
            async_std::task::sleep(Duration::from_secs(10)).await;
        }
    });

    // Our subscription will see all of the transfer events emitted as a result of this:
    while let Some(transfer_event) = transfer_events.next().await {
        println!("Balance transfer event: {transfer_event:?}");
    }

    Ok(())
}
