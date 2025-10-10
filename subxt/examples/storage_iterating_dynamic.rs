#![allow(missing_docs)]
use subxt::ext::futures::StreamExt;
use subxt::utils::AccountId32;
use subxt::{
    OnlineClient, PolkadotConfig,
    dynamic::{At, Value},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a dynamic storage query to access account information.
    // here, we assume that there is one value to provide at this entry
    // to access a value; an AccountId32. In this example we don't know the
    // return type and so we set it to `Value`, which anything can decode into.
    let storage_query = subxt::dynamic::storage::<(AccountId32,), Value>("System", "Account");

    // Use that query to access a storage entry, iterate over it and decode values.
    let client_at = api.storage().at_latest().await?;
    let entry = client_at.entry(storage_query)?;
    let mut values = entry.iter(()).await?;

    while let Some(kv) = values.next().await {
        let kv = kv?;

        // The key decodes into the first type we provided in the address. Since there's just
        // one key, it is a tuple of one entry, an AccountId32. If we didn't know how many
        // keys or their type, we could set the key to `Vec<Value>` instead.
        let (account_id32,) = kv.key()?.decode()?;

        // The value decodes into the second type we provided in the address. In this example,
        // we just decode it into our `Value` type and then look at the "data" field in this
        // (which implicitly assumes we get a struct shaped thing back with such a field).
        let value = kv.value().decode()?;

        let value_data = value.at("data").unwrap();
        println!("{account_id32}:\n  {value_data}");
    }

    Ok(())
}
