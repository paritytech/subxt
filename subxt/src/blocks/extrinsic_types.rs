// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    blocks::block_types::{get_events, CachedEvents},
    client::{OfflineClientT, OnlineClientT},
    config::{Config, Hasher},
    error::{BlockError, Error},
    events,
    metadata::{DecodeWithMetadata, ExtrinsicMetadata},
    rpc::types::ChainBlockExtrinsic,
    Metadata,
};

use derivative::Derivative;
use frame_metadata::v15::RuntimeMetadataV15;
use scale_decode::DecodeAsFields;
use std::{collections::HashMap, sync::Arc};

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

/// This trait is implemented on the statically generated root extrinsic type, so that we're able
/// to decode it properly via a pallet that impls `DecodeAsMetadata`. This is necessary
/// because the "root extrinsic" type is generated using pallet info but doesn't actually exist in the
/// metadata types, so we have no easy way to decode things into it via type information and need a
/// little help via codegen.
#[doc(hidden)]
pub trait RootExtrinsic: Sized {
    /// Given details of the pallet extrinsic we want to decode, and the name of the pallet, try to hand
    /// back a "root extrinsic".
    fn root_extrinsic(
        pallet_bytes: &[u8],
        pallet_name: &str,
        pallet_extrinsic_ty: u32,
        metadata: &Metadata,
    ) -> Result<Self, Error>;
}

/// The body of a block.
pub struct Extrinsics<T: Config, C> {
    client: C,
    extrinsics: Vec<ChainBlockExtrinsic>,
    cached_events: CachedEvents<T>,
    ids: ExtrinsicIds,
    hash: T::Hash,
}

impl<T, C> Extrinsics<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    pub(crate) fn new(
        client: C,
        extrinsics: Vec<ChainBlockExtrinsic>,
        cached_events: CachedEvents<T>,
        ids: ExtrinsicIds,
        hash: T::Hash,
    ) -> Self {
        Self {
            client,
            extrinsics,
            cached_events,
            ids,
            hash,
        }
    }

    /// Returns an iterator over the extrinsics in the block body.
    // Dev note: The returned iterator is 'static + Send so that we can box it up and make
    // use of it with our `FilterExtrinsic` stuff.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<ExtrinsicDetails<T, C>, Error>> + Send + Sync + 'static {
        let extrinsics = self.extrinsics.clone();
        let num_extrinsics = self.extrinsics.len();
        let client = self.client.clone();
        let hash = self.hash.clone();
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
    pub fn find<E: StaticExtrinsic>(&self) -> impl Iterator<Item = Result<E, Error>> + '_ {
        self.iter().filter_map(|e| {
            e.and_then(|e| e.as_extrinsic::<E>().map_err(Into::into))
                .transpose()
        })
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the first extrinsic found which decodes to the provided `E` type.
    pub fn find_first<E: StaticExtrinsic>(&self) -> Result<Option<E>, Error> {
        self.find::<E>().next().transpose()
    }

    /// Iterate through the extrinsics using metadata to dynamically decode and skip
    /// them, and return the last extrinsic found which decodes to the provided `Ev` type.
    pub fn find_last<E: StaticExtrinsic>(&self) -> Result<Option<E>, Error> {
        self.find::<E>().last().transpose()
    }

    /// Find an extrinsics that decodes to the type provided. Returns true if it was found.
    pub fn has<E: StaticExtrinsic>(&self) -> Result<bool, Error> {
        Ok(self.find::<E>().next().transpose()?.is_some())
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
    /// Subxt metadata to fetch the extrinsic metadata.
    metadata: Metadata,
    _marker: std::marker::PhantomData<T>,
}

impl<T, C> ExtrinsicDetails<T, C>
where
    T: Config,
    C: OfflineClientT<T>,
{
    // Attempt to dynamically decode a single extrinsic from the given input.
    pub(crate) fn decode_from(
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
            return Err(BlockError::InsufficientData.into());
        }

        let version = extrinsic_bytes[0] & VERSION_MASK;
        if version != LATEST_EXTRINSIC_VERSION {
            return Err(BlockError::UnsupportedVersion(version).into());
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
            return Err(BlockError::InsufficientData.into());
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
            metadata,
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

    /// Return only the bytes representing this extrinsic call:
    /// - First byte is the pallet index
    /// - Second byte is the variant (call) index
    /// - Followed by field bytes.
    ///
    /// # Note
    ///
    /// Please use [`Self::bytes`] if you want to get all extrinsic bytes.
    pub fn call_bytes(&self) -> &[u8] {
        &self.bytes[self.call_start_idx..]
    }

    /// Return the bytes representing the fields stored in this extrinsic.
    ///
    /// # Note
    ///
    /// This is a subset of [`Self::call_bytes`] that does not include the
    /// first two bytes that denote the pallet index and the variant index.
    pub fn field_bytes(&self) -> &[u8] {
        // Note: this cannot panic because we checked the extrinsic bytes
        // to contain at least two bytes.
        &self.call_bytes()[2..]
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

    /// The index of the pallet that the extrinsic originated from.
    pub fn pallet_index(&self) -> u8 {
        self.pallet_index
    }

    /// The index of the extrinsic variant that the extrinsic originated from.
    pub fn variant_index(&self) -> u8 {
        self.variant_index
    }

    /// The name of the pallet from whence the extrinsic originated.
    pub fn pallet_name(&self) -> &str {
        self.extrinsic_metadata().pallet()
    }

    /// The name of the call (ie the name of the variant that it corresponds to).
    pub fn variant_name(&self) -> &str {
        self.extrinsic_metadata().call()
    }

    /// Fetch the metadata for this extrinsic.
    pub fn extrinsic_metadata(&self) -> &ExtrinsicMetadata {
        self.metadata
            .extrinsic(self.pallet_index(), self.variant_index())
            .expect("this must exist in order to have produced the ExtrinsicDetails")
    }

    /// Decode and provide the extrinsic fields back in the form of a [`scale_value::Composite`]
    /// type which represents the named or unnamed fields that were
    /// present in the extrinsic.
    pub fn field_values(
        &self,
    ) -> Result<scale_value::Composite<scale_value::scale::TypeId>, Error> {
        let bytes = &mut self.field_bytes();
        let extrinsic_metadata = self.extrinsic_metadata();

        let decoded = <scale_value::Composite<scale_value::scale::TypeId>>::decode_as_fields(
            bytes,
            extrinsic_metadata.fields(),
            &self.metadata.runtime_metadata().types,
        )?;

        Ok(decoded)
    }

    /// Attempt to statically decode these [`ExtrinsicDetails`] into a type representing the extrinsic
    /// fields. This leans directly on [`codec::Decode`]. You can also attempt to decode the entirety
    /// of the extrinsic using [`Self::as_root_extrinsic()`], which is more lenient because it's able
    /// to lean on [`scale_decode::DecodeAsType`].
    pub fn as_extrinsic<E: StaticExtrinsic>(&self) -> Result<Option<E>, Error> {
        let extrinsic_metadata = self.extrinsic_metadata();
        if extrinsic_metadata.pallet() == E::PALLET && extrinsic_metadata.call() == E::CALL {
            let decoded = E::decode_as_fields(
                &mut self.field_bytes(),
                extrinsic_metadata.fields(),
                self.metadata.types(),
            )?;
            Ok(Some(decoded))
        } else {
            Ok(None)
        }
    }

    /// Attempt to decode these [`ExtrinsicDetails`] into a pallet extrinsic type (which includes
    /// the pallet enum variants as well as the extrinsic fields). These extrinsics can be found in
    /// the static codegen under a path like `pallet_name::Call`.
    pub fn as_pallet_extrinsic<E: DecodeWithMetadata>(&self) -> Result<E, Error> {
        let pallet = self.metadata.pallet(self.pallet_name())?;
        let extrinsic_ty = pallet.call_ty_id().ok_or_else(|| {
            Error::Metadata(crate::metadata::MetadataError::ExtrinsicNotFound(
                pallet.index(),
                self.variant_index(),
            ))
        })?;

        // Ignore the root enum index, so start 1 byte after that:
        let decoded =
            E::decode_with_metadata(&mut &self.call_bytes()[1..], extrinsic_ty, &self.metadata)?;
        Ok(decoded)
    }

    /// Attempt to decode these [`ExtrinsicDetails`] into a root extrinsic type (which includes
    /// the pallet and extrinsic enum variants as well as the extrinsic fields). A compatible
    /// type for this is exposed via static codegen as a root level `Call` type.
    pub fn as_root_extrinsic<E: RootExtrinsic>(&self) -> Result<E, Error> {
        let pallet = self.metadata.pallet(self.pallet_name())?;
        let pallet_extrinsic_ty = pallet.call_ty_id().ok_or_else(|| {
            Error::Metadata(crate::metadata::MetadataError::ExtrinsicNotFound(
                pallet.index(),
                self.variant_index(),
            ))
        })?;

        // Ignore root enum index.
        E::root_extrinsic(
            &self.call_bytes()[1..],
            self.pallet_name(),
            pallet_extrinsic_ty,
            &self.metadata,
        )
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
    pub(crate) fn new(metadata: &RuntimeMetadataV15) -> Result<Self, BlockError> {
        const ADDRESS: &str = "Address";
        const CALL: &str = "Call";
        const SIGNATURE: &str = "Signature";
        const EXTRA: &str = "Extra";

        let id = metadata.extrinsic.ty.id;

        let Some(ty) = metadata.types.resolve(id) else {
            return Err(BlockError::MissingType);
        };

        let params: HashMap<_, _> = ty
            .type_params
            .iter()
            .map(|ty_param| {
                let Some(ty) = ty_param.ty else {
                    return Err(BlockError::MissingType);
                };

                Ok((ty_param.name.as_str(), ty.id))
            })
            .collect::<Result<_, _>>()?;

        let Some(address) = params.get(ADDRESS) else {
            return Err(BlockError::MissingType);
        };
        let Some(call) = params.get(CALL) else {
            return Err(BlockError::MissingType);
        };
        let Some(signature) = params.get(SIGNATURE) else {
            return Err(BlockError::MissingType);
        };
        let Some(extra) = params.get(EXTRA) else {
            return Err(BlockError::MissingType);
        };

        Ok(ExtrinsicIds {
            address: *address,
            call: *call,
            signature: *signature,
            extra: *extra,
        })
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
