// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    client::OnlineClientT,
    error::Error,
    events::{
        EventSub,
        EventSubscription,
        Events,
        FinalizedEventSub,
    },
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
    pub fn at(
        &self,
        block_hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Events<T>, Error>> + Send + 'static {
        // Clone and pass the client in like this so that we can explicitly
        // return a Future that's Send + 'static, rather than tied to &self.
        let client = self.client.clone();
        async move { at(client, block_hash).await }
    }

    /// Subscribe to all events from blocks.
    ///
    /// **Note:** these blocks haven't necessarily been finalised yet; prefer
    /// [`EventsClient::subscribe_finalized()`] if that is important.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// use futures::StreamExt;
    /// use subxt::{ OnlineClient, PolkadotConfig };
    ///
    /// let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    ///
    /// let mut events = api.events().subscribe().await.unwrap();
    ///
    /// while let Some(ev) = events.next().await {
    ///     // Obtain all events from this block.
    ///     let ev = ev.unwrap();
    ///     // Print block hash.
    ///     println!("Event at block hash {:?}", ev.block_hash());
    ///     // Iterate over all events.
    ///     let mut iter = ev.iter();
    ///     while let Some(event_details) = iter.next() {
    ///         println!("Event details {:?}", event_details);
    ///     }
    /// }
    /// # }
    /// ```
    pub fn subscribe(
        &self,
    ) -> impl Future<
        Output = Result<EventSubscription<T, Client, EventSub<T::Header>>, Error>,
    > + Send
           + 'static
    where
        Client: Send + Sync + 'static,
        T: Send + Sync,
    {
        let client = self.client.clone();
        async move { subscribe(client).await }
    }

    /// Subscribe to events from finalized blocks. See [`EventsClient::subscribe()`] for details.
    pub fn subscribe_finalized(
        &self,
    ) -> impl Future<
        Output = Result<
            EventSubscription<T, Client, FinalizedEventSub<T::Header>>,
            Error,
        >,
    > + Send
           + 'static
    where
        Client: Send + Sync + 'static,
        T: Send + Sync,
    {
        let client = self.client.clone();
        async move { subscribe_finalized(client).await }
    }
}

async fn at<T, Client>(
    client: Client,
    block_hash: Option<T::Hash>,
) -> Result<Events<T>, Error>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    // If block hash is not provided, get the hash
    // for the latest block and use that.
    let block_hash = match block_hash {
        Some(hash) => hash,
        None => {
            client
                .rpc()
                .block_hash(None)
                .await?
                .expect("didn't pass a block number; qed")
        }
    };

    let event_bytes = client
        .rpc()
        .storage(&*system_events_key().0, Some(block_hash))
        .await?
        .map(|e| e.0)
        .unwrap_or_else(Vec::new);

    Ok(Events::new(client.metadata(), block_hash, event_bytes))
}

async fn subscribe<T, Client>(
    client: Client,
) -> Result<EventSubscription<T, Client, EventSub<T::Header>>, Error>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    let block_subscription = client.blocks().subscribe_headers().await?;
    Ok(EventSubscription::new(client, Box::pin(block_subscription)))
}

/// Subscribe to events from finalized blocks.
async fn subscribe_finalized<T, Client>(
    client: Client,
) -> Result<EventSubscription<T, Client, FinalizedEventSub<T::Header>>, Error>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    let block_subscription = client.blocks().subscribe_finalized_headers().await?;
    Ok(EventSubscription::new(client, Box::pin(block_subscription)))
}

// The storage key needed to access events.
fn system_events_key() -> StorageKey {
    let mut storage_key = twox_128(b"System").to_vec();
    storage_key.extend(twox_128(b"Events").to_vec());
    StorageKey(storage_key)
}
