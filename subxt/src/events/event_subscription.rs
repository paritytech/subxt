// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Subscribing to events.

use crate::{
    error::BasicError,
    OnlineClient,
    Config,
};
use codec::Decode;
use derivative::Derivative;
use futures::{
    future::Either,
    stream::{
        self,
        BoxStream,
    },
    Future,
    FutureExt,
    Stream,
    StreamExt,
};
use jsonrpsee::core::client::Subscription;
use sp_runtime::traits::Header;
use std::{
    marker::Unpin,
    task::Poll,
};

pub use super::{
    at,
    EventDetails,
    EventFilter,
    Events,
    FilterEvents,
    RawEventDetails,
};

/// Subscribe to events from blocks.
///
/// **Note:** these blocks haven't necessarily been finalised yet; prefer
/// [`Events::subscribe_finalized()`] if that is important.
///
/// **Note:** This function is hidden from the documentation
/// and is exposed only to be called via the codegen. It may
/// break between minor releases.
#[doc(hidden)]
pub async fn subscribe<Client, T, Evs>(
    client: Client,
) -> Result<EventSubscription<EventSub<T::Header>, T, Evs>, BasicError>
where
    Client: Into<OnlineClient<T>>,
    T: Config,
    Evs: Decode + 'static
{
    let client = client.into();
    let block_subscription = client.rpc().subscribe_blocks().await?;
    Ok(EventSubscription::new(client, block_subscription))
}

/// Subscribe to events from finalized blocks.
///
/// **Note:** This function is hidden from the documentation
/// and is exposed only to be called via the codegen. It may
/// break between minor releases.
#[doc(hidden)]
pub async fn subscribe_finalized<Client, T, Evs>(
    client: Client,
) -> Result<EventSubscription<FinalizedEventSub<T::Header>, T, Evs>, BasicError>
where
    Client: Into<OnlineClient<T>>,
    T: Config,
    Evs: Decode + 'static
{
    let client = client.into();

    // fetch the last finalised block details immediately, so that we'll get
    // events for each block after this one.
    let last_finalized_block_hash = client.rpc().finalized_head().await?;
    let last_finalized_block_number = client
        .rpc()
        .header(Some(last_finalized_block_hash))
        .await?
        .map(|h| (*h.number()).into());

    let sub = client.rpc().subscribe_finalized_blocks().await?;

    // Fill in any gaps between the block above and the finalized blocks reported.
    let block_subscription = subscribe_to_block_headers_filling_in_gaps(
        client.clone(),
        last_finalized_block_number,
        sub,
    );

    Ok(EventSubscription::new(client, Box::pin(block_subscription)))
}

/// Take a subscription that returns block headers, and if any block numbers are missed out
/// betweem the block number provided and what's returned from the subscription, we fill in
/// the gaps and get hold of all intermediate block headers.
///
/// **Note:** This is exposed so that we can run integration tests on it, but otherwise
/// should not be used directly and may break between minor releases.
#[doc(hidden)]
pub fn subscribe_to_block_headers_filling_in_gaps<Client, S, E, T>(
    client: Client,
    mut last_block_num: Option<u64>,
    sub: S,
) -> impl Stream<Item = Result<T::Header, BasicError>> + Send
where
    Client: Into<OnlineClient<T>>,
    S: Stream<Item = Result<T::Header, E>> + Send,
    E: Into<BasicError> + Send + 'static,
    T: Config,
{
    let client = client.into();
    sub.flat_map(move |s| {
        let client = client.clone();

        // Get the header, or return a stream containing just the error. Our EventSubscription
        // stream will return `None` as soon as it hits an error like this.
        let header = match s {
            Ok(header) => header,
            Err(e) => return Either::Left(stream::once(async { Err(e.into()) })),
        };

        // We want all previous details up to, but not including this current block num.
        let end_block_num = (*header.number()).into();

        // This is one after the last block we returned details for last time.
        let start_block_num = last_block_num.map(|n| n + 1).unwrap_or(end_block_num);

        // Iterate over all of the previous blocks we need headers for, ignoring the current block
        // (which we already have the header info for):
        let previous_headers = stream::iter(start_block_num..end_block_num)
            .then(move |n| {
                let client = client.clone();
                async move {
                    let hash = client.rpc().block_hash(Some(n.into())).await?;
                    let header = client.rpc().header(hash).await?;
                    Ok::<_, BasicError>(header)
                }
            })
            .filter_map(|h| async { h.transpose() });

        // On the next iteration, we'll get details starting just after this end block.
        last_block_num = Some(end_block_num);

        // Return a combination of any previous headers plus the new header.
        Either::Right(previous_headers.chain(stream::once(async { Ok(header) })))
    })
}

/// A `jsonrpsee` Subscription. This forms a part of the `EventSubscription` type handed back
/// in codegen from `subscribe_finalized`, and is exposed to be used in codegen.
#[doc(hidden)]
pub type FinalizedEventSub<Header> = BoxStream<'static, Result<Header, BasicError>>;

/// A `jsonrpsee` Subscription. This forms a part of the `EventSubscription` type handed back
/// in codegen from `subscribe`, and is exposed to be used in codegen.
#[doc(hidden)]
pub type EventSub<Item> = Subscription<Item>;

/// A subscription to events that implements [`Stream`], and returns [`Events`] objects for each block.
#[derive(Derivative)]
#[derivative(Debug(bound = "Sub: std::fmt::Debug"))]
pub struct EventSubscription<Sub, T: Config, Evs: 'static> {
    finished: bool,
    client: OnlineClient<T>,
    block_header_subscription: Sub,
    #[derivative(Debug = "ignore")]
    at: Option<
        std::pin::Pin<
            Box<dyn Future<Output = Result<Events<T, Evs>, BasicError>> + Send>,
        >,
    >,
    _event_type: std::marker::PhantomData<Evs>,
}

impl<Sub, T: Config, Evs: Decode, E: Into<BasicError>>
    EventSubscription<Sub, T, Evs>
where
    Sub: Stream<Item = Result<T::Header, E>> + Unpin,
{
    fn new(client: impl Into<OnlineClient<T>>, block_header_subscription: Sub) -> Self {
        EventSubscription {
            finished: false,
            client: client.into(),
            block_header_subscription,
            at: None,
            _event_type: std::marker::PhantomData,
        }
    }

    /// Return only specific events matching the tuple of 1 or more event
    /// types that has been provided as the `Filter` type parameter.
    pub fn filter_events<Filter: EventFilter>(self) -> FilterEvents<'static, Self, T, Filter> {
        FilterEvents::new(self)
    }
}

impl<'a, T: Config, Sub: Unpin, Evs: Decode> Unpin
    for EventSubscription<Sub, T, Evs>
{
}

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
impl<Sub, T, Evs, E> Stream for EventSubscription<Sub, T, Evs>
where
    T: Config,
    Evs: Decode,
    Sub: Stream<Item = Result<T::Header, E>> + Unpin,
    E: Into<BasicError>,
{
    type Item = Result<Events<T, Evs>, BasicError>;

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
    #[allow(unused)]
    fn check_sendability() {
        fn assert_send<T: Send>() {}
        assert_send::<
            EventSubscription<
                EventSub<<crate::DefaultConfig as Config>::Header>,
                crate::DefaultConfig,
                (),
            >,
        >();
        assert_send::<
            EventSubscription<
                FinalizedEventSub<<crate::DefaultConfig as Config>::Header>,
                crate::DefaultConfig,
                (),
            >,
        >();
    }
}
