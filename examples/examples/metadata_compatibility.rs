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
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.13/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    PairSigner,
    PolkadotExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "examples/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    // Full metadata validation is not enabled by default.
    // Ensure that the static metadata used to generate the API
    // (i.e., runtime_metadata_path = "examples/polkadot_metadata.scale") is compatible with
    // the runtime metadata obtained from the polkadot node.
    //
    // Note: This step can be skipped if full metadata validation is not of interest.
    api.validate_metadata()?;

    // The pallet metadata compatibility is verified by default.
    // When obtaining the "Balance" pallet the API validates if the static metadata is
    // compatible with the runtime metadata of the pallet.
    //
    // Note: Pallets can be compatible even if the full metadata validation is not.
    // Therefore, this is the default behavior for customers who are interested in
    // working with a subset of the metadata.
    let hash = api
        .tx()
        .balances()? // Pallet metadata is validated.
        .transfer(AccountKeyring::Bob.to_account_id().into(), 10_000)
        .sign_and_submit_default(&signer)
        .await?;
    println!("Balance transfer extrinsic submitted: {}", hash);

    // The pallet metadata can be skipped when calling the `_unchecked` family of methods.
    let hash = api
        .tx()
        .balances_unchecked() // Pallet metadata validation is skipped.
        .transfer(dest, 123_456_789_012_345)
        .sign_and_submit_default(&signer)
        .await?;

    println!(
        "Balance transfer extrinsic submitted (without pallet metadata validation): {}",
        hash
    );

    Ok(())
}
