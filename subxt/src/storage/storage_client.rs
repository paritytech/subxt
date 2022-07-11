// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{
    Decode,
};
use sp_core::storage::{
    StorageData,
    StorageKey,
};
pub use sp_runtime::traits::SignedExtension;
use std::{
    marker::PhantomData,
    future::Future,
};
use crate::{
    error::BasicError,
    metadata::{
        MetadataError,
    },
    client::{
        OnlineClientT,
    },
    metadata::Metadata,
    Config,
};
use derivative::Derivative;
use super::storage_address::StorageAddress;

/// Query the runtime storage using [StorageClient].
///
/// This module is the core of performing runtime storage queries. While you can
/// work with it directly, it's prefer to use the generated `storage()` interface where
/// possible.
///
/// The exposed API is performing RPC calls to `state_getStorage` and `state_getKeysPaged`.
///
/// A runtime storage entry can be of type:
/// - [StorageEntryKey::Plain] for keys constructed just from the prefix
///   `twox_128(pallet) ++ twox_128(storage_item)`
/// - [StorageEntryKey::Map] for mapped keys constructed from the prefix,
///   plus other arguments `twox_128(pallet) ++ twox_128(storage_item) ++ hash(arg1) ++ arg1`
///
/// # Examples
///
/// ## Fetch Storage Keys
///
/// ```no_run
/// # use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
/// # use subxt::storage::StorageClient;
///
/// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
/// pub mod polkadot {}
///
/// # #[tokio::main]
/// # async fn main() {
/// # let api = ClientBuilder::new()
/// #     .build()
/// #     .await
/// #     .unwrap()
/// #     .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
/// # // Obtain the storage client wrapper from the API.
/// # let storage: StorageClient<_> = api.client.storage();
/// // Fetch just the keys, returning up to 10 keys.
/// let keys = storage
///     .fetch_keys::<polkadot::xcm_pallet::storage::VersionNotifiers>(10, None, None)
///     .await
///     .unwrap();
/// // Iterate over each key
/// for key in keys.iter() {
///     println!("Key: 0x{}", hex::encode(&key));
/// }
/// # }
/// ```
///
/// ## Iterate over Storage
///
/// ```no_run
/// # use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
/// # use subxt::storage::StorageClient;
///
/// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
/// pub mod polkadot {}
///
/// # #[tokio::main]
/// # async fn main() {
/// # let api = ClientBuilder::new()
/// #     .build()
/// #     .await
/// #     .unwrap()
/// #     .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
/// # // Obtain the storage client wrapper from the API.
/// # let storage: StorageClient<_> = api.client.storage();
/// // Iterate over keys and values.
/// let mut iter = storage
///     .iter::<polkadot::xcm_pallet::storage::VersionNotifiers>(None)
///     .await
///     .unwrap();
/// while let Some((key, value)) = iter.next().await.unwrap() {
///     println!("Key: 0x{}", hex::encode(&key));
///     println!("Value: {}", value);
/// }
/// # }
/// ```
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct StorageClient<T, Client> {
    client: Client,
    _marker: PhantomData<T>
}

impl<T, Client> StorageClient<T, Client>{
    /// Create a new [`StorageClient`]
    pub fn new(
        client: Client,
    ) -> Self {
        Self {
            client,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> StorageClient<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Fetch the raw encoded value at the address/key given.
    pub fn fetch_raw<K: Into<StorageKey>>(
        &self,
        key: K,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Option<Vec<u8>>, BasicError>> + 'static {
        let client = self.client.clone();
        let key = key.into();
        // Ensure that the returned future doesn't have a lifetime tied to api.storage(),
        // which is a temporary thing we'll be throwing away quickly:
        async move {
            let data = client.rpc().storage(&key, hash).await?;
            Ok(data.map(|d| d.0))
        }
    }

    /// Fetch a decoded value from storage at a given address and optional block hash.
    pub fn fetch<'a, ReturnTy: Decode>(
        &self,
        address: &'a StorageAddress<'_, ReturnTy>,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Option<ReturnTy>, BasicError>> + 'a {
        let client = self.client.clone();
        async move {
            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            if let Some(validation_hash) = address.validation_hash() {
                validate_storage(
                    address.pallet_name(),
                    address.entry_name(),
                    validation_hash,
                    &client.metadata()
                )?;
            }

            if let Some(data) = client.storage().fetch_raw(address, hash).await? {
                Ok(Some(Decode::decode(&mut &*data)?))
            } else {
                Ok(None)
            }
        }
    }

    /// Fetch a StorageKey that has a default value with an optional block hash.
    pub fn fetch_or_default<'a, ReturnTy: Decode>(
        &self,
        address: &'a StorageAddress<'_, ReturnTy>,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<ReturnTy, BasicError>> + 'a {
        let client = self.client.clone();
        async move {
            let pallet_name = address.pallet_name();
            let storage_name = address.entry_name();
            // Metadata validation happens via .fetch():
            if let Some(data) = client.storage().fetch(address, hash).await? {
                Ok(data)
            } else {
                let metadata = client.metadata();
                let pallet_metadata = metadata.pallet(pallet_name)?;
                let storage_metadata = pallet_metadata.storage(storage_name)?;
                let default = Decode::decode(&mut &storage_metadata.default[..])
                    .map_err(MetadataError::DefaultError)?;
                Ok(default)
            }
        }

    }

    /// Fetch up to `count` keys for a storage map in lexicographic order.
    ///
    /// Supports pagination by passing a value to `start_key`.
    pub fn fetch_keys<K: Into<StorageKey>>(
        &self,
        key: K,
        count: u32,
        start_key: Option<StorageKey>,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Vec<StorageKey>, BasicError>> + 'static {
        let client = self.client.clone();
        let key = key.into();
        async move {
            let keys = client
                .rpc()
                .storage_keys_paged(key, count, start_key, hash)
                .await?;
            Ok(keys)
        }
    }

    /// Returns an iterator of key value pairs.
    pub fn iter<'a, ReturnTy: Decode + 'static>(
        &self,
        address: StorageAddress<'a, ReturnTy>,
        page_size: u32,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<KeyIter<T, Client, ReturnTy>, BasicError>> + 'a {
        let client = self.client.clone();
        async move {
            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            if let Some(validation_hash) = address.validation_hash() {
                validate_storage(
                    address.pallet_name(),
                    address.entry_name(),
                    validation_hash,
                    &client.metadata()
                )?;
            }

            // Fetch a concrete block hash to iterate over. We do this so that if new blocks
            // are produced midway through iteration, we continue to iterate at the block
            // we started with and not the new block.
            let hash = if let Some(hash) = hash {
                hash
            } else {
                client
                    .rpc()
                    .block_hash(None)
                    .await?
                    .expect("didn't pass a block number; qed")
            };

            // Strip any keys off; we want the top level entry only.
            let root_addr = {
                let mut a = address.to_owned();
                a.root();
                a
            };

            Ok(KeyIter {
                client: client.storage(),
                address: root_addr,
                block_hash: hash,
                count: page_size,
                start_key: None,
                buffer: Default::default(),
            })
        }
    }
}

/// Iterates over key value pairs in a map.
pub struct KeyIter<T: Config, Client, ReturnTy: Decode> {
    client: StorageClient<T, Client>,
    address: StorageAddress<'static, ReturnTy>,
    count: u32,
    block_hash: T::Hash,
    start_key: Option<StorageKey>,
    buffer: Vec<(StorageKey, StorageData)>,
}

impl<'a, T: Config, Client: OnlineClientT<T>, ReturnTy: Decode> KeyIter<T, Client, ReturnTy> {
    /// Returns the next key value pair from a map.
    pub async fn next(&mut self) -> Result<Option<(StorageKey, ReturnTy)>, BasicError> {
        loop {
            if let Some((k, v)) = self.buffer.pop() {
                return Ok(Some((k, Decode::decode(&mut &v.0[..])?)))
            } else {
                let keys = self
                    .client
                    .fetch_keys(&self.address, self.count, self.start_key.take(), Some(self.block_hash))
                    .await?;

                if keys.is_empty() {
                    return Ok(None)
                }

                self.start_key = keys.last().cloned();

                let change_sets = self
                    .client
                    .client
                    .rpc()
                    .query_storage_at(&keys, Some(self.block_hash))
                    .await?;
                for change_set in change_sets {
                    for (k, v) in change_set.changes {
                        if let Some(v) = v {
                            self.buffer.push((k, v));
                        }
                    }
                }
                debug_assert_eq!(self.buffer.len(), keys.len());
            }
        }
    }
}

/// Validate a storage entry against the metadata.
fn validate_storage(pallet_name: &str, storage_name: &str, hash: [u8; 32], metadata: &Metadata) -> Result<(), BasicError> {
    let expected_hash = match metadata.storage_hash(pallet_name, storage_name) {
        Ok(hash) => hash,
        Err(e) => return Err(e.into())
    };
    match expected_hash != hash {
        true => Ok(()),
        false => Err(crate::error::MetadataError::IncompatibleMetadata.into())
    }
}