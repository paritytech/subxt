// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes the types and such necessary for working with events.
//! The two main entry points into events are [`crate::OnlineClient::events()`]
//! and calls like [crate::tx::TxProgress::wait_for_finalized_success()].

mod event_subscription;
mod events_client;
mod events_type;
mod filter_events;

pub use event_subscription::{
    EventSub,
    EventSubscription,
    FinalizedEventSub,
};
pub use events_client::{
    // Exposed only for testing:
    subscribe_to_block_headers_filling_in_gaps,
    EventsClient,
};
pub use events_type::{
    EventDetails,
    Events,
};
pub use filter_events::{
    EventFilter,
    FilterEvents,
    FilteredEventDetails,
};

use codec::{
    Decode,
    Encode,
};

/// Trait to uniquely identify the events's identity from the runtime metadata.
///
/// Generated API structures that represent an event implement this trait.
///
/// The trait is utilized to decode emitted events from a block, via obtaining the
/// form of the `Event` from the metadata.
pub trait StaticEvent: Decode {
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
