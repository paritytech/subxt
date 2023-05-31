use futures::StreamExt;
use std::fmt::Write;
use subxt::{self, OnlineClient, PolkadotConfig};
use yew::{AttrValue, Callback};

#[subxt::subxt(runtime_metadata_path = "../../artifacts/polkadot_metadata_small.scale")]
mod polkadot {}

pub(crate) async fn fetch_constant_block_length() -> Result<String, subxt::Error> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let constant_query = polkadot::constants().system().block_length();

    let value = api.constants().at(&constant_query)?;
    Ok(format!("{value:?}"))
}

pub(crate) async fn fetch_events_dynamically() -> Result<Vec<String>, subxt::Error> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let events = api.events().at_latest().await?;
    let mut event_strings = Vec::<String>::new();
    for event in events.iter() {
        let event = event?;
        let pallet = event.pallet_name();
        let variant = event.variant_name();
        let field_values = event.field_values()?;
        event_strings.push(format!("{pallet}::{variant}: {field_values}"));
    }
    Ok(event_strings)
}

/// subscribes to finalized blocks. When a block is received, it is formatted as a string and sent via the callback.
pub(crate) async fn subscribe_to_finalized_blocks(
    cb: Callback<AttrValue>,
) -> Result<(), subxt::Error> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;
    while let Some(block) = blocks_sub.next().await {
        let block = block?;
        let mut output = String::new();
        writeln!(output, "Block #{}:", block.header().number).ok();
        writeln!(output, "  Hash: {}", block.hash()).ok();
        writeln!(output, "  Extrinsics:").ok();
        let body = block.body().await?;
        for ext in body.extrinsics().iter() {
            let ext = ext?;
            let idx = ext.index();
            let events = ext.events().await?;
            let bytes_hex = format!("0x{}", hex::encode(ext.bytes()));

            // See the API docs for more ways to decode extrinsics:
            let decoded_ext = ext.as_root_extrinsic::<polkadot::Call>();

            writeln!(output, "    Extrinsic #{idx}:").ok();
            writeln!(output, "      Bytes: {bytes_hex}").ok();
            writeln!(output, "      Decoded: {decoded_ext:?}").ok();
            writeln!(output, "      Events:").ok();

            for evt in events.iter() {
                let evt = evt?;

                let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();
                let event_values = evt.field_values()?;

                writeln!(output, "        {pallet_name}_{event_name}").ok();
                writeln!(output, "          {}", event_values).ok();
            }
        }
        cb.emit(output.into())
    }
    Ok(())
}
