mod storage_info;

use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::StorageError;
use std::borrow::Cow;
use storage_info::AnyStorageInfo;

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
    pub fn entry<'names>(&self, pallet_name: impl Into<Cow<'names, str>>, storage_name: impl Into<Cow<'names, str>>) -> Result<StorageEntryClient<'names, 'atblock, Client, T>, StorageError> {
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
    pub fn entry(&self) -> Result<StorageEntryClient<'static, 'atblock, Client, T>, StorageError> {
        StorageClient { client: self.client, marker: std::marker::PhantomData }
            .entry(self.entry.pallet().to_owned(), self.entry.entry().to_owned())
    }
}

/// A client for working with a specific storage entry. This is an enum because the storage entry
/// might be either a map or a plain value, and each has a different interface.
pub enum StorageEntryClient<'names, 'atblock, Client, T> {
    Plain(StorageEntryPlainClient<'names, 'atblock, Client, T>),
    Map(StorageEntryMapClient<'names, 'atblock, Client, T>),
}

impl <'names, 'atblock, Client, T> StorageEntryClient<'names, 'atblock, Client, T>
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
}

/// A client for working with a plain storage entry.
pub struct StorageEntryPlainClient<'names, 'atblock, Client, T> {
    client: &'atblock Client,
    pallet_name: Cow<'names, str>,
    storage_name: Cow<'names, str>,
    info: AnyStorageInfo<'atblock>,
    marker: std::marker::PhantomData<T>,
}

/// A client for working with a storage entry that is a map.
pub struct StorageEntryMapClient<'names, 'atblock, Client, T> {
    client: &'atblock Client,
    pallet_name: Cow<'names, str>,
    storage_name: Cow<'names, str>,
    info: AnyStorageInfo<'atblock>,
    marker: std::marker::PhantomData<T>,
}

