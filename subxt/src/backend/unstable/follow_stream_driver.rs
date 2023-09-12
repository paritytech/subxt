// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::follow_stream_unpin::{BlockRef, FollowStreamUnpin};
use crate::backend::unstable::rpc_methods::FollowEvent;
use crate::config::BlockHash;
use crate::error::Error;
use futures::stream::{Stream, StreamExt};
use std::collections::{HashMap, VecDeque};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// The type of event we're emitting here.
type Ev<Hash> = FollowEvent<BlockRef<Hash>>;

/// This subscribes to chainHead_follow, and as long as it's being
/// polled it will receive pinned blocks, unpin them when appropriate
/// and broadcast the results to any interested subscribers.
#[derive(Debug)]
pub struct FollowStreamDriver<Hash: BlockHash> {
    inner: FollowStreamUnpin<Hash>,
    shared: Shared<Hash>,
}

impl<Hash: BlockHash> FollowStreamDriver<Hash> {
    /// Create a new [`FollowStreamDriver`]. This must be polled by some executor
    /// in order for any progress to be made. Things can subscribe to events.
    pub fn new(follow_unpin: FollowStreamUnpin<Hash>) -> Self {
        Self {
            inner: follow_unpin,
            shared: Shared::default(),
        }
    }

    /// Return a handle from which we can create new subscriptions to follow events.
    pub fn handle(&self) -> FollowStreamDriverHandle<Hash> {
        FollowStreamDriverHandle {
            shared: self.shared.clone(),
        }
    }
}

impl<Hash: BlockHash> Stream for FollowStreamDriver<Hash> {
    type Item = Result<(), Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let item = match self.inner.poll_next_unpin(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => {
                // Mark ourselves as done so that everything can end.
                self.shared.done();
                return Poll::Ready(None);
            }
            Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e))),
            Poll::Ready(Some(Ok(item))) => item,
        };

        self.shared.push_item(item);
        Poll::Ready(Some(Ok(())))
    }
}

/// A handle that can be used to create subscribers, but that doesn't
/// itself subscribe to events.
#[derive(Debug, Clone)]
pub struct FollowStreamDriverHandle<Hash: BlockHash> {
    shared: Shared<Hash>,
}

impl<Hash: BlockHash> FollowStreamDriverHandle<Hash> {
    /// Subscribe to follow events.
    pub fn subscribe(&self) -> FollowStreamDriverSubscription<Hash> {
        self.shared.subscribe()
    }
}

/// A subscription to events from the [`FollowStreamDriver`].
#[derive(Debug)]
pub struct FollowStreamDriverSubscription<Hash: BlockHash> {
    id: usize,
    done: bool,
    shared: Shared<Hash>,
    local_items: VecDeque<Ev<Hash>>,
}

impl<Hash: BlockHash> Stream for FollowStreamDriverSubscription<Hash> {
    type Item = Ev<Hash>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        loop {
            if let Some(item) = self.local_items.pop_front() {
                return Poll::Ready(Some(item));
            }

            let items = self.shared.take_items_and_save_waker(self.id, cx.waker());

            // If no items left, mark locally as done (to avoid further locking)
            // and return None to signal done-ness.
            let Some(items) = items else {
                self.done = true;
                return Poll::Ready(None);
            };

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

impl<Hash: BlockHash> Clone for FollowStreamDriverSubscription<Hash> {
    fn clone(&self) -> Self {
        self.shared.subscribe()
    }
}

impl<Hash: BlockHash> Drop for FollowStreamDriverSubscription<Hash> {
    fn drop(&mut self) {
        self.shared.remove_sub(self.id);
    }
}

/// Locked shared state.
#[derive(Debug, Clone)]
struct Shared<Hash: BlockHash>(Arc<Mutex<SharedState<Hash>>>);

impl<Hash: BlockHash> Default for Shared<Hash> {
    fn default() -> Self {
        Shared(Arc::new(Mutex::new(SharedState {
            next_id: 1,
            done: false,
            subscribers: HashMap::new(),
            block_events_from_last_finalized: VecDeque::new(),
        })))
    }
}

impl<Hash: BlockHash> Shared<Hash> {
    /// Set the shared state to "done"; no more items will be handed to it.
    pub fn done(&self) {
        let mut shared = self.0.lock().unwrap();
        shared.done = true;
    }

    /// Cleanup a subscription.
    pub fn remove_sub(&self, sub_id: usize) {
        let mut shared = self.0.lock().unwrap();
        shared.subscribers.remove(&sub_id);
    }

    /// Take items for some subscription ID and save the waker.
    pub fn take_items_and_save_waker(
        &self,
        sub_id: usize,
        waker: &Waker,
    ) -> Option<VecDeque<Ev<Hash>>> {
        let mut shared = self.0.lock().unwrap();

        let is_done = shared.done;
        let details = shared.subscribers.get_mut(&sub_id)?;

        // no more items to pull, and stream closed, so return None.
        if details.items.is_empty() && is_done {
            return None;
        }

        // else, take whatever items, and save the waker if not done yet.
        let items = std::mem::take(&mut details.items);
        if !is_done {
            details.waker = Some(waker.clone());
        }
        Some(items)
    }

    /// Push a new item out to subscribers.
    pub fn push_item(&self, item: Ev<Hash>) {
        let mut shared = self.0.lock().unwrap();

        // broadcast item to subscribers:
        for details in shared.subscribers.values_mut() {
            details.items.push_back(item.clone());
            if let Some(waker) = details.waker.take() {
                waker.wake();
            }
        }

        // Keep our buffer of block events from latest finalized block uptodate:
        if matches!(
            item,
            FollowEvent::Initialized(_)
                | FollowEvent::Finalized(_)
                | FollowEvent::NewBlock(_)
                | FollowEvent::BestBlockChanged(_)
        ) {
            if matches!(
                item,
                FollowEvent::Finalized(_) | FollowEvent::Initialized(_)
            ) {
                shared.block_events_from_last_finalized = VecDeque::new();
            }
            shared.block_events_from_last_finalized.push_back(item);
        }
    }

    /// Create a new subscription.
    pub fn subscribe(&self) -> FollowStreamDriverSubscription<Hash> {
        let mut shared = self.0.lock().unwrap();

        let id = shared.next_id;
        shared.next_id += 1;

        shared.subscribers.insert(
            id,
            SubscriberDetails {
                items: VecDeque::new(),
                waker: None,
            },
        );

        // The initial events in the stream will be any block events
        // since/including the last finalized block. This means that we can
        // immediately see the current finalized and best blocks for things like
        // submitting transactions, making calls etc.
        let block_events_so_far = shared.block_events_from_last_finalized.clone();

        drop(shared);

        FollowStreamDriverSubscription {
            id,
            done: false,
            shared: self.clone(),
            local_items: block_events_so_far,
        }
    }
}

/// Shared state.
#[derive(Debug)]
struct SharedState<Hash: BlockHash> {
    done: bool,
    next_id: usize,
    subscribers: HashMap<usize, SubscriberDetails<Hash>>,
    // Keep a buffer of all events from last finalized block so that new
    // subscriptions can be handed this info first.
    block_events_from_last_finalized: VecDeque<Ev<Hash>>,
}

/// Details for a given subscriber: any items it's not yet claimed,
/// and a way to wake it up when there are more items for it.
#[derive(Debug)]
struct SubscriberDetails<Hash: BlockHash> {
    items: VecDeque<Ev<Hash>>,
    waker: Option<Waker>,
}

#[cfg(test)]
mod test_utils {
    use super::super::follow_stream_unpin::test_utils::test_unpin_stream_getter;
    use super::*;

    /// Return a `FollowStreamDriver`
    pub fn test_follow_stream_driver_getter<Hash, F, I>(
        events: F,
        min_life: usize,
        max_life: usize,
    ) -> FollowStreamDriver<Hash>
    where
        Hash: BlockHash + 'static,
        F: Fn() -> I + Send + 'static,
        I: IntoIterator<Item = Result<FollowEvent<Hash>, Error>>,
    {
        let (stream, _) = test_unpin_stream_getter(events, min_life, max_life);
        FollowStreamDriver::new(stream)
    }
}

#[cfg(test)]
mod test {
    use super::super::follow_stream::test_utils::{
        ev_best_block, ev_finalized, ev_initialized, ev_new_block,
    };
    use super::super::follow_stream_unpin::test_utils::{
        ev_best_block_ref, ev_finalized_ref, ev_initialized_ref, ev_new_block_ref,
    };
    use super::test_utils::test_follow_stream_driver_getter;
    use super::*;

    #[test]
    fn follow_stream_driver_is_sendable() {
        fn assert_send<T: Send + 'static>(_: T) {}
        let stream_getter = test_follow_stream_driver_getter(|| [Ok(ev_initialized(1))], 0, 10);
        assert_send(stream_getter);
    }

    #[tokio::test]
    async fn subscribers_all_receive_events_and_finish_gracefully_on_error() {
        let mut driver = test_follow_stream_driver_getter(
            || {
                [
                    Ok(ev_initialized(0)),
                    Ok(ev_new_block(0, 1)),
                    Ok(ev_best_block(1)),
                    Ok(ev_finalized([1])),
                    Err(Error::Other("ended".to_owned())),
                ]
            },
            0,
            10,
        );

        let handle = driver.handle();

        let a = handle.subscribe();
        let b = handle.subscribe();
        let c = handle.subscribe();

        // Drive to completion (the sort of real life usage I'd expect):
        tokio::spawn(async move { while let Some(_) = driver.next().await {} });

        let a_vec: Vec<_> = a.collect().await;
        let b_vec: Vec<_> = b.collect().await;
        let c_vec: Vec<_> = c.collect().await;

        let expected = vec![
            ev_initialized_ref(0),
            ev_new_block_ref(0, 1),
            ev_best_block_ref(1),
            ev_finalized_ref([1]),
        ];

        assert_eq!(a_vec, expected);
        assert_eq!(b_vec, expected);
        assert_eq!(c_vec, expected);
    }

    #[tokio::test]
    async fn subscribers_receive_block_events_from_last_finalised() {
        let mut driver = test_follow_stream_driver_getter(
            || {
                [
                    Ok(ev_initialized(0)),
                    Ok(ev_new_block(0, 1)),
                    Ok(ev_best_block(1)),
                    Ok(ev_finalized([1])),
                    Ok(ev_new_block(1, 2)),
                    Ok(ev_new_block(2, 3)),
                    Err(Error::Other("ended".to_owned())),
                ]
            },
            0,
            10,
        );

        // Skip past init, new, best events.
        let _i0 = driver.next().await.unwrap();
        let _n1 = driver.next().await.unwrap();
        let _b1 = driver.next().await.unwrap();

        // THEN subscribe; subscription should still receive them:
        let evs: Vec<_> = driver.handle().subscribe().take(3).collect().await;
        let expected = vec![
            ev_initialized_ref(0),
            ev_new_block_ref(0, 1),
            ev_best_block_ref(1),
        ];
        assert_eq!(evs, expected);

        // Skip past finalized 1, new 2, new 3 events
        let _f1 = driver.next().await.unwrap();
        let _n2 = driver.next().await.unwrap();
        let _n3 = driver.next().await.unwrap();

        // THEN subscribe again; subscription should start at finalized 1.
        let evs: Vec<_> = driver.handle().subscribe().take(3).collect().await;
        let expected = vec![
            ev_finalized_ref([1]),
            ev_new_block_ref(1, 2),
            ev_new_block_ref(2, 3),
        ];
        assert_eq!(evs, expected);
    }
}
