// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    backend::{BackendExt, BlockRef},
    client::{OfflineClientT, OnlineClientT},
    config::{Config, HashFor},
    error::StorageError,
};
use derive_where::derive_where;
use futures::StreamExt;
use std::marker::PhantomData;
use subxt_core::Metadata;
use subxt_core::storage::{PrefixOf, address::Address};
use subxt_core::utils::{Maybe, Yes};

pub use subxt_core::storage::{StorageKeyValue, StorageValue};

/// Query the runtime storage.
#[derive_where(Clone; Client)]
pub struct StorageClientAt<T: Config, Client> {
    client: Client,
    metadata: Metadata,
    block_ref: BlockRef<HashFor<T>>,
    _marker: PhantomData<T>,
}

impl<T, Client> StorageClientAt<T, Client>
where
    T: Config,
    Client: OfflineClientT<T>,
{
    /// Create a new [`StorageClientAt`].
    pub(crate) fn new(client: Client, block_ref: BlockRef<HashFor<T>>) -> Self {
        // Retrieve and store metadata here so that we can borrow it in
        // subsequent structs, and thus also borrow storage info and
        // things that borrow from metadata.
        let metadata = client.metadata();

        Self {
            client,
            metadata,
            block_ref,
            _marker: PhantomData,
        }
    }
}

impl<T, Client> StorageClientAt<T, Client>
where
    T: Config,
    Client: OfflineClientT<T>,
{
    /// This returns a [`StorageEntryClient`], which allows working with the storage entry at the provided address.
    pub fn entry<Addr: Address>(
        &self,
        address: Addr,
    ) -> Result<StorageEntryClient<'_, T, Client, Addr, Addr::IsPlain>, StorageError> {
        let inner = subxt_core::storage::entry(address, &self.metadata)?;
        Ok(StorageEntryClient {
            inner,
            client: self.client.clone(),
            block_ref: self.block_ref.clone(),
            _marker: core::marker::PhantomData,
        })
    }
}

/// This represents a single storage entry (be it a plain value or map)
/// and the operations that can be performed on it.
pub struct StorageEntryClient<'atblock, T: Config, Client, Addr, IsPlain> {
    inner: subxt_core::storage::StorageEntry<'atblock, Addr, IsPlain>,
    client: Client,
    block_ref: BlockRef<HashFor<T>>,
    _marker: PhantomData<T>,
}

impl<'atblock, T, Client, Addr, IsPlain> StorageEntryClient<'atblock, T, Client, Addr, IsPlain>
where
    T: Config,
    Addr: Address,
{
    /// Name of the pallet containing this storage entry.
    pub fn pallet_name(&self) -> &str {
        self.inner.pallet_name()
    }

    /// Name of the storage entry.
    pub fn entry_name(&self) -> &str {
        self.inner.entry_name()
    }

    /// Is the storage entry a plain value?
    pub fn is_plain(&self) -> bool {
        self.inner.is_plain()
    }

    /// Is the storage entry a map?
    pub fn is_map(&self) -> bool {
        self.inner.is_map()
    }

    /// Return the default value for this storage entry, if there is one. Returns `None` if there
    /// is no default value.
    pub fn default_value(&self) -> Option<StorageValue<'atblock, Addr::Value>> {
        self.inner.default_value()
    }
}

// Plain values get a fetch method with no extra arguments.
impl<'atblock, T, Client, Addr> StorageEntryClient<'atblock, T, Client, Addr, Yes>
where
    T: Config,
    Addr: Address,
    Client: OnlineClientT<T>,
{
    pub async fn fetch(&self) -> Result<StorageValue<'atblock, Addr::Value>, StorageError> {
        let value = self.try_fetch().await?.map_or_else(
            || self.inner.default_value().ok_or(StorageError::NoValueFound),
            Ok,
        )?;

        Ok(value)
    }

    pub async fn try_fetch(
        &self,
    ) -> Result<Option<StorageValue<'atblock, Addr::Value>>, StorageError> {
        let value = self
            .client
            .backend()
            .storage_fetch_value(self.key_prefix().to_vec(), self.block_ref.hash())
            .await
            .map_err(StorageError::CannotFetchValue)?
            .map(|bytes| self.inner.value(bytes));

        Ok(value)
    }

    /// The keys for plain storage values are always 32 byte hashes.
    pub fn key_prefix(&self) -> [u8; 32] {
        self.inner.key_prefix()
    }
}

// When HasDefaultValue = Yes, we expect there to exist a valid default value and will use that
// if we fetch an entry and get nothing back.
impl<'atblock, T, Client, Addr> StorageEntryClient<'atblock, T, Client, Addr, Maybe>
where
    T: Config,
    Addr: Address,
    Client: OnlineClientT<T>,
{
    pub async fn fetch(
        &self,
        keys: Addr::KeyParts,
    ) -> Result<StorageValue<'atblock, Addr::Value>, StorageError> {
        let value = self
            .try_fetch(keys)
            .await?
            .or_else(|| self.default_value())
            .unwrap();

        Ok(value)
    }

    pub async fn try_fetch(
        &self,
        keys: Addr::KeyParts,
    ) -> Result<Option<StorageValue<'atblock, Addr::Value>>, StorageError> {
        let key = self.inner.fetch_key(keys)?;

        let value = self
            .client
            .backend()
            .storage_fetch_value(key, self.block_ref.hash())
            .await
            .map_err(StorageError::CannotFetchValue)?
            .map(|bytes| self.inner.value(bytes))
            .or_else(|| self.default_value());

        Ok(value)
    }

    pub async fn iter<Keys: PrefixOf<Addr::KeyParts>>(
        &self,
        keys: Keys,
    ) -> Result<
        impl futures::Stream<Item = Result<StorageKeyValue<'atblock, Addr>, StorageError>>
        + use<'atblock, Addr, Client, T, Keys>,
        StorageError,
    > {
        let key_bytes = self.inner.iter_key(keys)?;
        let block_hash = self.block_ref.hash();
        let inner = self.inner.clone();

        let stream = self
            .client
            .backend()
            .storage_fetch_descendant_values(key_bytes, block_hash)
            .await
            .map_err(StorageError::CannotIterateValues)?
            .map(move |kv| {
                let kv = match kv {
                    Ok(kv) => kv,
                    Err(e) => return Err(StorageError::StreamFailure(e)),
                };
                Ok(inner.key_value(kv.key, kv.value))
            });

        Ok(Box::pin(stream))
    }

    /// The first 32 bytes of the storage entry key, which points to the entry but not necessarily
    /// a single storage value (unless the entry is a plain value).
    pub fn key_prefix(&self) -> [u8; 32] {
        self.inner.key_prefix()
    }
}
