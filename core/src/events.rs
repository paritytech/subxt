// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Decode and work with events.
//!
//! # Example
//!
//! ```rust
//! use subxt_macro::subxt;
//! use subxt_core::config::PolkadotConfig;
//! use subxt_core::events;
//! use subxt_core::metadata;
//!
//! // If we generate types without `subxt`, we need to point to `::subxt_core`:
//! #[subxt(
//!     crate = "::subxt_core",
//!     runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale",
//! )]
//! pub mod polkadot {}
//!
//! // Some metadata we'll use to work with storage entries:
//! let metadata_bytes = include_bytes!("../../artifacts/polkadot_metadata_full.scale");
//! let metadata = metadata::decode_from(&metadata_bytes[..]).unwrap();
//!
//! // Some bytes representing events (located in System.Events storage):
//! let event_bytes = hex::decode("1c00000000000000a2e9b53d5517020000000100000000000310c96d901d0102000000020000000408d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27dbeea5a030000000000000000000000000000020000000402d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48102700000000000000000000000000000000020000000407be5ddb1579b72e84524fc29e78609e3caf42e85aa118ebfe0b0ad404b5bdd25fbeea5a030000000000000000000000000000020000002100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27dbeea5a03000000000000000000000000000000000000000000000000000000000000020000000000426df03e00000000").unwrap();
//!
//! // We can decode these bytes like so:
//! let evs = events::decode_from::<PolkadotConfig>(event_bytes, metadata);
//!
//! // And then do things like iterate over them and inspect details:
//! for ev in evs.iter() {
//!     let ev = ev.unwrap();
//!     println!("Index: {}", ev.index());
//!     println!("Name: {}.{}", ev.pallet_name(), ev.variant_name());
//!     println!("Fields: {:?}", ev.field_values().unwrap());
//! }
//! ```

use alloc::sync::Arc;
use alloc::vec::Vec;
use codec::{Compact, Decode, Encode};
use derive_where::derive_where;
use scale_decode::{DecodeAsFields, DecodeAsType};
use subxt_metadata::PalletMetadata;

use crate::{error::MetadataError, Config, Error, Metadata};

/// Create a new [`Events`] instance from the given bytes.
///
/// This is a shortcut for [`Events::decode_from`].
pub fn decode_from<T: Config>(event_bytes: Vec<u8>, metadata: Metadata) -> Events<T> {
    Events::decode_from(event_bytes, metadata)
}

/// Trait to uniquely identify the events's identity from the runtime metadata.
///
/// Generated API structures that represent an event implement this trait.
///
/// The trait is utilized to decode emitted events from a block, via obtaining the
/// form of the `Event` from the metadata.
pub trait StaticEvent: DecodeAsFields {
    /// Pallet name.
    const PALLET: &'static str;
    /// Event name.
    const EVENT: &'static str;

    /// Returns true if the given pallet and event names match this event.
    fn is_event(pallet: &str, event: &str) -> bool {
        Self::PALLET == pallet && Self::EVENT == event
    }
}

/// A collection of events obtained from a block, bundled with the necessary
/// information needed to decode and iterate over them.
#[derive_where(Clone)]
pub struct Events<T: Config> {
    metadata: Metadata,
    // Note; raw event bytes are prefixed with a Compact<u32> containing
    // the number of events to be decoded. The start_idx reflects that, so
    // that we can skip over those bytes when decoding them
    event_bytes: Arc<[u8]>,
    start_idx: usize,
    num_events: u32,
    marker: core::marker::PhantomData<T>,
}

// Ignore the Metadata when debug-logging events; it's big and distracting.
impl<T: Config> core::fmt::Debug for Events<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Events")
            .field("event_bytes", &self.event_bytes)
            .field("start_idx", &self.start_idx)
            .field("num_events", &self.num_events)
            .finish()
    }
}

impl<T: Config> Events<T> {
    /// Create a new [`Events`] instance from the given bytes.
    pub fn decode_from(event_bytes: Vec<u8>, metadata: Metadata) -> Self {
        // event_bytes is a SCALE encoded vector of events. So, pluck the
        // compact encoded length from the front, leaving the remaining bytes
        // for our iterating to decode.
        //
        // Note: if we get no bytes back, avoid an error reading vec length
        // and default to 0 events.
        let cursor = &mut &*event_bytes;
        let num_events = <Compact<u32>>::decode(cursor).unwrap_or(Compact(0)).0;

        // Start decoding after the compact encoded bytes.
        let start_idx = event_bytes.len() - cursor.len();

        Self {
            metadata,
            event_bytes: event_bytes.into(),
            start_idx,
            num_events,
            marker: core::marker::PhantomData,
        }
    }

    /// The number of events.
    pub fn len(&self) -> u32 {
        self.num_events
    }

    /// Are there no events in this block?
    // Note: mainly here to satisfy clippy.
    pub fn is_empty(&self) -> bool {
        self.num_events == 0
    }

    /// Return the bytes representing all of the events.
    pub fn bytes(&self) -> &[u8] {
        &self.event_bytes
    }

    /// Iterate over all of the events, using metadata to dynamically
    /// decode them as we go, and returning the raw bytes and other associated
    /// details. If an error occurs, all subsequent iterations return `None`.
    // Dev note: The returned iterator is 'static + Send so that we can box it up and make
    // use of it with our `FilterEvents` stuff.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<EventDetails<T>, Error>> + Send + Sync + 'static {
        // The event bytes ignoring the compact encoded length on the front:
        let event_bytes = self.event_bytes.clone();
        let metadata = self.metadata.clone();
        let num_events = self.num_events;

        let mut pos = self.start_idx;
        let mut index = 0;
        core::iter::from_fn(move || {
            if event_bytes.len() <= pos || num_events == index {
                None
            } else {
                match EventDetails::decode_from(metadata.clone(), event_bytes.clone(), pos, index) {
                    Ok(event_details) => {
                        // Skip over decoded bytes in next iteration:
                        pos += event_details.bytes().len();
                        // Increment the index:
                        index += 1;
                        // Return the event details:
                        Some(Ok(event_details))
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
    pub fn find<Ev: StaticEvent>(&self) -> impl Iterator<Item = Result<Ev, Error>> + '_ {
        self.iter().filter_map(|ev| {
            ev.and_then(|ev| ev.as_event::<Ev>().map_err(Into::into))
                .transpose()
        })
    }

    /// Iterate through the events using metadata to dynamically decode and skip
    /// them, and return the first event found which decodes to the provided `Ev` type.
    pub fn find_first<Ev: StaticEvent>(&self) -> Result<Option<Ev>, Error> {
        self.find::<Ev>().next().transpose()
    }

    /// Iterate through the events using metadata to dynamically decode and skip
    /// them, and return the last event found which decodes to the provided `Ev` type.
    pub fn find_last<Ev: StaticEvent>(&self) -> Result<Option<Ev>, Error> {
        self.find::<Ev>().last().transpose()
    }

    /// Find an event that decodes to the type provided. Returns true if it was found.
    pub fn has<Ev: StaticEvent>(&self) -> Result<bool, Error> {
        Ok(self.find::<Ev>().next().transpose()?.is_some())
    }
}

/// A phase of a block's execution.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Decode, Encode)]
pub enum Phase {
    /// Applying an extrinsic.
    ApplyExtrinsic(u32),
    /// Finalizing the block.
    Finalization,
    /// Initializing the block.
    Initialization,
}

/// The event details.
#[derive(Debug, Clone)]
pub struct EventDetails<T: Config> {
    phase: Phase,
    /// The index of the event in the list of events in a given block.
    index: u32,
    all_bytes: Arc<[u8]>,
    // start of the bytes (phase, pallet/variant index and then fields and then topic to follow).
    start_idx: usize,
    // start of the event (ie pallet/variant index and then the fields and topic after).
    event_start_idx: usize,
    // start of the fields (ie after phase and pallet/variant index).
    event_fields_start_idx: usize,
    // end of the fields.
    event_fields_end_idx: usize,
    // end of everything (fields + topics)
    end_idx: usize,
    metadata: Metadata,
    topics: Vec<T::Hash>,
}

impl<T: Config> EventDetails<T> {
    /// Attempt to dynamically decode a single event from our events input.
    fn decode_from(
        metadata: Metadata,
        all_bytes: Arc<[u8]>,
        start_idx: usize,
        index: u32,
    ) -> Result<EventDetails<T>, Error> {
        let input = &mut &all_bytes[start_idx..];

        let phase = Phase::decode(input)?;

        let event_start_idx = all_bytes.len() - input.len();

        let pallet_index = u8::decode(input)?;
        let variant_index = u8::decode(input)?;

        let event_fields_start_idx = all_bytes.len() - input.len();

        // Get metadata for the event:
        let event_pallet = metadata.pallet_by_index_err(pallet_index)?;
        let event_variant = event_pallet
            .event_variant_by_index(variant_index)
            .ok_or(MetadataError::VariantIndexNotFound(variant_index))?;
        tracing::debug!(
            "Decoding Event '{}::{}'",
            event_pallet.name(),
            &event_variant.name
        );

        // Skip over the bytes belonging to this event.
        for field_metadata in &event_variant.fields {
            // Skip over the bytes for this field:
            scale_decode::visitor::decode_with_visitor(
                input,
                field_metadata.ty.id,
                metadata.types(),
                scale_decode::visitor::IgnoreVisitor::new(),
            )
            .map_err(scale_decode::Error::from)?;
        }

        // the end of the field bytes.
        let event_fields_end_idx = all_bytes.len() - input.len();

        // topics come after the event data in EventRecord.
        let topics = Vec::<T::Hash>::decode(input)?;

        // what bytes did we skip over in total, including topics.
        let end_idx = all_bytes.len() - input.len();

        Ok(EventDetails {
            phase,
            index,
            start_idx,
            event_start_idx,
            event_fields_start_idx,
            event_fields_end_idx,
            end_idx,
            all_bytes,
            metadata,
            topics,
        })
    }

    /// When was the event produced?
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// What index is this event in the stored events for this block.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// The index of the pallet that the event originated from.
    pub fn pallet_index(&self) -> u8 {
        // Note: never panics; we expect these bytes to exist
        // in order that the EventDetails could be created.
        self.all_bytes[self.event_fields_start_idx - 2]
    }

    /// The index of the event variant that the event originated from.
    pub fn variant_index(&self) -> u8 {
        // Note: never panics; we expect these bytes to exist
        // in order that the EventDetails could be created.
        self.all_bytes[self.event_fields_start_idx - 1]
    }

    /// The name of the pallet from whence the Event originated.
    pub fn pallet_name(&self) -> &str {
        self.event_metadata().pallet.name()
    }

    /// The name of the event (ie the name of the variant that it corresponds to).
    pub fn variant_name(&self) -> &str {
        &self.event_metadata().variant.name
    }

    /// Fetch details from the metadata for this event.
    pub fn event_metadata(&self) -> EventMetadataDetails {
        let pallet = self
            .metadata
            .pallet_by_index(self.pallet_index())
            .expect("event pallet to be found; we did this already during decoding");
        let variant = pallet
            .event_variant_by_index(self.variant_index())
            .expect("event variant to be found; we did this already during decoding");

        EventMetadataDetails { pallet, variant }
    }

    /// Return _all_ of the bytes representing this event, which include, in order:
    /// - The phase.
    /// - Pallet and event index.
    /// - Event fields.
    /// - Event Topics.
    pub fn bytes(&self) -> &[u8] {
        &self.all_bytes[self.start_idx..self.end_idx]
    }

    /// Return the bytes representing the fields stored in this event.
    pub fn field_bytes(&self) -> &[u8] {
        &self.all_bytes[self.event_fields_start_idx..self.event_fields_end_idx]
    }

    /// Decode and provide the event fields back in the form of a [`scale_value::Composite`]
    /// type which represents the named or unnamed fields that were present in the event.
    pub fn field_values(&self) -> Result<scale_value::Composite<u32>, Error> {
        let bytes = &mut self.field_bytes();
        let event_metadata = self.event_metadata();

        let mut fields = event_metadata
            .variant
            .fields
            .iter()
            .map(|f| scale_decode::Field::new(f.ty.id, f.name.as_deref()));

        let decoded =
            scale_value::scale::decode_as_fields(bytes, &mut fields, self.metadata.types())?;

        Ok(decoded)
    }

    /// Attempt to decode these [`EventDetails`] into a type representing the event fields.
    /// Such types are exposed in the codegen as `pallet_name::events::EventName` types.
    pub fn as_event<E: StaticEvent>(&self) -> Result<Option<E>, Error> {
        let ev_metadata = self.event_metadata();
        if ev_metadata.pallet.name() == E::PALLET && ev_metadata.variant.name == E::EVENT {
            let mut fields = ev_metadata
                .variant
                .fields
                .iter()
                .map(|f| scale_decode::Field::new(f.ty.id, f.name.as_deref()));
            let decoded =
                E::decode_as_fields(&mut self.field_bytes(), &mut fields, self.metadata.types())?;
            Ok(Some(decoded))
        } else {
            Ok(None)
        }
    }

    /// Attempt to decode these [`EventDetails`] into a root event type (which includes
    /// the pallet and event enum variants as well as the event fields). A compatible
    /// type for this is exposed via static codegen as a root level `Event` type.
    pub fn as_root_event<E: DecodeAsType>(&self) -> Result<E, Error> {
        let bytes = &self.all_bytes[self.event_start_idx..self.event_fields_end_idx];

        let decoded = E::decode_as_type(
            &mut &bytes[..],
            self.metadata.outer_enums().event_enum_ty(),
            self.metadata.types(),
        )?;

        Ok(decoded)
    }

    /// Return the topics associated with this event.
    pub fn topics(&self) -> &[T::Hash] {
        &self.topics
    }
}

/// Details for the given event plucked from the metadata.
pub struct EventMetadataDetails<'a> {
    /// Metadata for the pallet that the event belongs to.
    pub pallet: PalletMetadata<'a>,
    /// Metadata for the variant which describes the pallet events.
    pub variant: &'a scale_info::Variant<scale_info::form::PortableForm>,
}

/// Event related test utilities used outside this module.
#[cfg(test)]
pub(crate) mod test_utils {
    use super::*;
    use crate::config::{Config, SubstrateConfig};
    use codec::Encode;
    use frame_metadata::{
        v15::{
            CustomMetadata, ExtrinsicMetadata, OuterEnums, PalletEventMetadata, PalletMetadata,
            RuntimeMetadataV15,
        },
        RuntimeMetadataPrefixed,
    };
    use scale_info::{meta_type, TypeInfo};

    /// An "outer" events enum containing exactly one event.
    #[derive(
        Encode,
        Decode,
        TypeInfo,
        Clone,
        Debug,
        PartialEq,
        Eq,
        scale_encode::EncodeAsType,
        scale_decode::DecodeAsType,
    )]
    pub enum AllEvents<Ev> {
        Test(Ev),
    }

    /// This encodes to the same format an event is expected to encode to
    /// in node System.Events storage.
    #[derive(Encode)]
    pub struct EventRecord<E: Encode> {
        phase: Phase,
        event: AllEvents<E>,
        topics: Vec<<SubstrateConfig as Config>::Hash>,
    }

    impl<E: Encode> EventRecord<E> {
        /// Create a new event record with the given phase, event, and topics.
        pub fn new(phase: Phase, event: E, topics: Vec<<SubstrateConfig as Config>::Hash>) -> Self {
            Self {
                phase,
                event: AllEvents::Test(event),
                topics,
            }
        }
    }

    /// Build an EventRecord, which encoded events in the format expected
    /// to be handed back from storage queries to System.Events.
    pub fn event_record<E: Encode>(phase: Phase, event: E) -> EventRecord<E> {
        EventRecord::new(phase, event, vec![])
    }

    /// Build fake metadata consisting of a single pallet that knows
    /// about the event type provided.
    pub fn metadata<E: TypeInfo + 'static>() -> Metadata {
        // Extrinsic needs to contain at least the generic type parameter "Call"
        // for the metadata to be valid.
        // The "Call" type from the metadata is used to decode extrinsics.
        // In reality, the extrinsic type has "Call", "Address", "Extra", "Signature" generic types.
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct ExtrinsicType<Call> {
            call: Call,
        }
        // Because this type is used to decode extrinsics, we expect this to be a TypeDefVariant.
        // Each pallet must contain one single variant.
        #[allow(unused)]
        #[derive(TypeInfo)]
        enum RuntimeCall {
            PalletName(Pallet),
        }
        // The calls of the pallet.
        #[allow(unused)]
        #[derive(TypeInfo)]
        enum Pallet {
            #[allow(unused)]
            SomeCall,
        }

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
            docs: vec![],
        }];

        let extrinsic = ExtrinsicMetadata {
            version: 0,
            signed_extensions: vec![],
            address_ty: meta_type::<()>(),
            call_ty: meta_type::<RuntimeCall>(),
            signature_ty: meta_type::<()>(),
            extra_ty: meta_type::<()>(),
        };

        let meta = RuntimeMetadataV15::new(
            pallets,
            extrinsic,
            meta_type::<()>(),
            vec![],
            OuterEnums {
                call_enum_ty: meta_type::<()>(),
                event_enum_ty: meta_type::<AllEvents<E>>(),
                error_enum_ty: meta_type::<()>(),
            },
            CustomMetadata {
                map: Default::default(),
            },
        );
        let runtime_metadata: RuntimeMetadataPrefixed = meta.into();
        let metadata: subxt_metadata::Metadata = runtime_metadata.try_into().unwrap();

        Metadata::from(metadata)
    }

    /// Build an `Events` object for test purposes, based on the details provided,
    /// and with a default block hash.
    pub fn events<E: Decode + Encode>(
        metadata: Metadata,
        event_records: Vec<EventRecord<E>>,
    ) -> Events<SubstrateConfig> {
        let num_events = event_records.len() as u32;
        let mut event_bytes = Vec::new();
        for ev in event_records {
            ev.encode_to(&mut event_bytes);
        }
        events_raw(metadata, event_bytes, num_events)
    }

    /// Much like [`events`], but takes pre-encoded events and event count, so that we can
    /// mess with the bytes in tests if we need to.
    pub fn events_raw(
        metadata: Metadata,
        event_bytes: Vec<u8>,
        num_events: u32,
    ) -> Events<SubstrateConfig> {
        // Prepend compact encoded length to event bytes:
        let mut all_event_bytes = Compact(num_events).encode();
        all_event_bytes.extend(event_bytes);
        Events::decode_from(all_event_bytes, metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_utils::{event_record, events, events_raw, AllEvents, EventRecord},
        *,
    };
    use crate::config::SubstrateConfig;
    use crate::events::Phase;
    use codec::Encode;
    use primitive_types::H256;
    use scale_info::TypeInfo;
    use scale_value::Value;

    /// Build a fake wrapped metadata.
    fn metadata<E: TypeInfo + 'static>() -> Metadata {
        test_utils::metadata::<E>()
    }

    /// [`RawEventDetails`] can be annoying to test, because it contains
    /// type info in the decoded field Values. Strip that here so that
    /// we can compare fields more easily.
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct TestRawEventDetails {
        pub phase: Phase,
        pub index: u32,
        pub pallet: String,
        pub pallet_index: u8,
        pub variant: String,
        pub variant_index: u8,
        pub fields: Vec<scale_value::Value>,
    }

    /// Compare some actual [`RawEventDetails`] with a hand-constructed
    /// (probably) [`TestRawEventDetails`].
    pub fn assert_raw_events_match(
        actual: EventDetails<SubstrateConfig>,
        expected: TestRawEventDetails,
    ) {
        let actual_fields_no_context: Vec<_> = actual
            .field_values()
            .expect("can decode field values (2)")
            .into_values()
            .map(|value| value.remove_context())
            .collect();

        // Check each of the other fields:
        assert_eq!(actual.phase(), expected.phase);
        assert_eq!(actual.index(), expected.index);
        assert_eq!(actual.pallet_name(), expected.pallet);
        assert_eq!(actual.pallet_index(), expected.pallet_index);
        assert_eq!(actual.variant_name(), expected.variant);
        assert_eq!(actual.variant_index(), expected.variant_index);
        assert_eq!(actual_fields_no_context, expected.fields);
    }

    #[test]
    fn statically_decode_single_root_event() {
        #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo, scale_decode::DecodeAsType)]
        enum Event {
            A(u8, bool, Vec<String>),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let event = Event::A(1, true, vec!["Hi".into()]);
        let events = events::<Event>(
            metadata,
            vec![event_record(Phase::ApplyExtrinsic(123), event.clone())],
        );

        let ev = events
            .iter()
            .next()
            .expect("one event expected")
            .expect("event should be extracted OK");

        // This is the line we're testing:
        let decoded_event = ev
            .as_root_event::<AllEvents<Event>>()
            .expect("can decode event into root enum again");

        // It should equal the event we put in:
        assert_eq!(decoded_event, AllEvents::Test(event));
    }

    #[test]
    fn dynamically_decode_single_event() {
        #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
        enum Event {
            A(u8, bool, Vec<String>),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let event = Event::A(1, true, vec!["Hi".into()]);
        let events = events::<Event>(
            metadata,
            vec![event_record(Phase::ApplyExtrinsic(123), event)],
        );

        let mut event_details = events.iter();
        assert_raw_events_match(
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                phase: Phase::ApplyExtrinsic(123),
                index: 0,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![
                    Value::u128(1),
                    Value::bool(true),
                    Value::unnamed_composite(vec![Value::string("Hi")]),
                ],
            },
        );
        assert!(event_details.next().is_none());
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
            metadata,
            vec![
                event_record(Phase::Initialization, event1),
                event_record(Phase::ApplyExtrinsic(123), event2),
                event_record(Phase::Finalization, event3),
            ],
        );

        let mut event_details = events.iter();

        assert_raw_events_match(
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 0,
                phase: Phase::Initialization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::u128(1)],
            },
        );
        assert_raw_events_match(
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 1,
                phase: Phase::ApplyExtrinsic(123),
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "B".to_string(),
                variant_index: 1,
                fields: vec![Value::bool(true)],
            },
        );
        assert_raw_events_match(
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 2,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::u128(234)],
            },
        );
        assert!(event_details.next().is_none());
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
        event_record(Phase::ApplyExtrinsic(123), Event::B(true)).encode_to(&mut event_bytes);

        // Push a few naff bytes to the end (a broken third event):
        event_bytes.extend_from_slice(&[3, 127, 45, 0, 2]);

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let events = events_raw(
            metadata,
            event_bytes,
            3, // 2 "good" events, and then it'll hit the naff bytes.
        );

        let mut events_iter = events.iter();
        assert_raw_events_match(
            events_iter.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 0,
                phase: Phase::Initialization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::u128(1)],
            },
        );
        assert_raw_events_match(
            events_iter.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 1,
                phase: Phase::ApplyExtrinsic(123),
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "B".to_string(),
                variant_index: 1,
                fields: vec![Value::bool(true)],
            },
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
            metadata,
            vec![event_record(Phase::Finalization, Event::A(1))],
        );

        // Dynamically decode:
        let mut event_details = events.iter();
        assert_raw_events_match(
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 0,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::u128(1)],
            },
        );
        assert!(event_details.next().is_none());
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
            metadata,
            vec![event_record(
                Phase::Finalization,
                Event::A(CompactWrapper(1)),
            )],
        );

        // Dynamically decode:
        let mut event_details = events.iter();
        assert_raw_events_match(
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 0,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::unnamed_composite(vec![Value::u128(1)])],
            },
        );
        assert!(event_details.next().is_none());
    }

    #[test]
    fn event_containing_explicit_index() {
        #[derive(Clone, Debug, PartialEq, Eq, Decode, Encode, TypeInfo)]
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
            metadata,
            vec![event_record(Phase::Finalization, Event::A(MyType::B))],
        );

        // Dynamically decode:
        let mut event_details = events.iter();
        assert_raw_events_match(
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 0,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::unnamed_variant("B", vec![])],
            },
        );
        assert!(event_details.next().is_none());
    }

    #[test]
    fn topics() {
        #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo, scale_decode::DecodeAsType)]
        enum Event {
            A(u8, bool, Vec<String>),
        }

        // Create fake metadata that knows about our single event, above:
        let metadata = metadata::<Event>();

        // Encode our events in the format we expect back from a node, and
        // construct an Events object to iterate them:
        let event = Event::A(1, true, vec!["Hi".into()]);
        let topics = vec![H256::from_low_u64_le(123), H256::from_low_u64_le(456)];
        let events = events::<Event>(
            metadata,
            vec![EventRecord::new(
                Phase::ApplyExtrinsic(123),
                event,
                topics.clone(),
            )],
        );

        let ev = events
            .iter()
            .next()
            .expect("one event expected")
            .expect("event should be extracted OK");

        assert_eq!(topics, ev.topics());
    }
}
