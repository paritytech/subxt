// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    blocks::{extrinsic_types::ExtrinsicPartTypeIds, Extrinsics},
    client::{OfflineClientT, OnlineClientT},
    config::{Config, Header},
    error::{BlockError, Error},
    events,
    rpc::types::ChainBlockResponse,
    runtime_api::RuntimeApi,
    storage::Storage,
};

use futures::lock::Mutex as AsyncMutex;
use std::sync::Arc;

/// A representation of a block.
pub struct Block<T: Config, C> {
    header: T::Header,
    client: C,
    // Since we obtain the same events for every extrinsic, let's
    // cache them so that we only ever do that once:
    cached_events: CachedEvents<T>,
}

// A cache for our events so we don't fetch them more than once when
// iterating over events for extrinsics.
pub(crate) type CachedEvents<T> = Arc<AsyncMutex<Option<events::Events<T>>>>;

impl<T, C> Block<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    pub(crate) fn new(header: T::Header, client: C) -> Self {
        Block {
            header,
            client,
            cached_events: Default::default(),
        }
    }

    /// Return the block hash.
    pub fn hash(&self) -> T::Hash {
        self.header.hash()
    }

    /// Return the block number.
    pub fn number(&self) -> <T::Header as crate::config::Header>::Number {
        self.header().number()
    }

    /// Return the entire block header.
    pub fn header(&self) -> &T::Header {
        &self.header
    }
}

impl<T, C> Block<T, C>
where
    T: Config,
    C: OnlineClientT<T>,
{
    /// Return the events associated with the block, fetching them from the node if necessary.
    pub async fn events(&self) -> Result<events::Events<T>, Error> {
        get_events(&self.client, self.header.hash(), &self.cached_events).await
    }

    /// Fetch and return the block body.
    pub async fn body(&self) -> Result<BlockBody<T, C>, Error> {
        let ids = ExtrinsicPartTypeIds::new(&self.client.metadata())?;
        let block_hash = self.header.hash();
        let Some(block_details) = self.client.rpc().block(Some(block_hash)).await? else {
            return Err(BlockError::not_found(block_hash).into());
        };

        Ok(BlockBody::new(
            self.client.clone(),
            block_details,
            self.cached_events.clone(),
            ids,
        ))
    }

    /// Work with storage.
    pub fn storage(&self) -> Storage<T, C> {
        let block_hash = self.hash();
        Storage::new(self.client.clone(), block_hash)
    }

    /// Execute a runtime API call at this block.
    pub async fn runtime_api(&self) -> Result<RuntimeApi<T, C>, Error> {
        Ok(RuntimeApi::new(self.client.clone(), self.hash()))
    }
}

/// The body of a block.
pub struct BlockBody<T: Config, C> {
    details: ChainBlockResponse<T>,
    client: C,
    cached_events: CachedEvents<T>,
    ids: ExtrinsicPartTypeIds,
}

impl<T, C> BlockBody<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    pub(crate) fn new(
        client: C,
        details: ChainBlockResponse<T>,
        cached_events: CachedEvents<T>,
        ids: ExtrinsicPartTypeIds,
    ) -> Self {
        Self {
            details,
            client,
            cached_events,
            ids,
        }
    }

    /// Returns an iterator over the extrinsics in the block body.
    // Dev note: The returned iterator is 'static + Send so that we can box it up and make
    // use of it with our `FilterExtrinsic` stuff.
    pub fn extrinsics(&self) -> Extrinsics<T, C> {
        Extrinsics::new(
            self.client.clone(),
            self.details.block.extrinsics.clone(),
            self.cached_events.clone(),
            self.ids,
            self.details.block.header.hash(),
        )
    }
}

// Return Events from the cache, or fetch from the node if needed.
pub(crate) async fn get_events<C, T>(
    client: &C,
    block_hash: T::Hash,
    cached_events: &AsyncMutex<Option<events::Events<T>>>,
) -> Result<events::Events<T>, Error>
where
    T: Config,
    C: OnlineClientT<T>,
{
    // Acquire lock on the events cache. We either get back our events or we fetch and set them
    // before unlocking, so only one fetch call should ever be made. We do this because the
    // same events can be shared across all extrinsics in the block.
    let lock = cached_events.lock().await;
    let events = match &*lock {
        Some(events) => events.clone(),
        None => {
            events::EventsClient::new(client.clone())
                .at(block_hash)
                .await?
        }
    };

    Ok(events)
}
