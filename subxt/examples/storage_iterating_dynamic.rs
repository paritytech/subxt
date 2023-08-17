use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a dynamic storage query to iterate account information.
    let storage_query = subxt::dynamic::storage_root("System", "Account");

    // Use that query to return an iterator over the results.
    let mut results = api
        .storage()
        .at_latest()
        .await?
        .iter(storage_query)
        .await?;

    while let Some(Ok((key, value))) = results.next_item().await {
        println!("Key: 0x{}", hex::encode(&key));
        println!("Value: {:?}", value.to_value()?);
    }

    Ok(())
}
