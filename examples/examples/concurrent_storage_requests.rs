// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot polkadot 0.9.25-5174e9ae75b.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.25/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use futures::join;
use sp_keyring::AccountKeyring;
use subxt::{
    OnlineClient,
    PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let addr = AccountKeyring::Bob.to_account_id();

    // Construct storage addresses to access:
    let staking_bonded = polkadot::storage().staking().bonded(&addr);
    let staking_ledger = polkadot::storage().staking().ledger(&addr);

    // For storage requests, we can join futures together to
    // await multiple futures concurrently:
    let a_fut = api.storage().fetch(&staking_bonded, None);
    let b_fut = api.storage().fetch(&staking_ledger, None);
    let (a, b) = join!(a_fut, b_fut);

    println!("{a:?}, {b:?}");

    Ok(())
}
