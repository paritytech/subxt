// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Subscribing to events.

use crate::{
    client::OnlineClientT,
    error::Error,
    events::EventsClient,
    Config,
};
use derivative::Derivative;
use futures::{
    stream::BoxStream,
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
    EventDetails,
    EventFilter,
    Events,
    FilterEvents,
};

/// A `jsonrpsee` Subscription. This forms a part of the `EventSubscription` type handed back
/// in codegen from `subscribe_finalized`, and is exposed to be used in codegen.
#[doc(hidden)]
pub type FinalizedEventSub<Header> = BoxStream<'static, Result<Header, Error>>;

/// A `jsonrpsee` Subscription. This forms a part of the `EventSubscription` type handed back
/// in codegen from `subscribe`, and is exposed to be used in codegen.
#[doc(hidden)]
pub type EventSub<Item> = Subscription<Item>;

/// A subscription to events that implements [`Stream`], and returns [`Events`] objects for each block.
#[derive(Derivative)]
#[derivative(Debug(bound = "Sub: std::fmt::Debug, Client: std::fmt::Debug"))]
pub struct EventSubscription<T: Config, Client, Sub> {
    finished: bool,
    client: Client,
    block_header_subscription: Sub,
    #[derivative(Debug = "ignore")]
    at: Option<std::pin::Pin<Box<dyn Future<Output = Result<Events<T>, Error>> + Send>>>,
}

impl<T: Config, Client, Sub, E: Into<Error>> EventSubscription<T, Client, Sub>
where
    Sub: Stream<Item = Result<T::Header, E>> + Unpin,
{
    /// Create a new [`EventSubscription`] from a client and a subscription
    /// which returns block headers.
    pub fn new(client: Client, block_header_subscription: Sub) -> Self {
        EventSubscription {
            finished: false,
            client,
            block_header_subscription,
            at: None,
        }
    }

    /// Return only specific events matching the tuple of 1 or more event
    /// types that has been provided as the `Filter` type parameter.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use futures::StreamExt;
    /// use subxt::{OnlineClient, PolkadotConfig};
    ///
    /// #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
    /// pub mod polkadot {}
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    ///
    /// let mut events = api
    ///     .events()
    ///     .subscribe()
    ///     .await
    ///     .unwrap()
    ///     .filter_events::<(
    ///         polkadot::balances::events::Transfer,
    ///         polkadot::balances::events::Deposit
    ///     )>();
    ///
    /// while let Some(ev) = events.next().await {
    ///     let event_details = ev.unwrap();
    ///     match event_details.event {
    ///         (Some(transfer), None) => println!("Balance transfer event: {transfer:?}"),
    ///         (None, Some(deposit)) => println!("Balance deposit event: {deposit:?}"),
    ///         _ => unreachable!()
    ///     }
    /// }
    /// # }
    /// ```
    pub fn filter_events<Filter: EventFilter>(
        self,
    ) -> FilterEvents<'static, Self, T, Filter> {
        FilterEvents::new(self)
    }
}

impl<T: Config, Client, Sub: Unpin> Unpin for EventSubscription<T, Client, Sub> {}

// We want `EventSubscription` to implement Stream. The below implementation is the rather verbose
// way to roughly implement the following function:
//
// ```
// fn subscribe_events<T: Config, Evs: Decode>(client: &'_ Client<T>, block_sub: Subscription<T::Header>) -> impl Stream<Item=Result<Events<'_, T, Evs>, Error>> + '_ {
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
impl<T, Client, Sub, E> Stream for EventSubscription<T, Client, Sub>
where
    T: Config,
    Client: OnlineClientT<T>,
    Sub: Stream<Item = Result<T::Header, E>> + Unpin,
    E: Into<Error>,
{
    type Item = Result<Events<T>, Error>;

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
                    let at = EventsClient::new(self.client.clone())
                        .at(Some(block_header.hash()));
                    self.at = Some(Box::pin(at));
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
                crate::SubstrateConfig,
                (),
                EventSub<<crate::SubstrateConfig as Config>::Header>,
            >,
        >();
        assert_send::<
            EventSubscription<
                crate::SubstrateConfig,
                (),
                FinalizedEventSub<<crate::SubstrateConfig as Config>::Header>,
            >,
        >();
    }
}
