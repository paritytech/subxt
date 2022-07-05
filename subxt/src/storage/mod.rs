// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Query the runtime storage using [StorageClient].
//!
//! This module is the core of performing runtime storage queries. While you can
//! work with it directly, it's prefer to use the generated `storage()` interface where
//! possible.
//!
//! The exposed API is performing RPC calls to `state_getStorage` and `state_getKeysPaged`.
//!
//! A runtime storage entry can be of type:
//! - [StorageEntryKey::Plain] for keys constructed just from the prefix
//!   `twox_128(pallet) ++ twox_128(storage_item)`
//! - [StorageEntryKey::Map] for mapped keys constructed from the prefix,
//!   plus other arguments `twox_128(pallet) ++ twox_128(storage_item) ++ hash(arg1) ++ arg1`
//!
//! # Examples
//!
//! ## Fetch Storage Keys
//!
//! ```no_run
//! # use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
//! # use subxt::storage::StorageClient;
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
//! pub mod polkadot {}
//!
//! # #[tokio::main]
//! # async fn main() {
//! # let api = ClientBuilder::new()
//! #     .build()
//! #     .await
//! #     .unwrap()
//! #     .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
//! # // Obtain the storage client wrapper from the API.
//! # let storage: StorageClient<_> = api.client.storage();
//! // Fetch just the keys, returning up to 10 keys.
//! let keys = storage
//!     .fetch_keys::<polkadot::xcm_pallet::storage::VersionNotifiers>(10, None, None)
//!     .await
//!     .unwrap();
//! // Iterate over each key
//! for key in keys.iter() {
//!     println!("Key: 0x{}", hex::encode(&key));
//! }
//! # }
//! ```
//!
//! ## Iterate over Storage
//!
//! ```no_run
//! # use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
//! # use subxt::storage::StorageClient;
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
//! pub mod polkadot {}
//!
//! # #[tokio::main]
//! # async fn main() {
//! # let api = ClientBuilder::new()
//! #     .build()
//! #     .await
//! #     .unwrap()
//! #     .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
//! # // Obtain the storage client wrapper from the API.
//! # let storage: StorageClient<_> = api.client.storage();
//! // Iterate over keys and values.
//! let mut iter = storage
//!     .iter::<polkadot::xcm_pallet::storage::VersionNotifiers>(None)
//!     .await
//!     .unwrap();
//! while let Some((key, value)) = iter.next().await.unwrap() {
//!     println!("Key: 0x{}", hex::encode(&key));
//!     println!("Value: {}", value);
//! }
//! # }
//! ```

mod storage_client;

pub use storage_client::{
    StorageClient,
    KeyIter,
    StorageEntry,
    StorageEntryKey,
    StorageMapKey,
};