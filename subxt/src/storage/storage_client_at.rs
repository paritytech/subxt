// Copyright 2019-2026 Parity Technologies (UK) Ltd.
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

impl<T, Client> StorageClientAt<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// This is essentially a shorthand for `client.entry(addr)?.fetch(key_parts)`. See [`StorageEntryClient::fetch()`].
    pub async fn fetch<Addr: Address>(
        &self,
        addr: Addr,
        key_parts: Addr::KeyParts,
    ) -> Result<StorageValue<'_, Addr::Value>, StorageError> {
        let entry = subxt_core::storage::entry(addr, &self.metadata)?;
        fetch(&entry, &self.client, self.block_ref.hash(), key_parts).await
    }

    /// This is essentially a shorthand for `client.entry(addr)?.try_fetch(key_parts)`. See [`StorageEntryClient::try_fetch()`].
    pub async fn try_fetch<Addr: Address>(
        &self,
        addr: Addr,
        key_parts: Addr::KeyParts,
    ) -> Result<Option<StorageValue<'_, Addr::Value>>, StorageError> {
        let entry = subxt_core::storage::entry(addr, &self.metadata)?;
        try_fetch(&entry, &self.client, self.block_ref.hash(), key_parts).await
    }

    /// This is essentially a shorthand for `client.entry(addr)?.iter(key_parts)`. See [`StorageEntryClient::iter()`].
    pub async fn iter<Addr: Address, KeyParts: PrefixOf<Addr::KeyParts>>(
        &'_ self,
        addr: Addr,
        key_parts: KeyParts,
    ) -> Result<
        impl futures::Stream<Item = Result<StorageKeyValue<'_, Addr>, StorageError>>
        + use<'_, Addr, Client, T, KeyParts>,
        StorageError,
    > {
        let entry = subxt_core::storage::entry(addr, &self.metadata)?;
        iter(entry, &self.client, self.block_ref.hash(), key_parts).await
    }

    /// In rare cases, you may wish to fetch a storage value that does not live at a typical address. This method
    /// is a fallback for those cases, and allows you to provide the raw storage key bytes corresponding to the
    /// entry you wish to obtain. The response will either be the bytes for the value found at that location, or
    /// otherwise an error. [`StorageError::NoValueFound`] will be returned in the event that the request was valid
    /// but no value lives at the given location).
    pub async fn fetch_raw(&self, key_bytes: Vec<u8>) -> Result<Vec<u8>, StorageError> {
        let block_hash = self.block_ref.hash();
        let value = self
            .client
            .backend()
            .storage_fetch_value(key_bytes, block_hash)
            .await
            .map_err(StorageError::CannotFetchValue)?
            .ok_or(StorageError::NoValueFound)?;

        Ok(value)
    }

    /// The storage version of a pallet.
    /// The storage version refers to the `frame_support::traits::Metadata::StorageVersion` type.
    pub async fn storage_version(&self, pallet_name: impl AsRef<str>) -> Result<u16, StorageError> {
        // construct the storage key. This is done similarly in
        // `frame_support::traits::metadata::StorageVersion::storage_key()`:
        let mut key_bytes: Vec<u8> = vec![];
        key_bytes.extend(&sp_crypto_hashing::twox_128(
            pallet_name.as_ref().as_bytes(),
        ));
        key_bytes.extend(&sp_crypto_hashing::twox_128(b":__STORAGE_VERSION__:"));

        // fetch the raw bytes and decode them into the StorageVersion struct:
        let storage_version_bytes = self.fetch_raw(key_bytes).await?;

        <u16 as codec::Decode>::decode(&mut &storage_version_bytes[..])
            .map_err(StorageError::CannotDecodeStorageVersion)
    }

    /// Fetch the runtime WASM code.
    pub async fn runtime_wasm_code(&self) -> Result<Vec<u8>, StorageError> {
        // note: this should match the `CODE` constant in `sp_core::storage::well_known_keys`
        self.fetch_raw(b":code".to_vec()).await
    }
}

/// This represents a single storage entry (be it a plain value or map)
/// and the operations that can be performed on it.
pub struct StorageEntryClient<'atblock, T: Config, Client, Addr, IsPlain> {
    inner: subxt_core::storage::StorageEntry<'atblock, Addr>,
    client: Client,
    block_ref: BlockRef<HashFor<T>>,
    _marker: PhantomData<(T, IsPlain)>,
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
    /// Fetch the storage value at this location. If no value is found, the default value will be returned
    /// for this entry if one exists. If no value is found and no default value exists, an error will be returned.
    pub async fn fetch(&self) -> Result<StorageValue<'atblock, Addr::Value>, StorageError> {
        let value = self.try_fetch().await?.map_or_else(
            || self.inner.default_value().ok_or(StorageError::NoValueFound),
            Ok,
        )?;

        Ok(value)
    }

    /// Fetch the storage value at this location. If no value is found, `None` will be returned.
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

    /// This is identical to [`StorageEntryClient::key_prefix()`] and is the full
    /// key for this storage entry.
    pub fn key(&self) -> [u8; 32] {
        self.inner.key_prefix()
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
    /// Fetch a storage value within this storage entry.
    ///
    /// This entry may be a map, and so you must provide the relevant values for each part of the storage
    /// key that is required in order to point to a single value.
    ///
    /// If no value is found, the default value will be returned for this entry if one exists. If no value is
    /// found and no default value exists, an error will be returned.
    pub async fn fetch(
        &self,
        key_parts: Addr::KeyParts,
    ) -> Result<StorageValue<'atblock, Addr::Value>, StorageError> {
        fetch(&self.inner, &self.client, self.block_ref.hash(), key_parts).await
    }

    /// Fetch a storage value within this storage entry.
    ///
    /// This entry may be a map, and so you must provide the relevant values for each part of the storage
    /// key that is required in order to point to a single value.
    ///
    /// If no value is found, `None` will be returned.
    pub async fn try_fetch(
        &self,
        key_parts: Addr::KeyParts,
    ) -> Result<Option<StorageValue<'atblock, Addr::Value>>, StorageError> {
        try_fetch(&self.inner, &self.client, self.block_ref.hash(), key_parts).await
    }

    /// Iterate over storage values within this storage entry.
    ///
    /// You may provide any prefix of the values needed to point to a single value. Normally you will
    /// provide `()` to iterate over _everything_, or `(first_key,)` to iterate over everything underneath
    /// `first_key` in the map, or `(first_key, second_key)` to iterate over everything underneath `first_key`
    /// and `second_key` in the map, and so on, up to the actual depth of the map - 1.
    pub async fn iter<KeyParts: PrefixOf<Addr::KeyParts>>(
        &self,
        key_parts: KeyParts,
    ) -> Result<
        impl futures::Stream<Item = Result<StorageKeyValue<'atblock, Addr>, StorageError>>
        + use<'atblock, Addr, Client, T, KeyParts>,
        StorageError,
    > {
        iter(
            self.inner.clone(),
            &self.client,
            self.block_ref.hash(),
            key_parts,
        )
        .await
    }

    /// This returns a full key to a single value in this storage entry.
    pub fn key(&self, key_parts: Addr::KeyParts) -> Result<Vec<u8>, StorageError> {
        let key = self.inner.fetch_key(key_parts)?;
        Ok(key)
    }

    /// This returns valid keys to iterate over the storage entry at the available levels.
    pub fn iter_key<KeyParts: PrefixOf<Addr::KeyParts>>(
        &self,
        key_parts: KeyParts,
    ) -> Result<Vec<u8>, StorageError> {
        let key = self.inner.iter_key(key_parts)?;
        Ok(key)
    }

    /// The first 32 bytes of the storage entry key, which points to the entry but not necessarily
    /// a single storage value (unless the entry is a plain value).
    pub fn key_prefix(&self) -> [u8; 32] {
        self.inner.key_prefix()
    }
}

async fn fetch<'atblock, T: Config, Client: OnlineClientT<T>, Addr: Address>(
    entry: &subxt_core::storage::StorageEntry<'atblock, Addr>,
    client: &Client,
    block_hash: HashFor<T>,
    key_parts: Addr::KeyParts,
) -> Result<StorageValue<'atblock, Addr::Value>, StorageError> {
    let value = try_fetch(entry, client, block_hash, key_parts)
        .await?
        .or_else(|| entry.default_value())
        .unwrap();

    Ok(value)
}

async fn try_fetch<'atblock, T: Config, Client: OnlineClientT<T>, Addr: Address>(
    entry: &subxt_core::storage::StorageEntry<'atblock, Addr>,
    client: &Client,
    block_hash: HashFor<T>,
    key_parts: Addr::KeyParts,
) -> Result<Option<StorageValue<'atblock, Addr::Value>>, StorageError> {
    let key = entry.fetch_key(key_parts)?;

    let value = client
        .backend()
        .storage_fetch_value(key, block_hash)
        .await
        .map_err(StorageError::CannotFetchValue)?
        .map(|bytes| entry.value(bytes))
        .or_else(|| entry.default_value());

    Ok(value)
}

async fn iter<
    'atblock,
    T: Config,
    Client: OnlineClientT<T>,
    Addr: Address,
    KeyParts: PrefixOf<Addr::KeyParts>,
>(
    entry: subxt_core::storage::StorageEntry<'atblock, Addr>,
    client: &Client,
    block_hash: HashFor<T>,
    key_parts: KeyParts,
) -> Result<
    impl futures::Stream<Item = Result<StorageKeyValue<'atblock, Addr>, StorageError>>
    + use<'atblock, Addr, Client, T, KeyParts>,
    StorageError,
> {
    let key_bytes = entry.iter_key(key_parts)?;

    let stream = client
        .backend()
        .storage_fetch_descendant_values(key_bytes, block_hash)
        .await
        .map_err(StorageError::CannotIterateValues)?
        .map(move |kv| {
            let kv = match kv {
                Ok(kv) => kv,
                Err(e) => return Err(StorageError::StreamFailure(e)),
            };
            Ok(entry.key_value(kv.key, kv.value))
        });

    Ok(Box::pin(stream))
}
