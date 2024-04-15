#![allow(missing_docs)]
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build a storage query to access account information.
    let account = dev::alice().public_key().into();
    let storage_query = polkadot::storage().system().account(&account);

    // Use that query to `fetch` a result. This returns an `Option<_>`, which will be
    // `None` if no value exists at the given address. You can also use `fetch_default`
    // where applicable, which will return the default value if none exists.
    let result = api
        .storage()
        .at_latest()
        .await?
        .fetch_raw(subxt_core::storage::get_address_bytes(&storage_query, &api.metadata()).unwrap())
        .await?;

    let v = hex::encode(result.unwrap());
    println!("Alice: {v}");
    Ok(())
}
