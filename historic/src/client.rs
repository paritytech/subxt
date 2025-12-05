mod offline_client;
mod online_client;

use crate::config::Config;
use crate::extrinsics::ExtrinsicsClient;
use crate::storage::StorageClient;
use crate::utils::AnyResolver;
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

    /// Return something which implements [`scale_type_resolver::TypeResolver`] and
    /// can be used in conjnction with type IDs in `.visit` methods.
    pub fn resolver(&self) -> AnyResolver<'_, 'client> {
        match self.client.metadata() {
            RuntimeMetadata::V0(_)
            | RuntimeMetadata::V1(_)
            | RuntimeMetadata::V2(_)
            | RuntimeMetadata::V3(_)
            | RuntimeMetadata::V4(_)
            | RuntimeMetadata::V5(_)
            | RuntimeMetadata::V6(_)
            | RuntimeMetadata::V7(_)
            | RuntimeMetadata::V8(_)
            | RuntimeMetadata::V9(_)
            | RuntimeMetadata::V10(_)
            | RuntimeMetadata::V11(_)
            | RuntimeMetadata::V12(_)
            | RuntimeMetadata::V13(_) => AnyResolver::B(self.client.legacy_types()),
            RuntimeMetadata::V14(m) => AnyResolver::A(&m.types),
            RuntimeMetadata::V15(m) => AnyResolver::A(&m.types),
            RuntimeMetadata::V16(m) => AnyResolver::A(&m.types),
        }
    }
}
