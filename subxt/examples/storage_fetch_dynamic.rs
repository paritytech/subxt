use subxt::dynamic::{At, Value};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a dynamic storage query to access account information.
    let account = dev::alice().public_key();
    let storage_query =
        subxt::dynamic::storage("System", "Account", vec![Value::from_bytes(account)]);

    // Use that query to `fetch` a result. Because the query is dynamic, we don't know what the result
    // type will be either, and so we get a type back that can be decoded into a dynamic Value type.
    let result = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let value = result.unwrap().to_value()?;

    println!("Alice has free balance: {:?}", value.at("data").at("free"));
    Ok(())
}
