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

use subxt::{
    tx::SubmittableExtrinsic,
    OnlineClient,
    PolkadotConfig,
};

use sp_keyring::AccountKeyring;
use subxt::tx::PairSigner;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to communicate with the chain.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Note: This can be replaced with the extrinsic hex from polkadot.js directly.
    let extrinsic_hex = {
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        let dest = AccountKeyring::Bob.to_account_id().into();

        let api = OnlineClient::<PolkadotConfig>::new().await?;

        let tx = polkadot::tx()
            .balances()
            .transfer(dest, 123_456_789_012_345);

        let signed_tx = api
            .tx()
            .create_signed(&tx, &signer, Default::default())
            .await?;
        let bytes = signed_tx.encoded();
        let extrinsic_hex = hex::encode(bytes);
        println!("Extrinsic hex: {:?}", extrinsic_hex);

        // TODO: Comment this.
        extrinsic_hex
        // TODO: Place your extrinsic here
        // "0x..".to_string()
    };

    println!("Decoding the extrinsic bytes...");
    // Obtain the raw bytes from the extrinsic hexadecimal representation.
    let extrinsic_bytes = hex::decode(extrinsic_hex.trim_start_matches("0x"))?;

    println!("Creating submittable extrinsic from raw bytes...");
    // The raw bytes are wrapped into `Encoded` to ensure that `scale::encode` calls
    // will have no effect upon the raw bytes.
    let extrinsic = SubmittableExtrinsic::from_bytes(api.clone(), extrinsic_bytes);

    println!("Submit extrinsic and wait for success...");
    let extrinic_events = extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?;

    println!("Finding extrinsic event...");

    // This presumes the extrinsic is a transfer one.
    let transfer_event =
        extrinic_events.find_first::<polkadot::balances::events::Transfer>()?;

    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    } else {
        println!("Failed to find Balances::Transfer Event");
    }

    Ok(())
}
