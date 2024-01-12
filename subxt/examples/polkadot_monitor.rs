#![allow(missing_docs)]
use subxt::{
    utils::{AccountId32, MultiAddress},
    OnlineClient, PolkadotConfig,
};

use codec::Decode;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

use polkadot::balances::calls::types::TransferKeepAlive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client that subscribes to blocks of the live Polkadot network.
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;

    // For each block, print details about the `TransferKeepAlive` transactions we are interested in.
    while let Some(block) = blocks_sub.next().await {
        let block = block?;
        let block_number = block.header().number;
        let block_hash = block.hash();
        println!("Block #{block_number}  ({block_hash}):");

        let extrinsics = block.extrinsics().await?;
        for ext in extrinsics.iter() {
            let ext = ext?;
            if let Ok(Some(transfer)) = ext.as_extrinsic::<TransferKeepAlive>() {
                if let Some(extensions) = ext.signed_extensions() {
                    ext.address_bytes().unwrap();
                    let addr_bytes = ext
                        .address_bytes()
                        .expect("TransferKeepAlive should be signed");
                    let sender = MultiAddress::<AccountId32, ()>::decode(&mut &addr_bytes[..])
                        .expect("Decoding should work");
                    let sender = display_address(&sender);
                    let receiver = display_address(&transfer.dest);
                    let value = transfer.value;
                    let tip = extensions.tip().expect("Should have tip");
                    let nonce = extensions.nonce().expect("Should have nonce");

                    println!(
                        "    Transfer of {value} DOT:\n        {sender} (Tip: {tip}, Nonce: {nonce}) ---> {receiver}",
                    );
                } else {
                    panic!("TransferKeepAlive should be signed")
                }
            }
        }
    }

    Ok(())
}

fn display_address(addr: &MultiAddress<AccountId32, ()>) -> String {
    if let MultiAddress::Id(id32) = addr {
        format!("{id32}")
    } else {
        "MultiAddress::...".into()
    }
}
