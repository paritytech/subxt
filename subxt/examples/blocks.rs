//! Subscribe to blocks.
use subxt::dynamic::Value;
use subxt::{Error, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let config = PolkadotConfig::new();
    let api = OnlineClient::new(config).await?;

    // Stream the finalized blocks. See the OnlineClient docs for how to
    // stream best blocks or all new blocks.
    let mut blocks = api.stream_blocks().await?;

    while let Some(block) = blocks.next().await {
        let block = block?;

        let block_number = block.number();
        let block_hash = block.hash();

        println!("Block #{block_number}:");
        println!("  Hash: {block_hash}");
        println!("  Extrinsics:");

        // Create a client to work at this block. From here you have the
        // same interface as `api.block(block.hash()).await`.
        let at_block = block.at().await?;

        // Here we'll iterate over the extrinsics and display information about each one:
        let extrinsics = at_block.extrinsics().fetch().await?;
        for ext in extrinsics.iter() {
            let ext = ext?;

            let idx = ext.index();
            let pallet_name = ext.pallet_name();
            let call_name = ext.call_name();
            let bytes_hex = format!("0x{}", hex::encode(ext.bytes()));
            let events = ext.events().await?;

            // See the API docs for more ways to decode extrinsics. Here we decode into
            // a statically generated type, but any type implementing scale_decode::DecodeAsType
            // can be used here, for instance subxt::dynamic::Value.
            let decoded_ext = ext.decode_call_data_as::<polkadot::Call>()?;

            println!("    #{idx}: {pallet_name}.{call_name}:");
            println!("      Bytes: {bytes_hex}");
            println!("      Decoded: {decoded_ext:?}");

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

            if let Some(transaction_extensions) = ext.transaction_extensions() {
                println!("      Transaction Extensions:");
                for transaction_extension in transaction_extensions.iter() {
                    let name = transaction_extension.name();
                    let value = transaction_extension.decode_unchecked_as::<Value>()?;
                    println!("        {name}: {value}");
                }
            }
        }

        // Instead of iterating, we can also use the static interface to search & decode specific
        // extrinsics if we know what we are looking for:
        if let Some(ext) = extrinsics.find_first::<polkadot::para_inherent::calls::Enter>() {
            let ext = ext?;

            println!("ParaInherent.Enter");
            println!("  backed_candidated: {:?}", ext.data.backed_candidates);
            println!("  disputes: {:?}", ext.data.disputes);
        }
    }

    Ok(())
}
