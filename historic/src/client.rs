mod offline_client;
mod online_client;

use crate::config::Config;
use crate::extrinsics::ExtrinsicsClient;
use crate::storage::StorageClient;
use frame_metadata::RuntimeMetadata;
use std::marker::PhantomData;

pub use offline_client::{OfflineClient, OfflineClientAtBlock, OfflineClientAtBlockT};
pub use online_client::{OnlineClient, OnlineClientAtBlock, OnlineClientAtBlockT};

/// This represents a client at a specific block number.
pub struct ClientAtBlock<Client, T> {
    client: Client,
    marker: PhantomData<T>,
}

impl<Client, T> ClientAtBlock<Client, T> {
    /// Construct a new client at some block.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            marker: PhantomData,
        }
    }
}

impl<'client, T, Client> ClientAtBlock<Client, T>
where
    T: Config + 'client,
    Client: OfflineClientAtBlockT<'client, T>,
{
    /// Work with extrinsics.
    pub fn extrinsics(&'_ self) -> ExtrinsicsClient<'_, Client, T> {
        ExtrinsicsClient::new(&self.client)
    }

    /// Work with storage.
    pub fn storage(&'_ self) -> StorageClient<'_, Client, T> {
        StorageClient::new(&self.client)
    }

    /// Return the metadata in use at this block.
    pub fn metadata(&self) -> &RuntimeMetadata {
        self.client.metadata()
    }
}
