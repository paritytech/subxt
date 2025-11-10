#![allow(missing_docs)]
use subxt::dynamic::Value;
use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // We can query a constant by providing a tuple of the pallet and constant name. The return type
    // will be `Value` if we pass this query:
    let constant_query = ("System", "BlockLength");
    let _value = api.constants().at(&constant_query)?;

    // Or we can use the library function to query a constant, which allows us to pass a generic type
    // that Subxt will attempt to decode the constant into:
    let constant_query = subxt::dynamic::constant::<Value>("System", "BlockLength");
    let value = api.constants().at(&constant_query)?;

    // Or we can obtain the bytes for the constant, using either form of query.
    let bytes = api.constants().bytes_at(&constant_query)?;

    println!("Constant bytes: {:?}", bytes);
    println!("Constant value: {}", value);
    Ok(())
}
