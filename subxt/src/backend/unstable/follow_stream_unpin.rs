// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::follow_stream::{FollowStream, FollowStreamMsg};
use super::UnstableRpcMethods;
use crate::config::{BlockHash, Config};
use crate::error::Error;
use futures::stream::{FuturesUnordered, Stream, StreamExt};
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

// Events we'll emit here.
pub use crate::backend::unstable::rpc_methods::{
    BestBlockChanged, Finalized, FollowEvent, Initialized, NewBlock, OperationBodyDone,
    OperationCallDone, OperationError, OperationId, OperationStorageItems,
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

impl<Hash: BlockHash> std::marker::Unpin for FollowStreamUnpin<Hash> {}

impl<Hash: BlockHash> Stream for FollowStreamUnpin<Hash> {
    type Item = Result<FollowEvent<BlockRef<Hash>>, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();

        loop {
            // Poll the unpin tasks while they are completing. if we get back None, then
            // no tasks in the list, and if pending, we'll be woken when we can poll again.
            if let Poll::Ready(Some(())) = this.unpin_futs.poll_next_unpin(cx) {
                continue;
            };

            // Poll the inner stream for the next event.
            let Poll::Ready(ev) = this.inner.poll_next_unpin(cx) else {
                return Poll::Pending;
            };

            // No more progress to be made if inner stream done.
            let Some(ev) = ev else {
                return Poll::Ready(None);
            };

            // Error? just return it and do nothing further.
            let ev = match ev {
                Ok(ev) => ev,
                Err(e) => {
                    return Poll::Ready(Some(Err(e)));
                }
            };

            // Update subscription ID if a new one comes in.
            let ev = match ev {
                FollowStreamMsg::Ready(subscription_id) => {
                    // update the subscription ID we'll use to unpin things.
                    this.subscription_id = Some(subscription_id.into());
                    // nothing to return; loop around.
                    continue;
                }
                FollowStreamMsg::Event(ev) => ev,
            };

            // React to any actual FollowEvent we get back.
            let ev = match ev {
                FollowEvent::Initialized(details) => {
                    // The first finalized block gets the starting block_num.
                    let rel_block_num = this.rel_block_num;
                    let block_ref = this.pin_block_at(rel_block_num, details.finalized_block_hash);

                    FollowEvent::Initialized(Initialized {
                        finalized_block_hash: block_ref,
                        finalized_block_runtime: details.finalized_block_runtime,
                    })
                }
                FollowEvent::NewBlock(details) => {
                    // One bigger than our parent, and if no parent seen (maybe it was
                    // unpinned already), then one bigger than the last finalized block num
                    // as a best guess.
                    let parent_rel_block_num = this
                        .pinned
                        .get(&details.parent_block_hash)
                        .map(|p| p.rel_block_num)
                        .unwrap_or(this.rel_block_num);

                    let block_ref = this.pin_block_at(parent_rel_block_num + 1, details.block_hash);
                    let parent_block_ref =
                        this.pin_block_at(parent_rel_block_num, details.parent_block_hash);

                    FollowEvent::NewBlock(NewBlock {
                        block_hash: block_ref,
                        parent_block_hash: parent_block_ref,
                        new_runtime: details.new_runtime,
                    })
                }
                FollowEvent::BestBlockChanged(details) => {
                    // We expect this block to already exist, so it'll keep its existing block_num,
                    // but worst case it'll just get the current finalized block_num + 1.
                    let rel_block_num = this.rel_block_num + 1;
                    let block_ref = this.pin_block_at(rel_block_num, details.best_block_hash);

                    FollowEvent::BestBlockChanged(BestBlockChanged {
                        best_block_hash: block_ref,
                    })
                }
                FollowEvent::Finalized(details) => {
                    let finalized_block_refs: Vec<_> = details
                        .finalized_block_hashes
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

                    // Our relative block height is increased by however many finalized
                    // blocks we've seen.
                    this.rel_block_num += finalized_block_refs.len();

                    let pruned_block_refs: Vec<_> = details
                        .pruned_block_hashes
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
                        pruned_block_hashes: pruned_block_refs,
                    })
                }
                FollowEvent::Stop => {
                    // clear out "old" things that are no longer applicable since
                    // the subscription has ended (a new one will be created under the hood).
                    this.pinned.clear();
                    this.unpin_futs.clear();
                    this.unpin_flags.lock().unwrap().clear();
                    this.rel_block_num = 0;

                    FollowEvent::Stop
                }
                // These events aren't intresting; we just forward them on:
                FollowEvent::OperationBodyDone(details) => FollowEvent::OperationBodyDone(details),
                FollowEvent::OperationCallDone(details) => FollowEvent::OperationCallDone(details),
                FollowEvent::OperationStorageItems(details) => {
                    FollowEvent::OperationStorageItems(details)
                }
                FollowEvent::OperationWaitingForContinue(details) => {
                    FollowEvent::OperationWaitingForContinue(details)
                }
                FollowEvent::OperationStorageDone(details) => {
                    FollowEvent::OperationStorageDone(details)
                }
                FollowEvent::OperationInaccessible(details) => {
                    FollowEvent::OperationInaccessible(details)
                }
                FollowEvent::OperationError(details) => FollowEvent::OperationError(details),
            };

            // Return our event.
            return Poll::Ready(Some(Ok(ev)));
        }
    }
}

impl<Hash: BlockHash> FollowStreamUnpin<Hash> {
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

        FollowStreamUnpin::new(follow_stream, unpin_method, min_block_life, max_block_life)
    }

    /// Is the block hash currently pinned.
    pub fn is_pinned(&self, hash: &Hash) -> bool {
        self.pinned.contains_key(hash)
    }

    /// Pin a block, or return the reference to an already-pinned block. If the block has been registered to
    /// be unpinned, we'll clear those flags, so that it won't be unpinned. If the unpin request has already
    /// been sent though, then the block will be unpinned.
    fn pin_block_at(&mut self, rel_block_num: usize, hash: Hash) -> BlockRef<Hash> {
        let entry = self
            .pinned
            .entry(hash)
            // Only if there's already an entry do we need to clear any unpin flags set against it.
            .and_modify(|_| {
                self.unpin_flags.lock().unwrap().remove(&hash);
            })
            // If there's not an entry already, make one and return it.
            .or_insert_with(|| PinnedDetails {
                rel_block_num,
                block_ref: BlockRef {
                    inner: Arc::new(BlockRefInner {
                        hash,
                        unpin_flags: self.unpin_flags.clone(),
                    }),
                },
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
            if rel_block_num.saturating_sub(details.rel_block_num) >= self.max_block_life {
                // The block is too old so it has to go.
                blocks_to_unpin.push(*hash);
            } else if rel_block_num.saturating_sub(details.rel_block_num) >= self.min_block_life
                && unpin_flags.contains(hash)
            {
                // the block is old enough to be unpinned, and is flagged as such.
                blocks_to_unpin.push(*hash);
            }
        }

        if blocks_to_unpin.is_empty() {
            return;
        }

        for hash in blocks_to_unpin {
            self.pinned.remove(&hash);
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

#[derive(Debug)]
struct PinnedDetails<Hash: BlockHash> {
    /// How old is the block?
    rel_block_num: usize,
    /// A block ref we can hand out to keep blocks pinned.
    /// Because we store one here until it's unpinned, the live count
    /// will only drop to 1 when no external refs are left.
    block_ref: BlockRef<Hash>,
}

/// All blocks reported will be wrapped in this.
#[derive(Debug, Clone)]
pub struct BlockRef<Hash: BlockHash> {
    inner: Arc<BlockRefInner<Hash>>,
}

#[derive(Debug)]
struct BlockRefInner<Hash> {
    hash: Hash,
    unpin_flags: UnpinFlags<Hash>,
}

impl<Hash: BlockHash> BlockRef<Hash> {
    // Return the hash for this block.
    pub fn hash(&self) -> Hash {
        self.inner.hash
    }
}

impl<Hash: BlockHash> PartialEq for BlockRef<Hash> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.hash == other.inner.hash
    }
}

impl<Hash: BlockHash> PartialEq<Hash> for BlockRef<Hash> {
    fn eq(&self, other: &Hash) -> bool {
        &self.inner.hash == other
    }
}

impl<Hash: BlockHash> Drop for BlockRef<Hash> {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::backend::unstable::follow_stream::{test_utils::test_stream_getter, FollowStream};
    use crate::backend::unstable::rpc_methods::{Finalized, Initialized, NewBlock};
    use crate::config::substrate::H256;
    use assert_matches::assert_matches;
    use std::time::Duration;

    type UnpinRx<Hash> = std::sync::mpsc::Receiver<(Hash, Arc<str>)>;

    fn test_unpin_stream_getter<Hash, F, I>(
        events: F,
        min_life: usize,
        max_life: usize,
    ) -> (FollowStreamUnpin<Hash>, UnpinRx<Hash>)
    where
        Hash: BlockHash + 'static,
        F: Fn() -> I + Send + 'static,
        I: IntoIterator<Item = Result<FollowEvent<Hash>, Error>>,
    {
        // Unpin requests will come here so that we can look out for them.
        let (unpin_tx, unpin_rx) = std::sync::mpsc::channel();

        let follow_stream = FollowStream::new(test_stream_getter(events));
        let unpin_method: UnpinMethod<Hash> = Box::new(move |hash, sub_id| {
            unpin_tx.send((hash, sub_id)).unwrap();
            Box::pin(std::future::ready(()))
        });

        let follow_unpin = FollowStreamUnpin::new(follow_stream, unpin_method, min_life, max_life);
        (follow_unpin, unpin_rx)
    }

    fn ev_initialized(n: u64) -> FollowEvent<H256> {
        FollowEvent::Initialized(Initialized {
            finalized_block_hash: H256::from_low_u64_le(n),
            finalized_block_runtime: None,
        })
    }

    fn ev_new_block(parent: u64, n: u64) -> FollowEvent<H256> {
        FollowEvent::NewBlock(NewBlock {
            parent_block_hash: H256::from_low_u64_le(parent),
            block_hash: H256::from_low_u64_le(n),
            new_runtime: None,
        })
    }

    fn ev_finalized(ns: impl IntoIterator<Item = u64>) -> FollowEvent<H256> {
        FollowEvent::Finalized(Finalized {
            finalized_block_hashes: ns.into_iter().map(H256::from_low_u64_le).collect(),
            pruned_block_hashes: vec![],
        })
    }

    #[tokio::test]
    async fn hands_back_blocks() {
        let (follow_unpin, _) = test_unpin_stream_getter(
            || {
                [
                    Ok(ev_new_block(0, 1)),
                    Ok(ev_new_block(1, 2)),
                    Ok(ev_new_block(2, 3)),
                    Err(Error::Other("ended".to_owned())),
                ]
            },
            0,
            10,
        );

        let mut out: Vec<_> = follow_unpin
            .filter_map(|e| async move { e.ok() })
            .collect()
            .await;

        assert_eq!(out.len(), 3);
        assert_matches!(
            out.remove(0),
            FollowEvent::NewBlock(NewBlock {
                parent_block_hash,
                block_hash,
                ..
            }) if parent_block_hash == H256::from_low_u64_le(0) && block_hash == H256::from_low_u64_le(1)
        );
        assert_matches!(
            out.remove(0),
            FollowEvent::NewBlock(NewBlock {
                parent_block_hash,
                block_hash,
                ..
            }) if parent_block_hash == H256::from_low_u64_le(1) && block_hash == H256::from_low_u64_le(2)
        );
        assert_matches!(
            out.remove(0),
            FollowEvent::NewBlock(NewBlock {
                parent_block_hash,
                block_hash,
                ..
            }) if parent_block_hash == H256::from_low_u64_le(2) && block_hash == H256::from_low_u64_le(3)
        );
    }

    #[tokio::test]
    async fn unpins_old_blocks() {
        let (mut follow_unpin, unpin_rx) = test_unpin_stream_getter(
            || {
                [
                    Ok(ev_new_block(0, 1)),
                    Ok(ev_new_block(1, 2)),
                    Ok(ev_new_block(2, 3)),
                    Ok(ev_finalized([1, 2, 3])),
                    Err(Error::Other("ended".to_owned())),
                ]
            },
            0,
            2,
        );

        let _n1 = follow_unpin.next().await.unwrap().unwrap();
        let _n2 = follow_unpin.next().await.unwrap().unwrap();
        let _n3 = follow_unpin.next().await.unwrap().unwrap();

        // n1 is old but no finalized ev for it yet so nothing back.
        unpin_rx
            .recv_timeout(Duration::from_millis(50))
            .expect_err("nothing yet");

        // Finalized event will unpin old blocks. max age is 2 here, so first new block too old.
        let _f123 = follow_unpin.next().await.unwrap().unwrap();

        // Check that we get the expected unpin event.
        let (h, s) = unpin_rx
            .recv_timeout(Duration::from_millis(50))
            .expect("unpin call should have happened");
        assert_eq!(h, H256::from_low_u64_le(1));
        assert_eq!(&*s, "sub_id_0");
    }

    #[tokio::test]
    async fn unpins_dropped_blocks() {
        let (mut follow_unpin, unpin_rx) = test_unpin_stream_getter(
            || {
                [
                    Ok(ev_initialized(0)),
                    Ok(ev_new_block(0, 1)),
                    Ok(ev_finalized([1])),
                    Ok(ev_new_block(1, 2)),
                    Ok(ev_finalized([2])),
                    Err(Error::Other("ended".to_owned())),
                ]
            },
            0,
            10,
        );

        let _i0 = follow_unpin.next().await.unwrap().unwrap();
        let n1 = follow_unpin.next().await.unwrap().unwrap();
        let f1 = follow_unpin.next().await.unwrap().unwrap();
        let n2 = follow_unpin.next().await.unwrap().unwrap();

        // all are still alive and shouldnt be unpinned.
        unpin_rx
            .recv_timeout(Duration::from_millis(50))
            .expect_err("nothing yet");

        // We don't care about 1 any more; drop all references to it (it's a parent in n2);
        // block 0 is still referenced and block 2 will be referenced by f2.
        drop(n1);
        drop(f1);
        drop(n2);

        // Now, next time a finalized event is seen we'll unpin block 1.
        let _f2 = follow_unpin.next().await.unwrap().unwrap();

        // Check that we get the expected unpin event.
        let (h, s) = unpin_rx
            .recv_timeout(Duration::from_millis(50))
            .expect("unpin call should have happened");
        assert_eq!(h, H256::from_low_u64_le(1));
        assert_eq!(&*s, "sub_id_0");

        // Confirm that 0 and 2 are still pinned and that 1 isnt.
        assert!(!follow_unpin.is_pinned(&H256::from_low_u64_le(1)));
        assert!(follow_unpin.is_pinned(&H256::from_low_u64_le(0)));
        assert!(follow_unpin.is_pinned(&H256::from_low_u64_le(2)));
    }
}
