#![allow(missing_docs)]
use subxt_historic::{Error, OnlineClient, PolkadotConfig, ext::StreamExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Configuration for the Polkadot relay chain.
    let config = PolkadotConfig::new();

    // Create an online client for the Polkadot relay chain, pointed at a Polkadot archive node.
    let client = OnlineClient::from_url(config, "wss://rpc.polkadot.io").await?;

    // Iterate through some randomly selected blocks to show how to fetch and decode storage.
    for block_number in 12345678.. {
        println!("=== Block {block_number} ===");

        // Point the client at a specific block number. By default this will download and cache
        // metadata for the required spec version (so it's cheaper to instantiate again), if it
        // hasn't already, and borrow the relevant legacy types from the client.
        let client_at_block = client.at(block_number).await?;

        // We'll work the account balances at the given block, for this example.
        let account_balances = client_at_block
            .storage()
            .entry("System", "Account")?
            .into_map()?;

        // We can fetch a specific account balance by its key, like so (here I just picked a random key
        // I knew to exist from iterating over storage entries):
        let account_id_hex = "9a4d0faa2ba8c3cc5711852960940793acf55bf195b6eecf88fa78e961d0ce4a";
        let account_id = hex::decode(account_id_hex).unwrap();
        if let Some(entry) = account_balances.fetch((account_id,)).await? {
            // We can decode the value into our generic `scale_value::Value` type, which can
            // represent any SCALE-encoded value, like so:
            let _balance_info = entry.decode::<scale_value::Value>()?;

            // Or, if we know what shape to expect, we can decode the parts of the value that we care
            // about directly into a static type, which is more efficient and allows easy type-safe
            // access, like so:
            #[derive(scale_decode::DecodeAsType)]
            struct BalanceInfo {
                data: BalanceInfoData,
            }
            #[derive(scale_decode::DecodeAsType)]
            struct BalanceInfoData {
                free: u128,
                reserved: u128,
                misc_frozen: u128,
                fee_frozen: u128,
            }
            let balance_info = entry.decode::<BalanceInfo>()?;

            println!(
                "  Single balance info from {account_id_hex} => free: {} reserved: {} misc_frozen: {} fee_frozen: {}",
                balance_info.data.free,
                balance_info.data.reserved,
                balance_info.data.misc_frozen,
                balance_info.data.fee_frozen,
            );
        }

        // Or we can iterate over all of the account balances and print them out, like so. Here we provide an
        // empty tuple, indicating that we want to iterate over everything and not only things under a certain key
        // (in the case of account balances, there is only one key anyway, but other storage entries may map from
        // several keys to a value, and for those we can choose which depth we iterate at by providing as many keys
        // as we want and leaving the rest). Here I only take the first 10 accounts I find for the sake of the example.
        let mut all_balances = account_balances.iter(()).await?.take(10);
        while let Some(entry) = all_balances.next().await {
            let entry = entry?;
            let key = entry.decode_key()?;

            // Decode the account ID from the key (we know here that we're working
            // with a map which has one value, an account ID, so we just decode that part:
            let account_id = key
                .part(0)
                .unwrap()
                .decode::<[u8; 32]>()?
                .expect("We expect this key to decode into a 32 byte AccountId");

            let account_id_hex = hex::encode(account_id);

            // Decode these values into our generic scale_value::Value type. Less efficient than
            // defining a static type as above, but easier for the sake of the example.
            let balance_info = entry.decode_value::<scale_value::Value>()?;
            println!("  {account_id_hex} => {balance_info}");
        }
    }

    Ok(())
}
