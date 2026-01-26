//! Fetching and iterating over storage entries.
use subxt::dynamic::Value;
use subxt::utils::AccountId32;
use subxt::{Error, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let at_block = api.at_current_block().await?;

    // Here we use a statically generated address to fetch the storage entry, which
    // gives us type information for future actions on it.
    let account_balances = at_block
        .storage()
        .entry(polkadot::storage().system().account())?;

    // We can see the default value for this entry at this block, if one exists.
    if let Some(default_value) = account_balances.default_value() {
        let default_balance_info = default_value.decode_as::<Value>()?;
        println!("Default balance info: {default_balance_info}");
    }

    // We can fetch a specific account balance by its key, like so (here I just picked a random key
    // I knew to exist from iterating over storage entries):
    let account_id = {
        let hex = "9a4d0faa2ba8c3cc5711852960940793acf55bf195b6eecf88fa78e961d0ce4a";
        let bytes: [u8; 32] = hex::decode(hex).unwrap().try_into().unwrap();
        AccountId32::from(bytes)
    };
    let entry = account_balances.fetch((account_id,)).await?;

    // We can decode the value into any type implementing DecodeAsType:
    let _balance_info = entry.decode_as::<Value>()?;
    // Or we can decode into the type stored by the static code, since we used a static address:
    let balance_info = entry.decode()?;

    println!(
        "Single balance info from {account_id} => free: {} reserved: {} frozen: {} flags: {:?}",
        balance_info.data.free,
        balance_info.data.reserved,
        balance_info.data.frozen,
        balance_info.data.flags,
    );

    // Or we can iterate over all of the account balances and print them out. Here we provide an
    // empty tuple, indicating that we want to iterate over everything and not only things under a certain key
    // (in the case of account balances, there is only one key anyway, but other storage entries may map from
    // several keys to a value, and for those we can choose which depth we iterate at by providing as many keys
    // as we want and leaving the rest).
    let mut all_balances = account_balances.iter(()).await?;
    while let Some(entry) = all_balances.next().await {
        let entry = entry?;
        let key = entry.key()?;

        // Because we provided a statically typed Address when we originally obtained this
        // storage entry (ie `polkadot::storage().system().account()`), we can statically
        // decode the key into its well typed constituent parts:
        let key_parts = key.decode()?;
        println!("Account ID: {}", key_parts.0);

        // Alternately, if we don't have type information available (or just want to decode
        // into some different type), we can do something like this instead:
        let account_id = key
            .part(0)
            .unwrap()
            .decode_as::<AccountId32>()?
            .expect("We expect this key to decode into a 32 byte AccountId");

        // Decode these values into our generic scale_value::Value type. Less efficient than
        // defining a static type as above, but easier for the sake of the example.
        let balance_info = entry.value().decode_as::<scale_value::Value>()?;
        println!("  {account_id} => {balance_info}");
    }

    Ok(())
}
