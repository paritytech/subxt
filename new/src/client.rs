mod offline_client;
mod online_client;

use crate::config::{Config, HashFor};
use core::marker::PhantomData;
use subxt_metadata::Metadata;

// We keep these traits internal, so that we can mess with them later if needed,
// and instead only the concrete types are public which wrap these trait impls.
pub(crate) use offline_client::OfflineClientAtBlockT;
pub(crate) use online_client::OnlineClientAtBlockT;

pub use offline_client::{OfflineClient, OfflineClientAtBlock};
pub use online_client::{OnlineClient, OnlineClientAtBlock};

/// This represents a client at a specific block number.
#[derive(Clone, Debug)]
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

impl<Client, T> ClientAtBlock<Client, T>
where
    T: Config,
    Client: OfflineClientAtBlockT,
{
    pub fn metadata_ref(&self) -> &Metadata {
        self.client.metadata_ref()
    }
}

impl<Client, T> ClientAtBlock<Client, T>
where
    T: Config,
    Client: OnlineClientAtBlockT<T>,
{
    pub fn block_hash(&self) -> HashFor<T> {
        self.client.block_hash()
    }
}
