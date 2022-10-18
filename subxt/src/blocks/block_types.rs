// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    Config,
    error::{
        Error,
    },
    client::{
        OfflineClientT,
        OnlineClientT,
    },
    rpc::{
        ChainBlockResponse,
    },
    events
};
use derivative::Derivative;
use sp_runtime::traits::Hash;

/// A representation of a block from which you can obtain details
/// including the block header, extrinsics and events for the block.
pub struct Block<T: Config, C> {
    hash: T::Hash,
    details: ChainBlockResponse<T>,
    client: C,
}

impl <T, C> Block<T, C>
where
    T: Config,
    C: OfflineClientT<T>
{
    pub (crate) fn new(
        hash: T::Hash,
        details: ChainBlockResponse<T>,
        client: C
    ) -> Self {
        Block {
            hash, details, client
        }
    }

    /// Return the block hash.
    pub fn hash(&self) -> T::Hash {
        self.hash
    }

    /// Return the block header.
    pub fn header(&self) -> &T::Header {
        &self.details.block.header
    }

    /// Returns an iterator over the extrinsics in the block.
    pub fn extrinsics<'a>(&'a self) -> impl Iterator<Item = Extrinsic<'a, T, C>> {
        self.details.block.extrinsics.iter().enumerate().map(|(idx, e)| {
            Extrinsic {
                index: idx as u32,
                bytes: &e.0,
                client: self.client.clone(),
                block_hash: self.hash,
                _marker: std::marker::PhantomData
            }
        })
    }
}

/// A single extrinsic in a block.
pub struct Extrinsic<'a, T: Config, C> {
    index: u32,
    bytes: &'a [u8],
    client: C,
    block_hash: T::Hash,
    _marker: std::marker::PhantomData<T>
}

impl <'a, T, C> Extrinsic<'a, T, C>
where
    T: Config,
    C: OfflineClientT<T>
{
    /// The index of the extrinsic in the block.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// The bytes of the extrinsic.
    pub fn bytes(&self) -> &'a [u8] {
        &self.bytes
    }
}

impl <'a, T, C> Extrinsic<'a, T, C>
where
    T: Config,
    C: OnlineClientT<T>
{
    /// The events associated with the extrinsic.
    pub async fn events(&self) -> Result<ExtrinsicEvents<T>, Error> {
        let ext_hash = T::Hashing::hash_of(&self.bytes);
        let events = events::EventsClient::new(self.client.clone())
            .at(Some(self.block_hash))
            .await?;

        Ok(ExtrinsicEvents::new(ext_hash, self.index, events))
    }
}

/// The events associated with a given extrinsic.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct ExtrinsicEvents<T: Config> {
    // The hash of the extrinsic (handy to expose here because
    // this type is returned from TxProgress things in the most
    // basic flows, so it's the only place people can access it
    // without complicating things for themselves).
    ext_hash: T::Hash,
    // The index of the extrinsic:
    idx: u32,
    // All of the events in the block:
    events: events::Events<T>
}

impl<T: Config> ExtrinsicEvents<T> {
    pub (crate) fn new(
        ext_hash: T::Hash,
        idx: u32,
        events: events::Events<T>
    ) -> Self {
        Self { ext_hash, idx, events }
    }

    /// Return the hash of the block that the extrinsic is in.
    pub fn block_hash(&self) -> T::Hash {
        self.events.block_hash()
    }

    /// Return the hash of the extrinsic.
    pub fn extrinsic_hash(&self) -> T::Hash {
        self.ext_hash
    }

    /// Return all of the events in the block that the extrinsic is in.
    pub fn all_events_in_block(&self) -> &events::Events<T> {
        &self.events
    }

    /// Iterate over all of the raw events associated with this transaction.
    ///
    /// This works in the same way that [`events::Events::iter()`] does, with the
    /// exception that it filters out events not related to the submitted extrinsic.
    pub fn iter(&self) -> impl Iterator<Item = Result<events::EventDetails, Error>> + '_ {
        self.events.iter().filter(|ev| {
            ev.as_ref()
                .map(|ev| ev.phase() == events::Phase::ApplyExtrinsic(self.idx))
                .unwrap_or(true) // Keep any errors.
        })
    }

    /// Find all of the transaction events matching the event type provided as a generic parameter.
    ///
    /// This works in the same way that [`events::Events::find()`] does, with the
    /// exception that it filters out events not related to the submitted extrinsic.
    pub fn find<Ev: events::StaticEvent>(&self) -> impl Iterator<Item = Result<Ev, Error>> + '_ {
        self.iter().filter_map(|ev| {
            ev.and_then(|ev| ev.as_event::<Ev>().map_err(Into::into))
                .transpose()
        })
    }

    /// Iterate through the transaction events using metadata to dynamically decode and skip
    /// them, and return the first event found which decodes to the provided `Ev` type.
    ///
    /// This works in the same way that [`events::Events::find_first()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn find_first<Ev: events::StaticEvent>(&self) -> Result<Option<Ev>, Error> {
        self.find::<Ev>().next().transpose()
    }

    /// Find an event in those associated with this transaction. Returns true if it was found.
    ///
    /// This works in the same way that [`events::Events::has()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn has<Ev: events::StaticEvent>(&self) -> Result<bool, Error> {
        Ok(self.find::<Ev>().next().transpose()?.is_some())
    }
}
