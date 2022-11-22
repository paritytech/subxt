// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    client::OnlineClientT,
    error::Error,
    events::Events,
    Config,
};
use derivative::Derivative;
use sp_core::{
    storage::StorageKey,
    twox_128,
};
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
        block_hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Events<T>, Error>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move {
            // If block hash is not provided, get the hash
            // for the latest block and use that.
            let (block_hash, metadata) = match block_hash {
                Some(hash) => {
                    // If the block hash is provided, the hash can be of an older
                    // block where the metadata was different. Fetch the metadata
                    // at the specific block to ensure proper dynamic decoding of
                    // events.
                    (hash, client.rpc().metadata(Some(hash)).await?)
                }
                None => {
                    // If the block hash was not provided, the metadata is
                    // extracted from the client and is presumed to be up to
                    // date (ie client should subscribe to the runtime upgrades).
                    (
                        client
                            .rpc()
                            .block_hash(None)
                            .await?
                            .expect("didn't pass a block number; qed"),
                        client.metadata(),
                    )
                }
            };

            let event_bytes = client
                .rpc()
                .storage(&system_events_key().0, Some(block_hash))
                .await?
                .map(|e| e.0)
                .unwrap_or_else(Vec::new);

            Ok(Events::new(metadata, block_hash, event_bytes))
        }
    }
}

// The storage key needed to access events.
fn system_events_key() -> StorageKey {
    let mut storage_key = twox_128(b"System").to_vec();
    storage_key.extend(twox_128(b"Events").to_vec());
    StorageKey(storage_key)
}
