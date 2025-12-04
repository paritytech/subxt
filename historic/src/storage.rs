mod list_storage_entries_any;
mod storage_entry;
mod storage_info;
mod storage_key;
mod storage_value;

use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::StorageError;
use crate::storage::storage_info::with_info;
use std::borrow::Cow;
use std::sync::Arc;
use storage_info::AnyStorageInfo;

pub use storage_entry::StorageEntry;
pub use storage_key::{StorageHasher, StorageKey, StorageKeyPart};
pub use storage_value::StorageValue;
// We take how storage keys can be passed in from `frame-decode`, so re-export here.
pub use frame_decode::storage::{EncodableValues, IntoEncodableValues};

/// Work with storage.
pub struct StorageClient<'atblock, Client, T> {
    client: &'atblock Client,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, Client, T> StorageClient<'atblock, Client, T> {
    /// Work with storage.
    pub(crate) fn new(client: &'atblock Client) -> Self {
        Self {
            client,
            marker: std::marker::PhantomData,
        }
    }
}

// Things that we can do offline with storage.
impl<'atblock, Client, T> StorageClient<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OfflineClientAtBlockT<'atblock, T>,
{
    /// Select the storage entry you'd like to work with.
    pub fn entry(
        &self,
        pallet_name: impl Into<String>,
        entry_name: impl Into<String>,
    ) -> Result<StorageEntryClient<'atblock, Client, T>, StorageError> {
        let pallet_name = pallet_name.into();
        let entry_name = entry_name.into();

        let storage_info = AnyStorageInfo::new(
            &pallet_name,
            &entry_name,
            self.client.metadata(),
            self.client.legacy_types(),
        )?;

        Ok(StorageEntryClient {
            client: self.client,
            pallet_name,
            entry_name,
            info: Arc::new(storage_info),
            marker: std::marker::PhantomData,
        })
    }

    /// Iterate over all of the storage entries listed in the metadata for the current block. This does **not** include well known
    /// storage entries like `:code` which are not listed in the metadata.
    pub fn entries(&self) -> impl Iterator<Item = StorageEntriesItem<'atblock, Client, T>> {
        let client = self.client;
        let metadata = client.metadata();

        let mut pallet_name = Cow::Borrowed("");
        list_storage_entries_any::list_storage_entries_any(metadata).filter_map(move |entry| {
            match entry {
                frame_decode::storage::StorageEntry::In(name) => {
                    // Set the pallet name for upcoming entries:
                    pallet_name = name;
                    None
                }
                frame_decode::storage::StorageEntry::Name(entry_name) => {
                    // Output each entry with the last seen pallet name:
                    Some(StorageEntriesItem {
                        pallet_name: pallet_name.clone(),
                        entry_name,
                        client: self.client,
                        marker: std::marker::PhantomData,
                    })
                }
            }
        })
    }
}

/// Working with a specific storage entry.
pub struct StorageEntriesItem<'atblock, Client, T> {
    pallet_name: Cow<'atblock, str>,
    entry_name: Cow<'atblock, str>,
    client: &'atblock Client,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, Client, T> StorageEntriesItem<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OfflineClientAtBlockT<'atblock, T>,
{
    /// The pallet name.
    pub fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    /// The storage entry name.
    pub fn entry_name(&self) -> &str {
        &self.entry_name
    }

    /// Extract the relevant storage information so that we can work with this entry.
    pub fn entry(&self) -> Result<StorageEntryClient<'atblock, Client, T>, StorageError> {
        StorageClient {
            client: self.client,
            marker: std::marker::PhantomData,
        }
        .entry(&*self.pallet_name, &*self.entry_name)
    }
}

/// A client for working with a specific storage entry.
pub struct StorageEntryClient<'atblock, Client, T> {
    client: &'atblock Client,
    pallet_name: String,
    entry_name: String,
    info: Arc<AnyStorageInfo<'atblock>>,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, Client, T> StorageEntryClient<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OfflineClientAtBlockT<'atblock, T>,
{
    /// Get the pallet name.
    pub fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    /// Get the storage entry name.
    pub fn entry_name(&self) -> &str {
        &self.entry_name
    }

    /// The key which points to this storage entry (but not necessarily any values within it).
    pub fn key_prefix(&self) -> [u8; 32] {
        let pallet_name = &*self.pallet_name;
        let entry_name = &*self.entry_name;

        frame_decode::storage::encode_storage_key_prefix(pallet_name, entry_name)
    }

    /// Return the default value for this storage entry, if there is one. Returns `None` if there
    /// is no default value.
    pub fn default_value(&self) -> Option<StorageValue<'atblock>> {
        with_info!(info = &*self.info => {
            info.info.default_value.as_ref().map(|default_value| {
                StorageValue::new(self.info.clone(), default_value.clone())
            })
        })
    }
}

impl<'atblock, Client, T> StorageEntryClient<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OnlineClientAtBlockT<'atblock, T>,
{
    /// Fetch a specific key in this map. If the number of keys provided is not equal
    /// to the number of keys required to fetch a single value from the map, then an error
    /// will be emitted. If no value exists but there is a default value for this storage
    /// entry, then the default value will be returned. Else, `None` will be returned.
    pub async fn fetch<Keys: IntoEncodableValues>(
        &self,
        keys: Keys,
    ) -> Result<Option<StorageValue<'atblock>>, StorageError> {
        let expected_num_keys = with_info!(info = &*self.info => {
            info.info.keys.len()
        });

        // For fetching, we need exactly as many keys as exist for a storage entry.
        if expected_num_keys != keys.num_encodable_values() {
            return Err(StorageError::WrongNumberOfKeysProvidedForFetch {
                num_keys_provided: keys.num_encodable_values(),
                num_keys_expected: expected_num_keys,
            });
        }

        let key_bytes = self.key(keys)?;
        let info = self.info.clone();
        let value = fetch(self.client, &key_bytes)
            .await?
            .map(|bytes| StorageValue::new(info, Cow::Owned(bytes)))
            .or_else(|| self.default_value());

        Ok(value)
    }

    /// Iterate over the values underneath the provided keys.
    pub async fn iter<Keys: IntoEncodableValues>(
        &self,
        keys: Keys,
    ) -> Result<
        impl futures::Stream<Item = Result<StorageEntry<'atblock>, StorageError>>
        + Unpin
        + use<'atblock, Client, T, Keys>,
        StorageError,
    > {
        use futures::stream::StreamExt;
        use subxt_rpcs::methods::chain_head::{
            ArchiveStorageEvent, ArchiveStorageQuery, StorageQueryType,
        };

        let expected_num_keys = with_info!(info = &*self.info => {
            info.info.keys.len()
        });

        // For iterating, we need at most one less key than the number that exists for a storage entry.
        // TODO: The error message will be confusing if == keys are provided!
        if keys.num_encodable_values() >= expected_num_keys {
            return Err(StorageError::TooManyKeysProvidedForIter {
                num_keys_provided: keys.num_encodable_values(),
                max_keys_expected: expected_num_keys - 1,
            });
        }

        let block_hash = self.client.block_hash();
        let key_bytes = self.key(keys)?;

        let items = std::iter::once(ArchiveStorageQuery {
            key: &*key_bytes,
            query_type: StorageQueryType::DescendantsValues,
            pagination_start_key: None,
        });

        let sub = self
            .client
            .rpc_methods()
            .archive_v1_storage(block_hash.into(), items, None)
            .await
            .map_err(|e| StorageError::RpcError { reason: e })?;

        let info = self.info.clone();
        let sub = sub.filter_map(move |item| {
            let info = info.clone();
            async move {
                let item = match item {
                    Ok(ArchiveStorageEvent::Item(item)) => item,
                    Ok(ArchiveStorageEvent::Error(err)) => {
                        return Some(Err(StorageError::StorageEventError { reason: err.error }));
                    }
                    Ok(ArchiveStorageEvent::Done) => return None,
                    Err(e) => return Some(Err(StorageError::RpcError { reason: e })),
                };

                item.value
                    .map(|value| Ok(StorageEntry::new(info, item.key.0, Cow::Owned(value.0))))
            }
        });

        Ok(Box::pin(sub))
    }

    // Encode a storage key for this storage entry to bytes. The key can be a partial key
    // (i.e there are still multiple values below it) or a complete key that points to a specific value.
    //
    // Dev note: We don't have any functions that can take an already-encoded key and fetch an entry from
    // it yet, so we don't expose this. If we did expose it, we might want to return some struct that wraps
    // the key bytes and some metadata about them. Or maybe just fetch_raw and iter_raw.
    fn key<Keys: IntoEncodableValues>(&self, keys: Keys) -> Result<Vec<u8>, StorageError> {
        with_info!(info = &*self.info => {
            let key_bytes = frame_decode::storage::encode_storage_key_with_info(
                &self.pallet_name,
                &self.entry_name,
                keys,
                &info.info,
                info.resolver,
            ).map_err(|e| StorageError::KeyEncodeError { reason: e })?;
            Ok(key_bytes)
        })
    }
}

// Fetch a single storage value by its key.
async fn fetch<'atblock, Client, T>(
    client: &Client,
    key_bytes: &[u8],
) -> Result<Option<Vec<u8>>, StorageError>
where
    T: Config + 'atblock,
    Client: OnlineClientAtBlockT<'atblock, T>,
{
    use subxt_rpcs::methods::chain_head::{
        ArchiveStorageEvent, ArchiveStorageQuery, StorageQueryType,
    };

    let query = ArchiveStorageQuery {
        key: key_bytes,
        query_type: StorageQueryType::Value,
        pagination_start_key: None,
    };

    let mut response_stream = client
        .rpc_methods()
        .archive_v1_storage(client.block_hash().into(), std::iter::once(query), None)
        .await
        .map_err(|e| StorageError::RpcError { reason: e })?;

    let value = response_stream
        .next()
        .await
        .transpose()
        .map_err(|e| StorageError::RpcError { reason: e })?;

    // No value found.
    let Some(value) = value else {
        return Ok(None);
    };

    let item = match value {
        ArchiveStorageEvent::Item(item) => item,
        // if it errors, return the error:
        ArchiveStorageEvent::Error(err) => {
            return Err(StorageError::StorageEventError { reason: err.error });
        }
        // if it's done, it means no value was returned:
        ArchiveStorageEvent::Done => return Ok(None),
    };

    // This shouldn't happen, but if it does, the value we wanted wasn't found.
    if item.key.0 != key_bytes {
        return Ok(None);
    }

    // The bytes for the storage value. If this is None, then the API is misbehaving,
    // ot no matching value was found.
    let Some(value_bytes) = item.value else {
        return Ok(None);
    };

    Ok(Some(value_bytes.0))
}
