mod offline_client;
mod online_client;

use crate::config::{Config, HashFor};
use crate::constants::ConstantsClient;
use crate::custom_values::CustomValuesClient;
use crate::error::BlockError;
use crate::events::EventsClient;
use crate::extrinsics::ExtrinsicsClient;
use crate::runtime_apis::RuntimeApisClient;
use crate::storage::StorageClient;
use crate::transactions::TransactionsClient;
use crate::view_functions::ViewFunctionsClient;
use core::marker::PhantomData;
use subxt_metadata::Metadata;

pub use offline_client::{OfflineClient, OfflineClientAtBlock, OfflineClientAtBlockT};
pub use online_client::{
    BlockNumberOrRef, OnlineClient, OnlineClientAtBlock, OnlineClientAtBlockT,
};

/// This represents a client at a specific block number.
#[derive(Clone, Debug)]
pub struct ClientAtBlock<T, Client> {
    pub(crate) client: Client,
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
    /// Construct and submit transactions. This is a
    /// shorthand to [`Self::transactions()`].
    pub fn tx(&self) -> TransactionsClient<'_, T, Client> {
        TransactionsClient::new(&self.client)
    }

    /// Construct and submit transactions.
    pub fn transactions(&self) -> TransactionsClient<'_, T, Client> {
        TransactionsClient::new(&self.client)
    }

    /// Access storage at this block.
    pub fn storage(&self) -> StorageClient<'_, T, Client> {
        StorageClient::new(&self.client)
    }

    /// Access constants at this block.
    pub fn constants(&self) -> ConstantsClient<'_, T, Client> {
        ConstantsClient::new(&self.client)
    }

    /// Access custom values at this block.
    pub fn custom_values(&self) -> CustomValuesClient<'_, T, Client> {
        CustomValuesClient::new(&self.client)
    }

    /// Work with the extrinsics in this block.
    pub fn extrinsics(&self) -> ExtrinsicsClient<'_, T, Client> {
        ExtrinsicsClient::new(&self.client)
    }

    /// Work with the events at this block.
    pub fn events(&self) -> EventsClient<'_, T, Client> {
        EventsClient::new(&self.client)
    }

    /// Access runtime APIs at this block.
    pub fn runtime_apis(&self) -> RuntimeApisClient<'_, T, Client> {
        RuntimeApisClient::new(&self.client)
    }

    pub fn view_functions(&self) -> ViewFunctionsClient<'_, T, Client> {
        ViewFunctionsClient::new(&self.client)
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
    /// The current block hash.
    pub fn block_hash(&self) -> HashFor<T> {
        self.client.block_hash()
    }

    /// The header for this block.
    pub async fn block_header(&self) -> Result<T::Header, BlockError> {
        let block_hash = self.block_hash();
        let header = self
            .client
            .backend()
            .block_header(block_hash)
            .await
            .map_err(|e| BlockError::CouldNotDownloadBlockHeader {
                block_hash: block_hash.into(),
                reason: e,
            })?
            .ok_or_else(|| BlockError::BlockNotFound {
                block_hash: block_hash.into(),
            })?;
        Ok(header)
    }
}
