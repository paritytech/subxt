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

use sp_keyring::AccountKeyring;
use subxt::{
    tx::PairSigner,
    OnlineClient,
    PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Create a transaction to submit:
    let tx = polkadot::tx()
        .balances()
        .transfer(dest, 123_456_789_012_345);

    // Submit the transaction with default params:
    let hash = api.tx().sign_and_submit_default(&tx, &signer).await?;

    println!("Balance transfer extrinsic submitted: {}", hash);

    Ok(())
}
