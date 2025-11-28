mod offline_client;
mod online_client;

use core::marker::PhantomData;

// We keep these traits internal, so that we can mess with them later if needed,
// and instead only the concrete types are public which wrap these trait impls.
pub(crate) use offline_client::OfflineClientAtBlockT;
pub(crate) use online_client::OnlineClientAtBlockT;

pub use offline_client::OfflineClient;
pub use online_client::OnlineClient;

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