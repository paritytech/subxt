use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::StorageError;
use std::borrow::Cow;

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
    pub fn entry<'names>(&self, pallet_name: &'names str, storage_name: &'names str) -> StorageEntryClient<'atblock, 'names, Client, T> {
        StorageEntryClient {
            pallet_name: Cow::Borrowed(pallet_name),
            storage_name: Cow::Borrowed(storage_name),
            client: self.client,
            marker: std::marker::PhantomData,
        }
    }

    /// Iterate over all of the storage entries listed in the runtime metadata. This does **not** include well known
    /// storage entries like `:code` which are not listed in the runtime metadata.
    pub fn entries(&self) -> impl Iterator<Item = StorageEntryClient<'atblock, 'atblock, Client, T>> {
        let client = self.client;
        let metadata = client.metadata();
        frame_decode::helpers::list_storage_entries_any(metadata).map(|entry| {
            StorageEntryClient {
                pallet_name: Cow::Owned(entry.pallet().into()),
                storage_name: Cow::Owned(entry.entry().into()),
                client: self.client,
                marker: std::marker::PhantomData,
            }
        })
    }
}

/// Working with a specific storage entry.
pub struct StorageEntryClient<'atblock, 'names, Client, T> {
    pallet_name: Cow<'names, str>,
    storage_name: Cow<'names, str>,
    client: &'atblock Client,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, 'names, 'client: 'atblock, Client, T> StorageEntryClient<'atblock, 'names, Client, T>
where
    T: Config + 'client,
    Client: OnlineClientAtBlockT<'client, T>,
{
    /// Get the value of the storage entry at a specific block. This will return either a struct representing
    /// a plain value that can be decoded, or if the storage entry is a map, it will return a struct representing
    /// this map and allowing iteration over the keys in it.
    pub async fn fetch(&self) -> Result<Option<T>, StorageError> {

        // TODO: get storage info, work out whether single value or map, then return relevant enum ready
        // to work with this entry. prob want to rename. enum can then fetch or iterate and be unwrapped into Option
        // for simplicity when you know what you're asking for. Could provide "higher level" APIs that do some of
        // this internally.

        todo!()
    }
}
