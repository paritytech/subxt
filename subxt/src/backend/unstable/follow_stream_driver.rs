// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::follow_stream_unpin::{FollowStreamUnpin, BlockRef};
use super::UnstableRpcMethods;
use crate::config::{BlockHash, Config};
use crate::error::Error;
use futures::stream::{FuturesUnordered, Stream, StreamExt};
use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::task::{Context, Poll, Waker};
use crate::backend::unstable::rpc_methods::{
    BestBlockChanged, Finalized, FollowEvent, Initialized, NewBlock, OperationBodyDone,
    OperationCallDone, OperationError, OperationId, OperationStorageItems,
};

/// This subscribes to chainHead_follow, and as long as it's being
/// polled it will receive pinned blocks, unpin them when appropriate
/// and broadcast the results to any interested subscribers.
#[derive(Debug)]
pub struct FollowStreamDriver<Hash: BlockHash> {
    inner: FollowStreamUnpin<Hash>,
    shared: Shared<Hash>
}

#[derive(Debug)]
impl <Hash: BlockHash> FollowStreamDriver<Hash> {
    /// Return a handle from which we can create new subscriptions to follow events.
    pub fn handle(&self) -> FollowStreamDriverHandle<Hash> {
        FollowStreamDriverHandle { shared: self.shared.clone() }
    }
}

impl <Hash: BlockHash> Stream for FollowStreamDriver<Hash> {
    type Item = Result<(), Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let item = match self.inner.poll_next_unpin(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => return Poll::Ready(None),
            Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e))),
            Poll::Ready(Some(Ok(item))) => item
        };

        self.shared.push_item(item);
        Poll::Ready(Some(Ok(())))
    }
}

/// A handle that can be used to create subscribers, but that doesn't
/// itself subscribe to events.
#[derive(Debug, Clone)]
pub struct FollowStreamDriverHandle<Hash: BlockHash> {
    shared: Shared<Hash>
}

impl <Hash: BlockHash> FollowStreamDriverHandle<Hash> {
    /// Subscribe to follow events.
    pub fn subscribe(&self) -> FollowStreamDriverSubscription<Hash> {
        self.shared.subscribe()
    }
}

/// A subscription to events from the [`FollowStreamDriver`].
#[derive(Debug)]
pub struct FollowStreamDriverSubscription<Hash: BlockHash> {
    id: usize,
    shared: Shared<Hash>,
    local_items: VecDeque<FollowEvent<BlockRef<Hash>>>,
}

impl <Hash: BlockHash> Stream for FollowStreamDriverSubscription<Hash> {
    type Item = FollowEvent<BlockRef<Hash>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(item) = self.local_items.pop_front() {
                return Poll::Ready(Some(item))
            }

            let items = self
                .shared
                .take_items_and_save_waker(self.id, cx.waker())
                .expect("subscription should exist");

            // No items? We've saved the waker so we'll be told when more come.
            // Else, save the items locally and loop around to pop from them.
            if items.is_empty() {
                return Poll::Pending;
            } else {
                self.local_items = items;
            }
        }
    }
}

impl <Hash: BlockHash> Clone for FollowStreamDriverSubscription<Hash> {
    fn clone(&self) -> Self {
        self.shared.subscribe()
    }
}

impl <Hash: BlockHash> Drop for FollowStreamDriverSubscription<Hash> {
    fn drop(&mut self) {
        self.shared.remove_sub(self.id);
    }
}

/// Locked shared state.
#[derive(Debug, Clone)]
struct Shared<Hash: BlockHash>(Arc<Mutex<SharedState<Hash>>>);

impl <Hash: BlockHash> Shared<Hash> {
    /// Cleanup a subscription.
    pub fn remove_sub(&self, sub_id: usize) {
        let mut shared = self.0.lock().unwrap();
        shared.subscribers.remove(&sub_id);
    }

    /// Take items for some subscription ID and save the waker.
    pub fn take_items_and_save_waker(&self, sub_id: usize, waker: &Waker) -> Option<VecDeque<FollowEvent<BlockRef<Hash>>>> {
        let mut shared = self.0.lock().unwrap();
        let details = shared.subscribers.get_mut(&sub_id)?;
        let items = std::mem::take(&mut details.items);
        details.waker = Some(waker.clone());
        Some(items)
    }

    /// Push a new item out to subscribers.
    pub fn push_item(&self, item: FollowEvent<BlockRef<Hash>>) {
        let mut shared = self.0.lock().unwrap();
        for details in shared.subscribers.values_mut() {
            details.items.push_back(item.clone());
            if let Some(waker) = details.waker.take() {
                waker.wake();
            }
        }
    }

    /// Create a new subscription.
    pub fn subscribe(&self) -> FollowStreamDriverSubscription<Hash> {
        let mut shared = self.0.lock().unwrap();

        let id = shared.next_id;
        shared.next_id += 1;

        shared.subscribers.insert(id, SubscriberDetails {
            items: VecDeque::new(),
            waker: None
        });

        drop(shared);

        FollowStreamDriverSubscription {
            id,
            shared: self.clone(),
            local_items: VecDeque::new()
        }
    }
}

/// Shared state.
#[derive(Debug)]
struct SharedState<Hash: BlockHash> {
    next_id: usize,
    subscribers: HashMap<usize, SubscriberDetails<Hash>>
}

/// Details for a given subscriber: any items it's not yet claimed,
/// and a way to wake it up when there are more items for it.
#[derive(Debug)]
struct SubscriberDetails<Hash: BlockHash> {
    items: VecDeque<FollowEvent<BlockRef<Hash>>>,
    waker: Option<Waker>
}

