use subxt::{OnlineClient, PolkadotConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/test_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

    // Build the query.
    let storage_query = polkadot::storage().paras().parachains();

    // Fetch the result.
    let result = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;

    println!("polkadot::storage().paras().parachains(): {result:?}");

    Ok(())
}
