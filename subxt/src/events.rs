// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! For working with events.

use crate::{
    error::BasicError,
    metadata::MetadataError,
    Client,
    Config,
    Event,
    Metadata,
    Phase,
};
use bitvec::{
    order::Lsb0,
    vec::BitVec,
};
use codec::{
    Codec,
    Compact,
    Decode,
    Error as CodecError,
    Input,
};
use derivative::Derivative;
use futures::{
    Future,
    FutureExt,
    Stream,
    StreamExt,
};
use jsonrpsee::core::client::Subscription;
use scale_info::{
    PortableRegistry,
    TypeDef,
    TypeDefPrimitive,
};
use sp_core::{
    storage::StorageKey,
    twox_128,
    Bytes,
};
use std::{
    marker::Unpin,
    task::Poll,
};

/// Obtain events at some block hash. The generic parameter is what we
/// will attempt to decode each event into if using [`Events::iter()`],
/// and is expected to be the outermost event enum that contains all of
/// the possible events across all pallets.
///
/// **Note:** This function is hidden from the documentation
/// and is exposed only to be called via the codegen. Thus, prefer to use
/// `api.events().at(block_hash)` over calling this directly.
#[doc(hidden)]
pub async fn at<T: Config, Evs: Decode>(
    client: &'_ Client<T>,
    block_hash: T::Hash,
) -> Result<Events<'_, T, Evs>, BasicError> {
    let mut event_bytes = client
        .rpc()
        .storage(&system_events_key(), Some(block_hash))
        .await?
        .map(|s| s.0)
        .unwrap_or_else(Vec::new);

    // event_bytes is a SCALE encoded vector of events. So, pluck the
    // compact encoded length from the front, leaving the remaining bytes
    // for our iterating to decode.
    //
    // Note: if we get no bytes back, avoid an error reading vec length
    // and default to 0 events.
    let cursor = &mut &*event_bytes;
    let num_events = <Compact<u32>>::decode(cursor).unwrap_or(Compact(0)).0;
    let event_bytes_len = event_bytes.len();
    let remaining_len = cursor.len();
    event_bytes.drain(0..event_bytes_len - remaining_len);

    Ok(Events {
        metadata: client.metadata(),
        block_hash,
        event_bytes,
        num_events,
        _event_type: std::marker::PhantomData,
    })
}

/// Subscribe to events from blocks.
///
/// **Note:** these blocks haven't necessarily been finalised yet; prefer
/// [`Events::subscribe_finalized()`] if that is important.
///
/// **Note:** This function is hidden from the documentation
/// and is exposed only to be called via the codegen. Thus, prefer to use
/// `api.events().subscribe()` over calling this directly.
#[doc(hidden)]
pub async fn subscribe<T: Config, Evs: Decode + 'static>(
    client: &'_ Client<T>,
) -> Result<EventSubscription<'_, T, Evs>, BasicError> {
    let block_subscription = client.rpc().subscribe_blocks().await?;
    Ok(EventSubscription::new(client, block_subscription))
}

/// Subscribe to events from finalized blocks.
///
/// **Note:** This function is hidden from the documentation
/// and is exposed only to be called via the codegen. Thus, prefer to use
/// `api.events().subscribe_finalized()` over calling this directly.
#[doc(hidden)]
pub async fn subscribe_finalized<T: Config, Evs: Decode + 'static>(
    client: &'_ Client<T>,
) -> Result<EventSubscription<'_, T, Evs>, BasicError> {
    let block_subscription = client.rpc().subscribe_finalized_blocks().await?;
    Ok(EventSubscription::new(client, block_subscription))
}

/// A subscription to events that implements [`Stream`], and returns [`Events`] objects for each block.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct EventSubscription<'a, T: Config, Evs: Decode + 'static> {
    finished: bool,
    client: &'a Client<T>,
    block_header_subscription: Subscription<T::Header>,
    #[derivative(Debug = "ignore")]
    at: Option<
        std::pin::Pin<
            Box<dyn Future<Output = Result<Events<'a, T, Evs>, BasicError>> + 'a>,
        >,
    >,
    _event_type: std::marker::PhantomData<Evs>,
}

impl<'a, T: Config, Evs: Decode> EventSubscription<'a, T, Evs> {
    fn new(
        client: &'a Client<T>,
        block_header_subscription: Subscription<T::Header>,
    ) -> Self {
        EventSubscription {
            finished: false,
            client,
            block_header_subscription,
            at: None,
            _event_type: std::marker::PhantomData,
        }
    }
}

impl<'a, T: Config, Evs: Decode> Unpin for EventSubscription<'a, T, Evs> {}

// We want `EventSubscription` to implement Stream. The below implementation is the rather verbose
// way to roughly implement the following function:
//
// ```
// fn subscribe_events<T: Config, Evs: Decode>(client: &'_ Client<T>, block_sub: Subscription<T::Header>) -> impl Stream<Item=Result<Events<'_, T, Evs>, BasicError>> + '_ {
//     use futures::StreamExt;
//     block_sub.then(move |block_header_res| async move {
//         use sp_runtime::traits::Header;
//         let block_header = block_header_res?;
//         let block_hash = block_header.hash();
//         at(client, block_hash).await
//     })
// }
// ```
//
// The advantage of this manual implementation is that we have a named type that we (and others)
// can derive things on, store away, alias etc.
impl<'a, T: Config, Evs: Decode> Stream for EventSubscription<'a, T, Evs> {
    type Item = Result<Events<'a, T, Evs>, BasicError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // We are finished; return None.
        if self.finished {
            return Poll::Ready(None)
        }

        // If there isn't an `at` function yet that's busy resolving a block hash into
        // some event details, then poll the block header subscription to get one.
        if self.at.is_none() {
            match futures::ready!(self.block_header_subscription.poll_next_unpin(cx)) {
                None => {
                    self.finished = true;
                    return Poll::Ready(None)
                }
                Some(Err(e)) => {
                    self.finished = true;
                    return Poll::Ready(Some(Err(e.into())))
                }
                Some(Ok(block_header)) => {
                    use sp_runtime::traits::Header;
                    // Note [jsdw]: We may be able to get rid of the per-item allocation
                    // with https://github.com/oblique/reusable-box-future.
                    self.at = Some(Box::pin(at(self.client, block_header.hash())));
                    // Continue, so that we poll this function future we've just created.
                }
            }
        }

        // If we get here, there will be an `at` function stored. Unwrap it and poll it to
        // completion to get our events, throwing it away as soon as it is ready.
        let at_fn = self
            .at
            .as_mut()
            .expect("'at' function should have been set above'");
        let events = futures::ready!(at_fn.poll_unpin(cx));
        self.at = None;
        Poll::Ready(Some(events))
    }
}

/// A collection of events obtained from a block, bundled with the necessary
/// information needed to decode and iterate over them.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct Events<'a, T: Config, Evs: Decode> {
    metadata: &'a Metadata,
    block_hash: T::Hash,
    // Note; raw event bytes are prefixed with a Compact<u32> containing
    // the number of events to be decoded. We should have stripped that off
    // before storing the bytes here.
    event_bytes: Vec<u8>,
    num_events: u32,
    _event_type: std::marker::PhantomData<Evs>,
}

impl<'a, T: Config, Evs: Decode> Events<'a, T, Evs> {
    /// The number of events.
    pub fn len(&self) -> u32 {
        self.num_events
    }

    /// Are there no events in this block?
    // Note: mainly here to satisfy clippy.
    pub fn is_empty(&self) -> bool {
        self.num_events == 0
    }

    /// Return the block hash that these events are from.
    pub fn block_hash(&self) -> T::Hash {
        self.block_hash
    }

    /// Iterate over the events, statically decoding them as we go.
    /// If an event is encountered that cannot be statically decoded,
    /// a [`codec::Error`] will be returned.
    ///
    /// If the generated code does not know about all of the pallets that exist
    /// in the runtime being targeted, it may not know about all of the
    /// events either, and so this method should be avoided in favout of [`Events::iter_raw()`],
    /// which uses runtime metadata to skip over unknown events.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<EventDetails<Evs>, BasicError>> + '_ {
        let event_bytes = &self.event_bytes;

        let mut pos = 0;
        let mut index = 0;
        std::iter::from_fn(move || {
            let cursor = &mut &event_bytes[pos..];
            let start_len = cursor.len();

            if start_len == 0 || self.num_events == index {
                None
            } else {
                let mut decode_one_event = || -> Result<_, BasicError> {
                    let phase = Phase::decode(cursor)?;
                    let ev = Evs::decode(cursor)?;
                    let _topics = Vec::<T::Hash>::decode(cursor)?;
                    Ok((phase, ev))
                };
                match decode_one_event() {
                    Ok((phase, event)) => {
                        // Skip over decoded bytes in next iteration:
                        pos += start_len - cursor.len();
                        // Gather the event details before incrementing the index for the next iter.
                        let res = Some(Ok(EventDetails {
                            phase,
                            index,
                            event,
                        }));
                        index += 1;
                        res
                    }
                    Err(e) => {
                        // By setting the position to the "end" of the event bytes,
                        // the cursor len will become 0 and the iterator will return `None`
                        // from now on:
                        pos = event_bytes.len();
                        Some(Err(e))
                    }
                }
            }
        })
    }

    /// Iterate over all of the events, using metadata to dynamically
    /// decode them as we go, and returning the raw bytes and other associated
    /// details. If an error occurs, all subsequent iterations return `None`.
    ///
    /// This method is safe to use even if you do not statically know about
    /// all of the possible events; it splits events up using the metadata
    /// obtained at runtime, which does.
    pub fn iter_raw(
        &self,
    ) -> impl Iterator<Item = Result<RawEventDetails, BasicError>> + '_ {
        let event_bytes = &self.event_bytes;

        let mut pos = 0;
        let mut index = 0;
        std::iter::from_fn(move || {
            let cursor = &mut &event_bytes[pos..];
            let start_len = cursor.len();

            if start_len == 0 || self.num_events == index {
                None
            } else {
                match decode_raw_event_details::<T>(self.metadata, index, cursor) {
                    Ok(raw_event) => {
                        // Skip over decoded bytes in next iteration:
                        pos += start_len - cursor.len();
                        // Increment the index:
                        index += 1;
                        // Return the event details:
                        Some(Ok(raw_event))
                    }
                    Err(e) => {
                        // By setting the position to the "end" of the event bytes,
                        // the cursor len will become 0 and the iterator will return `None`
                        // from now on:
                        pos = event_bytes.len();
                        Some(Err(e))
                    }
                }
            }
        })
    }

    /// Iterate through the events using metadata to dynamically decode and skip
    /// them, and return only those which should decode to the provided `Ev` type.
    /// If an error occurs, all subsequent iterations return `None`.
    ///
    /// **Note:** This method internally uses [`Events::iter_raw()`], so it is safe to
    /// use even if you do not statically know about all of the possible events.
    pub fn find<Ev: Event>(&self) -> impl Iterator<Item = Result<Ev, BasicError>> + '_ {
        self.iter_raw().filter_map(|ev| {
            ev.and_then(|ev| ev.as_event::<Ev>().map_err(Into::into))
                .transpose()
        })
    }

    /// Iterate through the events using metadata to dynamically decode and skip
    /// them, and return the first event found which decodes to the provided `Ev` type.
    ///
    /// **Note:** This method internally uses [`Events::iter_raw()`], so it is safe to
    /// use even if you do not statically know about all of the possible events.
    pub fn find_first_event<Ev: Event>(&self) -> Result<Option<Ev>, BasicError> {
        self.find::<Ev>().next().transpose()
    }

    /// Find an event that decodes to the type provided. Returns true if it was found.
    ///
    /// **Note:** This method internally uses [`Events::iter_raw()`], so it is safe to
    /// use even if you do not statically know about all of the possible events.
    pub fn has<Ev: crate::Event>(&self) -> Result<bool, BasicError> {
        Ok(self.find::<Ev>().next().transpose()?.is_some())
    }
}

/// A decoded event and associated details.
#[derive(Debug, Clone, PartialEq)]
pub struct EventDetails<Evs> {
    /// During which [`Phase`] was the event produced?
    pub phase: Phase,
    /// What index is this event in the stored events for this block.
    pub index: u32,
    /// The event itself.
    pub event: Evs,
}

/// The raw bytes for an event with associated details about
/// where and when it was emitted.
#[derive(Debug, Clone, PartialEq)]
pub struct RawEventDetails {
    /// When was the event produced?
    pub phase: Phase,
    /// What index is this event in the stored events for this block.
    pub index: u32,
    /// The name of the pallet from whence the Event originated.
    pub pallet: String,
    /// The index of the pallet from whence the Event originated.
    pub pallet_index: u8,
    /// The name of the pallet Event variant.
    pub variant: String,
    /// The index of the pallet Event variant.
    pub variant_index: u8,
    /// The raw Event data
    pub data: Bytes,
}

impl RawEventDetails {
    /// Attempt to decode this [`RawEventDetails`] into a specific event.
    pub fn as_event<E: Event>(&self) -> Result<Option<E>, CodecError> {
        if self.pallet == E::PALLET && self.variant == E::EVENT {
            Ok(Some(E::decode(&mut &self.data[..])?))
        } else {
            Ok(None)
        }
    }
}

// Attempt to dynamically decode a single event from our events input.
fn decode_raw_event_details<T: Config>(
    metadata: &Metadata,
    index: u32,
    input: &mut &[u8],
) -> Result<RawEventDetails, BasicError> {
    // Decode basic event details:
    let phase = Phase::decode(input)?;
    let pallet_index = input.read_byte()?;
    let variant_index = input.read_byte()?;
    log::debug!(
        "phase {:?}, pallet_index {}, event_variant: {}",
        phase,
        pallet_index,
        variant_index
    );
    log::debug!("remaining input: {}", hex::encode(&input));

    // Get metadata for the event:
    let event_metadata = metadata.event(pallet_index, variant_index)?;
    log::debug!(
        "Decoding Event '{}::{}'",
        event_metadata.pallet(),
        event_metadata.event()
    );

    // Use metadata to figure out which bytes belong to this event:
    let mut event_bytes = Vec::new();
    for arg in event_metadata.variant().fields() {
        let type_id = arg.ty().id();
        let all_bytes = *input;
        // consume some bytes, moving the cursor forward:
        decode_and_consume_type(type_id, &metadata.runtime_metadata().types, input)?;
        // count how many bytes were consumed based on remaining length:
        let consumed_len = all_bytes.len() - input.len();
        // move those consumed bytes to the output vec unaltered:
        event_bytes.extend(&all_bytes[0..consumed_len]);
    }

    // topics come after the event data in EventRecord. They aren't used for
    // anything at the moment, so just decode and throw them away.
    let topics = Vec::<T::Hash>::decode(input)?;
    log::debug!("topics: {:?}", topics);

    Ok(RawEventDetails {
        phase,
        index,
        pallet_index,
        pallet: event_metadata.pallet().to_string(),
        variant_index,
        variant: event_metadata.event().to_string(),
        data: event_bytes.into(),
    })
}

// The storage key needed to access events.
fn system_events_key() -> StorageKey {
    let mut storage_key = twox_128(b"System").to_vec();
    storage_key.extend(twox_128(b"Events").to_vec());
    StorageKey(storage_key)
}

// Given a type Id and a type registry, attempt to consume the bytes
// corresponding to that type from our input.
fn decode_and_consume_type(
    type_id: u32,
    types: &PortableRegistry,
    input: &mut &[u8],
) -> Result<(), BasicError> {
    let ty = types
        .resolve(type_id)
        .ok_or(MetadataError::TypeNotFound(type_id))?;

    fn consume_type<T: Codec>(input: &mut &[u8]) -> Result<(), BasicError> {
        T::decode(input)?;
        Ok(())
    }

    match ty.type_def() {
        TypeDef::Composite(composite) => {
            for field in composite.fields() {
                decode_and_consume_type(field.ty().id(), types, input)?
            }
            Ok(())
        }
        TypeDef::Variant(variant) => {
            let variant_index = u8::decode(input)?;
            let variant = variant
                .variants()
                .iter()
                .find(|v| v.index() == variant_index)
                .ok_or_else(|| {
                    BasicError::Other(format!("Variant {} not found", variant_index))
                })?;
            for field in variant.fields() {
                decode_and_consume_type(field.ty().id(), types, input)?;
            }
            Ok(())
        }
        TypeDef::Sequence(seq) => {
            let len = <Compact<u32>>::decode(input)?;
            for _ in 0..len.0 {
                decode_and_consume_type(seq.type_param().id(), types, input)?;
            }
            Ok(())
        }
        TypeDef::Array(arr) => {
            for _ in 0..arr.len() {
                decode_and_consume_type(arr.type_param().id(), types, input)?;
            }
            Ok(())
        }
        TypeDef::Tuple(tuple) => {
            for field in tuple.fields() {
                decode_and_consume_type(field.id(), types, input)?;
            }
            Ok(())
        }
        TypeDef::Primitive(primitive) => {
            match primitive {
                TypeDefPrimitive::Bool => consume_type::<bool>(input),
                TypeDefPrimitive::Char => {
                    Err(
                        EventsDecodingError::UnsupportedPrimitive(TypeDefPrimitive::Char)
                            .into(),
                    )
                }
                TypeDefPrimitive::Str => consume_type::<String>(input),
                TypeDefPrimitive::U8 => consume_type::<u8>(input),
                TypeDefPrimitive::U16 => consume_type::<u16>(input),
                TypeDefPrimitive::U32 => consume_type::<u32>(input),
                TypeDefPrimitive::U64 => consume_type::<u64>(input),
                TypeDefPrimitive::U128 => consume_type::<u128>(input),
                TypeDefPrimitive::U256 => {
                    Err(
                        EventsDecodingError::UnsupportedPrimitive(TypeDefPrimitive::U256)
                            .into(),
                    )
                }
                TypeDefPrimitive::I8 => consume_type::<i8>(input),
                TypeDefPrimitive::I16 => consume_type::<i16>(input),
                TypeDefPrimitive::I32 => consume_type::<i32>(input),
                TypeDefPrimitive::I64 => consume_type::<i64>(input),
                TypeDefPrimitive::I128 => consume_type::<i128>(input),
                TypeDefPrimitive::I256 => {
                    Err(
                        EventsDecodingError::UnsupportedPrimitive(TypeDefPrimitive::I256)
                            .into(),
                    )
                }
            }
        }
        TypeDef::Compact(compact) => {
            let inner = types
                .resolve(compact.type_param().id())
                .ok_or(MetadataError::TypeNotFound(type_id))?;
            let mut decode_compact_primitive = |primitive: &TypeDefPrimitive| {
                match primitive {
                    TypeDefPrimitive::U8 => consume_type::<Compact<u8>>(input),
                    TypeDefPrimitive::U16 => consume_type::<Compact<u16>>(input),
                    TypeDefPrimitive::U32 => consume_type::<Compact<u32>>(input),
                    TypeDefPrimitive::U64 => consume_type::<Compact<u64>>(input),
                    TypeDefPrimitive::U128 => consume_type::<Compact<u128>>(input),
                    prim => {
                        Err(EventsDecodingError::InvalidCompactPrimitive(prim.clone())
                            .into())
                    }
                }
            };
            match inner.type_def() {
                TypeDef::Primitive(primitive) => decode_compact_primitive(primitive),
                TypeDef::Composite(composite) => {
                    match composite.fields() {
                        [field] => {
                            let field_ty =
                                types.resolve(field.ty().id()).ok_or_else(|| {
                                    MetadataError::TypeNotFound(field.ty().id())
                                })?;
                            if let TypeDef::Primitive(primitive) = field_ty.type_def() {
                                decode_compact_primitive(primitive)
                            } else {
                                Err(EventsDecodingError::InvalidCompactType(
                                    "Composite type must have a single primitive field"
                                        .into(),
                                )
                                .into())
                            }
                        }
                        _ => {
                            Err(EventsDecodingError::InvalidCompactType(
                                "Composite type must have a single field".into(),
                            )
                            .into())
                        }
                    }
                }
                _ => {
                    Err(EventsDecodingError::InvalidCompactType(
                        "Compact type must be a primitive or a composite type".into(),
                    )
                    .into())
                }
            }
        }
        TypeDef::BitSequence(bitseq) => {
            let bit_store_def = types
                .resolve(bitseq.bit_store_type().id())
                .ok_or(MetadataError::TypeNotFound(type_id))?
                .type_def();

            // We just need to consume the correct number of bytes. Roughly, we encode this
            // as a Compact<u32> length, and then a slice of T of that length, where T is the
            // bit store type. So, we ignore the bit order and only care that the bit store type
            // used lines up in terms of the number of bytes it will take to encode/decode it.
            match bit_store_def {
                TypeDef::Primitive(TypeDefPrimitive::U8) => {
                    consume_type::<BitVec<Lsb0, u8>>(input)
                }
                TypeDef::Primitive(TypeDefPrimitive::U16) => {
                    consume_type::<BitVec<Lsb0, u16>>(input)
                }
                TypeDef::Primitive(TypeDefPrimitive::U32) => {
                    consume_type::<BitVec<Lsb0, u32>>(input)
                }
                TypeDef::Primitive(TypeDefPrimitive::U64) => {
                    consume_type::<BitVec<Lsb0, u64>>(input)
                }
                store => {
                    return Err(EventsDecodingError::InvalidBitSequenceType(format!(
                        "{:?}",
                        store
                    ))
                    .into())
                }
            }
        }
    }
}

/// The possible errors that we can run into attempting to decode events.
#[derive(Debug, thiserror::Error)]
pub enum EventsDecodingError {
    /// Unsupported primitive type
    #[error("Unsupported primitive type {0:?}")]
    UnsupportedPrimitive(TypeDefPrimitive),
    /// Invalid compact type, must be an unsigned int.
    #[error("Invalid compact primitive {0:?}")]
    InvalidCompactPrimitive(TypeDefPrimitive),
    /// Invalid compact type; error details in string.
    #[error("Invalid compact composite type {0}")]
    InvalidCompactType(String),
    /// Invalid bit sequence type; bit store type or bit order type used aren't supported.
    #[error("Invalid bit sequence type; bit store type {0} is not supported")]
    InvalidBitSequenceType(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error::GenericError::{
            Codec,
            EventsDecoding,
            Other,
        },
        events::EventsDecodingError::UnsupportedPrimitive,
        Config,
        DefaultConfig,
        Phase,
    };
    use assert_matches::assert_matches;
    use codec::Encode;
    use frame_metadata::{
        v14::{
            ExtrinsicMetadata,
            PalletEventMetadata,
            PalletMetadata,
            RuntimeMetadataLastVersion,
        },
        RuntimeMetadataPrefixed,
    };
    use scale_info::{
        meta_type,
        TypeInfo,
    };
    use std::convert::TryFrom;

    type TypeId = scale_info::interner::UntrackedSymbol<std::any::TypeId>;

    /// An "outer" events enum containing exactly one event.
    #[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq)]
    pub enum AllEvents<Ev> {
        E(Ev),
    }

    /// This encodes to the same format an event is expected to encode to
    /// in node System.Events storage.
    #[derive(Encode)]
    pub struct EventRecord<E: Encode> {
        phase: Phase,
        event: AllEvents<E>,
        topics: Vec<<DefaultConfig as Config>::Hash>,
    }

    /// Build an EventRecord, which encoded events in the format expected
    /// to be handed back from storage queries to System.Events.
    fn event_record<E: Encode>(phase: Phase, event: E) -> EventRecord<E> {
        EventRecord {
            phase,
            event: AllEvents::E(event),
            topics: vec![],
        }
    }

    /// Build a type registry that knows about the single type provided.
    fn singleton_type_registry<T: scale_info::TypeInfo + 'static>(
    ) -> (TypeId, PortableRegistry) {
        let m = scale_info::MetaType::new::<T>();
        let mut types = scale_info::Registry::new();
        let id = types.register_type(&m);
        let portable_registry: PortableRegistry = types.into();

        (id, portable_registry)
    }

    /// Build fake metadata consisting of a single pallet that knows
    /// about the event type provided.
    fn metadata<E: TypeInfo + 'static>() -> Metadata {
        let pallets = vec![PalletMetadata {
            name: "Test",
            storage: None,
            calls: None,
            event: Some(PalletEventMetadata {
                ty: meta_type::<E>(),
            }),
            constants: vec![],
            error: None,
            index: 0,
        }];

        let extrinsic = ExtrinsicMetadata {
            ty: meta_type::<()>(),
            version: 0,
            signed_extensions: vec![],
        };

        let v14 = RuntimeMetadataLastVersion::new(pallets, extrinsic, meta_type::<()>());
        let runtime_metadata: RuntimeMetadataPrefixed = v14.into();

        Metadata::try_from(runtime_metadata).unwrap()
    }

    /// Build an `Events` object for test purposes, based on the details provided,
    /// and with a default block hash.
    fn events<E: Decode + Encode>(
        metadata: &'_ Metadata,
        event_records: Vec<EventRecord<E>>,
    ) -> Events<'_, DefaultConfig, AllEvents<E>> {
        let num_events = event_records.len() as u32;
        let mut event_bytes = Vec::new();
        for ev in event_records {
            ev.encode_to(&mut event_bytes);
        }
        events_raw(metadata, event_bytes, num_events)
    }

    /// Much like [`events`], but takes pre-encoded events and event count, so that we can
    /// mess with the bytes in tests if we need to.
    fn events_raw<E: Decode + Encode>(
        metadata: &'_ Metadata,
        event_bytes: Vec<u8>,
        num_events: u32,
    ) -> Events<'_, DefaultConfig, AllEvents<E>> {
        Events {
            block_hash: <DefaultConfig as Config>::Hash::default(),
            event_bytes,
            metadata,
            num_events,
            _event_type: std::marker::PhantomData,
        }
    }

    fn decode_and_consume_type_consumes_all_bytes<
        T: codec::Encode + scale_info::TypeInfo + 'static,
    >(
        val: T,
    ) {
        let (type_id, registry) = singleton_type_registry::<T>();
        let bytes = val.encode();
        let cursor = &mut &*bytes;

        decode_and_consume_type(type_id.id(), &registry, cursor).unwrap();
        assert_eq!(cursor.len(), 0);
    }

    #[test]
    fn statically_decode_single_event() {
        #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
        enum Event {
            A(u8),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let events = events::<Event>(
            &metadata,
            vec![event_record(Phase::Finalization, Event::A(1))],
        );

        let event_details: Vec<EventDetails<AllEvents<Event>>> =
            events.iter().collect::<Result<_, _>>().unwrap();
        assert_eq!(
            event_details,
            vec![EventDetails {
                index: 0,
                phase: Phase::Finalization,
                event: AllEvents::E(Event::A(1))
            }]
        );
    }

    #[test]
    fn statically_decode_multiple_events() {
        #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
        enum Event {
            A(u8),
            B(bool),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let events = events::<Event>(
            &metadata,
            vec![
                event_record(Phase::Initialization, Event::A(1)),
                event_record(Phase::ApplyExtrinsic(123), Event::B(true)),
                event_record(Phase::Finalization, Event::A(234)),
            ],
        );

        let event_details: Vec<EventDetails<AllEvents<Event>>> =
            events.iter().collect::<Result<_, _>>().unwrap();
        assert_eq!(
            event_details,
            vec![
                EventDetails {
                    index: 0,
                    phase: Phase::Initialization,
                    event: AllEvents::E(Event::A(1))
                },
                EventDetails {
                    index: 1,
                    phase: Phase::ApplyExtrinsic(123),
                    event: AllEvents::E(Event::B(true))
                },
                EventDetails {
                    index: 2,
                    phase: Phase::Finalization,
                    event: AllEvents::E(Event::A(234))
                },
            ]
        );
    }

    #[test]
    fn statically_decode_multiple_events_until_error() {
        #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
        enum Event {
            A(u8),
            B(bool),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode 2 events:
        let mut event_bytes = vec![];
        event_record(Phase::Initialization, Event::A(1)).encode_to(&mut event_bytes);
        event_record(Phase::ApplyExtrinsic(123), Event::B(true))
            .encode_to(&mut event_bytes);

        // Push a few naff bytes to the end (a broken third event):
        event_bytes.extend_from_slice(&[3, 127, 45, 0, 2]);

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let events = events_raw::<Event>(
            &metadata,
            event_bytes,
            3, // 2 "good" events, and then it'll hit the naff bytes.
        );

        let mut events_iter = events.iter();
        assert_eq!(
            events_iter.next().unwrap().unwrap(),
            EventDetails {
                index: 0,
                phase: Phase::Initialization,
                event: AllEvents::E(Event::A(1))
            }
        );
        assert_eq!(
            events_iter.next().unwrap().unwrap(),
            EventDetails {
                index: 1,
                phase: Phase::ApplyExtrinsic(123),
                event: AllEvents::E(Event::B(true))
            }
        );

        // We'll hit an error trying to decode the third event:
        assert!(events_iter.next().unwrap().is_err());
        // ... and then "None" from then on.
        assert!(events_iter.next().is_none());
        assert!(events_iter.next().is_none());
    }

    #[test]
    fn dynamically_decode_single_event() {
        #[derive(Clone, Copy, Debug, PartialEq, Decode, Encode, TypeInfo)]
        enum Event {
            A(u8),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let event = Event::A(1);
        let events = events::<Event>(
            &metadata,
            vec![event_record(Phase::ApplyExtrinsic(123), event)],
        );

        let event_details: Vec<RawEventDetails> =
            events.iter_raw().collect::<Result<_, _>>().unwrap();
        let expected_event_data = {
            let mut bytes = event.encode();
            // Strip variant tag off event bytes:
            bytes.drain(0..1);
            bytes
        };

        assert_eq!(
            event_details,
            vec![RawEventDetails {
                index: 0,
                phase: Phase::ApplyExtrinsic(123),
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                data: expected_event_data.into()
            }]
        );
    }

    #[test]
    fn dynamically_decode_multiple_events() {
        #[derive(Clone, Copy, Debug, PartialEq, Decode, Encode, TypeInfo)]
        enum Event {
            A(u8),
            B(bool),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let event1 = Event::A(1);
        let event2 = Event::B(true);
        let event3 = Event::A(234);

        let events = events::<Event>(
            &metadata,
            vec![
                event_record(Phase::Initialization, event1),
                event_record(Phase::ApplyExtrinsic(123), event2),
                event_record(Phase::Finalization, event3),
            ],
        );

        let event_details: Vec<RawEventDetails> =
            events.iter_raw().collect::<Result<_, _>>().unwrap();
        let event_bytes = |ev: Event| {
            let mut bytes = ev.encode();
            // Strip variant tag off event bytes:
            bytes.drain(0..1);
            bytes.into()
        };

        assert_eq!(
            event_details,
            vec![
                RawEventDetails {
                    index: 0,
                    phase: Phase::Initialization,
                    pallet: "Test".to_string(),
                    pallet_index: 0,
                    variant: "A".to_string(),
                    variant_index: 0,
                    data: event_bytes(event1)
                },
                RawEventDetails {
                    index: 1,
                    phase: Phase::ApplyExtrinsic(123),
                    pallet: "Test".to_string(),
                    pallet_index: 0,
                    variant: "B".to_string(),
                    variant_index: 1,
                    data: event_bytes(event2)
                },
                RawEventDetails {
                    index: 2,
                    phase: Phase::Finalization,
                    pallet: "Test".to_string(),
                    pallet_index: 0,
                    variant: "A".to_string(),
                    variant_index: 0,
                    data: event_bytes(event3)
                },
            ]
        );
    }

    #[test]
    fn dynamically_decode_multiple_events_until_error() {
        #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
        enum Event {
            A(u8),
            B(bool),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode 2 events:
        let mut event_bytes = vec![];
        event_record(Phase::Initialization, Event::A(1)).encode_to(&mut event_bytes);
        event_record(Phase::ApplyExtrinsic(123), Event::B(true))
            .encode_to(&mut event_bytes);

        // Push a few naff bytes to the end (a broken third event):
        event_bytes.extend_from_slice(&[3, 127, 45, 0, 2]);

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let events = events_raw::<Event>(
            &metadata,
            event_bytes,
            3, // 2 "good" events, and then it'll hit the naff bytes.
        );

        let event_bytes = |ev: Event| {
            let mut bytes = ev.encode();
            // Strip variant tag off event bytes:
            bytes.drain(0..1);
            bytes.into()
        };

        let mut events_iter = events.iter_raw();
        assert_eq!(
            events_iter.next().unwrap().unwrap(),
            RawEventDetails {
                index: 0,
                phase: Phase::Initialization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                data: event_bytes(Event::A(1))
            }
        );
        assert_eq!(
            events_iter.next().unwrap().unwrap(),
            RawEventDetails {
                index: 1,
                phase: Phase::ApplyExtrinsic(123),
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "B".to_string(),
                variant_index: 1,
                data: event_bytes(Event::B(true))
            }
        );

        // We'll hit an error trying to decode the third event:
        assert!(events_iter.next().unwrap().is_err());
        // ... and then "None" from then on.
        assert!(events_iter.next().is_none());
        assert!(events_iter.next().is_none());
    }

    #[test]
    fn compact_event_field() {
        #[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
        enum Event {
            A(#[codec(compact)] u32),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let events = events::<Event>(
            &metadata,
            vec![event_record(Phase::Finalization, Event::A(1))],
        );

        // Statically decode:
        let event_details: Vec<EventDetails<AllEvents<Event>>> =
            events.iter().collect::<Result<_, _>>().unwrap();
        assert_eq!(
            event_details,
            vec![EventDetails {
                index: 0,
                phase: Phase::Finalization,
                event: AllEvents::E(Event::A(1))
            }]
        );

        // Dynamically decode:
        let event_details: Vec<RawEventDetails> =
            events.iter_raw().collect::<Result<_, _>>().unwrap();
        let expected_event_data = {
            let mut bytes = Event::A(1).encode();
            // Strip variant tag off event bytes:
            bytes.drain(0..1);
            bytes
        };
        assert_eq!(
            event_details,
            vec![RawEventDetails {
                index: 0,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                data: expected_event_data.into()
            }]
        );
    }

    #[test]
    fn compact_wrapper_struct_field() {
        #[derive(Clone, Decode, Debug, PartialEq, Encode, TypeInfo)]
        enum Event {
            A(#[codec(compact)] CompactWrapper),
        }

        #[derive(Clone, Decode, Debug, PartialEq, codec::CompactAs, Encode, TypeInfo)]
        struct CompactWrapper(u64);

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construct an Events object to iterate them:
        let events = events::<Event>(
            &metadata,
            vec![event_record(
                Phase::Finalization,
                Event::A(CompactWrapper(1)),
            )],
        );

        // Statically decode:
        let event_details: Vec<EventDetails<AllEvents<Event>>> =
            events.iter().collect::<Result<_, _>>().unwrap();
        assert_eq!(
            event_details,
            vec![EventDetails {
                index: 0,
                phase: Phase::Finalization,
                event: AllEvents::E(Event::A(CompactWrapper(1)))
            }]
        );

        // Dynamically decode:
        let event_details: Vec<RawEventDetails> =
            events.iter_raw().collect::<Result<_, _>>().unwrap();
        let expected_event_data = {
            let mut bytes = Event::A(CompactWrapper(1)).encode();
            // Strip variant tag off event bytes:
            bytes.drain(0..1);
            bytes
        };
        assert_eq!(
            event_details,
            vec![RawEventDetails {
                index: 0,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                data: expected_event_data.into()
            }]
        );
    }

    #[test]
    fn event_containing_explicit_index() {
        #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
        #[repr(u8)]
        #[allow(trivial_numeric_casts, clippy::unnecessary_cast)] // required because the Encode derive produces a warning otherwise
        pub enum MyType {
            B = 10u8,
        }

        #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
        enum Event {
            A(MyType),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construct an Events object to iterate them:
        let events = events::<Event>(
            &metadata,
            vec![event_record(Phase::Finalization, Event::A(MyType::B))],
        );

        // Statically decode:
        let event_details: Vec<EventDetails<AllEvents<Event>>> =
            events.iter().collect::<Result<_, _>>().unwrap();
        assert_eq!(
            event_details,
            vec![EventDetails {
                index: 0,
                phase: Phase::Finalization,
                event: AllEvents::E(Event::A(MyType::B))
            }]
        );

        // Dynamically decode:
        let event_details: Vec<RawEventDetails> =
            events.iter_raw().collect::<Result<_, _>>().unwrap();
        let expected_event_data = {
            let mut bytes = Event::A(MyType::B).encode();
            // Strip variant tag off event bytes:
            bytes.drain(0..1);
            bytes
        };
        assert_eq!(
            event_details,
            vec![RawEventDetails {
                index: 0,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                data: expected_event_data.into()
            }]
        );
    }

    #[test]
    fn decode_bitvec() {
        use bitvec::order::Msb0;

        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Lsb0, u8; 0, 1, 1, 0, 1],
        );
        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Msb0, u8; 0, 1, 1, 0, 1, 0, 1, 0, 0],
        );

        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Lsb0, u16; 0, 1, 1, 0, 1],
        );
        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Msb0, u16; 0, 1, 1, 0, 1, 0, 1, 0, 0],
        );

        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Lsb0, u32; 0, 1, 1, 0, 1],
        );
        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Msb0, u32; 0, 1, 1, 0, 1, 0, 1, 0, 0],
        );

        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Lsb0, u64; 0, 1, 1, 0, 1],
        );
        decode_and_consume_type_consumes_all_bytes(
            bitvec::bitvec![Msb0, u64; 0, 1, 1, 0, 1, 0, 1, 0, 0],
        );
    }

    #[test]
    fn decode_primitive() {
        decode_and_consume_type_consumes_all_bytes(false);
        decode_and_consume_type_consumes_all_bytes(true);

        let dummy_data = vec![0u8];
        let dummy_cursor = &mut &*dummy_data;
        let (id, reg) = singleton_type_registry::<char>();
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(
            res,
            Err(EventsDecoding(UnsupportedPrimitive(TypeDefPrimitive::Char)))
        );

        decode_and_consume_type_consumes_all_bytes("str".to_string());

        decode_and_consume_type_consumes_all_bytes(1u8);
        decode_and_consume_type_consumes_all_bytes(1i8);

        decode_and_consume_type_consumes_all_bytes(1u16);
        decode_and_consume_type_consumes_all_bytes(1i16);

        decode_and_consume_type_consumes_all_bytes(1u32);
        decode_and_consume_type_consumes_all_bytes(1i32);

        decode_and_consume_type_consumes_all_bytes(1u64);
        decode_and_consume_type_consumes_all_bytes(1i64);

        decode_and_consume_type_consumes_all_bytes(1u128);
        decode_and_consume_type_consumes_all_bytes(1i128);
    }

    #[test]
    fn decode_tuple() {
        decode_and_consume_type_consumes_all_bytes(());

        decode_and_consume_type_consumes_all_bytes((true,));

        decode_and_consume_type_consumes_all_bytes((true, "str"));

        // Incomplete bytes for decoding
        let dummy_data = false.encode();
        let dummy_cursor = &mut &*dummy_data;
        let (id, reg) = singleton_type_registry::<(bool, &'static str)>();
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(res, Err(Codec(_)));

        // Incomplete bytes for decoding, with invalid char type
        let dummy_data = (false, "str", 0u8).encode();
        let dummy_cursor = &mut &*dummy_data;
        let (id, reg) = singleton_type_registry::<(bool, &'static str, char)>();
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(
            res,
            Err(EventsDecoding(UnsupportedPrimitive(TypeDefPrimitive::Char)))
        );
        // The last byte (0x0 u8) should not be consumed
        assert_eq!(dummy_cursor.len(), 1);
    }

    #[test]
    fn decode_array_and_seq() {
        decode_and_consume_type_consumes_all_bytes([0]);
        decode_and_consume_type_consumes_all_bytes([1, 2, 3, 4, 5]);
        decode_and_consume_type_consumes_all_bytes([0; 500]);
        decode_and_consume_type_consumes_all_bytes(["str", "abc", "cde"]);

        decode_and_consume_type_consumes_all_bytes(vec![0]);
        decode_and_consume_type_consumes_all_bytes(vec![1, 2, 3, 4, 5]);
        decode_and_consume_type_consumes_all_bytes(vec!["str", "abc", "cde"]);
    }

    #[test]
    fn decode_variant() {
        #[derive(Clone, Encode, TypeInfo)]
        enum EnumVar {
            A,
            B((&'static str, u8)),
            C { named: i16 },
        }
        const INVALID_TYPE_ID: u32 = 1024;

        decode_and_consume_type_consumes_all_bytes(EnumVar::A);
        decode_and_consume_type_consumes_all_bytes(EnumVar::B(("str", 1)));
        decode_and_consume_type_consumes_all_bytes(EnumVar::C { named: 1 });

        // Invalid variant index
        let dummy_data = 3u8.encode();
        let dummy_cursor = &mut &*dummy_data;
        let (id, reg) = singleton_type_registry::<EnumVar>();
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(res, Err(Other(_)));

        // Valid index, incomplete data
        let dummy_data = 2u8.encode();
        let dummy_cursor = &mut &*dummy_data;
        let res = decode_and_consume_type(id.id(), &reg, dummy_cursor);
        assert_matches!(res, Err(Codec(_)));

        let res = decode_and_consume_type(INVALID_TYPE_ID, &reg, dummy_cursor);
        assert_matches!(res, Err(crate::error::GenericError::Metadata(_)));
    }

    #[test]
    fn decode_composite() {
        #[derive(Clone, Encode, TypeInfo)]
        struct Composite {}
        decode_and_consume_type_consumes_all_bytes(Composite {});

        #[derive(Clone, Encode, TypeInfo)]
        struct CompositeV2 {
            id: u32,
            name: String,
        }
        decode_and_consume_type_consumes_all_bytes(CompositeV2 {
            id: 10,
            name: "str".to_string(),
        });

        #[derive(Clone, Encode, TypeInfo)]
        struct CompositeV3<T> {
            id: u32,
            extra: T,
        }
        decode_and_consume_type_consumes_all_bytes(CompositeV3 {
            id: 10,
            extra: vec![0, 1, 2],
        });
        decode_and_consume_type_consumes_all_bytes(CompositeV3 {
            id: 10,
            extra: bitvec::bitvec![Lsb0, u8; 0, 1, 1, 0, 1],
        });
        decode_and_consume_type_consumes_all_bytes(CompositeV3 {
            id: 10,
            extra: ("str", 1),
        });
        decode_and_consume_type_consumes_all_bytes(CompositeV3 {
            id: 10,
            extra: CompositeV2 {
                id: 2,
                name: "str".to_string(),
            },
        });

        #[derive(Clone, Encode, TypeInfo)]
        struct CompositeV4(u32, bool);
        decode_and_consume_type_consumes_all_bytes(CompositeV4(1, true));

        #[derive(Clone, Encode, TypeInfo)]
        struct CompositeV5(u32);
        decode_and_consume_type_consumes_all_bytes(CompositeV5(1));
    }

    #[test]
    fn decode_compact() {
        #[derive(Clone, Encode, TypeInfo)]
        enum Compact {
            A(#[codec(compact)] u32),
        }
        decode_and_consume_type_consumes_all_bytes(Compact::A(1));

        #[derive(Clone, Encode, TypeInfo)]
        struct CompactV2(#[codec(compact)] u32);
        decode_and_consume_type_consumes_all_bytes(CompactV2(1));

        #[derive(Clone, Encode, TypeInfo)]
        struct CompactV3 {
            #[codec(compact)]
            val: u32,
        }
        decode_and_consume_type_consumes_all_bytes(CompactV3 { val: 1 });

        #[derive(Clone, Encode, TypeInfo)]
        struct CompactV4<T> {
            #[codec(compact)]
            val: T,
        }
        decode_and_consume_type_consumes_all_bytes(CompactV4 { val: 0u8 });
        decode_and_consume_type_consumes_all_bytes(CompactV4 { val: 1u16 });
    }
}
