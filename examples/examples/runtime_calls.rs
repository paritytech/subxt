// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot v0.9.28-9ffe6e9e3da.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.28/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{
    config::{
        polkadot::{
            Era,
            PlainTip,
            PolkadotExtrinsicParamsBuilder as Params,
        },
        PolkadotConfig,
    },
    tx::PairSigner,
    OnlineClient,
};

use codec::Decode;

#[subxt::subxt(runtime_metadata_url = "http://localhost:9933")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api_tx = polkadot::runtime_api::Core::version();
    println!("RuntimeApi payload: {:?}", api_tx);

    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let bytes = api.runtime_api().at(None).await?.call(api_tx).await?;

    println!("Result: {:?}", bytes);
    let result: polkadot::runtime_api::Core::version_target =
        Decode::decode(&mut &bytes[..])?;

    println!("Result is: {:?}", result);

    Ok(())
}
