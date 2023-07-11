//! In this example we connect to the official polkadot asset hub parachain and subscribe to blocks that get produced.
//! You can just switch out `StatemintConfig` for `StatemintConfigWithSubxtTypes` or `StatemintConfigComposed` and the behavior should be the same.
//!
//! To run this example:
//! ```txt
//! cargo run --bin fetch_blocks
//! ```

use parachain_example::statemint;
use futures::StreamExt;
use parachain_example::statemint_config_composed::StatemintConfig;
use subxt::OnlineClient;

/// cargo run --bin fetch_blocks
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_endpoint = "wss://polkadot-asset-hub-rpc.polkadot.io:443";

    // here we use the config:
    let api = OnlineClient::<StatemintConfig>::from_url(rpc_endpoint).await?;

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;

    // For each block, print a bunch of information about it:
    while let Some(block) = blocks_sub.next().await {
        let block = block?;

        let block_number = block.header().number;
        let block_hash = block.hash();

        println!("Block #{block_number}:");
        println!("  Hash: {block_hash}");
        println!("  Extrinsics:");

        let body = block.body().await?;
        for ext in body.extrinsics().iter() {
            let ext = ext?;
            let idx = ext.index();
            let events = ext.events().await?;

            // here we make use of the generated metadata code:
            let decoded_ext = ext.as_root_extrinsic::<statemint::Call>();

            println!("    Extrinsic #{idx}:");
            println!("      Bytes: {}", ext.bytes().len());
            println!("      Decoded: {decoded_ext:?}");
            println!("      Events:");

            for evt in events.iter() {
                let evt = evt?;

                let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();
                let event_values = evt.field_values()?;

                println!("        {pallet_name}_{event_name}");
                println!("          {}", event_values);
            }
        }
    }

    Ok(())
}
