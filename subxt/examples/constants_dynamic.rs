#![allow(missing_docs)]
use subxt::dynamic::Value;
use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // A dynamic query to obtain some constant:
    let constant_query = subxt::dynamic::constant::<Value>("System", "BlockLength");

    // Obtain the decoded constant:
    let value = api.constants().at(&constant_query)?;

    // Or obtain the bytes for the constant:
    let bytes = api.constants().bytes_at(&constant_query)?;

    println!("Constant bytes: {:?}", bytes);
    println!("Constant value: {}", value);
    Ok(())
}
