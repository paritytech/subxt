use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Get events for the latest block:
    let events = api.events().at_latest().await?;

    // We can dynamically decode events:
    println!("Dynamic event details:");
    for event in events.iter() {
        let event = event?;

        let pallet = event.pallet_name();
        let variant = event.variant_name();
        let field_values = event.field_values()?;

        println!("{pallet}::{variant}: {field_values}");
    }

    // Or we can attempt to statically decode them into the root Event type:
    println!("Static event details:");
    for event in events.iter() {
        let event = event?;

        if let Ok(ev) = event.as_root_event::<polkadot::Event>() {
            println!("{ev:?}");
        } else {
            println!("<Cannot decode event>");
        }
    }

    // Or we can look for specific events which match our statically defined ones:
    let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    if let Some(ev) = transfer_event {
        println!("  - Balance transfer success: value: {:?}", ev.amount);
    } else {
        println!("  - No balance transfer event found in this block");
    }

    Ok(())
}
