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

//! Filtering individual events from subscriptions.

use super::{
    EventSubscription,
    Events,
};
use crate::{
    BasicError,
    Config,
    Event,
    Phase,
};
use codec::Decode;
use futures::{
    Stream,
    StreamExt,
};
use std::{
    marker::Unpin,
    task::Poll,
};

/// A stream which filters events based on the `Filter` param provided.
/// If `Filter` is a 1-tuple of a single `Event` type, it will return every
/// instance of that event as it's found. If `filter` is ` tuple of multiple
/// `Event` types, it will return a corresponding tuple of `Option`s, where
/// exactly one of these will be `Some(event)` each iteration.
pub struct FilterEvents<'a, T: Config, Evs: 'static, Filter: EventFilter> {
    sub: EventSubscription<'a, T, Evs>,
    events: Option<
        Box<
            dyn Iterator<
                    Item = Result<
                        FilteredEventDetails<T::Hash, Filter::ReturnType>,
                        BasicError,
                    >,
                > + 'a,
        >,
    >,
}

impl<'a, T: Config, Evs, Filter: EventFilter> Unpin for FilterEvents<'a, T, Evs, Filter> {}

impl<'a, T: Config, Evs, Filter: EventFilter> FilterEvents<'a, T, Evs, Filter> {
    pub(crate) fn new(sub: EventSubscription<'a, T, Evs>) -> Self {
        Self { sub, events: None }
    }
}

impl<'a, T: Config, Evs: Decode, Filter: EventFilter> Stream
    for FilterEvents<'a, T, Evs, Filter>
{
    type Item = Result<FilteredEventDetails<T::Hash, Filter::ReturnType>, BasicError>;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        loop {
            // Drain the current events we're iterating over first:
            if let Some(events_iter) = self.events.as_mut() {
                match events_iter.next() {
                    Some(res) => return Poll::Ready(Some(res)),
                    None => {
                        self.events = None;
                    }
                }
            }

            // Wait for new events to come in:
            match futures::ready!(self.sub.poll_next_unpin(cx)) {
                None => return Poll::Ready(None),
                Some(Err(e)) => return Poll::Ready(Some(Err(e))),
                Some(Ok(events)) => {
                    self.events = Some(Filter::filter(events));
                }
            };
        }
    }
}

/// This is returned from the [`FilterEvents`] impl of [`Stream`]. It contains
/// some type representing an event we've filtered on, along with couple of additional
/// pieces of information about that event.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FilteredEventDetails<BlockHash, Evs> {
    /// During which [`Phase`] was the event produced?
    pub phase: Phase,
    /// Hash of the block that this event came from.
    pub block_hash: BlockHash,
    /// A type containing an event that we've filtered on.
    /// Depending on the filter type, this may be a tuple
    /// or a single event.
    pub event: Evs,
}

/// This trait is implemented for tuples of Event types; any such tuple (up to size 8) can be
/// used to filter an event subscription to return only the specified events.
pub trait EventFilter: private::Sealed {
    /// The type we'll be handed back from filtering.
    type ReturnType;
    /// Filter the events based on the type implementing this trait.
    fn filter<'a, T: Config, Evs: Decode + 'static>(
        events: Events<'a, T, Evs>,
    ) -> Box<
        dyn Iterator<
                Item = Result<
                    FilteredEventDetails<T::Hash, Self::ReturnType>,
                    BasicError,
                >,
            > + 'a,
    >;
}

// Prevent userspace implementations of the above trait; the interface is not considered stable
// and is not a particularly nice API to work with (particularly because it involves boxing, which
// would be nice to get rid of eventually).
pub(crate) mod private {
    pub trait Sealed {}
}

// A special case impl for searching for a tuple of exactly one event (in this case, we don't
// need to return an `(Option<Event>,)`; we can just return `Event`.
impl<Ev: Event> private::Sealed for (Ev,) {}
impl<Ev: Event> EventFilter for (Ev,) {
    type ReturnType = Ev;
    fn filter<'a, T: Config, Evs: Decode + 'static>(
        events: Events<'a, T, Evs>,
    ) -> Box<
        dyn Iterator<Item = Result<FilteredEventDetails<T::Hash, Ev>, BasicError>> + 'a,
    > {
        let block_hash = events.block_hash();
        let mut iter = events.into_iter_raw();
        Box::new(std::iter::from_fn(move || {
            for ev in iter.by_ref() {
                // Forward any error immediately:
                let raw_event = match ev {
                    Ok(ev) => ev,
                    Err(e) => return Some(Err(e)),
                };
                // Try decoding each type until we hit a match or an error:
                let ev = raw_event.as_event::<Ev>();
                if let Ok(Some(event)) = ev {
                    // We found a match; return our tuple.
                    return Some(Ok(FilteredEventDetails {
                        phase: raw_event.phase,
                        block_hash,
                        event,
                    }))
                }
                if let Err(e) = ev {
                    // We hit an error. Return it.
                    return Some(Err(e.into()))
                }
            }
            None
        }))
    }
}

// A generalised impl for tuples of sizes greater than 1:
macro_rules! impl_event_filter {
    ($($ty:ident $idx:tt),+) => {
        impl <$($ty: Event),+> private::Sealed for ( $($ty,)+ ) {}
        impl <$($ty: Event),+> EventFilter for ( $($ty,)+ ) {
            type ReturnType = ( $(Option<$ty>,)+ );
            fn filter<'a, T: Config, Evs: Decode + 'static>(
                events: Events<'a, T, Evs>
            ) -> Box<dyn Iterator<Item=Result<FilteredEventDetails<T::Hash,Self::ReturnType>, BasicError>> + 'a> {
                let block_hash = events.block_hash();
                let mut iter = events.into_iter_raw();
                Box::new(std::iter::from_fn(move || {
                    let mut out: ( $(Option<$ty>,)+ ) = Default::default();
                    for ev in iter.by_ref() {
                        // Forward any error immediately:
                        let raw_event = match ev {
                            Ok(ev) => ev,
                            Err(e) => return Some(Err(e))
                        };
                        // Try decoding each type until we hit a match or an error:
                        $({
                            let ev = raw_event.as_event::<$ty>();
                            if let Ok(Some(ev)) = ev {
                                // We found a match; return our tuple.
                                out.$idx = Some(ev);
                                return Some(Ok(FilteredEventDetails {
                                    phase: raw_event.phase,
                                    block_hash,
                                    event: out
                                }))
                            }
                            if let Err(e) = ev {
                                // We hit an error. Return it.
                                return Some(Err(e.into()))
                            }
                        })+
                    }
                    None
                }))
            }
        }
    }
}

impl_event_filter!(A 0, B 1);
impl_event_filter!(A 0, B 1, C 2);
impl_event_filter!(A 0, B 1, C 2, D 3);
impl_event_filter!(A 0, B 1, C 2, D 3, E 4);
impl_event_filter!(A 0, B 1, C 2, D 3, E 4, F 5);
impl_event_filter!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
impl_event_filter!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
