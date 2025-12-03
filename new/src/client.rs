mod offline_client;
mod online_client;

use crate::config::{Config, HashFor};
use crate::transactions::Transactions;
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
    /// Construct transactions.
    pub fn tx(&self) -> Transactions<T, Client> {
        Transactions::new(self.client.clone())
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
}
