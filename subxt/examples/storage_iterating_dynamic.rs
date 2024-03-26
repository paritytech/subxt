#![allow(missing_docs)]
use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a dynamic storage query to iterate account information.
    // With a dynamic query, we can just provide an empty vector as the keys to iterate over all entries.
    let keys: Vec<scale_value::Value> = vec![];
    let storage_query = subxt::dynamic::storage("System", "Account", keys);

    // Use that query to return an iterator over the results.
    let mut results = api.storage().at_latest().await?.iter(storage_query).await?;

    while let Some(Ok(kv)) = results.next().await {
        println!("Keys decoded: {:?}", kv.keys);
        println!("Key: 0x{}", hex::encode(&kv.key_bytes));
        println!("Value: {:?}", kv.value.to_value()?);
    }

    Ok(())
}
