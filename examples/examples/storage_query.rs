// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! To run this example, a local polkadot node should be running. Example verified against polkadot 0.9.18-4542a603cc-aarch64-macos.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.18/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use codec::Decode;
use subxt::{
    rpc::Rpc,
    storage::{
        StorageClient,
        StorageKeyPrefix,
    },
    ClientBuilder,
    DefaultConfig,
    PolkadotExtrinsicParams,
    StorageEntryKey,
    StorageMapKey,
};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    // Obtain the storage client wrapper from the API.
    let storage: StorageClient<_> = api.client.storage();

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
        // Fetch at most 10 keys from below the prefix XcmPallet' VersionNotifiers.
        let keys = storage
            .fetch_keys::<polkadot::xcm_pallet::storage::VersionNotifiers>(10, None, None)
            .await?;

        println!("Example 1. Obtained keys:");
        for key in keys.iter() {
            println!("Key: 0x{}", hex::encode(&key));

            if let Some(storage_data) = storage.fetch_raw(key.clone(), None).await? {
                // We know the return value to be `QueryId` (`u64`) from inspecting either:
                // - polkadot code
                // - polkadot.rs generated file under `version_notifiers()` fn
                // - metadata in json format
                let value = u64::decode(&mut &storage_data.0[..])?;
                println!("  Value: {}", value);
            }
        }
    }

    // Example 2. Iterate over (keys, value) using the storage client.
    {
        let mut iter = storage
            .iter::<polkadot::xcm_pallet::storage::VersionNotifiers>(None)
            .await?;

        println!("\nExample 2. Obtained keys:");
        while let Some((key, value)) = iter.next().await? {
            println!("Key: 0x{}", hex::encode(&key));
            println!("  Value: {}", value);
        }
    }

    // Example 3. Iterate over (keys, value) using the polkadot API.
    {
        let mut iter = api
            .storage()
            .xcm_pallet()
            .version_notifiers_iter(None)
            .await?;

        println!("\nExample 3. Obtained keys:");
        while let Some((key, value)) = iter.next().await? {
            println!("Key: 0x{}", hex::encode(&key));
            println!("  Value: {}", value);
        }
    }

    // Example 4. Custom iteration over double maps.
    {
        // Obtain the inner RPC from the API.
        let rpc: &Rpc<_> = api.client.rpc();

        // Obtain the prefixed `twox_128("XcmPallet") ++ twox_128("VersionNotifiers")`
        let prefix =
            StorageKeyPrefix::new::<polkadot::xcm_pallet::storage::VersionNotifiers>();
        // From the VersionNotifiers definition above, the first key is represented by
        // ```
        // 		Twox64Concat,
        // 		XcmVersion,
        // ```
        // while `XcmVersion` is `u32`.
        // Pass `2` as `XcmVersion` and concatenate the key to the prefix.
        let entry_key = StorageEntryKey::Map(vec![StorageMapKey::new(
            &2u32,
            ::subxt::StorageHasher::Twox64Concat,
        )]);

        // The final query key is:
        // `twox_128("XcmPallet") ++ twox_128("VersionNotifiers") ++ twox_64(2u32) ++ 2u32`
        let query_key = entry_key.final_key(prefix);
        println!("\nExample 4\nQuery key: 0x{}", hex::encode(&query_key));

        let keys = rpc
            .storage_keys_paged(Some(query_key), 10, None, None)
            .await?;

        println!("Obtained keys:");
        for key in keys.iter() {
            println!("Key: 0x{}", hex::encode(&key));

            if let Some(storage_data) = storage.fetch_raw(key.clone(), None).await? {
                // We know the return value to be `QueryId` (`u64`) from inspecting either:
                // - polkadot code
                // - polkadot.rs generated file under `version_notifiers()` fn
                // - metadata in json format
                let value = u64::decode(&mut &storage_data.0[..])?;
                println!("  Value: {}", value);
            }
        }
    }

    Ok(())
}
