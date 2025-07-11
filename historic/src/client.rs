mod online_client;
mod offline_client;

use std::marker::PhantomData;
use crate::config::Config;
use crate::extrinsics::ExtrinsicsClient;

// We keep these traits internal, so that we can mess with them later if needed,
// and instead only the concrete types are public which wrap these trait impls.
pub (crate) use online_client::OnlineClientAtBlockT;
pub (crate) use offline_client::OfflineClientAtBlockT;

pub use online_client::OnlineClient;
pub use offline_client::OfflineClient;

/// This represents a client at a specific block number.
pub struct ClientAtBlock<Client, T> {
    client: Client,
    marker: PhantomData<T>,
}

impl <Client, T> ClientAtBlock<Client, T> {
    /// Construct a new client at some block.
    pub (crate) fn new(client: Client) -> Self {
        Self {
            client,
            marker: PhantomData,
        }
    }
}

impl <'client, T, Client> ClientAtBlock<Client, T> 
where
    T: Config + 'client,
    Client: OfflineClientAtBlockT<'client, T>
{
    /// Work with extrinsics.
    pub fn extrinsics(&'_ self) -> ExtrinsicsClient<'_, Client, T> {
        ExtrinsicsClient::new(&self.client)
    }
}