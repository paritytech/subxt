#![allow(missing_docs)]
use subxt_historic::{Error, OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Configuration for the Polkadot relay chain.
    let config = PolkadotConfig::new();

    // Create an online client for the Polkadot relay chain, pointed at a Polkadot archive node.
    let client = OnlineClient::from_url(config, "wss://rpc.polkadot.io").await?;

    // Iterate through some randomly selected old blocks to show how to fetch and decode extrinsics.
    for block_number in 123456.. {
        println!("=== Block {block_number} ===");

        // Point the client at a specific block number. By default this will download and cache
        // metadata for the required spec version (so it's cheaper to instantiate again), if it
        // hasn't already, and borrow the relevant legacy types from the client.
        let client_at_block = client.at(block_number).await?;

        // Fetch the extrinsics at that block.
        let extrinsics = client_at_block.extrinsics().fetch().await?;

        // Now, we have various operations to work with them. Here we print out various details
        // about each extrinsic.
        for extrinsic in extrinsics.iter() {
            println!(
                "{}.{}",
                extrinsic.call().pallet_name(),
                extrinsic.call().name()
            );

            if let Some(signature) = extrinsic.signature_bytes() {
                println!("  Signature: 0x{}", hex::encode(signature));
            }

            println!("  Call Data:");

            // We can decode each of the fields (in this example we decode everything into a
            // scale_value::Value type, which can represent any SCALE encoded data, but if you
            // have an idea of the type then you can try to decode into that type instead):
            for field in extrinsic.call().fields().iter() {
                println!(
                    "    {}: {}",
                    field.name(),
                    field.decode::<scale_value::Value>().unwrap()
                );
            }

            // Or, all of them at once:
            println!(
                "    All: {}",
                extrinsic
                    .call()
                    .fields()
                    .decode::<scale_value::Composite<_>>()
                    .unwrap()
            );

            // We can also look at things like the transaction extensions:
            if let Some(extensions) = extrinsic.transaction_extensions() {
                println!("  Transaction Extensions:");

                // We can decode each of them:
                for extension in extensions.iter() {
                    println!(
                        "    {}: {}",
                        extension.name(),
                        extension.decode::<scale_value::Value>().unwrap()
                    );
                }

                // Or all of them at once:
                println!(
                    "    All: {}",
                    extensions.decode::<scale_value::Composite<_>>().unwrap()
                );
            }
        }
    }

    Ok(())
}
