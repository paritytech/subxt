#![allow(missing_docs)]
use subxt::dynamic::{At, Value};
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a dynamic storage query to access account information.
    // here, we assume that there is one value to provide at this entry
    // to access a value; an AccountId32. In this example we don't know the
    // return type and so we set it to `Value`, which anything can decode into.
    let account: AccountId32 = dev::alice().public_key().into();
    let storage_query = subxt::dynamic::storage::<(AccountId32,), Value>("System", "Account");

    // Use that query to access a storage entry, fetch a result and decode the value.
    let client_at = api.storage().at_latest().await?;
    let account_info = client_at
        .entry(storage_query)?
        .fetch((account,))
        .await?
        .decode()?;

    // With out `Value` type we can dig in to find what we want using the `At`
    // trait and `.at()` method that this provides on the Value.
    println!(
        "Alice has free balance: {}",
        account_info.at("data").at("free").unwrap()
    );
    Ok(())
}
