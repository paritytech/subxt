use crate::{
    Error, Metadata,
    config::{Config, HashFor},
};
use derive_where::derive_where;
use scale_decode::DecodeAsType;
use subxt_core::events::{EventDetails as CoreEventDetails, Events as CoreEvents};

pub use subxt_core::events::{EventMetadataDetails, Phase, StaticEvent};

/// A collection of events obtained from a block, bundled with the necessary
/// information needed to decode and iterate over them.
// Dev note: we are just wrapping the subxt_core types here to avoid leaking them
// in Subxt and map any errors into Subxt errors so that we don't have this part of the
// API returning a different error type (ie the subxt_core::Error).
#[derive_where(Clone, Debug)]
pub struct Events<T> {
    inner: CoreEvents<T>,
}

impl<T: Config> Events<T> {
    /// Create a new [`Events`] instance from the given bytes.
    pub fn decode_from(event_bytes: Vec<u8>, metadata: Metadata) -> Self {
        Self {
            inner: CoreEvents::decode_from(event_bytes, metadata),
        }
    }

    /// The number of events.
    pub fn len(&self) -> u32 {
        self.inner.len()
    }

    /// Are there no events in this block?
    // Note: mainly here to satisfy clippy..
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Return the bytes representing all of the events.
    pub fn bytes(&self) -> &[u8] {
        self.inner.bytes()
    }

    /// Iterate over all of the events, using metadata to dynamically
    /// decode them as we go, and returning the raw bytes and other associated
    /// details. If an error occurs, all subsequent iterations return `None`.
    // Dev note: The returned iterator is 'static + Send so that we can box it up and make
    // use of it with our `FilterEvents` stuff.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<EventDetails<T>, Error>> + Send + Sync + 'static {
        self.inner
            .iter()
            .map(|item| item.map(|e| EventDetails { inner: e }).map_err(Into::into))
    }

    /// Iterate through the events using metadata to dynamically decode and skip
    /// them, and return only those which should decode to the provided `Ev` type.
    /// If an error occurs, all subsequent iterations return `None`.
    pub fn find<Ev: StaticEvent>(&self) -> impl Iterator<Item = Result<Ev, Error>> {
        self.inner.find::<Ev>().map(|item| item.map_err(Into::into))
    }

    /// Iterate through the events using metadata to dynamically decode and skip
    /// them, and return the first event found which decodes to the provided `Ev` type.
    pub fn find_first<Ev: StaticEvent>(&self) -> Result<Option<Ev>, Error> {
        self.inner.find_first::<Ev>().map_err(Into::into)
    }

    /// Iterate through the events using metadata to dynamically decode and skip
    /// them, and return the last event found which decodes to the provided `Ev` type.
    pub fn find_last<Ev: StaticEvent>(&self) -> Result<Option<Ev>, Error> {
        self.inner.find_last::<Ev>().map_err(Into::into)
    }

    /// Find an event that decodes to the type provided. Returns true if it was found.
    pub fn has<Ev: StaticEvent>(&self) -> Result<bool, Error> {
        self.inner.has::<Ev>().map_err(Into::into)
    }
}

/// The event details.
#[derive(Debug, Clone)]
pub struct EventDetails<T: Config> {
    inner: CoreEventDetails<T>,
}

impl<T: Config> EventDetails<T> {
    /// When was the event produced?
    pub fn phase(&self) -> Phase {
        self.inner.phase()
    }

    /// What index is this event in the stored events for this block.
    pub fn index(&self) -> u32 {
        self.inner.index()
    }

    /// The index of the pallet that the event originated from.
    pub fn pallet_index(&self) -> u8 {
        self.inner.pallet_index()
    }

    /// The index of the event variant that the event originated from.
    pub fn variant_index(&self) -> u8 {
        self.inner.variant_index()
    }

    /// The name of the pallet from whence the Event originated.
    pub fn pallet_name(&self) -> &str {
        self.inner.pallet_name()
    }

    /// The name of the event (ie the name of the variant that it corresponds to).
    pub fn variant_name(&self) -> &str {
        self.inner.variant_name()
    }

    /// Fetch details from the metadata for this event.
    pub fn event_metadata(&self) -> EventMetadataDetails<'_> {
        self.inner.event_metadata()
    }

    /// Return _all_ of the bytes representing this event, which include, in order:
    /// - The phase.
    /// - Pallet and event index.
    /// - Event fields.
    /// - Event Topics.
    pub fn bytes(&self) -> &[u8] {
        self.inner.bytes()
    }

    /// Return the bytes representing the fields stored in this event.
    pub fn field_bytes(&self) -> &[u8] {
        self.inner.field_bytes()
    }

    /// Decode and provide the event fields back in the form of a [`scale_value::Composite`]
    /// type which represents the named or unnamed fields that were present in the event.
    pub fn field_values(&self) -> Result<scale_value::Composite<u32>, Error> {
        self.inner.field_values().map_err(Into::into)
    }

    /// Attempt to decode these [`EventDetails`] into a type representing the event fields.
    /// Such types are exposed in the codegen as `pallet_name::events::EventName` types.
    pub fn as_event<E: StaticEvent>(&self) -> Result<Option<E>, Error> {
        self.inner.as_event::<E>().map_err(Into::into)
    }

    /// Attempt to decode these [`EventDetails`] into a root event type (which includes
    /// the pallet and event enum variants as well as the event fields). A compatible
    /// type for this is exposed via static codegen as a root level `Event` type.
    pub fn as_root_event<E: DecodeAsType>(&self) -> Result<E, Error> {
        self.inner.as_root_event::<E>().map_err(Into::into)
    }

    /// Return the topics associated with this event.
    pub fn topics(&self) -> &[HashFor<T>] {
        self.inner.topics()
    }
}
