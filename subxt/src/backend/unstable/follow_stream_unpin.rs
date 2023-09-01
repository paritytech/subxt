// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::UnstableRpcMethods;
use super::follow_stream::{ FollowStream, FollowStreamMsg };
use std::sync::{ Mutex, Arc };
use std::collections::{ HashMap, HashSet };
use std::future::Future;
use std::pin::Pin;
use std::task::{ Context, Poll, Waker };
use crate::error::Error;
use crate::config::{Config, BlockHash};
use futures::stream::{ Stream, StreamExt, FuturesUnordered };

// Events we'll emit here.
pub use crate::backend::unstable::rpc_methods::{
    FollowEvent,
    Initialized, NewBlock, BestBlockChanged, Finalized,
    OperationBodyDone, OperationCallDone, OperationStorageItems,
    OperationId, OperationError
};

/// This subscribes to `chainHead_follow` when polled, and also
/// keeps track of pinned blocks, unpinning anything that gets too
/// old. When blocks that are handed out are dropped, they are also
/// unpinned.
pub struct FollowStreamUnpin<Hash: BlockHash> {
    // The underlying stream of events.
    inner: FollowStream<Hash>,
    // A method to call to unpin a block, given a block hash and a subscription ID.
    unpin_method: UnpinMethod<Hash>,
    // Futures for sending unpin events that we'll poll to completion as
    // part of polling the stream as a whole.
    unpin_futs: FuturesUnordered<UnpinFut>,
    // Each new finalized block increments this. Allows us to track
    // the age of blocks so that we can unpin old ones.
    rel_block_num: usize,
    // The latest ID of the FollowStream subscription, which we can use
    // to unpin blocks.
    subscription_id: Option<Arc<str>>,
    // The longest period a block can be pinned for.
    max_block_life: usize,
    // The shortest period a block will be pinned for.
    min_block_life: usize,
    // The currently seen and pinned blocks.
    pinned: HashMap<Hash, PinnedDetails<Hash>>,
    // Shared state about blocks we've flagged to unpin from elsewhere
    unpin_flags: UnpinFlags<Hash>,
}

/// The type of the unpin method that we need to provide.
pub type UnpinMethod<Hash> = Box<dyn FnMut(Hash, Arc<str>) -> UnpinFut>;

/// The future returned from [`UnpinMethod`].
pub type UnpinFut = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

impl <Hash: BlockHash> std::marker::Unpin for FollowStreamUnpin<Hash> {}

impl <Hash: BlockHash> Stream for FollowStreamUnpin<Hash> {
    type Item = Result<FollowEvent<BlockRef<Hash>>, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();

        loop {
            // Poll the unpin tasks until we are pending (at this point
            // we know we'll be woken again once something becomes ready).
            if let Poll::Ready(_) = this.unpin_futs.poll_next_unpin(cx) {
                continue;
            };

            // Poll the inner stream for the next event.
            let Poll::Ready(ev) = this.inner.poll_next_unpin(cx) else {
                return Poll::Pending
            };

            // No more progress to be made if inner stream done.
            let Some(ev) = ev else {
                return Poll::Ready(None)
            };

            // Error? just return it and do nothing further.
            let ev = match ev {
                Ok(ev) => ev,
                Err(e) => { return Poll::Ready(Some(Err(e))); }
            };

            // Update subscription ID if a new one comes in.
            let ev = match ev {
                FollowStreamMsg::Ready(subscription_id) => {
                    // update the subscription ID we'll use to unpin things.
                    this.subscription_id = Some(subscription_id.into());
                    // nothing to return; loop around.
                    continue;
                },
                FollowStreamMsg::Event(ev) => ev
            };

            // React to any actual FollowEvent we get back.
            let ev = match ev {
                FollowEvent::Initialized(details) => {
                    // The first finalized block gets the starting block_num.
                    let rel_block_num = this.rel_block_num;
                    let block_ref = this.pin_block_at(rel_block_num, details.finalized_block_hash);

                    FollowEvent::Initialized(Initialized {
                        finalized_block_hash: block_ref,
                        finalized_block_runtime: details.finalized_block_runtime
                    })
                },
                FollowEvent::NewBlock(details) => {
                    // One bigger than our parent, and if no parent seen (maybe it was
                    // unpinned already), then one bigger than the last finalized block num
                    // as a best guess.
                    let rel_block_num = this.pinned
                        .get(&details.parent_block_hash)
                        .map(|p| p.rel_block_num)
                        .unwrap_or(this.rel_block_num) + 1;

                    let block_ref = this.pin_block_at(rel_block_num, details.block_hash);
                    let parent_block_ref = this.pin_block_at(rel_block_num, details.parent_block_hash);

                    FollowEvent::NewBlock(NewBlock {
                        block_hash: block_ref,
                        parent_block_hash: parent_block_ref,
                        new_runtime: details.new_runtime
                    })
                },
                FollowEvent::BestBlockChanged(details) => {
                    // We expect this block to already exist, so it'll keep its existing block_num,
                    // but worst case it'll just get the current finalized block_num + 1.
                    let rel_block_num = this.rel_block_num + 1;
                    let block_ref = this.pin_block_at(rel_block_num, details.best_block_hash);

                    FollowEvent::BestBlockChanged(BestBlockChanged {
                        best_block_hash: block_ref
                    })
                },
                FollowEvent::Finalized(details) => {
                    let finalized_block_refs: Vec<_> = details.finalized_block_hashes
                        .into_iter()
                        .enumerate()
                        .map(|(idx, hash)| {
                            // These blocks _should_ exist already and so will have a known block num,
                            // but if they don't, we just increment the num from the last finalized block
                            // we saw, which should be accurate.
                            let rel_block_num = this.rel_block_num + idx + 1;
                            this.pin_block_at(rel_block_num, hash)
                        })
                        .collect();

                    let pruned_block_refs: Vec<_> = details.pruned_block_hashes
                        .into_iter()
                        .map(|hash| {
                            // We should know about these, too, and if not we set their age to last_finalized + 1
                            let rel_block_num = this.rel_block_num + 1;
                            this.pin_block_at(rel_block_num, hash)
                        })
                        .collect();

                    // At this point, we also check to see which blocks we should submit unpin events
                    // for. When we see a block hash as finalized, we know that it won't be reported again
                    // (except as a parent hash of a new block), so we can safely make an unpin call for it
                    // without worrying about the hash being returned again despite the block not being pinned.
                    this.unpin_blocks(cx.waker());

                    FollowEvent::Finalized(Finalized {
                        finalized_block_hashes: finalized_block_refs,
                        pruned_block_hashes: pruned_block_refs
                    })
                },
                FollowEvent::Stop => {
                    // clear out "old" things that are no longer applicable since
                    // the subscription has ended (a new one will be created under the hood).
                    this.pinned.clear();
                    this.unpin_futs.clear();
                    this.unpin_flags.lock().unwrap().clear();
                    this.rel_block_num = 0;

                    FollowEvent::Stop
                },
                // These events aren't intresting; we just forward them on:
                FollowEvent::OperationBodyDone(details) => {
                    FollowEvent::OperationBodyDone(details)
                },
                FollowEvent::OperationCallDone(details) => {
                    FollowEvent::OperationCallDone(details)
                },
                FollowEvent::OperationStorageItems(details) => {
                    FollowEvent::OperationStorageItems(details)
                },
                FollowEvent::OperationWaitingForContinue(details) => {
                    FollowEvent::OperationWaitingForContinue(details)
                },
                FollowEvent::OperationStorageDone(details) => {
                    FollowEvent::OperationStorageDone(details)
                },
                FollowEvent::OperationInaccessible(details) => {
                    FollowEvent::OperationInaccessible(details)
                },
                FollowEvent::OperationError(details) => {
                    FollowEvent::OperationError(details)
                },
            };

            // Return our event.
            return Poll::Ready(Some(Ok(ev)));
        }
    }
}

impl <Hash: BlockHash> FollowStreamUnpin<Hash> {
    /// Create a new [`FollowStreamUnpin`].
    pub fn new(
        follow_stream: FollowStream<Hash>,
        unpin_method: UnpinMethod<Hash>,
        min_block_life: usize,
        max_block_life: usize,
    ) -> Self {
        Self {
            inner: follow_stream,
            unpin_method,
            min_block_life,
            max_block_life,
            pinned: Default::default(),
            subscription_id: None,
            rel_block_num: 0,
            unpin_flags: Default::default(),
            unpin_futs: Default::default(),
        }
    }

    /// Create a new [`FollowStreamUnpin`] given the RPC methods.
    pub fn from_methods<T: Config>(
        follow_stream: FollowStream<T::Hash>,
        methods: UnstableRpcMethods<T>,
        min_block_life: usize,
        max_block_life: usize,
    ) -> FollowStreamUnpin<T::Hash> {
        let unpin_method = Box::new(move |hash: T::Hash, sub_id: Arc<str>| {
            let methods = methods.clone();
            let fut: UnpinFut = Box::pin(async move {
                // We ignore any errors trying to unpin at the moment.
                let _ = methods.chainhead_unstable_unpin(&sub_id, hash).await;
            });
            fut
        });

        FollowStreamUnpin::new(
            follow_stream,
            unpin_method,
            min_block_life,
            max_block_life
        )
    }

    /// Pin a block, or return the reference to an already-pinned block. If the block has been registered to
    /// be unpinned, we'll clear those flags, so that it won't be unpinned. If the unpin request has already
    /// been sent though, then the block will be unpinned.
    fn pin_block_at(&mut self, rel_block_num: usize, hash: Hash) -> BlockRef<Hash> {
        let entry =self.pinned.entry(hash)
            // Only if there's already an entry do we need to clear any unpin flags set against it.
            .and_modify(|_| {
                self.unpin_flags.lock().unwrap().remove(&hash);
            })
            // If there's not an entry already, make one and return it.
            .or_insert_with(|| PinnedDetails {
                rel_block_num,
                block_ref: BlockRef { inner: Arc::new(BlockRefInner { hash, unpin_flags: self.unpin_flags.clone() }) }
            });

        entry.block_ref.clone()
    }

    /// Unpin any blocks that are either too old, or have the unpin flag set and are old enough.
    fn unpin_blocks(&mut self, waker: &Waker) {
        let unpin_flags = std::mem::take(&mut *self.unpin_flags.lock().unwrap());
        let rel_block_num = self.rel_block_num;

        // If we asked to unpin and there was no subscription_id, then there's nothing to
        // do here, and we've cleared the flags now above anyway.
        let Some(sub_id) = &self.subscription_id else {
            return;
        };

        let mut blocks_to_unpin = vec![];
        for (hash, details) in &self.pinned {
            if rel_block_num - details.rel_block_num > self.max_block_life {
                // The block is too old so it has to go.
                blocks_to_unpin.push(*hash);
            } else if rel_block_num - details.rel_block_num > self.min_block_life && unpin_flags.contains(hash) {
                // the block is old enough to be unpinned, and is flagged as such.
                blocks_to_unpin.push(*hash);
            }
        }

        if blocks_to_unpin.is_empty() {
            return
        }

        for hash in blocks_to_unpin {
            let fut = (self.unpin_method)(hash, sub_id.clone());
            self.unpin_futs.push(fut);
        }

        // Any new futures pushed above need polling to start. We could
        // just wait for the next stream event, but let's wake the task to
        // have it polled sooner, just incase it's slow to receive things.
        waker.wake_by_ref();
    }
}

// The set of block hashes that can be unpinned when ready.
// BlockRefs write to this when they are dropped.
type UnpinFlags<Hash> = Arc<Mutex<HashSet<Hash>>>;

struct PinnedDetails<Hash: BlockHash> {
    /// How old is the block?
    rel_block_num: usize,
    /// A block ref we can hand out to keep blocks pinned.
    /// Because we store one here until it's unpinned, the live count
    /// will only drop to 1 when no external refs are left.
    block_ref: BlockRef<Hash>
}

/// All blocks reported will be wrapped in this.
#[derive(Debug, Clone)]
pub struct BlockRef<Hash: BlockHash> {
    inner: Arc<BlockRefInner<Hash>>,
}

#[derive(Debug)]
struct BlockRefInner<Hash> {
    hash: Hash,
    unpin_flags: UnpinFlags<Hash>
}

impl <Hash: BlockHash> Drop for BlockRef<Hash> {
    fn drop(&mut self) {
        // PinnedDetails keeps one ref, so if this is the second ref, it's the
        // only "external" one left and we should ask to unpin it now. if it's
        // the only ref remaining, it means that it's already been unpinned, so
        // nothing to do here anyway.
        if Arc::strong_count(&self.inner) == 2 {
            if let Ok(mut unpin_flags) = self.inner.unpin_flags.lock() {
                unpin_flags.insert(self.inner.hash);
            }
        }
    }
}
