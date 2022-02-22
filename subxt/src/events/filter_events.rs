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

use codec::Decode;
use super::{ RawEventDetails, EventSubscription };
use crate::{ Config, BasicError, Event };
use futures::{ Stream, StreamExt };
use std::task::Poll;
use std::marker::Unpin;

/// A stream which returns tuples containing exactly one of the
/// given event types back on each iteration.
pub struct FilterEvents<'a, T: Config, Evs: 'static, Filter: EventFilter> {
    sub: EventSubscription<'a, T, Evs>,
    // Once we get a block back, we'll
    events: Option<Box<dyn Iterator<Item=Result<Filter::ReturnType, BasicError>> + 'a>>
}

impl <'a, T: Config, Evs, Filter: EventFilter> Unpin for FilterEvents<'a, T, Evs, Filter> {}

impl <'a, T: Config, Evs, Filter: EventFilter> FilterEvents<'a, T, Evs, Filter> {
    pub (crate) fn new(sub: EventSubscription<'a, T, Evs>) -> Self {
        Self {
            sub,
            events: None
        }
    }
}

impl <'a, T: Config, Evs: Decode, Filter: EventFilter> Stream for FilterEvents<'a, T, Evs, Filter> {
    type Item = Result<Filter::ReturnType, BasicError>;
    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
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
                    self.events = Some(Filter::filter(events.into_iter_raw()));
                }
            };
        }
    }
}

/// This trait is implemented for tuples of Event types; any such tuple (up to size 8) can be
/// used to filter an event subscription to return only the specified events.
pub trait EventFilter: private::Sealed {
    /// The type we'll be handed back from filtering.
    type ReturnType;
    /// Filter the events based on the type implementing this trait.
    fn filter<'a>(events: impl Iterator<Item=Result<RawEventDetails, BasicError>> + 'a) -> Box<dyn Iterator<Item=Result<Self::ReturnType, BasicError>> + 'a>;
}

// Prevent userspace implementations of the above trait; the interface is not considered stable
// and is not a particularly nice API to work with (particularly because it involves boxing, which
// would be nice to get rid of eventually).
pub (crate) mod private {
    pub trait Sealed {}
}

// A special case impl for searching for a tuple of exactly one event (in this case, we don't
// need to return an `(Option<Event>,)`; we can just return `Event`.
impl <A: Event> private::Sealed for (A,) {}
impl <A: Event> EventFilter for (A,) {
    type ReturnType = A;
    fn filter<'a>(mut events: impl Iterator<Item=Result<RawEventDetails, BasicError>> + 'a) -> Box<dyn Iterator<Item=Result<A, BasicError>> + 'a> {
        // Return an iterator that populates exactly 1 of the tuple options each pass,
        // or bails with None if none of them could be populated.
        Box::new(std::iter::from_fn(move || {
            while let Some(ev) = events.next() {
                // Forward any error immediately:
                let ev = match ev {
                    Ok(ev) => ev,
                    Err(e) => return Some(Err(e.into()))
                };
                // Try decoding each type until we hit a match or an error:
                let ev = ev.as_event::<A>();
                if let Ok(Some(ev)) = ev {
                    // We found a match; return our tuple.
                    return Some(Ok(ev));
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
            fn filter<'a>(mut events: impl Iterator<Item=Result<RawEventDetails, BasicError>> + 'a) -> Box<dyn Iterator<Item=Result<Self::ReturnType, BasicError>> + 'a> {
                // Return an iterator that populates exactly 1 of the tuple options each pass,
                // or bails with None if none of them could be populated.
                Box::new(std::iter::from_fn(move || {
                    let mut out: ( $(Option<$ty>,)+ ) = Default::default();
                    while let Some(ev) = events.next() {
                        // Forward any error immediately:
                        let ev = match ev {
                            Ok(ev) => ev,
                            Err(e) => return Some(Err(e.into()))
                        };
                        // Try decoding each type until we hit a match or an error:
                        $({
                            let ev = ev.as_event::<$ty>();
                            if let Ok(Some(ev)) = ev {
                                // We found a match; return our tuple.
                                out.$idx = Some(ev);
                                return Some(Ok(out));
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

