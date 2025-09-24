#![allow(missing_docs)]
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../westend_ah.scale")]
pub mod runtime {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://westend-asset-hub-rpc.polkadot.io").await?;

    let query = runtime::storage().staking().era_pruning_state_iter();
    let mut results = api.storage().at_latest().await?.iter(query).await?;

    while let Some(Ok(kv)) = results.next().await {
        println!("Keys decoded: {:?}", kv.keys);
        println!("Key: 0x{}", hex::encode(&kv.key_bytes));
        println!("Value: {:?}", kv.value);
    }

    Ok(())
}
