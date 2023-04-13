// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot v0.9.28-9ffe6e9e3da.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.28/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use codec::{Decode, Encode};
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Example 1. Iterate over (keys, value) using the storage client.
    // This is the standard and most ergonomic approach.
    {
        let key_addr = polkadot::storage().xcm_pallet().version_notifiers_root();

        let mut iter = api.storage().at_latest().await?.iter(key_addr, 10).await?;

        println!("\nExample 1. Obtained keys:");
        while let Some((key, value)) = iter.next().await? {
            println!("Key: 0x{}", hex::encode(key));
            println!("  Value: {value}");
        }
    }

    // Example 2. Iterate over fetched keys manually. Here, you forgo any static type
    // safety and work directly with the bytes on either side.
    {
        let key_addr = polkadot::storage().xcm_pallet().version_notifiers_root();

        // Fetch at most 10 keys from below the prefix XcmPallet' VersionNotifiers.
        let keys = api
            .storage()
            .at_latest()
            .await?
            .fetch_keys(&key_addr.to_root_bytes(), 10, None)
            .await?;

        println!("Example 2. Obtained keys:");
        for key in keys.iter() {
            println!("Key: 0x{}", hex::encode(key));

            if let Some(storage_data) = api.storage().at_latest().await?.fetch_raw(&key.0).await? {
                // We know the return value to be `QueryId` (`u64`) from inspecting either:
                // - polkadot code
                // - polkadot.rs generated file under `version_notifiers()` fn
                // - metadata in json format
                let value = u64::decode(&mut &*storage_data)?;
                println!("  Value: {value}");
            }
        }
    }

    // Example 3. Custom iteration over double maps. Here, we manually append one lookup
    // key to the root and just iterate over the values underneath that.
    {
        let key_addr = polkadot::storage().xcm_pallet().version_notifiers_root();

        // Obtain the root bytes (`twox_128("XcmPallet") ++ twox_128("VersionNotifiers")`).
        let mut query_key = key_addr.to_root_bytes();

        // We know that the first key is a u32 (the `XcmVersion`) and is hashed by twox64_concat.
        // twox64_concat is just the result of running the twox_64 hasher on some value and concatenating
        // the value itself after it:
        query_key.extend(subxt::ext::sp_core::twox_64(&2u32.encode()));
        query_key.extend(&2u32.encode());

        // The final query key is essentially the result of:
        // `twox_128("XcmPallet") ++ twox_128("VersionNotifiers") ++ twox_64(scale_encode(2u32)) ++ scale_encode(2u32)`
        println!("\nExample 3\nQuery key: 0x{}", hex::encode(&query_key));

        let keys = api
            .storage()
            .at_latest()
            .await?
            .fetch_keys(&query_key, 10, None)
            .await?;

        println!("Obtained keys:");
        for key in keys.iter() {
            println!("Key: 0x{}", hex::encode(key));

            if let Some(storage_data) = api.storage().at_latest().await?.fetch_raw(&key.0).await? {
                // We know the return value to be `QueryId` (`u64`) from inspecting either:
                // - polkadot code
                // - polkadot.rs generated file under `version_notifiers()` fn
                // - metadata in json format
                let value = u64::decode(&mut &storage_data[..])?;
                println!("  Value: {value}");
            }
        }
    }

    Ok(())
}
