use crate::backend::BackendExt;
use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::StorageError;
use crate::storage::address::Address;
use crate::storage::{PrefixOf, StorageKeyValue, StorageValue};
use crate::utils::{Maybe, Yes, YesMaybe};
use core::marker::PhantomData;
use frame_decode::storage::{IntoEncodableValues, StorageInfo, StorageTypeInfo};
use futures::StreamExt;
use std::sync::Arc;

/// This represents a single storage entry (be it a plain value or map)
/// and the operations that can be performed on it.
#[derive(Debug)]
pub struct StorageEntry<'atblock, T: Config, Client, Addr, IsPlain> {
    inner: Arc<StorageEntryInner<'atblock, Addr, Client>>,
    marker: PhantomData<(T, IsPlain)>,
}

impl<'atblock, T: Config, Client, Addr, IsPlain> Clone
    for StorageEntry<'atblock, T, Client, Addr, IsPlain>
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: self.marker,
        }
    }
}

#[derive(Debug)]
struct StorageEntryInner<'atblock, Addr, Client> {
    address: Addr,
    info: Arc<StorageInfo<'atblock, u32>>,
    client: &'atblock Client,
}

impl<'atblock, T, Client, Addr, IsPlain> StorageEntry<'atblock, T, Client, Addr, IsPlain>
where
    T: Config,
    Addr: Address,
    Client: OfflineClientAtBlockT<T>,
{
    pub(crate) fn new(client: &'atblock Client, address: Addr) -> Result<Self, StorageError> {
        let info = client
            .metadata_ref()
            .storage_info(address.pallet_name(), address.entry_name())
            .map_err(|e| StorageError::StorageInfoError(e.into_owned()))?;

        let inner = StorageEntryInner {
            address,
            info: Arc::new(info),
            client,
        };

        Ok(Self {
            inner: Arc::new(inner),
            marker: PhantomData,
        })
    }

    /// Name of the pallet containing this storage entry.
    pub fn pallet_name(&self) -> &str {
        self.inner.address.pallet_name()
    }

    /// Name of the storage entry.
    pub fn entry_name(&self) -> &str {
        self.inner.address.entry_name()
    }

    /// Is the storage entry a plain value?
    pub fn is_plain(&self) -> bool {
        self.inner.info.keys.is_empty()
    }

    /// Is the storage entry a map?
    pub fn is_map(&self) -> bool {
        !self.is_plain()
    }

    /// Return the default value for this storage entry, if there is one. Returns `None` if there
    /// is no default value.
    pub fn default_value(&self) -> Option<StorageValue<'atblock, Addr::Value>> {
        let info = &self.inner.info;
        let client = self.inner.client;
        info.default_value.as_ref().map(|default_value| {
            StorageValue::new(
                info.clone(),
                client.metadata_ref().types(),
                default_value.to_vec(),
            )
        })
    }

    /// Create the bytes for a storage key given the key parts.
    ///
    /// **Warning:** This provides no safety around the provided keys in order that it can be used
    /// behind the scenes in several places.
    pub(crate) fn internal_key_bytes<Keys: IntoEncodableValues>(
        &self,
        key_parts: Keys,
    ) -> Result<Vec<u8>, StorageError> {
        let key = frame_decode::storage::encode_storage_key_with_info(
            self.pallet_name(),
            self.entry_name(),
            key_parts,
            &self.inner.info,
            self.inner.client.metadata_ref().types(),
        )
        .map_err(StorageError::StorageKeyEncodeError)?;

        Ok(key)
    }
}

impl<'atblock, T, Client, Addr, IsPlain> StorageEntry<'atblock, T, Client, Addr, IsPlain>
where
    T: Config,
    Addr: Address,
    Client: OnlineClientAtBlockT<T>,
{
    /// Fetch a value, using the default value if none can be found, or returning an error
    /// if no value exists at this location and there is no default value.
    ///
    /// **Warning:** This provides no safety around the provided keys in order that it can be used
    /// behind the scenes in several places.
    pub(crate) async fn internal_fetch(
        &self,
        key_parts: impl IntoEncodableValues,
    ) -> Result<StorageValue<'atblock, Addr::Value>, StorageError> {
        let value = self
            .internal_try_fetch(key_parts)
            .await?
            .or_else(|| self.default_value())
            .ok_or(StorageError::NoValueFound)?;

        Ok(value)
    }

    /// Fetch a value, returning `None` if no value exists at that location.
    ///
    /// **Warning:** This provides no safety around the provided keys in order that it can be used
    /// behind the scenes in several places.
    pub(crate) async fn internal_try_fetch(
        &self,
        key_parts: impl IntoEncodableValues,
    ) -> Result<Option<StorageValue<'atblock, Addr::Value>>, StorageError> {
        let key = self.internal_key_bytes(key_parts)?;
        let block_hash = self.inner.client.block_hash();

        let value = self
            .inner
            .client
            .backend()
            .storage_fetch_value(key, block_hash)
            .await
            .map_err(StorageError::CannotFetchValue)?
            .map(|bytes| {
                StorageValue::new(
                    self.inner.info.clone(),
                    self.inner.client.metadata_ref().types(),
                    bytes,
                )
            })
            .or_else(|| self.default_value());

        Ok(value)
    }

    /// Iterate over the values under the provided key.
    ///
    /// **Warning:** This provides no safety around the provided keys in order that it can be used
    /// behind the scenes in several places.
    pub(crate) async fn internal_iter<KeyParts: PrefixOf<Addr::KeyParts>>(
        &self,
        key_parts: KeyParts,
    ) -> Result<
        impl futures::Stream<Item = Result<StorageKeyValue<'atblock, Addr>, StorageError>>
        + use<'atblock, Addr, Client, T, KeyParts, IsPlain>,
        StorageError,
    > {
        let info = self.inner.info.clone();
        let types = self.inner.client.metadata_ref().types();
        let key_bytes = self.internal_key_bytes(key_parts)?;
        let block_hash = self.inner.client.block_hash();

        let stream = self
            .inner
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
                Ok(StorageKeyValue::new(
                    info.clone(),
                    types,
                    kv.key.into(),
                    kv.value,
                ))
            });

        Ok(Box::pin(stream))
    }
}

// Plain values get a fetch method with no extra arguments.
impl<'atblock, T, Client, Addr> StorageEntry<'atblock, T, Client, Addr, Yes>
where
    T: Config,
    Addr: Address<IsPlain = Yes>,
    Client: OnlineClientAtBlockT<T>,
{
    /// Fetch the storage value at this location. If no value is found, the default value will be returned
    /// for this entry if one exists. If no value is found and no default value exists, an error will be returned.
    pub async fn fetch(&self) -> Result<StorageValue<'atblock, Addr::Value>, StorageError> {
        self.internal_fetch(()).await
    }

    /// Fetch the storage value at this location. If no value is found, `None` will be returned.
    pub async fn try_fetch(
        &self,
    ) -> Result<Option<StorageValue<'atblock, Addr::Value>>, StorageError> {
        self.internal_try_fetch(()).await
    }

    /// This is identical to [`StorageEntry::key_prefix()`] and is the full
    /// key for this storage entry.
    pub fn key(&self) -> [u8; 32] {
        self.key_prefix()
    }

    /// The keys for plain storage values are always 32 byte hashes.
    pub fn key_prefix(&self) -> [u8; 32] {
        frame_decode::storage::encode_storage_key_prefix(self.pallet_name(), self.entry_name())
    }
}

// When HasDefaultValue = Yes, we expect there to exist a valid default value and will use that
// if we fetch an entry and get nothing back.
impl<'atblock, T, Client, Addr> StorageEntry<'atblock, T, Client, Addr, Maybe>
where
    T: Config,
    Addr: Address<IsPlain = Maybe>,
    Client: OnlineClientAtBlockT<T>,
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
        self.internal_fetch(key_parts).await
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
        self.internal_try_fetch(key_parts).await
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
        self.internal_iter(key_parts).await
    }

    /// This returns a full key to a single value in this storage entry.
    pub fn key(&self, key_parts: Addr::KeyParts) -> Result<Vec<u8>, StorageError> {
        let num_keys = self.inner.info.keys.len();
        if key_parts.num_encodable_values() != num_keys {
            Err(StorageError::WrongNumberOfKeyPartsProvidedForFetching {
                expected: num_keys,
                got: key_parts.num_encodable_values(),
            })
        } else {
            self.internal_key_bytes(key_parts)
        }
    }

    /// This returns valid keys to iterate over the storage entry at the available levels.
    pub fn iter_key<KeyParts: PrefixOf<Addr::KeyParts>>(
        &self,
        key_parts: KeyParts,
    ) -> Result<Vec<u8>, StorageError> {
        let num_keys = self.inner.info.keys.len();
        if Addr::IsPlain::is_yes() {
            Err(StorageError::CannotIterPlainEntry {
                pallet_name: self.pallet_name().into(),
                entry_name: self.entry_name().into(),
            })
        } else if key_parts.num_encodable_values() >= num_keys {
            Err(StorageError::WrongNumberOfKeyPartsProvidedForIterating {
                max_expected: num_keys - 1,
                got: key_parts.num_encodable_values(),
            })
        } else {
            self.internal_key_bytes(key_parts)
        }
    }

    /// The first 32 bytes of the storage entry key, which points to the entry but not necessarily
    /// a single storage value (unless the entry is a plain value).
    pub fn key_prefix(&self) -> [u8; 32] {
        frame_decode::storage::encode_storage_key_prefix(self.pallet_name(), self.entry_name())
    }
}
