// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::follow_stream_driver::FollowStreamDriverHandle;
use super::follow_stream_unpin::BlockRef;
use super::rpc_methods::{
    FollowEvent, MethodResponse, StorageQuery, StorageResult, UnstableRpcMethods,
};
use crate::config::Config;
use crate::error::{Error, RpcError};
use futures::{FutureExt, Stream, StreamExt};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// Obtain a stream of storage items given some query. this handles continuing
/// and stopping under the hood, and returns a stream of `StorageResult`s.
pub struct StorageItems<T: Config> {
    done: bool,
    operation_id: Arc<str>,
    buffered_responses: VecDeque<StorageResult>,
    continue_call: ContinueFutGetter,
    continue_fut: Option<ContinueFut>,
    follow_event_stream: FollowEventStream<T::Hash>,
}

impl<T: Config> StorageItems<T> {
    // Subscribe to follow events, and return a stream of storage results
    // given some storage queries. The stream will automatically resume as
    // needed, and stop when done.
    pub async fn from_methods(
        queries: impl Iterator<Item = StorageQuery<&[u8]>>,
        at: T::Hash,
        follow_handle: &FollowStreamDriverHandle<T::Hash>,
        methods: UnstableRpcMethods<T>,
    ) -> Result<Self, Error> {
        let sub_id = super::get_subscription_id(follow_handle).await?;

        // Subscribe to events and make the initial request to get an operation ID.
        let follow_events = follow_handle.subscribe().events();
        let status = methods
            .chainhead_unstable_storage(&sub_id, at, queries, None)
            .await?;
        let operation_id: Arc<str> = match status {
            MethodResponse::LimitReached => {
                return Err(RpcError::request_rejected("limit reached").into())
            }
            MethodResponse::Started(s) => s.operation_id.into(),
        };

        // A function which returns the call to continue the subscription:
        let continue_call: ContinueFutGetter = {
            let operation_id = operation_id.clone();
            Box::new(move || {
                let sub_id = sub_id.clone();
                let operation_id = operation_id.clone();
                let methods = methods.clone();

                Box::pin(async move {
                    methods
                        .chainhead_unstable_continue(&sub_id, &operation_id)
                        .await
                })
            })
        };

        Ok(StorageItems::new(
            operation_id,
            continue_call,
            Box::pin(follow_events),
        ))
    }

    fn new(
        operation_id: Arc<str>,
        continue_call: ContinueFutGetter,
        follow_event_stream: FollowEventStream<T::Hash>,
    ) -> Self {
        Self {
            done: false,
            buffered_responses: VecDeque::new(),
            operation_id,
            continue_call,
            continue_fut: None,
            follow_event_stream,
        }
    }
}

pub type FollowEventStream<Hash> =
    Pin<Box<dyn Stream<Item = FollowEvent<BlockRef<Hash>>> + Send + 'static>>;
pub type ContinueFutGetter = Box<dyn Fn() -> ContinueFut + Send + 'static>;
pub type ContinueFut = Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'static>>;

impl<T: Config> Stream for StorageItems<T> {
    type Item = Result<StorageResult, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if self.done {
                return Poll::Ready(None);
            }

            if let Some(item) = self.buffered_responses.pop_front() {
                return Poll::Ready(Some(Ok(item)));
            }

            if let Some(mut fut) = self.continue_fut.take() {
                match fut.poll_unpin(cx) {
                    Poll::Pending => {
                        self.continue_fut = Some(fut);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(e)) => {
                        self.done = true;
                        return Poll::Ready(Some(Err(e)));
                    }
                    Poll::Ready(Ok(())) => {
                        // Finished; carry on.
                    }
                }
            }

            let ev = match self.follow_event_stream.poll_next_unpin(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Ready(Some(ev)) => ev,
            };

            match ev {
                FollowEvent::OperationWaitingForContinue(id)
                    if id.operation_id == *self.operation_id =>
                {
                    // Start a call to ask for more events
                    self.continue_fut = Some((self.continue_call)());
                    continue;
                }
                FollowEvent::OperationStorageDone(id) if id.operation_id == *self.operation_id => {
                    // We're finished!
                    self.done = true;
                    return Poll::Ready(None);
                }
                FollowEvent::OperationStorageItems(items)
                    if items.operation_id == *self.operation_id =>
                {
                    // We have items; buffer them to emit next loops.
                    self.buffered_responses = items.items;
                    continue;
                }
                FollowEvent::OperationError(err) if err.operation_id == *self.operation_id => {
                    // Something went wrong obtaining storage items; mark as done and return the error.
                    self.done = true;
                    return Poll::Ready(Some(Err(Error::Other(err.error))));
                }
                _ => {
                    // We don't care about this event; wait for the next.
                    continue;
                }
            }
        }
    }
}
