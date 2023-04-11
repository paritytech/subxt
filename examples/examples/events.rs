use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Get events for the latest block:
    let events = api.events().at(None).await?;

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

    // Or we can attempt to decode them into a specific arbitraty pallet enum
    // (We could also set the output type to Value to dynamically decode, here):
    println!("Event details for Balances pallet:");
    for event in events.iter() {
        let event = event?;

        if let Ok(ev) = event.as_pallet_event::<polkadot::balances::Event>() {
            println!("{ev:?}");
        } else {
            continue;
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
