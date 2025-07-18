mod storage_info;
mod storage_value;

use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::StorageError;
use storage_info::AnyStorageInfo;

pub use storage_value::StorageValue;

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
    pub fn entry(&self, pallet_name: impl Into<String>, storage_name: impl Into<String>) -> Result<StorageEntryClient<'atblock, Client, T>, StorageError> {
        let pallet_name = pallet_name.into();
        let storage_name = storage_name.into();
        
        let storage_info = AnyStorageInfo::new(
            &pallet_name,
            &storage_name,
            self.client.metadata(),
            self.client.legacy_types(),
        )?;

        if storage_info.is_map() {
            todo!()
        } else {
            todo!()
        }
    }

    /// Iterate over all of the storage entries listed in the metadata for the current block. This does **not** include well known
    /// storage entries like `:code` which are not listed in the metadata.
    pub fn entries(&self) -> impl Iterator<Item = StorageEntriesItem<'atblock, Client, T>> {
        let client = self.client;
        let metadata = client.metadata();
        frame_decode::helpers::list_storage_entries_any(metadata).map(|entry| {
            StorageEntriesItem {
                entry,
                client: self.client,
                marker: std::marker::PhantomData,
            }
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
        StorageClient { client: self.client, marker: std::marker::PhantomData }
            .entry(self.entry.pallet().to_owned(), self.entry.entry().to_owned())
    }
}

/// A client for working with a specific storage entry. This is an enum because the storage entry
/// might be either a map or a plain value, and each has a different interface.
pub enum StorageEntryClient<'atblock, Client, T> {
    Plain(StorageEntryPlainClient<'atblock, Client, T>),
    Map(StorageEntryMapClient<'atblock, Client, T>),
}

impl <'atblock, Client, T> StorageEntryClient<'atblock, Client, T>
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

    /// If this storage entry is a plain value, return the client for working with it. Else return `None`.
    pub fn into_plain(self) -> Option<StorageEntryPlainClient<'atblock, Client, T>> {
        match self {
            StorageEntryClient::Plain(client) => Some(client),
            StorageEntryClient::Map(_) => None,
        }
    }

    /// If this storage entry is a map, return the client for working with it. Else return `None`.
    pub fn into_map(self) -> Option<StorageEntryMapClient<'atblock, Client, T>> {
        match self {
            StorageEntryClient::Plain(_) => None,
            StorageEntryClient::Map(client) => Some(client),
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

impl <'atblock, Client, T> StorageEntryPlainClient<'atblock, Client, T>
where
    T: Config + 'atblock,
    Client: OnlineClientAtBlockT<'atblock, T>,
{
    /// Fetch the value for this storage entry.
    pub async fn fetch(&self) -> Result<StorageValue<'_, 'atblock>, StorageError> {
        use subxt_rpcs::methods::chain_head::{ StorageQuery, StorageQueryType, ArchiveStorageEvent };

        let pallet_name = &*self.pallet_name;
        let storage_name = &*self.storage_name;

        let key = frame_decode::storage::prefix(
            pallet_name,
            storage_name,
        );

        let query = StorageQuery { 
            key: &key[..], 
            query_type: StorageQueryType::Value
        };

        let mut response_stream = self.client.rpc_methods().archive_v1_storage(
            self.client.block_hash().into(),
            std::iter::once(query),
            None,
        ).await.map_err(|e| StorageError::FetchError { reason: e })?;

        let value = response_stream
            .next()
            .await
            .ok_or_else(|| StorageError::PlainValueNotFound { 
                pallet_name: pallet_name.to_string(), 
                storage_name: storage_name.to_string() 
            })?
            .map_err(|e| StorageError::FetchError { reason: e })?;

        let item = match value {
            ArchiveStorageEvent::Item(item) => item,
            // if it errors, return the error:
            ArchiveStorageEvent::Error(err) => return Err(StorageError::FetchStreamError { 
                reason: err.error 
            }),
            // if it's done, it means no value was returned:
            ArchiveStorageEvent::Done => return Err(StorageError::PlainValueNotFound {
                pallet_name: pallet_name.to_string(),
                storage_name: storage_name.to_string(),
            }),
        };

        // If the API does what it's supposed to, this shouldn't happen.
        if item.key.0 != key {
            return Err(StorageError::ApiMisbehaving {
                error: format!("Fetching entry {pallet_name}.{storage_name}: Expected value for key {key:?}, got key {:?}", item.key.0),
            });
        }

        // The bytes for the storage value. Again, if the API does what
        // it's supposed to, this shouldn't happen.
        let value_bytes = item.value.ok_or_else(|| StorageError::ApiMisbehaving {
            error: format!("Fetching entry {pallet_name}.{storage_name}: Expected a value to be returned in the response item"),
        })?.0;

        Ok(StorageValue::new(&self.info, value_bytes))
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

impl <'atblock, Client, T> StorageEntryMapClient<'atblock, Client, T>
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

    // TODO: iter function which returns structs containing value and key and fns to decode.
}
