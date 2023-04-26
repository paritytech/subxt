// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    client::{OfflineClientT, OnlineClientT},
    config::{Config, Hasher, Header},
    error::{BlockError, Error, ExtrinsicError},
    events,
    rpc::types::ChainBlockResponse,
    runtime_api::RuntimeApi,
    storage::Storage,
};
use codec::Decode;
use derivative::Derivative;
use frame_metadata::v15::RuntimeMetadataV15;
use futures::lock::Mutex as AsyncMutex;
use scale_decode::DecodeAsFields;
use std::{collections::HashMap, sync::Arc};

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
type CachedEvents<T> = Arc<AsyncMutex<Option<events::Events<T>>>>;

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
        let ids = self.ids.clone();
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
}

/// The type IDs extracted from the metadata that represent the
/// generic type parameters passed to the `UncheckedExtrinsic` from
/// the substrate-based chain.
#[derive(Debug, Copy, Clone)]
pub(crate) struct ExtrinsicIds {
    /// The address (source) of the extrinsic.
    address: u32,
    /// The extrinsic call type.
    call: u32,
    /// The signature of the extrinsic.
    signature: u32,
    /// The extra parameters of the extrinsic.
    extra: u32,
}

impl ExtrinsicIds {
    /// Extract the generic type parameters IDs from the extrinsic type.
    fn new(metadata: &RuntimeMetadataV15) -> Result<Self, ExtrinsicError> {
        const ADDRESS: &str = "Address";
        const CALL: &str = "Call";
        const SIGNATURE: &str = "Signature";
        const EXTRA: &str = "Extra";

        let id = metadata.extrinsic.ty.id;

        let Some(ty) = metadata.types.resolve(id) else {
            return Err(ExtrinsicError::MissingType);
        };

        let params: HashMap<_, _> = ty
            .type_params
            .iter()
            .map(|ty_param| {
                let Some(ty) = ty_param.ty else {
                    return Err(ExtrinsicError::MissingType);
                };

                Ok((ty_param.name.as_str(), ty.id))
            })
            .collect::<Result<_, _>>()?;

        let Some(address) = params.get(ADDRESS) else {
            return Err(ExtrinsicError::MissingType);
        };
        let Some(call) = params.get(CALL) else {
            return Err(ExtrinsicError::MissingType);
        };
        let Some(signature) = params.get(SIGNATURE) else {
            return Err(ExtrinsicError::MissingType);
        };
        let Some(extra) = params.get(EXTRA) else {
            return Err(ExtrinsicError::MissingType);
        };

        Ok(ExtrinsicIds {
            address: *address,
            call: *call,
            signature: *signature,
            extra: *extra,
        })
    }
}

/// A single extrinsic in a block.
pub struct ExtrinsicDetails<T: Config, C> {
    /// The index of the extrinsic in the block.
    index: u32,
    /// Extrinsic bytes.
    bytes: Arc<[u8]>,
    /// True if the extrinsic payload is signed.
    is_signed: bool,
    /// The start index in the `bytes` from which the address is encoded.
    address_start_idx: usize,
    /// The end index of the address in the encoded `bytes`.
    address_end_idx: usize,
    /// The start index in the `bytes` from which the call is encoded.
    call_start_idx: usize,
    /// The pallet index.
    pallet_index: u8,
    /// The variant index.
    variant_index: u8,
    /// The block hash of this extrinsic (needed to fetch events).
    block_hash: T::Hash,
    /// Subxt client.
    client: C,
    /// Cached events.
    cached_events: CachedEvents<T>,
    _marker: std::marker::PhantomData<T>,
}

/// Trait to uniquely identify the extrinsic's identity from the runtime metadata.
///
/// Generated API structures that represent an extrinsic implement this trait.
///
/// The trait is utilized to decode emitted extrinsics from a block, via obtaining the
/// form of the `Extrinsic` from the metadata.
pub trait StaticExtrinsic: DecodeAsFields {
    /// Pallet name.
    const PALLET: &'static str;
    /// Call name.
    const CALL: &'static str;

    /// Returns true if the given pallet and call names match this extrinsic.
    fn is_extrinsic(pallet: &str, call: &str) -> bool {
        Self::PALLET == pallet && Self::CALL == call
    }
}

impl<T, C> ExtrinsicDetails<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    // Attempt to dynamically decode a single extrinsic from the given input.
    fn decode_from(
        index: u32,
        extrinsic_bytes: Arc<[u8]>,
        client: C,
        block_hash: T::Hash,
        cached_events: CachedEvents<T>,
        ids: ExtrinsicIds,
    ) -> Result<ExtrinsicDetails<T, C>, Error> {
        const SIGNATURE_MASK: u8 = 0b1000_0000;
        const VERSION_MASK: u8 = 0b0111_1111;
        const LATEST_EXTRINSIC_VERSION: u8 = 4;

        let metadata = client.metadata();

        // Extrinsic are encoded in memory in the following way:
        //   - first byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
        //   - signature: [unknown TBD with metadata].
        //   - extrinsic data
        if extrinsic_bytes.is_empty() {
            return Err(ExtrinsicError::InsufficientData.into());
        }

        let version = extrinsic_bytes[0] & VERSION_MASK;
        if version != LATEST_EXTRINSIC_VERSION {
            return Err(ExtrinsicError::UnsupportedVersion(version).into());
        }

        let is_signed = extrinsic_bytes[0] & SIGNATURE_MASK != 0;

        // Skip over the first byte which denotes the version and signing.
        let cursor = &mut &extrinsic_bytes[1..];

        let mut address_start_idx = 0;
        let mut address_end_idx = 0;

        if is_signed {
            address_start_idx = extrinsic_bytes.len() - cursor.len();

            // Skip over the address, signature and extra fields.
            scale_decode::visitor::decode_with_visitor(
                cursor,
                ids.address,
                &metadata.runtime_metadata().types,
                scale_decode::visitor::IgnoreVisitor,
            )
            .map_err(scale_decode::Error::from)?;
            address_end_idx = extrinsic_bytes.len() - cursor.len();

            scale_decode::visitor::decode_with_visitor(
                cursor,
                ids.signature,
                &metadata.runtime_metadata().types,
                scale_decode::visitor::IgnoreVisitor,
            )
            .map_err(scale_decode::Error::from)?;

            scale_decode::visitor::decode_with_visitor(
                cursor,
                ids.extra,
                &metadata.runtime_metadata().types,
                scale_decode::visitor::IgnoreVisitor,
            )
            .map_err(scale_decode::Error::from)?;
        }

        let call_start_idx = extrinsic_bytes.len() - cursor.len();

        // Ensure the provided bytes are sound.
        scale_decode::visitor::decode_with_visitor(
            &mut *cursor,
            ids.call,
            &metadata.runtime_metadata().types,
            scale_decode::visitor::IgnoreVisitor,
        )
        .map_err(scale_decode::Error::from)?;

        // Decode the pallet index, then the call variant.
        let cursor = &mut &extrinsic_bytes[call_start_idx..];

        if cursor.len() < 2 {
            return Err(ExtrinsicError::InsufficientData.into());
        }
        let pallet_index = cursor[0];
        let variant_index = cursor[1];

        Ok(ExtrinsicDetails {
            index,
            bytes: extrinsic_bytes,
            is_signed,
            address_start_idx,
            address_end_idx,
            call_start_idx,
            pallet_index,
            variant_index,
            block_hash,
            client,
            cached_events,
            _marker: std::marker::PhantomData,
        })
    }

    /// Is the extrinsic signed?
    pub fn is_signed(&self) -> bool {
        self.is_signed
    }

    /// The index of the extrinsic in the block.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Return _all_ of the bytes representing this extrinsic, which include, in order:
    /// - First byte: abbbbbbb (a = 0 for unsigned, 1 for signed, b = version)
    /// - SignatureType (if the payload is signed)
    ///   - Address
    ///   - Signature
    ///   - Extra fields
    /// - Extrinsic call bytes
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return only the bytes representing this extrinsic call.
    ///
    /// # Note
    ///
    /// Please use `[Self::bytes]` if you want to get all extrinsic bytes.
    pub fn call_bytes(&self) -> &[u8] {
        &self.bytes[self.call_start_idx..]
    }

    /// Return only the bytes of the address that signed this extrinsic.
    ///
    /// # Note
    ///
    /// Returns `None` if the extrinsic is not signed.
    pub fn address_bytes(&self) -> Option<&[u8]> {
        self.is_signed
            .then(|| &self.bytes[self.address_start_idx..self.address_end_idx])
    }

    /// Attempt to statically decode the address bytes into the provided type.
    ///
    /// # Note
    ///
    /// Returns `None` if the extrinsic is not signed.
    pub fn as_address<Address: Decode>(&self) -> Option<Result<Address, Error>> {
        self.address_bytes()
            .map(|bytes| Address::decode(&mut &bytes[..]).map_err(Error::Codec))
    }

    /// Attempt to statically decode the extrinsic call bytes into the provided type.
    pub fn as_call<Call: Decode>(&self) -> Result<Call, Error> {
        let bytes = &mut &self.call_bytes()[..];
        Call::decode(bytes).map_err(Error::Codec)
    }

    /// The index of the pallet that the extrinsic originated from.
    pub fn pallet_index(&self) -> u8 {
        self.pallet_index
    }

    /// The index of the event variant that the event originated from.
    pub fn variant_index(&self) -> u8 {
        self.variant_index
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
        let ext_hash = T::Hasher::hash_of(&self.bytes);
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

    /// Return the hash of the block that the extrinsic is in.
    pub fn block_hash(&self) -> T::Hash {
        self.events.block_hash()
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

// Return Events from the cache, or fetch from the node if needed.
async fn get_events<C, T>(
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
