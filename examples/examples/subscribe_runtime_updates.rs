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
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.13/polkadot" --output /usr/local/bin/polkadot --location
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

#[subxt::subxt(runtime_metadata_path = "examples/polkadot_metadata.scale")]
pub mod polkadot {}

type RuntimeApi =
    polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>;

/// This function is representative of the main customer use case.
async fn user_use_case(api: &RuntimeApi) {
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
            .sign_and_submit_default(&signer)
            .await
            .unwrap();

        println!("Balance transfer extrinsic submitted: {}", hash);
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

/// This function handles runtime updates via subscribing to the
/// node's RuntimeVersion.
async fn runtime_update(api: &RuntimeApi) {
    // Obtain an update subscription to further detect changes in the runtime version of the node.
    let mut update_subscription =
        api.client.rpc().subscribe_runtime_version().await.unwrap();
    println!("    [RuntimeUpdate] Application subscribed to RuntimeVersion updates");

    while let Some(runtime_version) = update_subscription.next().await {
        // The Runtime Version obtained via subscription.
        let runtime_version = runtime_version.unwrap();
        // The Runtime Version of the client, as set during building the client.
        let current_runtime = api.client.runtime_version();

        // Ensure that the provided Runtime Version can be applied to the current
        // version of the client. There are cases when the subscription to the
        // Runtime Version of the node would produce spurious update events.
        // In those cases, set the Runtime Version on the client if and only if
        // the provided runtime version is bigger than what the client currently
        // has stored.
        if current_runtime.spec_version >= runtime_version.spec_version {
            println!(
                "    [RuntimeUpdate] Update not performed for received spec_version={}, client has spec_version={}",
                runtime_version.spec_version, current_runtime.spec_version
            );
            continue
        }

        // Perform the actual client update to ensure that further extrinsics
        // include the appropriate `spec_version` and `transaction_version`.
        println!(
            "    [RuntimeUpdate] Updating RuntimeVersion from {} to {}",
            current_runtime.spec_version, runtime_version.spec_version
        );
        api.client.set_runtime_version(runtime_version);
        println!("    [RuntimeUpdate] Update completed");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<RuntimeApi>();

    // Start two concurrent branches:
    //   - One branch performs runtime update
    //   - Another branch performs the main customer use case.
    //
    // Ideally this examples should be targeting a node that would perform
    // runtime updates to demonstrate the functionality.
    //
    // For more details on how to perform updates on a node, please follow:
    // https://docs.substrate.io/tutorials/v3/forkless-upgrades/
    tokio::select! {
        _ = runtime_update(&api) => {
            println!("Runtime update branch finished");
        }
        _ = user_use_case(&api) =>
        {
            println!("User main use case finished");
        }
    }

    Ok(())
}
