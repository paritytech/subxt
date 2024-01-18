#![allow(missing_docs)]
use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client that subscribes to blocks of the Polkadot network.
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;
    while let Some(block) = blocks_sub.next().await {
        let block = block?;
        let block_number = block.header().number;
        let block_hash = block.hash();
        println!("Block #{block_number}  ({block_hash})");

        // Decode each signed extrinsic in the block dynamically
        let extrinsics = block.extrinsics().await?;
        for ext in extrinsics.iter() {
            let ext = ext?;

            let Some(signed_extensions) = ext.signed_extensions() else {
                continue; // we do not look at inherents in this example
            };

            let meta = ext.extrinsic_metadata()?;
            let fields = ext.field_values()?;

            println!("  {}/{}", meta.pallet.name(), meta.variant.name);
            println!("    Signed Extensions:");
            for signed_ext in signed_extensions.iter() {
                let signed_ext = signed_ext?;
                // We only want to take a look at these 3 signed extensions, because the others all just have unit fields.
                if ["CheckMortality", "CheckNonce", "ChargeTransactionPayment"]
                    .contains(&signed_ext.name())
                {
                    println!("      {}: {}", signed_ext.name(), signed_ext.value()?);
                }
            }
            println!("    Fields:");
            println!("      {}\n", fields);
        }
    }

    Ok(())
}
