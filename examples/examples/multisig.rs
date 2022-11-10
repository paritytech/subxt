// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against substrate latest-38a955ba4a0.
//!
//! E.g.
//! ```bash
//! curl "https://releases.parity.io/substrate/x86_64-debian:stretch/latest/substrate/substrate" --output /usr/local/bin/substrate --location
//! substrate --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{
    tx::PairSigner,
    OnlineClient,
    SubstrateConfig,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/substrate_metadata.scale")]
pub mod substrate {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    // Create a client to use:
    let api = OnlineClient::<SubstrateConfig>::new().await?;

    // The call data of for the `Multisig::as_multi`.
    // This must be of type `RuntimeCall` and represents the `Assets::freeze_asset` method.
    // Otherwise, could have been obtained using
    // `substrate::tx().assets().freeze_asset(100)`
    let call = substrate::runtime_types::kitchensink_runtime::RuntimeCall::Assets(
        substrate::runtime_types::pallet_assets::pallet::Call::freeze_asset { id: 100 },
    );

    // The maximum weight (gas) for this extrinsic.
    let max_weight = substrate::runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 10000000000,
        proof_size: u64::MAX / 2,
    };

    // Construct the multisig extrinsic.
    let tx = substrate::tx().multisig().as_multi(
        // threashold
        1,
        // other signatories
        vec![AccountKeyring::Alice.to_account_id()],
        // maybe timepoint
        None,
        // call
        call,
        // max weight
        max_weight,
    );

    // Submit the transaction with default params:
    let hash = api.tx().sign_and_submit_default(&tx, &signer).await?;

    println!("Multisig extrinsic submitted: {:?}", hash);

    Ok(())
}
