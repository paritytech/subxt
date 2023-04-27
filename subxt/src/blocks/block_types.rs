// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    blocks::{extrinsic_types::ExtrinsicIds, ExtrinsicDetails, StaticExtrinsic},
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
        let ids = ExtrinsicIds::new(self.client.metadata().runtime_metadata())?;
        let block_details = self.block_details().await?;

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

    /// Fetch the block's body from the chain.
    async fn block_details(&self) -> Result<ChainBlockResponse<T>, Error> {
        let block_hash = self.header.hash();
        match self.client.rpc().block(Some(block_hash)).await? {
            Some(block) => Ok(block),
            None => Err(BlockError::not_found(block_hash).into()),
        }
    }
}

/// The body of a block.
pub struct BlockBody<T: Config, C> {
    details: ChainBlockResponse<T>,
    client: C,
    cached_events: CachedEvents<T>,
    ids: ExtrinsicIds,
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
        ids: ExtrinsicIds,
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
    pub fn extrinsics(
        &self,
    ) -> impl Iterator<Item = Result<ExtrinsicDetails<T, C>, Error>> + Send + Sync + 'static {
        let extrinsics = self.details.block.extrinsics.clone();
        let num_extrinsics = self.details.block.extrinsics.len();
        let client = self.client.clone();
        let hash = self.details.block.header.hash();
        let cached_events = self.cached_events.clone();
        let ids = self.ids;
        let mut index = 0;

        std::iter::from_fn(move || {
            if index == num_extrinsics {
                None
            } else {
                match ExtrinsicDetails::decode_from(
                    index as u32,
                    extrinsics[index].0.clone().into(),
                    client.clone(),
                    hash,
                    cached_events.clone(),
                    ids,
                ) {
                    Ok(extrinsic_details) => {
                        index += 1;
                        Some(Ok(extrinsic_details))
                    }
                    Err(e) => {
                        index = num_extrinsics;
                        Some(Err(e))
                    }
                }
            }
        })
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return only those which should decode to the provided `E` type.
    /// If an error occurs, all subsequent iterations return `None`.
    pub fn find_extrinsic<E: StaticExtrinsic>(
        &self,
    ) -> impl Iterator<Item = Result<E, Error>> + '_ {
        self.extrinsics().filter_map(|e| {
            e.and_then(|e| e.as_extrinsic::<E>().map_err(Into::into))
                .transpose()
        })
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the first extrinsic found which decodes to the provided `E` type.
    pub fn find_first_extrinsic<E: StaticExtrinsic>(&self) -> Result<Option<E>, Error> {
        self.find_extrinsic::<E>().next().transpose()
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the last extrinsic found which decodes to the provided `Ev` type.
    pub fn find_last<E: StaticExtrinsic>(&self) -> Result<Option<E>, Error> {
        self.find_extrinsic::<E>().last().transpose()
    }

    /// Find an extrinsics that decodes to the type provided. Returns true if it was found.
    pub fn has_extrinsic<E: StaticExtrinsic>(&self) -> Result<bool, Error> {
        Ok(self.find_extrinsic::<E>().next().transpose()?.is_some())
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
