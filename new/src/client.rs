mod offline_client;
mod online_client;

use crate::config::{Config, HashFor};
use crate::constants::ConstantsClient;
use crate::custom_values::CustomValuesClient;
use crate::error::{EventsError, ExtrinsicError};
use crate::events::Events;
use crate::extrinsics::Extrinsics;
use crate::runtime_apis::RuntimeApisClient;
use crate::storage::StorageClient;
use crate::transactions::TransactionsClient;
use crate::view_functions::ViewFunctionsClient;
use core::marker::PhantomData;
use subxt_metadata::Metadata;

pub use offline_client::{OfflineClient, OfflineClientAtBlock, OfflineClientAtBlockT};
pub use online_client::{OnlineClient, OnlineClientAtBlock, OnlineClientAtBlockT};

/// This represents a client at a specific block number.
#[derive(Clone, Debug)]
pub struct ClientAtBlock<T, Client> {
    client: Client,
    marker: PhantomData<T>,
}

impl<T, Client> ClientAtBlock<T, Client> {
    /// Construct a new client at some block.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            marker: PhantomData,
        }
    }
}

impl<T, Client> ClientAtBlock<T, Client>
where
    T: Config,
    Client: OfflineClientAtBlockT<T>,
{
    /// Construct and submit transactions.
    pub fn tx(&self) -> TransactionsClient<T, Client> {
        TransactionsClient::new(self.client.clone())
    }

    /// Access storage at this block.
    pub fn storage(&self) -> StorageClient<T, Client> {
        StorageClient::new(self.client.clone())
    }

    /// Access constants at this block.
    pub fn constants(&self) -> ConstantsClient<T, Client> {
        ConstantsClient::new(self.client.clone())
    }

    pub fn custom_values(&self) -> CustomValuesClient<T, Client> {
        CustomValuesClient::new(self.client.clone())
    }

    /// Access runtime APIs at this block.
    pub fn runtime_apis(&self) -> RuntimeApisClient<T, Client> {
        RuntimeApisClient::new(self.client.clone())
    }

    pub fn view_functions(&self) -> ViewFunctionsClient<T, Client> {
        ViewFunctionsClient::new(self.client.clone())
    }

    /// Obtain a reference to the metadata.
    pub fn metadata_ref(&self) -> &Metadata {
        self.client.metadata_ref()
    }

    /// The current block number.
    pub fn block_number(&self) -> u64 {
        self.client.block_number()
    }
}

impl<T, Client> ClientAtBlock<T, Client>
where
    T: Config,
    Client: OnlineClientAtBlockT<T>,
{
    /// Obtain the extrinsics in this block.
    pub async fn extrinsics(&self) -> Result<Extrinsics<T, Client>, ExtrinsicError> {
        Extrinsics::fetch(self.client.clone()).await
    }

    /// Obtain the extrinsic events at this block.
    pub async fn events(&self) -> Result<Events<T>, EventsError> {
        Events::fetch(self.client.clone()).await
    }

    /// The current block hash.
    pub fn block_hash(&self) -> HashFor<T> {
        self.client.block_hash()
    }
}
