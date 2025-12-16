//! We can configure Subxt to use a Smoldot based lightclient to connect to a chain.
use futures::StreamExt;
use subxt::{OnlineClient, PolkadotConfig, lightclient::LightClient};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

const POLKADOT_SPEC: &str = include_str!("../../artifacts/demo_chain_specs/polkadot.json");
const ASSET_HUB_SPEC: &str =
    include_str!("../../artifacts/demo_chain_specs/polkadot_asset_hub.json");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The lightclient logs are informative:
    tracing_subscriber::fmt::init();

    // (Optional) for dev purposes, we can use a Subxt utility function to fetch a chainspec from
    // a locally running node if we like, but in this example we use some pre-baked chainspecs:
    let _chain_spec = subxt::utils::fetch_chainspec_from_rpc_node("ws://127.0.0.1:9944").await;

    // Instantiate a light client with the Polkadot relay chain,
    // and connect it to Asset Hub, too.
    let (lightclient, polkadot_rpc) = LightClient::relay_chain(POLKADOT_SPEC)?;
    let asset_hub_rpc = lightclient.parachain(ASSET_HUB_SPEC)?;

    // Create Subxt clients from these Smoldot backed RPC clients.
    let config = PolkadotConfig::new();
    let polkadot_api =
        OnlineClient::<PolkadotConfig>::from_rpc_client(config.clone(), polkadot_rpc).await?;
    let asset_hub_api =
        OnlineClient::<PolkadotConfig>::from_rpc_client(config, asset_hub_rpc).await?;

    // Now we can use them as with any other Subxt instance. Here we fetch finalized blocks
    // from both chains and print some detail about the contained extrinsics.
    let polkadot_sub = polkadot_api
        .stream_blocks()
        .await?
        .map(|block| ("Polkadot", block));
    let parachain_sub = asset_hub_api
        .stream_blocks()
        .await?
        .map(|block| ("AssetHub", block));

    let mut stream_combinator = futures::stream::select(polkadot_sub, parachain_sub);

    while let Some((chain, block)) = stream_combinator.next().await {
        let block = block?;

        // Print some details about the blocks we fetch via the light client.
        println!("Chain {:?} hash={:?}", chain, block.hash());
        let at_block = block.at().await?;
        let extrinsics = at_block.extrinsics().fetch().await?;
        for ext in extrinsics.iter() {
            let ext = ext?;

            let idx = ext.index();
            let pallet_name = ext.pallet_name();
            let call_name = ext.call_name();
            println!("    #{idx}: {pallet_name}.{call_name}");
        }
    }

    Ok(())
}
