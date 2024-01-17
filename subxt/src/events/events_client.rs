// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::backend::{Backend, BackendExt, BlockRef};
use crate::{client::OnlineClientT, error::Error, events::Events, Config};
use crate::prelude::*;
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
        block_ref: impl Into<BlockRef<T::Hash>>,
    ) -> impl Future<Output = Result<Events<T>, Error>> + Send + 'static {
        self.at_or_latest(Some(block_ref.into()))
    }

    /// Obtain events for the latest block.
    pub fn at_latest(&self) -> impl Future<Output = Result<Events<T>, Error>> + Send + 'static {
        self.at_or_latest(None)
    }

    /// Obtain events at some block hash.
    fn at_or_latest(
        &self,
        block_ref: Option<BlockRef<T::Hash>>,
    ) -> impl Future<Output = Result<Events<T>, Error>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move {
            // If a block ref isn't provided, we'll get the latest finalized block to use.
            let block_ref = match block_ref {
                Some(r) => r,
                None => client.backend().latest_finalized_block_ref().await?,
            };

            let event_bytes = get_event_bytes(client.backend(), block_ref.hash()).await?;
            Ok(Events::new(
                client.metadata(),
                block_ref.hash(),
                event_bytes,
            ))
        }
    }
}

// The storage key needed to access events.
fn system_events_key() -> [u8; 32] {
    let a = sp_core_hashing::twox_128(b"System");
    let b = sp_core_hashing::twox_128(b"Events");
    let mut res = [0; 32];
    res[0..16].clone_from_slice(&a);
    res[16..32].clone_from_slice(&b);
    res
}

// Get the event bytes from the provided client, at the provided block hash.
pub(crate) async fn get_event_bytes<T: Config>(
    backend: &dyn Backend<T>,
    block_hash: T::Hash,
) -> Result<Vec<u8>, Error> {
    Ok(backend
        .storage_fetch_value(system_events_key().to_vec(), block_hash)
        .await?
        .unwrap_or_default())
}
