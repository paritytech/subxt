// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::rpc_methods::{FollowEvent, UnstableRpcMethods};
use crate::config::Config;
use crate::error::Error;
use futures::{FutureExt, Stream, StreamExt};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// This stream subscribes to `chainHead_follow` when polled, and
/// resubscribes automatically if it's stopped.
pub struct FollowStream<Hash> {
    // Using this and not just keeping a copy of the RPC methods
    // around means that we can test this in isolation with dummy streams.
    stream_getter: FollowEventStreamGetter<Hash>,
    stream: InnerStreamState<Hash>,
}

/// A getter function that returns an [`FollowEventStreamFut<Hash>`].
pub type FollowEventStreamGetter<Hash> = Box<dyn FnMut() -> FollowEventStreamFut<Hash> + Send>;

/// The future which will return a stream of follow events and the subscription ID for it.
pub type FollowEventStreamFut<Hash> = Pin<
    Box<dyn Future<Output = Result<(FollowEventStream<Hash>, String), Error>> + Send + 'static>,
>;

/// The stream of follow events.
pub type FollowEventStream<Hash> =
    Pin<Box<dyn Stream<Item = Result<FollowEvent<Hash>, Error>> + Send + 'static>>;

/// Either a ready message with the current subscription ID, or
/// an event from the stream itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FollowStreamMsg<Hash> {
    /// The stream is ready (and has a subscription ID)
    Ready(String),
    /// An event from the stream.
    Event(FollowEvent<Hash>),
}

enum InnerStreamState<Hash> {
    /// We've just created the stream; we'll start Initializing it
    New,
    /// We're fetching the inner subscription. Move to Ready when we have one.
    Initializing(FollowEventStreamFut<Hash>),
    /// Report back the subscription ID here, and then start ReceivingEvents.
    Ready(Option<(FollowEventStream<Hash>, String)>),
    /// We are polling for, and receiving events from the stream.
    ReceivingEvents(FollowEventStream<Hash>),
    /// We received a stop event. We'll send one on and restart the stream.
    Stopped,
    /// The stream is finished and will not restart (likely due to an error).
    Finished,
}

impl<Hash> FollowStream<Hash> {
    /// Create a new [`FollowStream`] given a function which returns the stream.
    pub fn new(stream_getter: FollowEventStreamGetter<Hash>) -> Self {
        Self {
            stream_getter,
            stream: InnerStreamState::New,
        }
    }

    /// Create a new [`FollowStream`] given the RPC methods.
    pub fn from_methods<T: Config>(methods: UnstableRpcMethods<T>) -> FollowStream<T::Hash> {
        FollowStream {
            stream_getter: Box::new(move || {
                let methods = methods.clone();
                Box::pin(async move {
                    // Make the RPC call:
                    let stream = methods.chainhead_unstable_follow(true).await?;
                    // Extract the subscription ID:
                    let Some(sub_id) = stream.subscription_id().map(ToOwned::to_owned) else {
                        return Err(Error::Other(
                            "Subscription ID expected for chainHead_follow response, but not given"
                                .to_owned(),
                        ));
                    };
                    // Return both:
                    let stream: FollowEventStream<T::Hash> = Box::pin(stream);
                    Ok((stream, sub_id))
                })
            }),
            stream: InnerStreamState::New,
        }
    }
}

impl<Hash> std::marker::Unpin for FollowStream<Hash> {}

impl<Hash> Stream for FollowStream<Hash> {
    type Item = Result<FollowStreamMsg<Hash>, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            match &mut this.stream {
                InnerStreamState::New => {
                    let fut = (this.stream_getter)();
                    this.stream = InnerStreamState::Initializing(fut);
                    continue;
                }
                InnerStreamState::Initializing(fut) => {
                    match fut.poll_unpin(cx) {
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                        Poll::Ready(Ok(sub_with_id)) => {
                            this.stream = InnerStreamState::Ready(Some(sub_with_id));
                            continue;
                        }
                        Poll::Ready(Err(e)) => {
                            // Finish forever if there's an error, passing it on.
                            this.stream = InnerStreamState::Finished;
                            return Poll::Ready(Some(Err(e)));
                        }
                    }
                }
                InnerStreamState::Ready(stream) => {
                    let (sub, sub_id) = stream.take().expect("should always be Some");
                    this.stream = InnerStreamState::ReceivingEvents(sub);
                    return Poll::Ready(Some(Ok(FollowStreamMsg::Ready(sub_id))));
                }
                InnerStreamState::ReceivingEvents(stream) => {
                    match stream.poll_next_unpin(cx) {
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                        Poll::Ready(None) => {
                            // No error happened but the stream ended; restart and
                            // pass on a Stop message anyway.
                            this.stream = InnerStreamState::Stopped;
                            continue;
                        }
                        Poll::Ready(Some(Ok(ev))) => {
                            if let FollowEvent::Stop = ev {
                                // A stop event means the stream has ended, so start
                                // over after passing on the stop message.
                                this.stream = InnerStreamState::Stopped;
                                continue;
                            }
                            return Poll::Ready(Some(Ok(FollowStreamMsg::Event(ev))));
                        }
                        Poll::Ready(Some(Err(e))) => {
                            // Finish forever if there's an error, passing it on.
                            this.stream = InnerStreamState::Finished;
                            return Poll::Ready(Some(Err(e)));
                        }
                    }
                }
                InnerStreamState::Stopped => {
                    this.stream = InnerStreamState::New;
                    return Poll::Ready(Some(Ok(FollowStreamMsg::Event(FollowEvent::Stop))));
                }
                InnerStreamState::Finished => {
                    return Poll::Ready(None);
                }
            }
        }
    }
}

#[cfg(test)]
pub(super) mod test_utils {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    // Given some events, returns a follow stream getter that we can use in
    // place of the usual RPC method.
    pub fn test_stream_getter<Hash, F, I>(events: F) -> FollowEventStreamGetter<Hash>
    where
        Hash: Send + 'static,
        F: Fn() -> I + Send + 'static,
        I: IntoIterator<Item = Result<FollowEvent<Hash>, Error>>,
    {
        let start_idx = Arc::new(AtomicUsize::new(0));

        Box::new(move || {
            // Start the events from where we left off last time.
            let start_idx = start_idx.clone();
            let this_idx = start_idx.load(Ordering::Relaxed);
            let events: Vec<_> = events().into_iter().skip(this_idx).collect();

            Box::pin(async move {
                // Increment start_idx for each event we see, so that if we get
                // the stream again, we get only the remaining events for it.
                let stream = futures::stream::iter(events).map(move |ev| {
                    start_idx.fetch_add(1, Ordering::Relaxed);
                    ev
                });

                let stream: FollowEventStream<Hash> = Box::pin(stream);
                Ok((stream, format!("sub_id_{this_idx}")))
            })
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::backend::unstable::rpc_methods::{Initialized, NewBlock};
    use crate::config::substrate::H256;
    use test_utils::test_stream_getter;

    #[tokio::test]
    async fn follow_stream_provides_messages_until_error() {
        // The events we'll get back on the stream.
        let stream_getter = test_stream_getter(|| {
            [
                Ok(FollowEvent::Initialized(Initialized {
                    finalized_block_hash: H256::from_low_u64_le(1),
                    finalized_block_runtime: None,
                })),
                // Stop should lead to a drop and resubscribe:
                Ok(FollowEvent::Stop),
                Ok(FollowEvent::Stop),
                Ok(FollowEvent::NewBlock(NewBlock {
                    parent_block_hash: H256::from_low_u64_le(2),
                    block_hash: H256::from_low_u64_le(3),
                    new_runtime: None,
                })),
                // Nothing should be emitted after an error:
                Err(Error::Other("ended".to_owned())),
                Ok(FollowEvent::NewBlock(NewBlock {
                    parent_block_hash: H256::from_low_u64_le(3),
                    block_hash: H256::from_low_u64_le(4),
                    new_runtime: None,
                })),
            ]
        });

        let s = FollowStream::new(stream_getter);
        let out: Vec<_> = s.filter_map(|e| async move { e.ok() }).collect().await;

        // The expected response, given the above.
        assert_eq!(
            out,
            vec![
                FollowStreamMsg::Ready("sub_id_0".to_owned()),
                FollowStreamMsg::Event(FollowEvent::Initialized(Initialized {
                    finalized_block_hash: H256::from_low_u64_le(1),
                    finalized_block_runtime: None
                })),
                FollowStreamMsg::Event(FollowEvent::Stop),
                FollowStreamMsg::Ready("sub_id_2".to_owned()),
                FollowStreamMsg::Event(FollowEvent::Stop),
                FollowStreamMsg::Ready("sub_id_3".to_owned()),
                FollowStreamMsg::Event(FollowEvent::NewBlock(NewBlock {
                    parent_block_hash: H256::from_low_u64_le(2),
                    block_hash: H256::from_low_u64_le(3),
                    new_runtime: None
                })),
            ]
        );
    }
}
