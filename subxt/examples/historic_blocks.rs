//! Working with historic blocks.
use subxt::dynamic::Value;
use subxt::{Error, OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Point at an archive node since we want to obtain old blocks.
    let config = PolkadotConfig::new();
    let api = OnlineClient::from_url(config, "wss://rpc.polkadot.io").await?;

    for block_number in 1234567u32.. {
        let at_block = api.at_block(block_number).await?;

        let block_number = at_block.block_number();
        let block_hash = at_block.block_hash();

        println!("Block #{block_number}:");
        println!("  Hash: {block_hash}");
        println!("  Extrinsics:");

        // Here we'll obtain and display the extrinsics:
        let extrinsics = at_block.extrinsics().fetch().await?;
        for ext in extrinsics.iter() {
            let ext = ext?;

            let idx = ext.index();
            let pallet_name = ext.pallet_name();
            let call_name = ext.call_name();
            let bytes_hex = format!("0x{}", hex::encode(ext.bytes()));
            let events = ext.events().await?;

            // See the API docs for more ways to decode extrinsics. Here, since we're
            // accessing a historic block, we don't use any generated types to help us.
            let decoded_ext = ext.decode_call_data_fields_unchecked_as::<Value>()?;

            println!("    #{idx}: {pallet_name}.{call_name}:");
            println!("      Bytes: {bytes_hex}");
            println!("      Decoded: {decoded_ext}");

            for evt in events.iter() {
                println!("      Events:");
                let evt = evt?;
                let event_idx = evt.event_index();
                let pallet_name = evt.pallet_name();
                let event_name = evt.event_name();
                let event_values = evt.decode_fields_unchecked_as::<Value>()?;

                println!("        #{event_idx}: {pallet_name}.{event_name}");
                println!("          {event_values}");
            }
        }
    }

    Ok(())
}
