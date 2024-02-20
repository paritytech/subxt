// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    backend::BlockRef,
    blocks::{extrinsic_types::ExtrinsicPartTypeIds, Extrinsics},
    client::{OfflineClientT, OnlineClientT},
    config::{Config, Header},
    error::{BlockError, DecodeError, Error},
    events,
    runtime_api::RuntimeApi,
    storage::Storage,
};

use codec::{Decode, Encode};
use futures::lock::Mutex as AsyncMutex;
use std::sync::Arc;

/// A representation of a block.
pub struct Block<T: Config, C> {
    header: T::Header,
    block_ref: BlockRef<T::Hash>,
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
    pub(crate) fn new(header: T::Header, block_ref: BlockRef<T::Hash>, client: C) -> Self {
        Block {
            header,
            block_ref,
            client,
            cached_events: Default::default(),
        }
    }

    /// Return a reference to the given block. While this reference is kept alive,
    /// the backend will (if possible) endeavour to keep hold of the block.
    pub fn reference(&self) -> BlockRef<T::Hash> {
        self.block_ref.clone()
    }

    /// Return the block hash.
    pub fn hash(&self) -> T::Hash {
        self.block_ref.hash()
    }

    /// Return the block number.
    pub fn number(&self) -> <T::Header as crate::config::Header>::Number {
        self.header().number()
    }

    /// Return the entire block header.
    pub fn header(&self) -> &T::Header {
        &self.header
    }

    /// Return the entire block header. Consumes the block itself
    pub fn into_header(self) -> T::Header {
        self.header
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

    /// Fetch and return the extrinsics in the block body.
    pub async fn extrinsics(&self) -> Result<Extrinsics<T, C>, Error> {
        let ids = ExtrinsicPartTypeIds::new(&self.client.metadata())?;
        let block_hash = self.header.hash();
        let Some(extrinsics) = self.client.backend().block_body(block_hash).await? else {
            return Err(BlockError::not_found(block_hash).into());
        };

        Ok(Extrinsics::new(
            self.client.clone(),
            extrinsics,
            self.cached_events.clone(),
            ids,
            block_hash,
        ))
    }

    /// Work with storage.
    pub fn storage(&self) -> Storage<T, C> {
        Storage::new(self.client.clone(), self.block_ref.clone())
    }

    /// Execute a runtime API call at this block.
    pub async fn runtime_api(&self) -> Result<RuntimeApi<T, C>, Error> {
        Ok(RuntimeApi::new(self.client.clone(), self.block_ref.clone()))
    }

    /// Get the account nonce for a given account ID at this block.
    pub async fn account_nonce(&self, account_id: &T::AccountId) -> Result<u64, Error> {
        get_account_nonce(&self.client, account_id, self.hash()).await
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
    let mut lock = cached_events.lock().await;
    let events = match &*lock {
        Some(events) => events.clone(),
        None => {
            let events = events::EventsClient::new(client.clone())
                .at(block_hash)
                .await?;
            lock.replace(events.clone());
            events
        }
    };

    Ok(events)
}

// Return the account nonce at some block hash for an account ID.
pub(crate) async fn get_account_nonce<C, T>(
    client: &C,
    account_id: &T::AccountId,
    block_hash: T::Hash,
) -> Result<u64, Error>
where
    C: OnlineClientT<T>,
    T: Config,
{
    let account_nonce_bytes = client
        .backend()
        .call(
            "AccountNonceApi_account_nonce",
            Some(&account_id.encode()),
            block_hash,
        )
        .await?;

    // custom decoding from a u16/u32/u64 into a u64, based on the number of bytes we got back.
    let cursor = &mut &account_nonce_bytes[..];
    let account_nonce: u64 = match account_nonce_bytes.len() {
        2 => u16::decode(cursor)?.into(),
        4 => u32::decode(cursor)?.into(),
        8 => u64::decode(cursor)?,
        _ => return Err(Error::Decode(DecodeError::custom_string(format!("state call AccountNonceApi_account_nonce returned an unexpected number of bytes: {} (expected 2, 4 or 8)", account_nonce_bytes.len()))))
    };
    Ok(account_nonce)
}
