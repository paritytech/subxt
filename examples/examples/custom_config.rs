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
    ClientBuilder,
    Config,
    DefaultConfig,
    PairSigner,
    PolkadotExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

/// Custom [`Config`] impl where the default types for the target chain differ from the
/// [`DefaultConfig`]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MyConfig;
impl Config for MyConfig {
    // This is different from the default `u32`.
    //
    // *Note* that in this example it does differ from the actual `Index` type in the
    // polkadot runtime used, so some operations will fail. Normally when using a custom `Config`
    // impl types MUST match exactly those used in the actual runtime.
    type Index = u64;
    type BlockNumber = <DefaultConfig as Config>::BlockNumber;
    type Hash = <DefaultConfig as Config>::Hash;
    type Hashing = <DefaultConfig as Config>::Hashing;
    type AccountId = <DefaultConfig as Config>::AccountId;
    type Address = <DefaultConfig as Config>::Address;
    type Header = <DefaultConfig as Config>::Header;
    type Signature = <DefaultConfig as Config>::Signature;
    type Extrinsic = <DefaultConfig as Config>::Extrinsic;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<MyConfig, PolkadotExtrinsicParams<MyConfig>>>();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let hash = api
        .tx()
        .balances()
        .transfer(dest, 10_000)?
        .sign_and_submit_default(&signer)
        .await?;

    println!("Balance transfer extrinsic submitted: {}", hash);

    Ok(())
}
