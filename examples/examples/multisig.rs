// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot v0.9.31-3711c6f9b2a.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.31/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // My account.
    let signer_account = AccountKeyring::Alice;
    let signer_account_id = signer_account.to_account_id();
    let signer = PairSigner::new(signer_account.pair());

    // Transfer balance to this destination:
    let dest = AccountKeyring::Bob.to_account_id();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Create the inner balance transfer call.
    //
    // Note: This call, being manually constructed, will have a specific pallet and call index
    // which is determined by the generated code. If you're trying to submit this to a node which
    // has the pallets/calls at different indexes, it will fail. See `dynamic_multisig.rs` for a
    // workaround in this case which will work regardless of pallet and call indexes.
    let inner_tx = polkadot::runtime_types::polkadot_runtime::RuntimeCall::Balances(
        polkadot::runtime_types::pallet_balances::pallet::Call::transfer {
            dest: dest.into(),
            value: 123_456_789_012_345,
        },
    );

    // Now, build an outer call which this inner call will be a part of.
    // This sets up the multisig arrangement.
    let tx = polkadot::tx().multisig().as_multi(
        // threshold
        1,
        // other signatories
        vec![signer_account_id.into()],
        // maybe timepoint
        None,
        // call
        inner_tx,
        // max weight
        polkadot::runtime_types::sp_weights::weight_v2::Weight {
            ref_time: 10000000000,
            proof_size: 1,
        },
    );

    // Submit the extrinsic with default params:
    let encoded = hex::encode(api.tx().call_data(&tx)?);
    println!("Call data: {encoded}");
    let tx_hash = api.tx().sign_and_submit_default(&tx, &signer).await?;
    println!("Submitted tx with hash {tx_hash}");

    Ok(())
}
