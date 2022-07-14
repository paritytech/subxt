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
        OfflineClientT,
    },
    metadata::Metadata,
    Config,
};
use derivative::Derivative;
use super::storage_address::{
    StorageAddress,
    AddressHasDefaultValue,
    AddressIsIterable,
};

/// Query the runtime storage.
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
    Client: OfflineClientT<T>,
{
    /// Run the validation logic against some storage address you'd like to access. Returns `Ok(())`
    /// if the address is valid (or if it's not possible to check since the address has no validation hash).
    /// Return an error if the address was not valid or something went wrong trying to validate it (ie
    /// the pallet or storage entry in question do not exist at all).
    pub fn validate<'a, ReturnTy, Iterable, Defaultable>(
        &self,
        address: &'a StorageAddress<'_, ReturnTy, Iterable, Defaultable>,
    ) -> Result<(), BasicError> {
        if let Some(hash) = address.validation_hash() {
            validate_storage(
                address.pallet_name(),
                address.entry_name(),
                hash,
                &self.client.metadata()
            )?;
        }
        Ok(())
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
    ///
    /// # Example
    ///
    /// ```no_run
    /// use subxt::{ PolkadotConfig, OnlineClient };
    ///
    /// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
    /// pub mod polkadot {}
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    ///
    /// // Address to a storage entry we'd like to access.
    /// let address = polkadot::storage().xcm_pallet().queries(&12345);
    ///
    /// // Fetch just the keys, returning up to 10 keys.
    /// let value = api
    ///     .storage()
    ///     .fetch(&address, None)
    ///     .await
    ///     .unwrap();
    ///
    /// println!("Value: {:?}", value);
    /// # }
    /// ```
    pub fn fetch<'a, ReturnTy: Decode, Iterable, Defaultable>(
        &self,
        address: &'a StorageAddress<'_, ReturnTy, Iterable, Defaultable>,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Option<ReturnTy>, BasicError>> + 'a {
        let client = self.clone();
        async move {
            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            client.validate(address)?;

            if let Some(data) = client.client.storage().fetch_raw(address, hash).await? {
                Ok(Some(Decode::decode(&mut &*data)?))
            } else {
                Ok(None)
            }
        }
    }

    /// Fetch a StorageKey that has a default value with an optional block hash.
    ///
    /// Note: The [`StorageAddress`] provided must be tagged with [`AddressHasDefaultValue`]
    /// in order to use this function. Statically generated storage addresses will be
    /// tagged appropriately.
    pub fn fetch_or_default<'a, ReturnTy: Decode, Iterable>(
        &self,
        address: &'a StorageAddress<'_, ReturnTy, Iterable, AddressHasDefaultValue>,
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
    ///
    /// Note: The [`StorageAddress`] provided must be tagged with [`AddressIsIterable`]
    /// in order to use this function. Statically generated storage addresses will be
    /// tagged appropriately.
    ///
    /// ```no_run
    /// use subxt::{ PolkadotConfig, OnlineClient };
    ///
    /// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
    /// pub mod polkadot {}
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    ///
    /// // Address to the root of a storage entry that we'd like to iterate over.
    /// let address = polkadot::storage().xcm_pallet().version_notifiers_root();
    ///
    /// // Iterate over keys and values at that address.
    /// let mut iter = api
    ///     .storage()
    ///     .iter(address, 10, None)
    ///     .await
    ///     .unwrap();
    ///
    /// while let Some((key, value)) = iter.next().await.unwrap() {
    ///     println!("Key: 0x{}", hex::encode(&key));
    ///     println!("Value: {}", value);
    /// }
    /// # }
    /// ```
    pub fn iter<'a, ReturnTy: Decode + 'static, Defaultable: 'static>(
        &self,
        address: StorageAddress<'a, ReturnTy, AddressIsIterable, Defaultable>,
        page_size: u32,
        hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<KeyIter<T, Client, ReturnTy, Defaultable>, BasicError>> + 'a {
        let client = self.clone();
        async move {
            // Metadata validation checks whether the static address given
            // is likely to actually correspond to a real storage entry or not.
            // if not, it means static codegen doesn't line up with runtime
            // metadata.
            client.validate(&address)?;

            // Fetch a concrete block hash to iterate over. We do this so that if new blocks
            // are produced midway through iteration, we continue to iterate at the block
            // we started with and not the new block.
            let hash = if let Some(hash) = hash {
                hash
            } else {
                client
                    .client
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
                client: client,
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
pub struct KeyIter<T: Config, Client, ReturnTy: Decode, Defaultable> {
    client: StorageClient<T, Client>,
    address: StorageAddress<'static, ReturnTy, AddressIsIterable, Defaultable>,
    count: u32,
    block_hash: T::Hash,
    start_key: Option<StorageKey>,
    buffer: Vec<(StorageKey, StorageData)>,
}

impl<'a, T: Config, Client: OnlineClientT<T>, ReturnTy: Decode, Defaultable> KeyIter<T, Client, ReturnTy, Defaultable> {
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
    match expected_hash == hash {
        true => Ok(()),
        false => Err(crate::error::MetadataError::IncompatibleMetadata.into())
    }
}