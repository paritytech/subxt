mod storage_entry;
mod storage_info;
mod storage_key;
mod storage_value;

use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::{StorageEntryIsNotAMap, StorageEntryIsNotAPlainValue, StorageError};
use crate::storage::storage_info::with_info;
use storage_info::AnyStorageInfo;

pub use storage_entry::StorageEntry;
pub use storage_key::StorageKey;
pub use storage_value::StorageValue;
// We take how storage keys can be passed in from `frame-decode`, so re-export here.
pub use frame_decode::storage::{IntoStorageKeys, StorageKeys};

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
impl<'atblock, 'client: 'atblock, Client, T> StorageClient<'atblock, Client, T>
where
    T: Config + 'client,
    Client: OfflineClientAtBlockT<'client, T>,
{
    /// Select the storage entry you'd like to work with.
    pub fn entry(
        &self,
        pallet_name: impl Into<String>,
        storage_name: impl Into<String>,
    ) -> Result<StorageEntryClient<'atblock, Client, T>, StorageError> {
        let pallet_name = pallet_name.into();
        let storage_name = storage_name.into();

        let storage_info = AnyStorageInfo::new(
            &pallet_name,
            &storage_name,
            self.client.metadata(),
            self.client.legacy_types(),
        )?;

        if storage_info.is_map() {
            Ok(StorageEntryClient::Map(StorageEntryMapClient {
                client: self.client,
                pallet_name,
                storage_name,
                info: storage_info,
                marker: std::marker::PhantomData,
            }))
        } else {
            Ok(StorageEntryClient::Plain(StorageEntryPlainClient {
                client: self.client,
                pallet_name,
                storage_name,
                info: storage_info,
                marker: std::marker::PhantomData,
            }))
        }
    }

    /// Iterate over all of the storage entries listed in the metadata for the current block. This does **not** include well known
    /// storage entries like `:code` which are not listed in the metadata.
    pub fn entries(&self) -> impl Iterator<Item = StorageEntriesItem<'atblock, Client, T>> {
        let client = self.client;
        let metadata = client.metadata();
        frame_decode::helpers::list_storage_entries_any(metadata).map(|entry| StorageEntriesItem {
            entry,
            client: self.client,
            marker: std::marker::PhantomData,
        })
    }
}

/// Working with a specific storage entry.
pub struct StorageEntriesItem<'atblock, Client, T> {
    entry: frame_decode::helpers::StorageEntry<'atblock>,
    client: &'atblock Client,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, 'client: 'atblock, Client, T> StorageEntriesItem<'atblock, Client, T>
where
    T: Config + 'client,
    Client: OfflineClientAtBlockT<'client, T>,
{
    /// The pallet name.
    pub fn pallet_name(&self) -> &str {
        self.entry.pallet()
    }

    /// The storage entry name.
    pub fn storage_name(&self) -> &str {
        self.entry.entry()
    }

    /// Extract the relevant storage information so that we can work with this entry.
    pub fn entry(&self) -> Result<StorageEntryClient<'atblock, Client, T>, StorageError> {
        StorageClient {
            client: self.client,
            marker: std::marker::PhantomData,
        }
        .entry(
            self.entry.pallet().to_owned(),
            self.entry.entry().to_owned(),
        )
    }
}

/// A client for working with a specific storage entry. This is an enum because the storage entry
/// might be either a map or a plain value, and each has a different interface.
pub enum StorageEntryClient<'atblock, Client, T> {
    Plain(StorageEntryPlainClient<'atblock, Client, T>),
    Map(StorageEntryMapClient<'atblock, Client, T>),
}

impl<'atblock, Client, T> StorageEntryClient<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OfflineClientAtBlockT<'atblock, T>,
{
    /// Get the pallet name.
    pub fn pallet_name(&self) -> &str {
        match self {
            StorageEntryClient::Plain(client) => &client.pallet_name,
            StorageEntryClient::Map(client) => &client.pallet_name,
        }
    }

    /// Get the storage entry name.
    pub fn storage_name(&self) -> &str {
        match self {
            StorageEntryClient::Plain(client) => &client.storage_name,
            StorageEntryClient::Map(client) => &client.storage_name,
        }
    }

    /// Is the storage entry a plain value?
    pub fn is_plain(&self) -> bool {
        matches!(self, StorageEntryClient::Plain(_))
    }

    /// Is the storage entry a map?
    pub fn is_map(&self) -> bool {
        matches!(self, StorageEntryClient::Map(_))
    }

    /// If this storage entry is a plain value, return the client for working with it. Else return an error.
    pub fn into_plain(
        self,
    ) -> Result<StorageEntryPlainClient<'atblock, Client, T>, StorageEntryIsNotAPlainValue> {
        match self {
            StorageEntryClient::Plain(client) => Ok(client),
            StorageEntryClient::Map(_) => Err(StorageEntryIsNotAPlainValue {
                pallet_name: self.pallet_name().into(),
                storage_name: self.storage_name().into(),
            }),
        }
    }

    /// If this storage entry is a map, return the client for working with it. Else return an error.
    pub fn into_map(
        self,
    ) -> Result<StorageEntryMapClient<'atblock, Client, T>, StorageEntryIsNotAMap> {
        match self {
            StorageEntryClient::Plain(_) => Err(StorageEntryIsNotAMap {
                pallet_name: self.pallet_name().into(),
                storage_name: self.storage_name().into(),
            }),
            StorageEntryClient::Map(client) => Ok(client),
        }
    }
}

/// A client for working with a plain storage entry.
pub struct StorageEntryPlainClient<'atblock, Client, T> {
    client: &'atblock Client,
    pallet_name: String,
    storage_name: String,
    info: AnyStorageInfo<'atblock>,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, Client, T> StorageEntryPlainClient<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OfflineClientAtBlockT<'atblock, T>,
{
    /// Get the pallet name.
    pub fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    /// Get the storage entry name.
    pub fn storage_name(&self) -> &str {
        &self.storage_name
    }
}

impl<'atblock, Client, T> StorageEntryPlainClient<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OnlineClientAtBlockT<'atblock, T>,
{
    /// Fetch the value for this storage entry.
    pub async fn fetch(&self) -> Result<Option<StorageValue<'_, 'atblock>>, StorageError> {
        let key_bytes = self.key();
        fetch(self.client, &key_bytes)
            .await
            .map(|v| v.map(|bytes| StorageValue::new(&self.info, bytes)))
    }

    /// The key for this storage entry.
    pub fn key(&self) -> [u8; 32] {
        let pallet_name = &*self.pallet_name;
        let storage_name = &*self.storage_name;

        frame_decode::storage::encode_prefix(pallet_name, storage_name)
    }
}

/// A client for working with a storage entry that is a map.
pub struct StorageEntryMapClient<'atblock, Client, T> {
    client: &'atblock Client,
    pallet_name: String,
    storage_name: String,
    info: AnyStorageInfo<'atblock>,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, Client, T> StorageEntryMapClient<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OfflineClientAtBlockT<'atblock, T>,
{
    /// Get the pallet name.
    pub fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    /// Get the storage entry name.
    pub fn storage_name(&self) -> &str {
        &self.storage_name
    }
}

impl<'atblock, Client, T> StorageEntryMapClient<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OnlineClientAtBlockT<'atblock, T>,
{
    /// Fetch a specific key in this map. If the number of keys provided is not equal
    /// to the number of keys required to fetch a single value from the map, then an error
    /// will be emitted.
    pub async fn fetch<Keys: IntoStorageKeys>(
        &self,
        keys: Keys,
    ) -> Result<Option<StorageValue<'_, 'atblock>>, StorageError> {
        let expected_num_keys = with_info!(info = &self.info => {
            info.info.keys.len()
        });

        if expected_num_keys != keys.num_keys() {
            return Err(StorageError::WrongNumberOfKeysProvided {
                num_keys_provided: keys.num_keys(),
                num_keys_expected: expected_num_keys,
            });
        }

        let key_bytes = self.key(keys)?;
        fetch(self.client, &key_bytes)
            .await
            .map(|v| v.map(|bytes| StorageValue::new(&self.info, bytes)))
    }

    /// Iterate over the values underneath the provided keys.
    pub async fn iter<Keys: IntoStorageKeys>(
        &self,
        keys: Keys,
    ) -> Result<
        impl futures::Stream<Item = Result<StorageEntry<'_, 'atblock>, StorageError>>,
        StorageError,
    > {
        use futures::stream::StreamExt;
        use subxt_rpcs::methods::chain_head::{
            ArchiveStorageEvent, StorageQuery, StorageQueryType,
        };

        let block_hash = self.client.block_hash();
        let key_bytes = self.key(keys)?;

        let items = std::iter::once(StorageQuery {
            key: &*key_bytes,
            query_type: StorageQueryType::DescendantsValues,
        });

        let sub = self
            .client
            .rpc_methods()
            .archive_v1_storage(block_hash.into(), items, None)
            .await
            .map_err(|e| StorageError::RpcError { reason: e })?;

        let sub = sub.filter_map(async |item| {
            let item = match item {
                Ok(ArchiveStorageEvent::Item(item)) => item,
                Ok(ArchiveStorageEvent::Error(err)) => {
                    return Some(Err(StorageError::StorageEventError { reason: err.error }));
                }
                Ok(ArchiveStorageEvent::Done) => return None,
                Err(e) => return Some(Err(StorageError::RpcError { reason: e })),
            };

            item.value
                .map(|value| Ok(StorageEntry::new(&self.info, item.key.0, value.0)))
        });

        Ok(sub)
    }

    // Encode a storage key for this storage entry to bytes. The key can be a partial key
    // (i.e there are still multiple values below it) or a complete key that points to a specific value.
    //
    // Dev note: We don't have any functions that can take an already-encoded key and fetch an entry from
    // it yet, so we don't expose this. If we did expose it, we might want to return some struct that wraps
    // the key bytes and some metadata about them. Or maybe just fetch_raw and iter_raw.
    fn key<Keys: IntoStorageKeys>(&self, keys: Keys) -> Result<Vec<u8>, StorageError> {
        with_info!(info = &self.info => {
            let mut key_bytes = Vec::new();
            frame_decode::storage::encode_storage_key_with_info_to(
                &self.pallet_name,
                &self.storage_name,
                keys,
                &info.info,
                info.resolver,
                &mut key_bytes,
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
    use subxt_rpcs::methods::chain_head::{ArchiveStorageEvent, StorageQuery, StorageQueryType};

    let query = StorageQuery {
        key: key_bytes,
        query_type: StorageQueryType::Value,
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
