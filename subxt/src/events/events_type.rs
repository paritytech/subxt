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

//! A representation of a block of events.

use crate::{
    error::BasicError,
    Client,
    Config,
    Event,
    Metadata,
    Phase,
};
use codec::{
    Compact,
    Decode,
    Error as CodecError,
    Input,
};
use derivative::Derivative;
use parking_lot::RwLock;
use sp_core::{
    storage::StorageKey,
    twox_128,
};
use std::sync::Arc;

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
) -> Result<Events<T, Evs>, BasicError> {
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

// The storage key needed to access events.
fn system_events_key() -> StorageKey {
    let mut storage_key = twox_128(b"System").to_vec();
    storage_key.extend(twox_128(b"Events").to_vec());
    StorageKey(storage_key)
}

/// A collection of events obtained from a block, bundled with the necessary
/// information needed to decode and iterate over them.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct Events<T: Config, Evs> {
    metadata: Arc<RwLock<Metadata>>,
    block_hash: T::Hash,
    // Note; raw event bytes are prefixed with a Compact<u32> containing
    // the number of events to be decoded. We should have stripped that off
    // before storing the bytes here.
    event_bytes: Vec<u8>,
    num_events: u32,
    _event_type: std::marker::PhantomData<Evs>,
}

impl<'a, T: Config, Evs: Decode> Events<T, Evs> {
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

        let metadata = {
            let metadata = self.metadata.read();
            metadata.clone()
        };

        let mut pos = 0;
        let mut index = 0;
        std::iter::from_fn(move || {
            let cursor = &mut &event_bytes[pos..];
            let start_len = cursor.len();

            if start_len == 0 || self.num_events == index {
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

    /// Iterate over all of the events, using metadata to dynamically
    /// decode them as we go, and returning the raw bytes and other associated
    /// details. If an error occurs, all subsequent iterations return `None`.
    ///
    /// This method is safe to use even if you do not statically know about
    /// all of the possible events; it splits events up using the metadata
    /// obtained at runtime, which does.
    ///
    /// Unlike [`Events::iter_raw()`] this consumes `self`, which can be useful
    /// if you need to store the iterator somewhere and avoid lifetime issues.
    pub fn into_iter_raw(
        self,
    ) -> impl Iterator<Item = Result<RawEventDetails, BasicError>> + 'a {
        let mut pos = 0;
        let mut index = 0;
        let metadata = {
            let metadata = self.metadata.read();
            metadata.clone()
        };

        std::iter::from_fn(move || {
            let cursor = &mut &self.event_bytes[pos..];
            let start_len = cursor.len();

            if start_len == 0 || self.num_events == index {
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
                        pos = self.event_bytes.len();
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
    pub fn find_first<Ev: Event>(&self) -> Result<Option<Ev>, BasicError> {
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

/// A Value which has been decoded from some raw bytes.
pub type DecodedValue = scale_value::Value<scale_value::scale::TypeId>;

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
    /// The bytes representing the fields contained within the event.
    pub bytes: Vec<u8>,
    /// Generic values representing each field of the event.
    pub fields: Vec<DecodedValue>,
}

impl RawEventDetails {
    /// Attempt to decode this [`RawEventDetails`] into a specific event.
    pub fn as_event<E: Event>(&self) -> Result<Option<E>, CodecError> {
        if self.pallet == E::PALLET && self.variant == E::EVENT {
            Ok(Some(E::decode(&mut &self.bytes[..])?))
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

    // Use metadata to figure out which bytes belong to this event:
    let mut event_bytes = Vec::new();
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

    Ok(RawEventDetails {
        phase,
        index,
        pallet_index,
        pallet: event_metadata.pallet().to_string(),
        variant_index,
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
        DefaultConfig,
        Phase,
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
        topics: Vec<<DefaultConfig as Config>::Hash>,
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
        metadata: Arc<RwLock<Metadata>>,
        event_records: Vec<EventRecord<E>>,
    ) -> Events<DefaultConfig, AllEvents<E>> {
        let num_events = event_records.len() as u32;
        let mut event_bytes = Vec::new();
        for ev in event_records {
            ev.encode_to(&mut event_bytes);
        }
        events_raw(metadata, event_bytes, num_events)
    }

    /// Much like [`events`], but takes pre-encoded events and event count, so that we can
    /// mess with the bytes in tests if we need to.
    pub fn events_raw<E: Decode + Encode>(
        metadata: Arc<RwLock<Metadata>>,
        event_bytes: Vec<u8>,
        num_events: u32,
    ) -> Events<DefaultConfig, AllEvents<E>> {
        Events {
            block_hash: <DefaultConfig as Config>::Hash::default(),
            event_bytes,
            metadata,
            num_events,
            _event_type: std::marker::PhantomData,
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
            AllEvents,
        },
        *,
    };
    use crate::Phase;
    use codec::Encode;
    use scale_info::TypeInfo;
    use scale_value::Value;

    /// Build a fake wrapped metadata.
    fn metadata<E: TypeInfo + 'static>() -> Arc<RwLock<Metadata>> {
        Arc::new(RwLock::new(test_utils::metadata::<E>()))
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
        metadata: &Arc<RwLock<Metadata>>,
        actual: RawEventDetails,
        expected: TestRawEventDetails,
    ) {
        let metadata = metadata.read();
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
            .fields
            .into_iter()
            .map(|f| f.remove_context())
            .collect();

        // Check each of the other fields:
        assert_eq!(actual.phase, expected.phase);
        assert_eq!(actual.index, expected.index);
        assert_eq!(actual.pallet, expected.pallet);
        assert_eq!(actual.pallet_index, expected.pallet_index);
        assert_eq!(actual.variant, expected.variant);
        assert_eq!(actual.variant_index, expected.variant_index);
        assert_eq!(actual_fields_no_context, expected.fields);
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
        // construct an Events object to iterate them:
        let events = events::<Event>(
            metadata,
            vec![event_record(Phase::Finalization, Event::A(1))],
        );

        let event_details: Vec<EventDetails<AllEvents<Event>>> =
            events.iter().collect::<Result<_, _>>().unwrap();
        assert_eq!(
            event_details,
            vec![EventDetails {
                index: 0,
                phase: Phase::Finalization,
                event: AllEvents::Test(Event::A(1))
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
            metadata,
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
                    event: AllEvents::Test(Event::A(1))
                },
                EventDetails {
                    index: 1,
                    phase: Phase::ApplyExtrinsic(123),
                    event: AllEvents::Test(Event::B(true))
                },
                EventDetails {
                    index: 2,
                    phase: Phase::Finalization,
                    event: AllEvents::Test(Event::A(234))
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
            metadata,
            event_bytes,
            3, // 2 "good" events, and then it'll hit the naff bytes.
        );

        let mut events_iter = events.iter();
        assert_eq!(
            events_iter.next().unwrap().unwrap(),
            EventDetails {
                index: 0,
                phase: Phase::Initialization,
                event: AllEvents::Test(Event::A(1))
            }
        );
        assert_eq!(
            events_iter.next().unwrap().unwrap(),
            EventDetails {
                index: 1,
                phase: Phase::ApplyExtrinsic(123),
                event: AllEvents::Test(Event::B(true))
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

        let mut event_details = events.iter_raw();
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

        let mut event_details = events.iter_raw();

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
        let events = events_raw::<Event>(
            metadata.clone(),
            event_bytes,
            3, // 2 "good" events, and then it'll hit the naff bytes.
        );

        let mut events_iter = events.iter_raw();
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

        // Statically decode:
        let event_details: Vec<EventDetails<AllEvents<Event>>> =
            events.iter().collect::<Result<_, _>>().unwrap();
        assert_eq!(
            event_details,
            vec![EventDetails {
                index: 0,
                phase: Phase::Finalization,
                event: AllEvents::Test(Event::A(1))
            }]
        );

        // Dynamically decode:
        let mut event_details = events.iter_raw();
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

        // Statically decode:
        let event_details: Vec<EventDetails<AllEvents<Event>>> =
            events.iter().collect::<Result<_, _>>().unwrap();
        assert_eq!(
            event_details,
            vec![EventDetails {
                index: 0,
                phase: Phase::Finalization,
                event: AllEvents::Test(Event::A(CompactWrapper(1)))
            }]
        );

        // Dynamically decode:
        let mut event_details = events.iter_raw();
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

        // Statically decode:
        let event_details: Vec<EventDetails<AllEvents<Event>>> =
            events.iter().collect::<Result<_, _>>().unwrap();
        assert_eq!(
            event_details,
            vec![EventDetails {
                index: 0,
                phase: Phase::Finalization,
                event: AllEvents::Test(Event::A(MyType::B))
            }]
        );

        // Dynamically decode:
        let mut event_details = events.iter_raw();
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
