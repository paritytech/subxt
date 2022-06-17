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

//! To run this example, a local polkadot node should be running. Example verified against polkadot 0.9.18-f6d6ab005d-aarch64-macos.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.18/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use std::time::Duration;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    PairSigner,
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

    // Start a new tokio task to perform the runtime updates while
    // utilizing the API for other use cases.
    let update_client = api.client.updates();
    tokio::spawn(async move {
        let result = update_client.perform_runtime_updates().await;
        println!("Runtime update failed with result={:?}", result);
    });

    // Make multiple transfers to simulate a long running `subxt::Client` use-case.
    //
    // Meanwhile, the tokio task above will perform any necessary updates to keep in sync
    // with the node we've connected to. Transactions submitted in the vicinity of a runtime
    // update may still fail, however, owing to a race between the update happening and
    // subxt synchronising its internal state with it.
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    // Make small balance transfers from Alice to Bob:
    for _ in 0..10 {
        let hash = api
            .tx()
            .balances()
            .transfer(
                AccountKeyring::Bob.to_account_id().into(),
                123_456_789_012_345,
            )
            .unwrap()
            .sign_and_submit_default(&signer)
            .await
            .unwrap();

        println!("Balance transfer extrinsic submitted: {}", hash);
        tokio::time::sleep(Duration::from_secs(30)).await;
    }

    Ok(())
}
