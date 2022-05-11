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

use super::Events;
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
pub struct FilterEvents<'a, Sub: 'a, T: Config, Filter: EventFilter> {
    // A subscription; in order for the Stream impl to apply, this will
    // impl `Stream<Item = Result<Events<'a, T, Evs>, BasicError>> + Unpin + 'a`.
    sub: Sub,
    // Each time we get Events from our subscription, they are stored here
    // and iterated through in future stream iterations until exhausted.
    events: Option<
        Box<
            dyn Iterator<
                    Item = Result<
                        FilteredEventDetails<T::Hash, Filter::ReturnType>,
                        BasicError,
                    >,
                > + Send
                + 'a,
        >,
    >,
}

impl<'a, Sub: 'a, T: Config, Filter: EventFilter> Unpin
    for FilterEvents<'a, Sub, T, Filter>
{
}

impl<'a, Sub: 'a, T: Config, Filter: EventFilter> FilterEvents<'a, Sub, T, Filter> {
    pub(crate) fn new(sub: Sub) -> Self {
        Self { sub, events: None }
    }
}

impl<'a, Sub, T, Evs, Filter> Stream for FilterEvents<'a, Sub, T, Filter>
where
    Sub: Stream<Item = Result<Events<T, Evs>, BasicError>> + Unpin + 'a,
    T: Config,
    Evs: Decode + 'static,
    Filter: EventFilter,
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
        events: Events<T, Evs>,
    ) -> Box<
        dyn Iterator<
                Item = Result<
                    FilteredEventDetails<T::Hash, Self::ReturnType>,
                    BasicError,
                >,
            > + Send
            + 'a,
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
        events: Events<T, Evs>,
    ) -> Box<
        dyn Iterator<Item = Result<FilteredEventDetails<T::Hash, Ev>, BasicError>>
            + Send
            + 'a,
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
                events: Events<T, Evs>
            ) -> Box<dyn Iterator<Item=Result<FilteredEventDetails<T::Hash,Self::ReturnType>, BasicError>> + Send + 'a> {
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

#[cfg(test)]
mod test {
    use super::{
        super::events_type::test_utils::{
            event_record,
            events,
            metadata,
            AllEvents,
        },
        *,
    };
    use crate::{
        Config,
        DefaultConfig,
        Metadata,
    };
    use codec::Encode;
    use futures::{
        stream,
        Stream,
        StreamExt,
    };
    use parking_lot::RwLock;
    use scale_info::TypeInfo;
    use std::sync::Arc;

    // Some pretend events in a pallet
    #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
    enum PalletEvents {
        A(EventA),
        B(EventB),
        C(EventC),
    }

    // An event in our pallet that we can filter on.
    #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
    struct EventA(u8);
    impl crate::Event for EventA {
        const PALLET: &'static str = "Test";
        const EVENT: &'static str = "A";
    }

    // An event in our pallet that we can filter on.
    #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
    struct EventB(bool);
    impl crate::Event for EventB {
        const PALLET: &'static str = "Test";
        const EVENT: &'static str = "B";
    }

    // An event in our pallet that we can filter on.
    #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
    struct EventC(u8, bool);
    impl crate::Event for EventC {
        const PALLET: &'static str = "Test";
        const EVENT: &'static str = "C";
    }

    // A stream of fake events for us to try filtering on.
    fn events_stream(
        metadata: Arc<RwLock<Metadata>>,
    ) -> impl Stream<Item = Result<Events<DefaultConfig, AllEvents<PalletEvents>>, BasicError>>
    {
        stream::iter(vec![
            events::<PalletEvents>(
                metadata.clone(),
                vec![
                    event_record(Phase::Initialization, PalletEvents::A(EventA(1))),
                    event_record(Phase::ApplyExtrinsic(0), PalletEvents::B(EventB(true))),
                    event_record(Phase::Finalization, PalletEvents::A(EventA(2))),
                ],
            ),
            events::<PalletEvents>(
                metadata.clone(),
                vec![event_record(
                    Phase::ApplyExtrinsic(1),
                    PalletEvents::B(EventB(false)),
                )],
            ),
            events::<PalletEvents>(
                metadata,
                vec![
                    event_record(Phase::ApplyExtrinsic(2), PalletEvents::B(EventB(true))),
                    event_record(Phase::ApplyExtrinsic(3), PalletEvents::A(EventA(3))),
                ],
            ),
        ])
        .map(Ok::<_, BasicError>)
    }

    #[tokio::test]
    async fn filter_one_event_from_stream() {
        let metadata = Arc::new(RwLock::new(metadata::<PalletEvents>()));

        // Filter out fake event stream to select events matching `EventA` only.
        let actual: Vec<_> =
            FilterEvents::<_, DefaultConfig, (EventA,)>::new(events_stream(metadata))
                .map(|e| e.unwrap())
                .collect()
                .await;

        let expected = vec![
            FilteredEventDetails {
                phase: Phase::Initialization,
                block_hash: <DefaultConfig as Config>::Hash::default(),
                event: EventA(1),
            },
            FilteredEventDetails {
                phase: Phase::Finalization,
                block_hash: <DefaultConfig as Config>::Hash::default(),
                event: EventA(2),
            },
            FilteredEventDetails {
                phase: Phase::ApplyExtrinsic(3),
                block_hash: <DefaultConfig as Config>::Hash::default(),
                event: EventA(3),
            },
        ];

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn filter_some_events_from_stream() {
        let metadata = Arc::new(RwLock::new(metadata::<PalletEvents>()));

        // Filter out fake event stream to select events matching `EventA` or `EventB`.
        let actual: Vec<_> = FilterEvents::<_, DefaultConfig, (EventA, EventB)>::new(
            events_stream(metadata),
        )
        .map(|e| e.unwrap())
        .collect()
        .await;

        let expected = vec![
            FilteredEventDetails {
                phase: Phase::Initialization,
                block_hash: <DefaultConfig as Config>::Hash::default(),
                event: (Some(EventA(1)), None),
            },
            FilteredEventDetails {
                phase: Phase::ApplyExtrinsic(0),
                block_hash: <DefaultConfig as Config>::Hash::default(),
                event: (None, Some(EventB(true))),
            },
            FilteredEventDetails {
                phase: Phase::Finalization,
                block_hash: <DefaultConfig as Config>::Hash::default(),
                event: (Some(EventA(2)), None),
            },
            FilteredEventDetails {
                phase: Phase::ApplyExtrinsic(1),
                block_hash: <DefaultConfig as Config>::Hash::default(),
                event: (None, Some(EventB(false))),
            },
            FilteredEventDetails {
                phase: Phase::ApplyExtrinsic(2),
                block_hash: <DefaultConfig as Config>::Hash::default(),
                event: (None, Some(EventB(true))),
            },
            FilteredEventDetails {
                phase: Phase::ApplyExtrinsic(3),
                block_hash: <DefaultConfig as Config>::Hash::default(),
                event: (Some(EventA(3)), None),
            },
        ];

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn filter_no_events_from_stream() {
        let metadata = Arc::new(RwLock::new(metadata::<PalletEvents>()));

        // Filter out fake event stream to select events matching `EventC` (none exist).
        let actual: Vec<_> =
            FilterEvents::<_, DefaultConfig, (EventC,)>::new(events_stream(metadata))
                .map(|e| e.unwrap())
                .collect()
                .await;

        assert_eq!(actual, vec![]);
    }
}
