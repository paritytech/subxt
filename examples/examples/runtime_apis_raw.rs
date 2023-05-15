use subxt::ext::codec::Compact;
use subxt::ext::frame_metadata::RuntimeMetadataPrefixed;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Use runtime APIs at the latest block:
    let runtime_apis = api.runtime_api().at_latest().await?;

    // Ask for metadata and decode it:
    let (_, meta): (Compact<u32>, RuntimeMetadataPrefixed) =
        runtime_apis.call_raw("Metadata_metadata", None).await?;

    println!("{meta:?}");
    Ok(())
}
