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
        .iter(storage_query, 10)
        .await?;

    while let Some((key, value)) = results.next().await? {
        println!("Key: 0x{}", hex::encode(&key));
        println!("Value: {:?}", value.to_value()?);
    }

    Ok(())
}
