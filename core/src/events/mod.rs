use alloc::sync::Arc;
use codec::{Decode, Encode};
use scale_decode::{DecodeAsFields, DecodeAsType};
use subxt_metadata::PalletMetadata;

use crate::{Config, Error, Metadata, MetadataError};

use alloc::vec::Vec;

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
    pub fn decode_from(
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
                &field_metadata.ty.id,
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
        self.event_metadata().pallet().name()
    }

    /// The name of the event (ie the name of the variant that it corresponds to).
    pub fn variant_name(&self) -> &str {
        &self.event_metadata().variant().name
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
            .map(|f| scale_decode::Field::new(&f.ty.id, f.name.as_deref()));

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
                .map(|f| scale_decode::Field::new(&f.ty.id, f.name.as_deref()));
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
            &self.metadata.outer_enums().event_enum_ty(),
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
    pallet: PalletMetadata<'a>,
    variant: &'a scale_info::Variant<scale_info::form::PortableForm>,
}

impl<'a> EventMetadataDetails<'a> {
    pub fn pallet(&self) -> PalletMetadata<'a> {
        self.pallet
    }
    pub fn variant(&self) -> &'a scale_info::Variant<scale_info::form::PortableForm> {
        self.variant
    }
}
