//! This module exposes [`EventsClient`], which has methods for working with eventts.
//! It's created by calling [`crate::client::ClientAtBlock::events()`].
//!
//! ```rust,no_run
//! pub use subxt::{OnlineClient, PolkadotConfig};
//!
//! let client = OnlineClient::new().await?;
//! let at_block = client.at_current_block().await?;
//!
//! let events = at_block.events();
//! ```

mod decode_as_event;

use crate::backend::BackendExt;
use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::{Config, HashFor};
use crate::error::EventsError;
use crate::{ArcMetadata, Metadata};
use codec::{Compact, Decode, Encode};
use scale_decode::{DecodeAsFields, DecodeAsType};
use std::marker::PhantomData;
use std::sync::Arc;

pub use decode_as_event::DecodeAsEvent;

/// A client for working with events. See [the module docs](crate::events) for more.
pub struct EventsClient<'atblock, T, Client> {
    client: &'atblock Client,
    marker: PhantomData<T>,
}

impl<'atblock, T, Client> EventsClient<'atblock, T, Client> {
    pub(crate) fn new(client: &'atblock Client) -> Self {
        EventsClient {
            client,
            marker: PhantomData,
        }
    }
}

impl<'atblock, T: Config, Client: OfflineClientAtBlockT<T>> EventsClient<'atblock, T, Client> {
    /// Work with the block event bytes given.
    ///
    /// No attempt to validate the provided bytes is made here; if invalid bytes are
    /// provided then attempting to iterate and decode them will fail.
    pub fn from_bytes(&self, event_bytes: Vec<u8>) -> Events<T> {
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

        Events {
            metadata: self.client.metadata(),
            event_bytes: event_bytes.into(),
            start_idx,
            num_events,
            marker: PhantomData,
        }
    }
}

impl<'atblock, T: Config, Client: OnlineClientAtBlockT<T>> EventsClient<'atblock, T, Client> {
    /// Fetch the events at this block.
    pub async fn fetch(&self) -> Result<Events<T>, EventsError> {
        let client = self.client;

        // Fetch the bytes. Ensure things work if we get 0 bytes back.
        let block_hash = client.block_hash();
        let event_bytes = client
            .backend()
            .storage_fetch_value(system_events_key().to_vec(), block_hash)
            .await
            .map_err(EventsError::CannotFetchEventBytes)?
            .unwrap_or_default();

        Ok(self.from_bytes(event_bytes))
    }
}

/// The events at some block.
// Dev note [jsdw]:
//  It would be nice if this borrowed &'atblock Metadata, to be
//  consistent with many other things and allow longer lifetimes
//  on a couple of bits, but we need to construct this from transaction
//  things and can't provide lifetimes from there.
#[derive(Debug)]
pub struct Events<T> {
    metadata: ArcMetadata,
    // Note; raw event bytes are prefixed with a Compact<u32> containing
    // the number of events to be decoded. The start_idx reflects that, so
    // that we can skip over those bytes when decoding them
    event_bytes: Arc<[u8]>,
    start_idx: usize,
    num_events: u32,
    marker: core::marker::PhantomData<T>,
}

impl<T: Config> Events<T> {
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
    pub fn iter(&'_ self) -> impl Iterator<Item = Result<Event<'_, T>, EventsError>> + Send + Sync {
        // The event bytes ignoring the compact encoded length on the front:
        let event_bytes = self.event_bytes.clone();
        let metadata = &*self.metadata;
        let num_events = self.num_events;

        let mut pos = self.start_idx;
        let mut index = 0;
        core::iter::from_fn(move || {
            if event_bytes.len() <= pos || num_events == index {
                None
            } else {
                match Event::decode_from(metadata, event_bytes.clone(), pos, index) {
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

    /// Iterate through the events, Decoding and returning any that match the given type.
    ///
    /// This is a convenience function for calling [`Self::iter`] and then [`Event::decode_call_data_fields_as`]
    /// on each event that we iterate over, filtering those that don't match.
    pub fn find<E: DecodeAsEvent>(&self) -> impl Iterator<Item = Result<E, EventsError>> {
        self.iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.decode_fields_as::<E>())
    }

    /// Find the first event matching the given type, returning `None` if it doesn't exist,
    /// and the result of decoding it if it does.
    pub fn find_first<E: DecodeAsEvent>(&self) -> Option<Result<E, EventsError>> {
        self.find::<E>().next()
    }

    /// Find an event matching the given type, returning true if it exists. This function does _not_
    /// try to actually decode the event bytes into the given type.
    pub fn has<E: DecodeAsEvent>(&self) -> bool {
        self.iter().filter_map(|e| e.ok()).any(|e| e.is::<E>())
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
pub struct Event<'events, T: Config> {
    pallet_name: &'events str,
    event_name: &'events str,
    metadata: &'events Metadata,
    // all of the event bytes (not just this one).
    all_bytes: Arc<[u8]>,
    // event phase.
    phase: Phase,
    /// The index of the event in the list of events in a given block.
    index: u32,
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
    // event topics.
    topics: Vec<HashFor<T>>,
}

impl<'events, T: Config> Event<'events, T> {
    /// Attempt to dynamically decode a single event from our events input.
    fn decode_from(
        metadata: &'events Metadata,
        all_bytes: Arc<[u8]>,
        start_idx: usize,
        index: u32,
    ) -> Result<Event<'events, T>, EventsError> {
        let input = &mut &all_bytes[start_idx..];

        let phase = Phase::decode(input).map_err(EventsError::CannotDecodePhase)?;

        let event_start_idx = all_bytes.len() - input.len();

        let pallet_index = u8::decode(input).map_err(EventsError::CannotDecodePalletIndex)?;
        let variant_index = u8::decode(input).map_err(EventsError::CannotDecodeVariantIndex)?;

        let event_fields_start_idx = all_bytes.len() - input.len();

        // Get metadata for the event:
        let event_pallet = metadata
            .pallet_by_event_index(pallet_index)
            .ok_or_else(|| EventsError::CannotFindPalletWithIndex(pallet_index))?;
        let event_variant = event_pallet
            .event_variant_by_index(variant_index)
            .ok_or_else(|| EventsError::CannotFindVariantWithIndex {
                pallet_name: event_pallet.name().to_string(),
                variant_index,
            })?;

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
            .map_err(|e| EventsError::CannotDecodeFieldInEvent {
                pallet_name: event_pallet.name().to_string(),
                event_name: event_variant.name.clone(),
                field_name: field_metadata
                    .name
                    .clone()
                    .unwrap_or("<unknown>".to_string()),
                reason: e,
            })?;
        }

        // the end of the field bytes.
        let event_fields_end_idx = all_bytes.len() - input.len();

        // topics come after the event data in EventRecord.
        let topics =
            Vec::<HashFor<T>>::decode(input).map_err(EventsError::CannotDecodeEventTopics)?;

        // what bytes did we skip over in total, including topics.
        let end_idx = all_bytes.len() - input.len();

        Ok(Event {
            pallet_name: event_pallet.name(),
            event_name: &event_variant.name,
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
    pub fn event_index(&self) -> u8 {
        // Note: never panics; we expect these bytes to exist
        // in order that the EventDetails could be created.
        self.all_bytes[self.event_fields_start_idx - 1]
    }

    /// The name of the pallet from whence the Event originated.
    pub fn pallet_name(&self) -> &'events str {
        self.pallet_name
    }

    /// The name of the event (ie the name of the variant that it corresponds to).
    pub fn event_name(&self) -> &'events str {
        self.event_name
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

    /// Return the topics associated with this event.
    pub fn topics(&self) -> &[HashFor<T>] {
        &self.topics
    }

    /// Return true if this [`Event`] matches the provided type.
    pub fn is<E: DecodeAsEvent>(&self) -> bool {
        E::is_event(self.pallet_name(), self.event_name())
    }

    /// Attempt to decode this [`Event`] into an outer event enum type (which includes
    /// the pallet and event enum variants as well as the event fields). One compatible
    /// type for this is exposed via static codegen as a root level `Event` type.
    pub fn decode_as<E: DecodeAsType>(&self) -> Result<E, EventsError> {
        let bytes = &self.all_bytes[self.event_start_idx..self.event_fields_end_idx];

        let decoded = E::decode_as_type(
            &mut &bytes[..],
            self.metadata.outer_enums().event_enum_ty(),
            self.metadata.types(),
        )
        .map_err(|e| {
            let md = self.event_metadata();
            EventsError::CannotDecodeEventEnum {
                pallet_name: md.pallet.name().to_string(),
                event_name: md.variant.name.clone(),
                reason: e,
            }
        })?;

        Ok(decoded)
    }

    /// Decode the event call data fields into some type which implements [`DecodeAsEvent`].
    ///
    /// Event types generated via the [`crate::subxt!`] macro implement this.
    pub fn decode_fields_as<E: DecodeAsEvent>(&self) -> Option<Result<E, EventsError>> {
        if self.is::<E>() {
            Some(self.decode_fields_unchecked_as::<E>())
        } else {
            None
        }
    }

    /// Decode the event call data fields into some type which implements [`DecodeAsFields`].
    ///
    /// This ignores the pallet and event name information, so you should check those via [`Self::pallet_name()`]
    /// and [`Self::event_name()`] to confirm that this event is the one you are intending to decode.
    ///
    /// Prefer to use [`Self::decode_call_data_fields_as`] where possible.
    pub fn decode_fields_unchecked_as<E: DecodeAsFields>(&self) -> Result<E, EventsError> {
        let bytes = &mut self.field_bytes();
        let event_metadata = self.event_metadata();

        let mut fields = event_metadata
            .variant
            .fields
            .iter()
            .map(|f| scale_decode::Field::new(f.ty.id, f.name.as_deref()));

        let decoded =
            E::decode_as_fields(bytes, &mut fields, self.metadata.types()).map_err(|e| {
                EventsError::CannotDecodeEventFields {
                    pallet_name: event_metadata.pallet.name().to_string(),
                    event_name: event_metadata.variant.name.clone(),
                    reason: e,
                }
            })?;

        Ok(decoded)
    }

    /// Fetch details from the metadata for this event. This is used for decoding but
    /// we try to avoid using it elsewhere.
    fn event_metadata(&self) -> EventMetadataDetails<'_> {
        let pallet = self
            .metadata
            .pallet_by_event_index(self.pallet_index())
            .expect("event pallet to be found; we did this already during decoding");
        let variant = pallet
            .event_variant_by_index(self.event_index())
            .expect("event variant to be found; we did this already during decoding");

        EventMetadataDetails { pallet, variant }
    }
}

// The storage key needed to access events.
fn system_events_key() -> [u8; 32] {
    let a = sp_crypto_hashing::twox_128(b"System");
    let b = sp_crypto_hashing::twox_128(b"Events");
    let mut res = [0; 32];
    res[0..16].clone_from_slice(&a);
    res[16..32].clone_from_slice(&b);
    res
}

/// Details for the given event plucked from the metadata.
struct EventMetadataDetails<'a> {
    /// Metadata for the pallet that the event belongs to.
    pub pallet: subxt_metadata::PalletMetadata<'a>,
    /// Metadata for the variant which describes the pallet events.
    pub variant: &'a scale_info::Variant<scale_info::form::PortableForm>,
}
