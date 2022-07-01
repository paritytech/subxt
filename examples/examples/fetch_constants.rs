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

use subxt::{
    ClientBuilder,
    SubstrateConfig,
    PolkadotExtrinsicParams,
};

// Generate the API from a static metadata path.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Upon connecting to the target polkadot node, the node's metadata is downloaded (referred to
    // as the runtime metadata).
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<SubstrateConfig, PolkadotExtrinsicParams<SubstrateConfig>>>();

    // Constants are queried from the node's runtime metadata.
    // Query the `ExistentialDeposit` constant from the `Balances` pallet.
    let existential_deposit = api
        // This is the constants query.
        .constants()
        // Constant from the `Balances` pallet.
        .balances()
        // Constant name.
        .existential_deposit()?;

    println!("Existential Deposit: {}", existential_deposit);

    Ok(())
}
