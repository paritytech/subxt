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

//! Subscribing to events.

use crate::{
    error::BasicError,
    Client,
    Config,
};
use codec::Decode;
use derivative::Derivative;
use futures::{
    Future,
    FutureExt,
    Stream,
    StreamExt,
};
use jsonrpsee::core::client::Subscription;
use std::{
    marker::Unpin,
    task::Poll,
};

pub use super::{
    at,
    EventDetails,
    EventFilter,
    Events,
    EventsDecodingError,
    FilterEvents,
    RawEventDetails,
};

/// Subscribe to events from blocks.
///
/// **Note:** these blocks haven't necessarily been finalised yet; prefer
/// [`Events::subscribe_finalized()`] if that is important.
///
/// **Note:** This function is hidden from the documentation
/// and is exposed only to be called via the codegen. Thus, prefer to use
/// `api.events().subscribe()` over calling this directly.
#[doc(hidden)]
pub async fn subscribe<T: Config, Evs: Decode + 'static>(
    client: &'_ Client<T>,
) -> Result<EventSubscription<'_, T, Evs>, BasicError> {
    let block_subscription = client.rpc().subscribe_blocks().await?;
    Ok(EventSubscription::new(client, block_subscription))
}

/// Subscribe to events from finalized blocks.
///
/// **Note:** This function is hidden from the documentation
/// and is exposed only to be called via the codegen. Thus, prefer to use
/// `api.events().subscribe_finalized()` over calling this directly.
#[doc(hidden)]
pub async fn subscribe_finalized<T: Config, Evs: Decode + 'static>(
    client: &'_ Client<T>,
) -> Result<EventSubscription<'_, T, Evs>, BasicError> {
    let block_subscription = client.rpc().subscribe_finalized_blocks().await?;
    Ok(EventSubscription::new(client, block_subscription))
}

/// A subscription to events that implements [`Stream`], and returns [`Events`] objects for each block.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct EventSubscription<'a, T: Config, Evs: 'static> {
    finished: bool,
    client: &'a Client<T>,
    block_header_subscription: Subscription<T::Header>,
    #[derivative(Debug = "ignore")]
    at: Option<
        std::pin::Pin<
            Box<dyn Future<Output = Result<Events<'a, T, Evs>, BasicError>> + Send + 'a>,
        >,
    >,
    _event_type: std::marker::PhantomData<Evs>,
}

impl<'a, T: Config, Evs: Decode> EventSubscription<'a, T, Evs> {
    fn new(
        client: &'a Client<T>,
        block_header_subscription: Subscription<T::Header>,
    ) -> Self {
        EventSubscription {
            finished: false,
            client,
            block_header_subscription,
            at: None,
            _event_type: std::marker::PhantomData,
        }
    }

    /// Return only specific events matching the tuple of 1 or more event
    /// types that has been provided as the `Filter` type parameter.
    pub fn filter_events<Filter: EventFilter>(self) -> FilterEvents<'a, Self, T, Filter> {
        FilterEvents::new(self)
    }
}

impl<'a, T: Config, Evs: Decode> Unpin for EventSubscription<'a, T, Evs> {}

// We want `EventSubscription` to implement Stream. The below implementation is the rather verbose
// way to roughly implement the following function:
//
// ```
// fn subscribe_events<T: Config, Evs: Decode>(client: &'_ Client<T>, block_sub: Subscription<T::Header>) -> impl Stream<Item=Result<Events<'_, T, Evs>, BasicError>> + '_ {
//     use futures::StreamExt;
//     block_sub.then(move |block_header_res| async move {
//         use sp_runtime::traits::Header;
//         let block_header = block_header_res?;
//         let block_hash = block_header.hash();
//         at(client, block_hash).await
//     })
// }
// ```
//
// The advantage of this manual implementation is that we have a named type that we (and others)
// can derive things on, store away, alias etc.
impl<'a, T: Config, Evs: Decode> Stream for EventSubscription<'a, T, Evs> {
    type Item = Result<Events<'a, T, Evs>, BasicError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // We are finished; return None.
        if self.finished {
            return Poll::Ready(None)
        }

        // If there isn't an `at` function yet that's busy resolving a block hash into
        // some event details, then poll the block header subscription to get one.
        if self.at.is_none() {
            match futures::ready!(self.block_header_subscription.poll_next_unpin(cx)) {
                None => {
                    self.finished = true;
                    return Poll::Ready(None)
                }
                Some(Err(e)) => {
                    self.finished = true;
                    return Poll::Ready(Some(Err(e.into())))
                }
                Some(Ok(block_header)) => {
                    use sp_runtime::traits::Header;
                    // Note [jsdw]: We may be able to get rid of the per-item allocation
                    // with https://github.com/oblique/reusable-box-future.
                    self.at = Some(Box::pin(at(self.client, block_header.hash())));
                    // Continue, so that we poll this function future we've just created.
                }
            }
        }

        // If we get here, there will be an `at` function stored. Unwrap it and poll it to
        // completion to get our events, throwing it away as soon as it is ready.
        let at_fn = self
            .at
            .as_mut()
            .expect("'at' function should have been set above'");
        let events = futures::ready!(at_fn.poll_unpin(cx));
        self.at = None;
        Poll::Ready(Some(events))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Ensure `EventSubscription` can be sent; only actually a compile-time check.
    #[test]
    fn check_sendability() {
        fn assert_send<T: Send>() {}
        assert_send::<EventSubscription<crate::DefaultConfig, ()>>();
    }
}
