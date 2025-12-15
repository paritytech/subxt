use crate::backend::{BackendExt, StorageResponse, StreamOf};
use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::{BackendError, StorageError};
use crate::storage::address::Address;
use crate::storage::{PrefixOf, StorageKeyValue, StorageValue};
use crate::utils::YesMaybe;
use core::marker::PhantomData;
use frame_decode::storage::{IntoEncodableValues, StorageInfo, StorageTypeInfo};
use futures::{Stream, StreamExt};
use scale_info::PortableRegistry;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;

/// This represents a single storage entry (be it a plain value or map)
/// and the operations that can be performed on it.
#[derive(Debug)]
pub struct StorageEntry<'atblock, T: Config, Client, Addr> {
    inner: Arc<StorageEntryInner<'atblock, Addr, Client>>,
    marker: PhantomData<T>,
}

impl<'atblock, T: Config, Client, Addr> Clone for StorageEntry<'atblock, T, Client, Addr> {
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

impl<'atblock, T, Client, Addr> StorageEntry<'atblock, T, Client, Addr>
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

    /// The keys for plain storage values are always 32 byte hashes.
    pub fn key_prefix(&self) -> [u8; 32] {
        frame_decode::storage::encode_storage_key_prefix(self.pallet_name(), self.entry_name())
    }

    /// This returns a full key to a single value in this storage entry.
    pub fn fetch_key(&self, key_parts: Addr::KeyParts) -> Result<Vec<u8>, StorageError> {
        let num_keys = self.inner.info.keys.len();
        if key_parts.num_encodable_values() != num_keys {
            return Err(StorageError::WrongNumberOfKeyPartsProvidedForFetching {
                expected: num_keys,
                got: key_parts.num_encodable_values(),
            });
        }

        self.key_from_any_parts(key_parts)
    }

    /// This returns a valid key suitable for iterating over the values in this storage entry.
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
            self.key_from_any_parts(key_parts)
        }
    }

    // This has a more lax type signature than `.key` and so can be used in a couple of places internally.
    fn key_from_any_parts(
        &self,
        key_parts: impl IntoEncodableValues,
    ) -> Result<Vec<u8>, StorageError> {
        frame_decode::storage::encode_storage_key_with_info(
            self.pallet_name(),
            self.entry_name(),
            key_parts,
            &self.inner.info,
            self.inner.client.metadata_ref().types(),
        )
        .map_err(StorageError::StorageKeyEncodeError)
    }
}

impl<'atblock, T, Client, Addr> StorageEntry<'atblock, T, Client, Addr>
where
    T: Config,
    Addr: Address,
    Client: OnlineClientAtBlockT<T>,
{
    /// Fetch a storage value within this storage entry.
    ///
    /// If the entry is a map, you'll need to provide the relevant values for each part of the storage
    /// key. If the entry is a plain value, you must provide an empty list of key parts, ie `()`.
    ///
    /// The type of these key parts is determined by the [`Address`] of this storage entry. If the address
    /// is generated via the `#[subxt]` macro then it will ensure you provide a valid type.
    ///
    /// If no value is found, the default value will be returned for this entry if one exists. If no value is
    /// found and no default value exists, an error will be returned.
    pub async fn fetch(
        &self,
        key_parts: Addr::KeyParts,
    ) -> Result<StorageValue<'atblock, Addr::Value>, StorageError> {
        let value = self
            .try_fetch(key_parts)
            .await?
            .or_else(|| self.default_value())
            .ok_or(StorageError::NoValueFound)?;

        Ok(value)
    }

    /// Fetch a storage value within this storage entry.
    ///
    /// If the entry is a map, you'll need to provide the relevant values for each part of the storage
    /// key. If the entry is a plain value, you must provide an empty list of key parts, ie `()`.
    ///
    /// The type of these key parts is determined by the [`Address`] of this storage entry. If the address
    /// is generated via the `#[subxt]` macro then it will ensure you provide a valid type.
    ///
    /// If no value is found, `None` will be returned.
    pub async fn try_fetch(
        &self,
        key_parts: Addr::KeyParts,
    ) -> Result<Option<StorageValue<'atblock, Addr::Value>>, StorageError> {
        let key = self.fetch_key(key_parts)?;
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

    /// Iterate over storage values within this storage entry.
    ///
    /// You'll need to provide a prefix of the key parts required to point to a single value in the map.
    /// Normally you will provide `()` to iterate over _everything_, `(first_key,)` to iterate over everything underneath
    /// `first_key` in the map, `(first_key, second_key)` to iterate over everything underneath `first_key`
    /// and `second_key` in the map, and so on, up to the actual depth of the map - 1.
    ///
    /// The possible types of these key parts is determined by the [`Address`] of this storage entry.
    /// If the address is generated via the `#[subxt]` macro then it will ensure you provide a valid type.
    ///
    /// For plain values, there is no valid type, since they cannot be iterated over.
    pub async fn iter<KeyParts: PrefixOf<Addr::KeyParts>>(
        &self,
        key_parts: KeyParts,
    ) -> Result<StorageEntries<'atblock, Addr>, StorageError> {
        let num_keys = self.inner.info.keys.len();
        if key_parts.num_encodable_values() >= num_keys {
            return Err(StorageError::WrongNumberOfKeyPartsProvidedForIterating {
                max_expected: num_keys - 1,
                got: key_parts.num_encodable_values(),
            });
        }

        let info = self.inner.info.clone();
        let types = self.inner.client.metadata_ref().types();
        let key_bytes = self.key_from_any_parts(key_parts)?;
        let block_hash = self.inner.client.block_hash();

        let stream = self
            .inner
            .client
            .backend()
            .storage_fetch_descendant_values(key_bytes, block_hash)
            .await
            .map_err(StorageError::CannotIterateValues)?;

        // .map(move |kv| {
        //     let kv = match kv {
        //         Ok(kv) => kv,
        //         Err(e) => return Err(StorageError::StreamFailure(e)),
        //     };
        //     Ok(StorageKeyValue::new(
        //         info.clone(),
        //         types,
        //         kv.key.into(),
        //         kv.value,
        //     ))
        // });

        Ok(StorageEntries {
            info,
            stream,
            types,
            marker: PhantomData,
        })
    }
}

/// A stream of storage entries.
pub struct StorageEntries<'atblock, Addr> {
    // The raw underlying stream:
    stream: StreamOf<Result<StorageResponse, BackendError>>,
    // things we need to convert this into what we want:
    info: Arc<StorageInfo<'atblock, u32>>,
    types: &'atblock PortableRegistry,
    marker: PhantomData<Addr>,
}

impl<'atblock, Addr: Address> StorageEntries<'atblock, Addr> {
    /// Get the next storage entry. This is an alias for `futures::StreamExt::next(self)`.
    pub async fn next(&mut self) -> Option<Result<StorageKeyValue<'atblock, Addr>, StorageError>> {
        StreamExt::next(self).await
    }
}

impl<'atblock, Addr> std::marker::Unpin for StorageEntries<'atblock, Addr> {}
impl<'atblock, Addr: Address> Stream for StorageEntries<'atblock, Addr> {
    type Item = Result<StorageKeyValue<'atblock, Addr>, StorageError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let val = match futures::ready!(self.stream.poll_next_unpin(cx)) {
            Some(Ok(val)) => val,
            Some(Err(e)) => return Poll::Ready(Some(Err(StorageError::StreamFailure(e)))),
            None => return Poll::Ready(None),
        };

        Poll::Ready(Some(Ok(StorageKeyValue::new(
            self.info.clone(),
            self.types,
            val.key.into(),
            val.value,
        ))))
    }
}
