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
    extrinsic::{
        Era,
        PlainTip,
    },
    ClientBuilder,
    SubstrateConfig,
    PairSigner,
    PolkadotExtrinsicParams,
    PolkadotExtrinsicParamsBuilder as Params,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<SubstrateConfig, PolkadotExtrinsicParams<SubstrateConfig>>>();

    // Configure the transaction tip and era:
    let tx_params = Params::new()
        .tip(PlainTip::new(20_000_000_000))
        .era(Era::Immortal, *api.client.genesis());

    // Send the transaction:
    let hash = api
        .tx()
        .balances()
        .transfer(dest, 123_456_789_012_345)?
        .sign_and_submit(&signer, tx_params)
        .await?;

    println!("Balance transfer extrinsic submitted: {}", hash);

    Ok(())
}
