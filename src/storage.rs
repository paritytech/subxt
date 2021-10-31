// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! For querying runtime storage.

use codec::{
    Decode,
    Encode,
};
use sp_core::storage::{
    StorageChangeSet,
    StorageData,
    StorageKey,
};
pub use sp_runtime::traits::SignedExtension;
pub use sp_version::RuntimeVersion;
use std::marker::PhantomData;

use crate::{
    metadata::{
        Metadata,
        MetadataError,
    },
    rpc::Rpc,
    Config,
    Error,
    StorageHasher,
};

/// Storage entry trait.
pub trait StorageEntry {
    /// Pallet name.
    const PALLET: &'static str;
    /// Storage name.
    const STORAGE: &'static str;
    /// Type of the storage entry value.
    type Value: Decode;
    /// Get the key data for the storage.
    fn key(&self) -> StorageEntryKey;
}

/// The prefix of the key to a [`StorageEntry`]
pub struct StorageKeyPrefix(Vec<u8>);

impl StorageKeyPrefix {
    /// Create the storage key prefix for a [`StorageEntry`]
    pub fn new<T: StorageEntry>() -> Self {
        let mut bytes = sp_core::twox_128(T::PALLET.as_bytes()).to_vec();
        bytes.extend(&sp_core::twox_128(T::STORAGE.as_bytes())[..]);
        Self(bytes)
    }

    /// Convert the prefix into a [`StorageKey`]
    pub fn to_storage_key(self) -> StorageKey {
        StorageKey(self.0)
    }
}

/// Storage key.
pub enum StorageEntryKey {
    /// Plain key.
    Plain,
    /// Map key(s).
    Map(Vec<StorageMapKey>),
}

impl StorageEntryKey {
    /// Construct the final [`sp_core::storage::StorageKey`] for the storage entry.
    pub fn final_key(&self, prefix: StorageKeyPrefix) -> sp_core::storage::StorageKey {
        let mut bytes = prefix.0;
        if let Self::Map(map_keys) = self {
            for map_key in map_keys {
                bytes.extend(Self::hash(&map_key.hasher, &map_key.value))
            }
        }
        sp_core::storage::StorageKey(bytes)
    }

    fn hash(hasher: &StorageHasher, bytes: &[u8]) -> Vec<u8> {
        match hasher {
            StorageHasher::Identity => bytes.to_vec(),
            StorageHasher::Blake2_128 => sp_core::blake2_128(bytes).to_vec(),
            StorageHasher::Blake2_128Concat => {
                // copied from substrate Blake2_128Concat::hash since StorageHasher is not public
                sp_core::blake2_128(bytes)
                    .iter()
                    .chain(bytes)
                    .cloned()
                    .collect()
            }
            StorageHasher::Blake2_256 => sp_core::blake2_256(bytes).to_vec(),
            StorageHasher::Twox128 => sp_core::twox_128(bytes).to_vec(),
            StorageHasher::Twox256 => sp_core::twox_256(bytes).to_vec(),
            StorageHasher::Twox64Concat => {
                sp_core::twox_64(bytes)
                    .iter()
                    .chain(bytes)
                    .cloned()
                    .collect()
            }
        }
    }
}

/// Storage key for a Map.
pub struct StorageMapKey {
    value: Vec<u8>,
    hasher: StorageHasher,
}

impl StorageMapKey {
    /// Create a new [`StorageMapKey`] with the encoded data and the hasher.
    pub fn new<T: Encode>(value: &T, hasher: StorageHasher) -> Self {
        Self {
            value: value.encode(),
            hasher,
        }
    }
}

/// Client for querying runtime storage.
#[derive(Clone)]
pub struct StorageClient<'a, T: Config> {
    rpc: &'a Rpc<T>,
    metadata: &'a Metadata,
    iter_page_size: u32,
}

impl<'a, T: Config> StorageClient<'a, T> {
    /// Create a new [`StorageClient`]
    pub fn new(rpc: &'a Rpc<T>, metadata: &'a Metadata, iter_page_size: u32) -> Self {
        Self {
            rpc,
            metadata,
            iter_page_size,
        }
    }

    /// Fetch the value under an unhashed storage key
    pub async fn fetch_unhashed<V: Decode>(
        &self,
        key: StorageKey,
        hash: Option<T::Hash>,
    ) -> Result<Option<V>, Error> {
        if let Some(data) = self.rpc.storage(&key, hash).await? {
            Ok(Some(Decode::decode(&mut &data.0[..])?))
        } else {
            Ok(None)
        }
    }

    /// Fetch the raw encoded value under the raw storage key.
    pub async fn fetch_raw(
        &self,
        key: StorageKey,
        hash: Option<T::Hash>,
    ) -> Result<Option<StorageData>, Error> {
        self.rpc.storage(&key, hash).await
    }

    /// Fetch a StorageKey with an optional block hash.
    pub async fn fetch<F: StorageEntry>(
        &self,
        store: &F,
        hash: Option<T::Hash>,
    ) -> Result<Option<F::Value>, Error> {
        let prefix = StorageKeyPrefix::new::<F>();
        let key = store.key().final_key(prefix);
        self.fetch_unhashed::<F::Value>(key, hash).await
    }

    /// Fetch a StorageKey that has a default value with an optional block hash.
    pub async fn fetch_or_default<F: StorageEntry>(
        &self,
        store: &F,
        hash: Option<T::Hash>,
    ) -> Result<F::Value, Error> {
        if let Some(data) = self.fetch(store, hash).await? {
            Ok(data)
        } else {
            let pallet_metadata = self.metadata.pallet(F::PALLET)?;
            let storage_metadata = pallet_metadata.storage(F::STORAGE)?;
            let default = Decode::decode(&mut &storage_metadata.default[..])
                .map_err(MetadataError::DefaultError)?;
            Ok(default)
        }
    }

    /// Query historical storage entries
    pub async fn query_storage(
        &self,
        keys: Vec<StorageKey>,
        from: T::Hash,
        to: Option<T::Hash>,
    ) -> Result<Vec<StorageChangeSet<T::Hash>>, Error> {
        self.rpc.query_storage(keys, from, to).await
    }

    /// Fetch up to `count` keys for a storage map in lexicographic order.
    ///
    /// Supports pagination by passing a value to `start_key`.
    pub async fn fetch_keys<F: StorageEntry>(
        &self,
        count: u32,
        start_key: Option<StorageKey>,
        hash: Option<T::Hash>,
    ) -> Result<Vec<StorageKey>, Error> {
        let prefix = StorageKeyPrefix::new::<F>();
        let keys = self
            .rpc
            .storage_keys_paged(Some(prefix), count, start_key, hash)
            .await?;
        Ok(keys)
    }

    /// Returns an iterator of key value pairs.
    pub async fn iter<F: StorageEntry>(
        &self,
        hash: Option<T::Hash>,
    ) -> Result<KeyIter<'a, T, F>, Error> {
        let hash = if let Some(hash) = hash {
            hash
        } else {
            self.rpc
                .block_hash(None)
                .await?
                .expect("didn't pass a block number; qed")
        };
        Ok(KeyIter {
            client: self.clone(),
            hash,
            count: self.iter_page_size,
            start_key: None,
            buffer: Default::default(),
            _marker: PhantomData,
        })
    }
}

/// Iterates over key value pairs in a map.
pub struct KeyIter<'a, T: Config, F: StorageEntry> {
    client: StorageClient<'a, T>,
    _marker: PhantomData<F>,
    count: u32,
    hash: T::Hash,
    start_key: Option<StorageKey>,
    buffer: Vec<(StorageKey, StorageData)>,
}

impl<'a, T: Config, F: StorageEntry> KeyIter<'a, T, F> {
    /// Returns the next key value pair from a map.
    pub async fn next(&mut self) -> Result<Option<(StorageKey, F::Value)>, Error> {
        loop {
            if let Some((k, v)) = self.buffer.pop() {
                return Ok(Some((k, Decode::decode(&mut &v.0[..])?)))
            } else {
                let keys = self
                    .client
                    .fetch_keys::<F>(self.count, self.start_key.take(), Some(self.hash))
                    .await?;

                if keys.is_empty() {
                    return Ok(None)
                }

                self.start_key = keys.last().cloned();

                let change_sets = self
                    .client
                    .rpc
                    .query_storage_at(&keys, Some(self.hash))
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
