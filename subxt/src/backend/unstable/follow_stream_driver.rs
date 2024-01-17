// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::follow_stream_unpin::{BlockRef, FollowStreamMsg, FollowStreamUnpin};
use crate::backend::unstable::rpc_methods::{FollowEvent, Initialized, RuntimeEvent};
use crate::config::BlockHash;
use crate::error::Error;
use futures::stream::{Stream, StreamExt};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// A `Stream` which builds on `FollowStreamDriver`, and allows multiple subscribers to obtain events
/// from the single underlying subscription (each being provided an `Initialized` message and all new
/// blocks since then, as if they were each creating a unique `chainHead_follow` subscription). This
/// is the "top" layer of our follow stream subscriptions, and the one that's interacted with elsewhere.
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
        match self.inner.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => {
                // Mark ourselves as done so that everything can end.
                self.shared.done();
                Poll::Ready(None)
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(Some(Ok(item))) => {
                // Push item to any subscribers.
                self.shared.push_item(item);
                Poll::Ready(Some(Ok(())))
            }
        }
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

/// A subscription to events from the [`FollowStreamDriver`]. All subscriptions
/// begin first with a `Ready` event containing the current subscription ID, and
/// then with an `Initialized` event containing the latest finalized block and latest
/// runtime information, and then any new/best block events and so on received since
/// the latest finalized block.
#[derive(Debug)]
pub struct FollowStreamDriverSubscription<Hash: BlockHash> {
    id: usize,
    done: bool,
    shared: Shared<Hash>,
    local_items: VecDeque<FollowStreamMsg<BlockRef<Hash>>>,
}

impl<Hash: BlockHash> Stream for FollowStreamDriverSubscription<Hash> {
    type Item = FollowStreamMsg<BlockRef<Hash>>;

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

impl<Hash: BlockHash> FollowStreamDriverSubscription<Hash> {
    /// Return the current subscription ID. If the subscription has stopped, then this will
    /// wait until a new subscription has started with a new ID.
    pub async fn subscription_id(self) -> Option<String> {
        let ready_event = self
            .skip_while(|ev| std::future::ready(!matches!(ev, FollowStreamMsg::Ready(_))))
            .next()
            .await?;

        match ready_event {
            FollowStreamMsg::Ready(sub_id) => Some(sub_id),
            _ => None,
        }
    }

    /// Subscribe to the follow events, ignoring any other messages.
    pub fn events(self) -> impl Stream<Item = FollowEvent<BlockRef<Hash>>> + Send + Sync {
        self.filter_map(|ev| std::future::ready(ev.into_event()))
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

/// Locked shared state. The driver stream will access this state to push
/// events to any subscribers, and subscribers will access it to pull the
/// events destined for themselves.
#[derive(Debug, Clone)]
struct Shared<Hash: BlockHash>(Arc<Mutex<SharedState<Hash>>>);

#[derive(Debug)]
struct SharedState<Hash: BlockHash> {
    done: bool,
    next_id: usize,
    subscribers: HashMap<usize, SubscriberDetails<Hash>>,
    /// Keep a buffer of all events that should be handed to a new subscription.
    block_events_for_new_subscriptions: VecDeque<FollowEvent<BlockRef<Hash>>>,
    // Keep track of the subscription ID we send out on new subs.
    current_subscription_id: Option<String>,
    // Keep track of the init message we send out on new subs.
    current_init_message: Option<Initialized<BlockRef<Hash>>>,
    // Runtime events by block hash; we need to track these to know
    // whether the runtime has changed when we see a finalized block notification.
    seen_runtime_events: HashMap<Hash, RuntimeEvent>,
}

impl<Hash: BlockHash> Default for Shared<Hash> {
    fn default() -> Self {
        Shared(Arc::new(Mutex::new(SharedState {
            next_id: 1,
            done: false,
            subscribers: HashMap::new(),
            current_init_message: None,
            current_subscription_id: None,
            seen_runtime_events: HashMap::new(),
            block_events_for_new_subscriptions: VecDeque::new(),
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
    ) -> Option<VecDeque<FollowStreamMsg<BlockRef<Hash>>>> {
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
    pub fn push_item(&self, item: FollowStreamMsg<BlockRef<Hash>>) {
        let mut shared = self.0.lock().unwrap();
        let shared = shared.deref_mut();

        // broadcast item to subscribers:
        for details in shared.subscribers.values_mut() {
            details.items.push_back(item.clone());
            if let Some(waker) = details.waker.take() {
                waker.wake();
            }
        }

        // Keep our buffer of ready/block events up-to-date:
        match item {
            FollowStreamMsg::Ready(sub_id) => {
                // Set new subscription ID when it comes in.
                shared.current_subscription_id = Some(sub_id);
            }
            FollowStreamMsg::Event(FollowEvent::Initialized(ev)) => {
                // New subscriptions will be given this init message:
                shared.current_init_message = Some(ev.clone());
                // Clear block cache (since a new finalized block hash is seen):
                shared.block_events_for_new_subscriptions.clear();
            }
            FollowStreamMsg::Event(FollowEvent::Finalized(finalized_ev)) => {
                // Update the init message that we'll hand out to new subscriptions. If the init message
                // is `None` for some reason, we just ignore this step.
                if let Some(init_message) = &mut shared.current_init_message {
                    // Find the latest runtime update that's been finalized.
                    let newest_runtime = finalized_ev
                        .finalized_block_hashes
                        .iter()
                        .rev()
                        .filter_map(|h| shared.seen_runtime_events.get(&h.hash()).cloned())
                        .next();

                    shared.seen_runtime_events.clear();

                    if let Some(finalized) = finalized_ev.finalized_block_hashes.last() {
                        init_message.finalized_block_hash = finalized.clone();
                    }
                    if let Some(runtime_ev) = newest_runtime {
                        init_message.finalized_block_runtime = Some(runtime_ev);
                    }
                }

                // The last finalized block will be reported as Initialized by our driver,
                // therefore there is no need to report NewBlock and BestBlock events for it.
                // If the Finalized event reported multiple finalized hashes, we only care about
                // the state at the head of the chain, therefore it is correct to remove those as well.
                // Idem for the pruned hashes; they will never be reported again and we remove
                // them from the window of events.
                let to_remove: HashSet<Hash> = finalized_ev
                    .finalized_block_hashes
                    .iter()
                    .chain(finalized_ev.pruned_block_hashes.iter())
                    .map(|h| h.hash())
                    .collect();

                shared
                    .block_events_for_new_subscriptions
                    .retain(|ev| match ev {
                        FollowEvent::NewBlock(new_block_ev) => {
                            !to_remove.contains(&new_block_ev.block_hash.hash())
                        }
                        FollowEvent::BestBlockChanged(best_block_ev) => {
                            !to_remove.contains(&best_block_ev.best_block_hash.hash())
                        }
                        _ => true,
                    });
            }
            FollowStreamMsg::Event(FollowEvent::NewBlock(new_block_ev)) => {
                // If a new runtime is seen, note it so that when a block is finalized, we
                // can associate that with a runtime update having happened.
                if let Some(runtime_event) = &new_block_ev.new_runtime {
                    shared
                        .seen_runtime_events
                        .insert(new_block_ev.block_hash.hash(), runtime_event.clone());
                }

                shared
                    .block_events_for_new_subscriptions
                    .push_back(FollowEvent::NewBlock(new_block_ev));
            }
            FollowStreamMsg::Event(ev @ FollowEvent::BestBlockChanged(_)) => {
                shared.block_events_for_new_subscriptions.push_back(ev);
            }
            FollowStreamMsg::Event(FollowEvent::Stop) => {
                // On a stop event, clear everything. Wait for resubscription and new ready/initialised events.
                shared.block_events_for_new_subscriptions.clear();
                shared.current_subscription_id = None;
                shared.current_init_message = None;
            }
            _ => {
                // We don't buffer any other events.
            }
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

        // Any new subscription should start with a "Ready" message and then an "Initialized"
        // message, and then any non-finalized block events since that. If these don't exist,
        // it means the subscription is currently stopped, and we should expect new Ready/Init
        // messages anyway once it restarts.
        let mut local_items = VecDeque::new();
        if let Some(sub_id) = &shared.current_subscription_id {
            local_items.push_back(FollowStreamMsg::Ready(sub_id.clone()));
        }
        if let Some(init_msg) = &shared.current_init_message {
            local_items.push_back(FollowStreamMsg::Event(FollowEvent::Initialized(
                init_msg.clone(),
            )));
        }
        for ev in &shared.block_events_for_new_subscriptions {
            local_items.push_back(FollowStreamMsg::Event(ev.clone()));
        }

        drop(shared);

        FollowStreamDriverSubscription {
            id,
            done: false,
            shared: self.clone(),
            local_items,
        }
    }
}

/// Details for a given subscriber: any items it's not yet claimed,
/// and a way to wake it up when there are more items for it.
#[derive(Debug)]
struct SubscriberDetails<Hash: BlockHash> {
    items: VecDeque<FollowStreamMsg<BlockRef<Hash>>>,
    waker: Option<Waker>,
}

#[cfg(test)]
mod test_utils {
    use super::super::follow_stream_unpin::test_utils::test_unpin_stream_getter;
    use super::*;

    /// Return a `FollowStreamDriver`
    pub fn test_follow_stream_driver_getter<Hash, F, I>(
        events: F,
        max_life: usize,
    ) -> FollowStreamDriver<Hash>
    where
        Hash: BlockHash + 'static,
        F: Fn() -> I + Send + 'static,
        I: IntoIterator<Item = Result<FollowEvent<Hash>, Error>>,
    {
        let (stream, _) = test_unpin_stream_getter(events, max_life);
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
        let stream_getter = test_follow_stream_driver_getter(|| [Ok(ev_initialized(1))], 10);
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
                    Ok(ev_finalized([1], [])),
                    Err(Error::Other("ended".to_owned())),
                ]
            },
            10,
        );

        let handle = driver.handle();

        let a = handle.subscribe();
        let b = handle.subscribe();
        let c = handle.subscribe();

        // Drive to completion (the sort of real life usage I'd expect):
        tokio::spawn(async move { while driver.next().await.is_some() {} });

        let a_vec: Vec<_> = a.collect().await;
        let b_vec: Vec<_> = b.collect().await;
        let c_vec: Vec<_> = c.collect().await;

        let expected = vec![
            FollowStreamMsg::Ready("sub_id_0".into()),
            FollowStreamMsg::Event(ev_initialized_ref(0)),
            FollowStreamMsg::Event(ev_new_block_ref(0, 1)),
            FollowStreamMsg::Event(ev_best_block_ref(1)),
            FollowStreamMsg::Event(ev_finalized_ref([1])),
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
                    Ok(ev_finalized([1], [])),
                    Ok(ev_new_block(1, 2)),
                    Ok(ev_new_block(2, 3)),
                    Err(Error::Other("ended".to_owned())),
                ]
            },
            10,
        );

        // Skip past ready, init, new, best events.
        let _r = driver.next().await.unwrap();
        let _i0 = driver.next().await.unwrap();
        let _n1 = driver.next().await.unwrap();
        let _b1 = driver.next().await.unwrap();

        // THEN subscribe; subscription should still receive them:
        let evs: Vec<_> = driver.handle().subscribe().take(4).collect().await;
        let expected = vec![
            FollowStreamMsg::Ready("sub_id_0".into()),
            FollowStreamMsg::Event(ev_initialized_ref(0)),
            FollowStreamMsg::Event(ev_new_block_ref(0, 1)),
            FollowStreamMsg::Event(ev_best_block_ref(1)),
        ];
        assert_eq!(evs, expected);

        // Skip past finalized 1, new 2, new 3 events
        let _f1 = driver.next().await.unwrap();
        let _n2 = driver.next().await.unwrap();
        let _n3 = driver.next().await.unwrap();

        // THEN subscribe again; new subs will see an updated initialized message
        // with the latest finalized block hash.
        let evs: Vec<_> = driver.handle().subscribe().take(4).collect().await;
        let expected = vec![
            FollowStreamMsg::Ready("sub_id_0".into()),
            FollowStreamMsg::Event(ev_initialized_ref(1)),
            FollowStreamMsg::Event(ev_new_block_ref(1, 2)),
            FollowStreamMsg::Event(ev_new_block_ref(2, 3)),
        ];
        assert_eq!(evs, expected);
    }

    #[tokio::test]
    async fn subscribers_receive_new_blocks_before_subscribing() {
        let mut driver = test_follow_stream_driver_getter(
            || {
                [
                    Ok(ev_initialized(0)),
                    Ok(ev_new_block(0, 1)),
                    Ok(ev_best_block(1)),
                    Ok(ev_new_block(1, 2)),
                    Ok(ev_new_block(2, 3)),
                    Ok(ev_finalized([1], [])),
                    Err(Error::Other("ended".to_owned())),
                ]
            },
            10,
        );

        // Skip to the first finalized block F1.
        let _r = driver.next().await.unwrap();
        let _i0 = driver.next().await.unwrap();
        let _n1 = driver.next().await.unwrap();
        let _b1 = driver.next().await.unwrap();
        let _n2 = driver.next().await.unwrap();
        let _n3 = driver.next().await.unwrap();
        let _f1 = driver.next().await.unwrap();

        // THEN subscribe; and make sure new block 1 and 2 are received.
        let evs: Vec<_> = driver.handle().subscribe().take(4).collect().await;
        let expected = vec![
            FollowStreamMsg::Ready("sub_id_0".into()),
            FollowStreamMsg::Event(ev_initialized_ref(1)),
            FollowStreamMsg::Event(ev_new_block_ref(1, 2)),
            FollowStreamMsg::Event(ev_new_block_ref(2, 3)),
        ];
        assert_eq!(evs, expected);
    }
}
