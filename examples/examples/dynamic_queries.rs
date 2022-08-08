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

// This example showcases working with dynamic values rather than those that are generated via the subxt proc macro.

use sp_keyring::AccountKeyring;
use subxt::{
    dynamic::Value,
    tx::PairSigner,
    OnlineClient,
    PolkadotConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // 1. Dynamic Balance Transfer (the dynamic equivalent to the balance_transfer example).

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id();

    // Create a transaction to submit:
    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer",
        vec![
            // A value representing a MultiAddress<AccountId32, _>. We want the "Id" variant, and that
            // will ultimately contain the bytes for our destination address (there is a new type wrapping
            // the address, but our encoding will happily ignore such things and do it's best to line up what
            // we provide with what it needs).
            Value::unnamed_variant("Id", [Value::from_bytes(&dest)]),
            // A value representing the amount we'd like to transfer.
            Value::u128(123_456_789_012_345),
        ],
    );

    // submit the transaction with default params:
    let hash = api.tx().sign_and_submit_default(&tx, &signer).await?;
    println!("Balance transfer extrinsic submitted: {}", hash);

    // 2. Dynamic constant access (the dynamic equivalent to the fetch_constants example).

    let constant_address = subxt::dynamic::constant("Balances", "ExistentialDeposit");
    let existential_deposit = api.constants().at(&constant_address)?;
    println!("Existential Deposit: {}", existential_deposit);

    // 3. Dynamic storage access

    let storage_address = subxt::dynamic::storage(
        "System",
        "Account",
        vec![
            // Something that encodes to an AccountId32 is what we need for the map key here:
            Value::from_bytes(&dest),
        ],
    );
    let account = api
        .storage()
        .fetch_or_default(&storage_address, None)
        .await?;
    println!("Bob's account details: {account}");

    // 4. Dynamic storage iteration (the dynamic equivalent to the fetch_all_accounts example).

    let storage_address = subxt::dynamic::storage_root("System", "Account");
    let mut iter = api.storage().iter(storage_address, 10, None).await?;
    while let Some((key, account)) = iter.next().await? {
        println!("{}: {}", hex::encode(key), account);
    }

    Ok(())
}
