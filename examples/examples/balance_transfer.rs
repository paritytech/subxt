// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

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
    SubstrateConfig,
    PairSigner,
    PolkadotExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<SubstrateConfig, PolkadotExtrinsicParams<SubstrateConfig>>>();

    // Submit the `transfer` extrinsic from Alice's account to Bob's.
    let dest = AccountKeyring::Bob.to_account_id().into();

    // Obtain an extrinsic, calling the "transfer" function in
    // the "balances" pallet.
    let extrinsic = api.tx().balances().transfer(dest, 123_456_789_012_345)?;

    // Sign and submit the extrinsic, returning its hash.
    let tx_hash = extrinsic.sign_and_submit_default(&signer).await?;

    println!("Balance transfer extrinsic submitted: {}", tx_hash);

    Ok(())
}
