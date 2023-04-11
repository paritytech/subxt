// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{client::OnlineClientT, error::Error, events::Events, rpc::types::StorageKey, Config};
use derivative::Derivative;
use std::future::Future;

/// A client for working with events.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct EventsClient<T, Client> {
    client: Client,
    _marker: std::marker::PhantomData<T>,
}

impl<T, Client> EventsClient<T, Client> {
    /// Create a new [`EventsClient`].
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, Client> EventsClient<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Obtain events at some block hash.
    ///
    /// # Warning
    ///
    /// This call only supports blocks produced since the most recent
    /// runtime upgrade. You can attempt to retrieve events from older blocks,
    /// but may run into errors attempting to work with them.
    pub fn at(
        &self,
        block_hash: T::Hash,
    ) -> impl Future<Output = Result<Events<T>, Error>> + Send + 'static {
        self.at_or_latest(Some(block_hash))
    }

    /// Obtain events at the latest block hash.
    pub fn at_latest(&self) -> impl Future<Output = Result<Events<T>, Error>> + Send + 'static {
        self.at_or_latest(None)
    }

    /// Obtain events at some block hash.
    fn at_or_latest(
        &self,
        block_hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Events<T>, Error>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move {
            // If block hash is not provided, get the hash
            // for the latest block and use that.
            let block_hash = match block_hash {
                Some(hash) => hash,
                None => client
                    .rpc()
                    .block_hash(None)
                    .await?
                    .expect("didn't pass a block number; qed"),
            };

            let event_bytes = get_event_bytes(&client, Some(block_hash)).await?;
            Ok(Events::new(client.metadata(), block_hash, event_bytes))
        }
    }
}

// The storage key needed to access events.
fn system_events_key() -> StorageKey {
    let mut storage_key = sp_core_hashing::twox_128(b"System").to_vec();
    storage_key.extend(sp_core_hashing::twox_128(b"Events").to_vec());
    StorageKey(storage_key)
}

// Get the event bytes from the provided client, at the provided block hash.
pub(crate) async fn get_event_bytes<T, Client>(
    client: &Client,
    block_hash: Option<T::Hash>,
) -> Result<Vec<u8>, Error>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    Ok(client
        .rpc()
        .storage(&system_events_key().0, block_hash)
        .await?
        .map(|e| e.0)
        .unwrap_or_else(Vec::new))
}
