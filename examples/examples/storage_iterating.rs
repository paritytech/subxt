use subxt::{OnlineClient, PolkadotConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a storage query to iterate over account information.
    let storage_query = polkadot::storage().system().account_root();

    // Get back an iterator of results (we acquire 10 at a time, here).
    let mut results = api
        .storage()
        .at_latest()
        .await?
        .iter(storage_query, 10)
        .await?;

    while let Some((key, value)) = results.next().await? {
        println!("Key: 0x{}", hex::encode(&key));
        println!("Value: {:?}", value);
    }

    Ok(())
}
