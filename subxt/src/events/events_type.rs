// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A representation of a block of events.

use super::{
    Phase,
    StaticEvent,
};
use crate::{
    error::Error,
    Config,
    Metadata,
};
use codec::{
    Compact,
    Decode,
    Error as CodecError,
    Input,
};
use derivative::Derivative;
use std::sync::Arc;

/// A collection of events obtained from a block, bundled with the necessary
/// information needed to decode and iterate over them.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct Events<T: Config> {
    metadata: Metadata,
    block_hash: T::Hash,
    // Note; raw event bytes are prefixed with a Compact<u32> containing
    // the number of events to be decoded. We should have stripped that off
    // before storing the bytes here.
    event_bytes: Arc<[u8]>,
    num_events: u32,
}

impl<T: Config> Events<T> {
    pub(crate) fn new(
        metadata: Metadata,
        block_hash: T::Hash,
        mut event_bytes: Vec<u8>,
    ) -> Self {
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

        Self {
            metadata,
            block_hash,
            event_bytes: event_bytes.into(),
            num_events,
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

    /// Return the block hash that these events are from.
    pub fn block_hash(&self) -> T::Hash {
        self.block_hash
    }

    /// Iterate over all of the events, using metadata to dynamically
    /// decode them as we go, and returning the raw bytes and other associated
    /// details. If an error occurs, all subsequent iterations return `None`.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<EventDetails, Error>> + Send + Sync + 'static {
        let event_bytes = self.event_bytes.clone();
        let num_events = self.num_events;

        let metadata = self.metadata.clone();
        let mut pos = 0;
        let mut index = 0;
        std::iter::from_fn(move || {
            let cursor = &mut &event_bytes[pos..];
            let start_len = cursor.len();

            if start_len == 0 || num_events == index {
                None
            } else {
                match decode_raw_event_details::<T>(&metadata, index, cursor) {
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

    /// Find an event that decodes to the type provided. Returns true if it was found.
    pub fn has<Ev: StaticEvent>(&self) -> Result<bool, Error> {
        Ok(self.find::<Ev>().next().transpose()?.is_some())
    }
}

/// The raw bytes for an event with associated details about
/// where and when it was emitted.
#[derive(Debug, Clone, PartialEq)]
pub struct EventDetails {
    phase: Phase,
    index: u32,
    pallet: String,
    variant: String,
    bytes: Vec<u8>,
    fields: Vec<scale_value::Value<scale_value::scale::TypeId>>,
}

impl EventDetails {
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
        self.bytes[0]
    }

    /// The index of the event variant that the event originated from.
    pub fn variant_index(&self) -> u8 {
        self.bytes[1]
    }

    /// The name of the pallet from whence the Event originated.
    pub fn pallet_name(&self) -> &str {
        &self.pallet
    }

    /// The name of the pallet's Event variant.
    pub fn variant_name(&self) -> &str {
        &self.variant
    }

    /// Return the bytes representing this event, which include the pallet
    /// and variant index that the event originated from.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the bytes representing the fields stored in this event.
    pub fn field_bytes(&self) -> &[u8] {
        &self.bytes[2..]
    }

    /// Decode and provide the event fields back in the form of a
    /// list of [`scale_value::Value`]s.
    // Dev note: if we can optimise Value decoding to avoid allocating
    // while working through events, or if the event structure changes
    // to allow us to skip over them, we'll no longer keep a copy of the
    // decoded events in the event, and the actual decoding will happen
    // when this method is called.
    pub fn field_values(&self) -> Vec<scale_value::Value<scale_value::scale::TypeId>> {
        self.fields.clone()
    }

    /// Attempt to decode these [`EventDetails`] into a specific static event.
    /// This targets the fields within the event directly. You can also attempt to
    /// decode the entirety of the event type (including the pallet and event
    /// variants) using [`EventDetails::as_root_event()`].
    pub fn as_event<E: StaticEvent>(&self) -> Result<Option<E>, CodecError> {
        if self.pallet == E::PALLET && self.variant == E::EVENT {
            Ok(Some(E::decode(&mut &self.bytes[2..])?))
        } else {
            Ok(None)
        }
    }

    /// Attempt to decode these [`EventDetails`] into a root event type (which includes
    /// the pallet and event enum variants as well as the event fields). A compatible
    /// type for this is exposed via static codegen as a root level `Event` type.
    pub fn as_root_event<E: Decode>(&self) -> Result<E, CodecError> {
        E::decode(&mut &self.bytes[..])
    }
}

// Attempt to dynamically decode a single event from our events input.
fn decode_raw_event_details<T: Config>(
    metadata: &Metadata,
    index: u32,
    input: &mut &[u8],
) -> Result<EventDetails, Error> {
    // Decode basic event details:
    let phase = Phase::decode(input)?;
    let pallet_index = input.read_byte()?;
    let variant_index = input.read_byte()?;
    tracing::debug!(
        "phase {:?}, pallet_index {}, event_variant: {}",
        phase,
        pallet_index,
        variant_index
    );
    tracing::debug!("remaining input: {}", hex::encode(&input));

    // Get metadata for the event:
    let event_metadata = metadata.event(pallet_index, variant_index)?;
    tracing::debug!(
        "Decoding Event '{}::{}'",
        event_metadata.pallet(),
        event_metadata.event()
    );

    // Use metadata to figure out which bytes belong to this event.
    // the event bytes also include the pallet/variant index so that, if we
    // like, we can decode them quite easily into a top level event type.
    let mut event_bytes = vec![pallet_index, variant_index];
    let mut event_fields = Vec::new();
    for arg in event_metadata.variant().fields() {
        let type_id = arg.ty().id();
        let all_bytes = *input;
        // consume some bytes for each event field, moving the cursor forward:
        let value = scale_value::scale::decode_as_type(
            input,
            type_id,
            &metadata.runtime_metadata().types,
        )?;
        event_fields.push(value);
        // count how many bytes were consumed based on remaining length:
        let consumed_len = all_bytes.len() - input.len();
        // move those consumed bytes to the output vec unaltered:
        event_bytes.extend(&all_bytes[0..consumed_len]);
    }

    // topics come after the event data in EventRecord. They aren't used for
    // anything at the moment, so just decode and throw them away.
    let topics = Vec::<T::Hash>::decode(input)?;
    tracing::debug!("topics: {:?}", topics);

    Ok(EventDetails {
        phase,
        index,
        pallet: event_metadata.pallet().to_string(),
        variant: event_metadata.event().to_string(),
        bytes: event_bytes,
        fields: event_fields,
    })
}

/// Event related test utilities used outside this module.
#[cfg(test)]
pub(crate) mod test_utils {
    use super::*;
    use crate::{
        Config,
        SubstrateConfig,
    };
    use codec::Encode;
    use frame_metadata::{
        v14::{
            ExtrinsicMetadata,
            PalletEventMetadata,
            PalletMetadata,
            RuntimeMetadataV14,
        },
        RuntimeMetadataPrefixed,
    };
    use scale_info::{
        meta_type,
        TypeInfo,
    };
    use std::convert::TryFrom;

    /// An "outer" events enum containing exactly one event.
    #[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq)]
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

    /// Build an EventRecord, which encoded events in the format expected
    /// to be handed back from storage queries to System.Events.
    pub fn event_record<E: Encode>(phase: Phase, event: E) -> EventRecord<E> {
        EventRecord {
            phase,
            event: AllEvents::Test(event),
            topics: vec![],
        }
    }

    /// Build fake metadata consisting of a single pallet that knows
    /// about the event type provided.
    pub fn metadata<E: TypeInfo + 'static>() -> Metadata {
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

        let v14 = RuntimeMetadataV14::new(pallets, extrinsic, meta_type::<()>());
        let runtime_metadata: RuntimeMetadataPrefixed = v14.into();

        Metadata::try_from(runtime_metadata).unwrap()
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
        Events {
            block_hash: <SubstrateConfig as Config>::Hash::default(),
            event_bytes: event_bytes.into(),
            metadata,
            num_events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_utils::{
            event_record,
            events,
            events_raw,
        },
        *,
    };
    use codec::Encode;
    use scale_info::TypeInfo;
    use scale_value::Value;

    /// Build a fake wrapped metadata.
    fn metadata<E: TypeInfo + 'static>() -> Metadata {
        test_utils::metadata::<E>()
    }

    /// [`RawEventDetails`] can be annoying to test, because it contains
    /// type info in the decoded field Values. Strip that here so that
    /// we can compare fields more easily.
    #[derive(Debug, PartialEq, Clone)]
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
        // Just for convenience, pass in the metadata type constructed
        // by the `metadata` function above to simplify caller code.
        metadata: &Metadata,
        actual: EventDetails,
        expected: TestRawEventDetails,
    ) {
        let types = &metadata.runtime_metadata().types;

        // Make sure that the bytes handed back line up with the fields handed back;
        // encode the fields back into bytes and they should be equal.
        let mut actual_bytes = vec![];
        for field in &actual.fields {
            scale_value::scale::encode_as_type(
                field.clone(),
                field.context,
                types,
                &mut actual_bytes,
            )
            .expect("should be able to encode properly");
        }
        assert_eq!(actual_bytes, actual.bytes);

        let actual_fields_no_context: Vec<_> = actual
            .field_values()
            .into_iter()
            .map(|f| f.remove_context())
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
            metadata.clone(),
            vec![event_record(Phase::ApplyExtrinsic(123), event)],
        );

        let mut event_details = events.iter();
        assert_raw_events_match(
            &metadata,
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                phase: Phase::ApplyExtrinsic(123),
                index: 0,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![
                    Value::uint(1u8),
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
            metadata.clone(),
            vec![
                event_record(Phase::Initialization, event1),
                event_record(Phase::ApplyExtrinsic(123), event2),
                event_record(Phase::Finalization, event3),
            ],
        );

        let mut event_details = events.iter();

        assert_raw_events_match(
            &metadata,
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 0,
                phase: Phase::Initialization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::uint(1u8)],
            },
        );
        assert_raw_events_match(
            &metadata,
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
            &metadata,
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 2,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::uint(234u8)],
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
        event_record(Phase::ApplyExtrinsic(123), Event::B(true))
            .encode_to(&mut event_bytes);

        // Push a few naff bytes to the end (a broken third event):
        event_bytes.extend_from_slice(&[3, 127, 45, 0, 2]);

        // Encode our events in the format we expect back from a node, and
        // construst an Events object to iterate them:
        let events = events_raw(
            metadata.clone(),
            event_bytes,
            3, // 2 "good" events, and then it'll hit the naff bytes.
        );

        let mut events_iter = events.iter();
        assert_raw_events_match(
            &metadata,
            events_iter.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 0,
                phase: Phase::Initialization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::uint(1u8)],
            },
        );
        assert_raw_events_match(
            &metadata,
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
            metadata.clone(),
            vec![event_record(Phase::Finalization, Event::A(1))],
        );

        // Dynamically decode:
        let mut event_details = events.iter();
        assert_raw_events_match(
            &metadata,
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 0,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::uint(1u8)],
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
            metadata.clone(),
            vec![event_record(
                Phase::Finalization,
                Event::A(CompactWrapper(1)),
            )],
        );

        // Dynamically decode:
        let mut event_details = events.iter();
        assert_raw_events_match(
            &metadata,
            event_details.next().unwrap().unwrap(),
            TestRawEventDetails {
                index: 0,
                phase: Phase::Finalization,
                pallet: "Test".to_string(),
                pallet_index: 0,
                variant: "A".to_string(),
                variant_index: 0,
                fields: vec![Value::unnamed_composite(vec![Value::uint(1u8)])],
            },
        );
        assert!(event_details.next().is_none());
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
            metadata.clone(),
            vec![event_record(Phase::Finalization, Event::A(MyType::B))],
        );

        // Dynamically decode:
        let mut event_details = events.iter();
        assert_raw_events_match(
            &metadata,
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
}
