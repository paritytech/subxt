// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot polkadot 0.9.25-5174e9ae75b.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.25/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use codec::Decode;
use subxt::{
    storage::address::{
        StorageHasher,
        StorageMapKey,
    },
    OnlineClient,
    PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // The VersionNotifiers type of the XcmPallet is defined as:
    //
    // ```
    //  All locations that we have requested version notifications from.
    // 	#[pallet::storage]
    // 	pub(super) type VersionNotifiers<T: Config> = StorageDoubleMap<
    // 		_,
    // 		Twox64Concat,
    // 		XcmVersion,
    // 		Blake2_128Concat,
    // 		VersionedMultiLocation,
    // 		QueryId,
    // 		OptionQuery,
    // 	>;
    // ```

    // Example 1. Iterate over fetched keys manually.
    {
        let key_addr = polkadot::storage().xcm_pallet().version_notifiers_root();

        // Fetch at most 10 keys from below the prefix XcmPallet' VersionNotifiers.
        let keys = api
            .storage()
            .fetch_keys(&key_addr.to_root_bytes(), 10, None, None)
            .await?;

        println!("Example 1. Obtained keys:");
        for key in keys.iter() {
            println!("Key: 0x{}", hex::encode(&key));

            if let Some(storage_data) = api.storage().fetch_raw(&key.0, None).await? {
                // We know the return value to be `QueryId` (`u64`) from inspecting either:
                // - polkadot code
                // - polkadot.rs generated file under `version_notifiers()` fn
                // - metadata in json format
                let value = u64::decode(&mut &*storage_data)?;
                println!("  Value: {}", value);
            }
        }
    }

    // Example 2. Iterate over (keys, value) using the storage client.
    {
        let key_addr = polkadot::storage().xcm_pallet().version_notifiers_root();

        let mut iter = api.storage().iter(key_addr, 10, None).await?;

        println!("\nExample 2. Obtained keys:");
        while let Some((key, value)) = iter.next().await? {
            println!("Key: 0x{}", hex::encode(&key));
            println!("  Value: {}", value);
        }
    }

    // Example 4. Custom iteration over double maps.
    {
        // Obtain the inner RPC from the API.
        let rpc = api.rpc();

        let key_addr = polkadot::storage().xcm_pallet().version_notifiers_root();

        // Obtain the prefixed `twox_128("XcmPallet") ++ twox_128("VersionNotifiers")`
        let mut query_key = key_addr.to_bytes();

        // From the VersionNotifiers definition above, the first key is represented by
        // ```
        // 		Twox64Concat,
        // 		XcmVersion,
        // ```
        // while `XcmVersion` is `u32`.
        // Pass `2` as `XcmVersion` and concatenate the key to the prefix.
        StorageMapKey::new(&2u32, StorageHasher::Twox64Concat).to_bytes(&mut query_key);

        // The final query key is:
        // `twox_128("XcmPallet") ++ twox_128("VersionNotifiers") ++ twox_64(2u32) ++ 2u32`
        println!("\nExample 4\nQuery key: 0x{}", hex::encode(&query_key));

        let keys = rpc.storage_keys_paged(&query_key, 10, None, None).await?;

        println!("Obtained keys:");
        for key in keys.iter() {
            println!("Key: 0x{}", hex::encode(&key));

            if let Some(storage_data) = api.storage().fetch_raw(&key.0, None).await? {
                // We know the return value to be `QueryId` (`u64`) from inspecting either:
                // - polkadot code
                // - polkadot.rs generated file under `version_notifiers()` fn
                // - metadata in json format
                let value = u64::decode(&mut &storage_data[..])?;
                println!("  Value: {}", value);
            }
        }
    }

    Ok(())
}
