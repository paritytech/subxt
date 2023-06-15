use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // A dynamic query to obtain some contant:
    let constant_query = subxt::dynamic::constant("System", "BlockLength");

    // Obtain the value:
    let value = api.constants().at(&constant_query)?;

    println!("Constant bytes: {:?}", value.encoded());
    println!("Constant value: {}", value.to_value()?);
    Ok(())
}
