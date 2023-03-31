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
use subxt::{dynamic::Value, tx::PairSigner, OnlineClient, PolkadotConfig};

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
    let inner_tx = subxt::dynamic::tx(
        "Balances",
        "transfer",
        vec![
            Value::unnamed_variant("Id", [Value::from_bytes(&dest)]),
            Value::u128(123_456_789_012_345),
        ],
    );

    // Now, build an outer call which this inner call will be a part of.
    // This sets up the multisig arrangement.
    //
    // Note: Since this is a dynamic call, we can either use named or unnamed
    // arguments (if unnamed, the order matters).
    let tx = subxt::dynamic::tx(
        "Multisig",
        "as_multi",
        vec![
            ("threshold", Value::u128(1)),
            (
                "other_signatories",
                Value::unnamed_composite([Value::from_bytes(&signer_account_id)]),
            ),
            ("maybe_timepoint", Value::unnamed_variant("None", [])),
            ("call", inner_tx.into_value()),
            (
                "max_weight",
                Value::named_composite([
                    ("ref_time", Value::u128(10000000000)),
                    ("proof_size", Value::u128(1)),
                ]),
            ),
        ],
    );

    // Submit it:
    let encoded = hex::encode(api.tx().call_data(&tx)?);
    println!("Call data: {encoded}");
    let tx_hash = api.tx().sign_and_submit_default(&tx, &signer).await?;
    println!("Submitted tx with hash {tx_hash}");

    Ok(())
}
