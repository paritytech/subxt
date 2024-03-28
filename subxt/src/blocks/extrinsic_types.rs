// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    blocks::block_types::{get_events, CachedEvents},
    client::{OfflineClientT, OnlineClientT},
    config::{Config, Hasher},
    error::{BlockError, Error},
    events,
};

use derive_where::derive_where;
use scale_decode::DecodeAsType;
use subxt_core::blocks::{
    Extrinsics as CoreExtrinsics,
    ExtrinsicDetails as CoreExtrinsicDetails,
};

// Re-export anything that's directly returned/used in the APIs below.
pub use subxt_core::blocks::{
    StaticExtrinsic,
    ExtrinsicSignedExtensions,
    ExtrinsicSignedExtension,
    ExtrinsicMetadataDetails,
};

/// The body of a block.
pub struct Extrinsics<T: Config, C> {
    inner: CoreExtrinsics<T>,
    client: C,
    cached_events: CachedEvents<T>,
    hash: T::Hash,
}

impl<T, C> Extrinsics<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    pub(crate) fn new(
        client: C,
        extrinsics: Vec<Vec<u8>>,
        cached_events: CachedEvents<T>,
        hash: T::Hash,
    ) -> Result<Self, BlockError> {
        let inner = CoreExtrinsics::new(extrinsics, client.metadata())?;
        Ok(Self {
            inner,
            client,
            cached_events,
            hash,
        })
    }

    /// See [`subxt_core::blocks::Extrinsics::len()`].
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// See [`subxt_core::blocks::Extrinsics::is_empty()`].
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Return the block hash that these extrinsics are from.
    pub fn block_hash(&self) -> T::Hash {
        self.hash
    }

    /// Returns an iterator over the extrinsics in the block body.
    // Dev note: The returned iterator is 'static + Send so that we can box it up and make
    // use of it with our `FilterExtrinsic` stuff.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<ExtrinsicDetails<T, C>, Error>> + Send + Sync + 'static {
        let client = self.client.clone();
        let cached_events = self.cached_events.clone();
        let block_hash = self.hash;

        self.inner.iter().map(move |res| {
            let inner = res?;
            Ok(ExtrinsicDetails::new(inner, client.clone(), block_hash, cached_events.clone()))
        })
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return only those which should decode to the provided `E` type.
    /// If an error occurs, all subsequent iterations return `None`.
    pub fn find<E: StaticExtrinsic>(
        &self,
    ) -> impl Iterator<Item = Result<FoundExtrinsic<T, C, E>, Error>> + '_ {
        self.inner.find::<E>().map(|res| {
            match res {
                Err(e) => Err(Error::from(e)),
                Ok(ext) => {
                    // Wrap details from subxt-core into what we want here:
                    let details = ExtrinsicDetails::new(
                        ext.details,
                        self.client.clone(),
                        self.hash,
                        self.cached_events.clone()
                    );

                    Ok(FoundExtrinsic { details, value: ext.value })
                }
            }
        })
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the first extrinsic found which decodes to the provided `E` type.
    pub fn find_first<E: StaticExtrinsic>(&self) -> Result<Option<FoundExtrinsic<T, C, E>>, Error> {
        self.find::<E>().next().transpose()
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the last extrinsic found which decodes to the provided `Ev` type.
    pub fn find_last<E: StaticExtrinsic>(&self) -> Result<Option<FoundExtrinsic<T, C, E>>, Error> {
        self.find::<E>().last().transpose()
    }

    /// Find an extrinsics that decodes to the type provided. Returns true if it was found.
    pub fn has<E: StaticExtrinsic>(&self) -> Result<bool, Error> {
        Ok(self.find::<E>().next().transpose()?.is_some())
    }
}

/// A single extrinsic in a block.
pub struct ExtrinsicDetails<T: Config, C> {
    inner: CoreExtrinsicDetails<T>,
    /// The block hash of this extrinsic (needed to fetch events).
    block_hash: T::Hash,
    /// Subxt client.
    client: C,
    /// Cached events.
    cached_events: CachedEvents<T>,
}

impl<T, C> ExtrinsicDetails<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    // Attempt to dynamically decode a single extrinsic from the given input.
    pub(crate) fn new(
        inner: CoreExtrinsicDetails<T>,
        client: C,
        block_hash: T::Hash,
        cached_events: CachedEvents<T>,
    ) -> ExtrinsicDetails<T, C> {
        ExtrinsicDetails {
            inner,
            client,
            block_hash,
            cached_events,
        }
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::is_signed()`].
    pub fn is_signed(&self) -> bool {
       self.inner.is_signed()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::index()`].
    pub fn index(&self) -> u32 {
        self.inner.index()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::bytes()`].
    pub fn bytes(&self) -> &[u8] {
        self.inner.bytes()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::call_bytes()`].
    pub fn call_bytes(&self) -> &[u8] {
        self.inner.call_bytes()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::field_bytes()`].
    pub fn field_bytes(&self) -> &[u8] {
        self.inner.field_bytes()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::address_bytes()`].
    pub fn address_bytes(&self) -> Option<&[u8]> {
        self.inner.address_bytes()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::signature_bytes()`].
    pub fn signature_bytes(&self) -> Option<&[u8]> {
        self.inner.signature_bytes()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::signed_extensions_bytes()`].
    pub fn signed_extensions_bytes(&self) -> Option<&[u8]> {
        self.inner.signed_extensions_bytes()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::signed_extensions()`].
    pub fn signed_extensions(&self) -> Option<ExtrinsicSignedExtensions<'_, T>> {
        self.inner.signed_extensions()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::pallet_index()`].
    pub fn pallet_index(&self) -> u8 {
        self.inner.pallet_index()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::variant_index()`].
    pub fn variant_index(&self) -> u8 {
        self.inner.variant_index()
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::pallet_name()`].
    pub fn pallet_name(&self) -> Result<&str, Error> {
        self.inner.pallet_name().map_err(Into::into)
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::variant_name()`].
    pub fn variant_name(&self) -> Result<&str, Error> {
        self.inner.variant_name().map_err(Into::into)
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::extrinsic_metadata()`].
    pub fn extrinsic_metadata(&self) -> Result<ExtrinsicMetadataDetails, Error> {
        self.inner.extrinsic_metadata().map_err(Into::into)
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::field_values()`].
    pub fn field_values(&self) -> Result<scale_value::Composite<u32>, Error> {
        self.inner.field_values().map_err(Into::into)
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::as_extrinsic()`].
    pub fn as_extrinsic<E: StaticExtrinsic>(&self) -> Result<Option<E>, Error> {
        self.inner.as_extrinsic::<E>().map_err(Into::into)
    }

    /// See [`subxt_core::blocks::ExtrinsicDetails::as_root_extrinsic()`].
    pub fn as_root_extrinsic<E: DecodeAsType>(&self) -> Result<E, Error> {
        self.inner.as_root_extrinsic::<E>().map_err(Into::into)
    }
}

impl<T, C> ExtrinsicDetails<T, C>
where
    T: Config,
    C: OnlineClientT<T>,
{
    /// The events associated with the extrinsic.
    pub async fn events(&self) -> Result<ExtrinsicEvents<T>, Error> {
        let events = get_events(&self.client, self.block_hash, &self.cached_events).await?;
        let ext_hash = T::Hasher::hash_of(&self.bytes());
        Ok(ExtrinsicEvents::new(ext_hash, self.index(), events))
    }
}

/// A Static Extrinsic found in a block coupled with it's details.
pub struct FoundExtrinsic<T: Config, C, E> {
    /// Details for the extrinsic.
    pub details: ExtrinsicDetails<T, C>,
    /// The decoded extrinsic value.
    pub value: E,
}

/// The events associated with a given extrinsic.
#[derive_where(Debug)]
pub struct ExtrinsicEvents<T: Config> {
    // The hash of the extrinsic (handy to expose here because
    // this type is returned from TxProgress things in the most
    // basic flows, so it's the only place people can access it
    // without complicating things for themselves).
    ext_hash: T::Hash,
    // The index of the extrinsic:
    idx: u32,
    // All of the events in the block:
    events: events::Events<T>,
}

impl<T: Config> ExtrinsicEvents<T> {
    pub(crate) fn new(ext_hash: T::Hash, idx: u32, events: events::Events<T>) -> Self {
        Self {
            ext_hash,
            idx,
            events,
        }
    }

    /// The index of the extrinsic that these events are produced from.
    pub fn extrinsic_index(&self) -> u32 {
        self.idx
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
    pub fn iter(&self) -> impl Iterator<Item = Result<events::EventDetails<T>, Error>> + '_ {
        self.events
            .iter()
            .filter(|ev| {
                ev.as_ref()
                    .map(|ev| ev.phase() == events::Phase::ApplyExtrinsic(self.idx))
                    .unwrap_or(true) // Keep any errors.
            })
            .map(|e| e.map_err(Error::from))
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

    /// Iterate through the transaction events using metadata to dynamically decode and skip
    /// them, and return the last event found which decodes to the provided `Ev` type.
    ///
    /// This works in the same way that [`events::Events::find_last()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn find_last<Ev: events::StaticEvent>(&self) -> Result<Option<Ev>, Error> {
        self.find::<Ev>().last().transpose()
    }

    /// Find an event in those associated with this transaction. Returns true if it was found.
    ///
    /// This works in the same way that [`events::Events::has()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn has<Ev: events::StaticEvent>(&self) -> Result<bool, Error> {
        Ok(self.find::<Ev>().next().transpose()?.is_some())
    }
}
