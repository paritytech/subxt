// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module will expose a backend implementation based on the new APIs
//! described at <https://github.com/paritytech/json-rpc-interface-spec/>. See
//! [`rpc_methods`] for the raw API calls.
//!
//! # Warning
//!
//! Everything in this module is **unstable**, meaning that it could change without
//! warning at any time.

mod follow_stream;
mod follow_stream_driver;
mod follow_stream_unpin;
mod storage_items;

use self::follow_stream_driver::FollowStreamFinalizedHeads;
use crate::backend::{
    Backend, BlockRef, BlockRefT, RuntimeVersion, StorageResponse, StreamOf, StreamOfResults,
    TransactionStatus, utils::retry,
};
use crate::config::{Config, Hash, HashFor};
use crate::error::{BackendError, RpcError};
use async_trait::async_trait;
use follow_stream_driver::{FollowStreamDriver, FollowStreamDriverHandle};
use futures::future::Either;
use futures::{Stream, StreamExt};
use std::collections::HashMap;
use std::task::Poll;
use storage_items::StorageItems;
use subxt_rpcs::RpcClient;
use subxt_rpcs::methods::chain_head::{
    FollowEvent, MethodResponse, RuntimeEvent, StorageQuery, StorageQueryType, StorageResultType,
};

/// Re-export RPC types and methods from [`subxt_rpcs::methods::chain_head`].
pub mod rpc_methods {
    pub use subxt_rpcs::methods::legacy::*;
}

// Expose the RPC methods.
pub use subxt_rpcs::methods::chain_head::ChainHeadRpcMethods;

/// Configure and build an [`ChainHeadBackend`].
pub struct ChainHeadBackendBuilder<T> {
    max_block_life: usize,
    transaction_timeout_secs: usize,
    submit_transactions_ignoring_follow_events: bool,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Config> Default for ChainHeadBackendBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Config> ChainHeadBackendBuilder<T> {
    /// Create a new [`ChainHeadBackendBuilder`].
    pub fn new() -> Self {
        Self {
            max_block_life: usize::MAX,
            transaction_timeout_secs: 240,
            submit_transactions_ignoring_follow_events: false,
            _marker: std::marker::PhantomData,
        }
    }

    /// The age of a block is defined here as the difference between the current finalized block number
    /// and the block number of a given block. Once the difference equals or exceeds the number given
    /// here, the block is unpinned.
    ///
    /// By default, we will never automatically unpin blocks, but if the number of pinned blocks that we
    /// keep hold of exceeds the number that the server can tolerate, then a `stop` event is generated and
    /// we are forced to resubscribe, losing any pinned blocks.
    pub fn max_block_life(mut self, max_block_life: usize) -> Self {
        self.max_block_life = max_block_life;
        self
    }

    /// When a transaction is submitted, we wait for events indicating it's successfully made it into a finalized
    /// block. If it takes too long for this to happen, we assume that something went wrong and that we should
    /// give up waiting.
    ///
    /// Provide a value here to denote how long, in seconds, to wait before giving up. Defaults to 240 seconds.
    ///
    /// If [`Self::submit_transactions_ignoring_follow_events()`] is called, this timeout is ignored.
    pub fn transaction_timeout(mut self, timeout_secs: usize) -> Self {
        self.transaction_timeout_secs = timeout_secs;
        self
    }

    /// When a transaction is submitted, we normally synchronize the events that we get back with events from
    /// our background `chainHead_follow` subscription, to ensure that any blocks hashes that we see can be
    /// immediately queried (for example to get events or state at that block), and are kept around unless they
    /// are no longer needed.
    ///
    /// The main downside of this synchronization is that there may be a delay in being handed back a
    /// [`TransactionStatus::InFinalizedBlock`] event while we wait to see the same block hash emitted from
    /// our background `chainHead_follow` subscription in order to ensure it's available for querying.
    ///
    /// Calling this method turns off this synchronization, speeding up the response and removing any reliance
    /// on the `chainHead_follow` subscription continuing to run without stopping throughout submitting a transaction.
    ///
    /// # Warning
    ///
    /// This can lead to errors when calling APIs like `wait_for_finalized_success`, which will try to retrieve events
    /// at the finalized block, because there will be a race and the finalized block may not be available for querying
    /// yet.
    pub fn submit_transactions_ignoring_follow_events(mut self) -> Self {
        self.submit_transactions_ignoring_follow_events = true;
        self
    }

    /// A low-level API to build the backend and driver which requires polling the driver for the backend
    /// to make progress.
    ///
    /// This is useful if you want to manage the driver yourself, for example if you want to run it in on
    /// a specific runtime.
    ///
    /// If you just want to run the driver in the background until completion in on the default runtime,
    /// use [`ChainHeadBackendBuilder::build_with_background_driver`] instead.
    pub fn build(
        self,
        client: impl Into<RpcClient>,
    ) -> (ChainHeadBackend<T>, ChainHeadBackendDriver<T>) {
        // Construct the underlying follow_stream layers:
        let rpc_methods = ChainHeadRpcMethods::new(client.into());
        let follow_stream =
            follow_stream::FollowStream::<HashFor<T>>::from_methods(rpc_methods.clone());
        let follow_stream_unpin =
            follow_stream_unpin::FollowStreamUnpin::<HashFor<T>>::from_methods(
                follow_stream,
                rpc_methods.clone(),
                self.max_block_life,
            );
        let follow_stream_driver = FollowStreamDriver::new(follow_stream_unpin);

        // Wrap these into the backend and driver that we'll expose.
        let backend = ChainHeadBackend {
            methods: rpc_methods,
            follow_handle: follow_stream_driver.handle(),
            transaction_timeout_secs: self.transaction_timeout_secs,
            submit_transactions_ignoring_follow_events: self
                .submit_transactions_ignoring_follow_events,
        };
        let driver = ChainHeadBackendDriver {
            driver: follow_stream_driver,
        };

        (backend, driver)
    }

    /// An API to build the backend and driver which will run in the background until completion
    /// on the default runtime.
    ///
    /// - On non-wasm targets, this will spawn the driver on `tokio`.
    /// - On wasm targets, this will spawn the driver on `wasm-bindgen-futures`.
    #[cfg(feature = "runtime")]
    #[cfg_attr(docsrs, doc(cfg(feature = "runtime")))]
    pub fn build_with_background_driver(self, client: impl Into<RpcClient>) -> ChainHeadBackend<T> {
        fn spawn<F: std::future::Future + Send + 'static>(future: F) {
            #[cfg(not(target_family = "wasm"))]
            tokio::spawn(async move {
                future.await;
            });
            #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
            wasm_bindgen_futures::spawn_local(async move {
                future.await;
            });
        }

        let (backend, mut driver) = self.build(client);
        spawn(async move {
            // NOTE: we need to poll the driver until it's done i.e returns None
            // to ensure that the backend is shutdown properly.
            while let Some(res) = driver.next().await {
                if let Err(err) = res {
                    tracing::debug!(target: "subxt", "chainHead backend error={err}");
                }
            }

            tracing::debug!(target: "subxt", "chainHead backend was closed");
        });

        backend
    }
}

/// Driver for the [`ChainHeadBackend`]. This must be polled in order for the
/// backend to make progress.
#[derive(Debug)]
pub struct ChainHeadBackendDriver<T: Config> {
    driver: FollowStreamDriver<HashFor<T>>,
}

impl<T: Config> Stream for ChainHeadBackendDriver<T> {
    type Item = <FollowStreamDriver<HashFor<T>> as Stream>::Item;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.driver.poll_next_unpin(cx)
    }
}

/// The chainHead backend.
#[derive(Debug, Clone)]
pub struct ChainHeadBackend<T: Config> {
    // RPC methods we'll want to call:
    methods: ChainHeadRpcMethods<T>,
    // A handle to the chainHead_follow subscription:
    follow_handle: FollowStreamDriverHandle<HashFor<T>>,
    // How long to wait until giving up on transactions:
    transaction_timeout_secs: usize,
    // Don't synchronise blocks with chainHead_follow when submitting txs:
    submit_transactions_ignoring_follow_events: bool,
}

impl<T: Config> ChainHeadBackend<T> {
    /// Configure and construct an [`ChainHeadBackend`] and the associated [`ChainHeadBackendDriver`].
    pub fn builder() -> ChainHeadBackendBuilder<T> {
        ChainHeadBackendBuilder::new()
    }

    /// Stream block headers based on the provided filter fn
    async fn stream_headers<F>(
        &self,
        f: F,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError>
    where
        F: Fn(
                FollowEvent<follow_stream_unpin::BlockRef<HashFor<T>>>,
            ) -> Vec<follow_stream_unpin::BlockRef<HashFor<T>>>
            + Send
            + Sync
            + 'static,
    {
        let methods = self.methods.clone();

        let headers =
            FollowStreamFinalizedHeads::new(self.follow_handle.subscribe(), f).flat_map(move |r| {
                let methods = methods.clone();

                let (sub_id, block_refs) = match r {
                    Ok(ev) => ev,
                    Err(e) => return Either::Left(futures::stream::once(async { Err(e) })),
                };

                Either::Right(
                    futures::stream::iter(block_refs).filter_map(move |block_ref| {
                        let methods = methods.clone();
                        let sub_id = sub_id.clone();

                        async move {
                            let res = methods
                                .chainhead_v1_header(&sub_id, block_ref.hash())
                                .await
                                .transpose()?;

                            let header = match res {
                                Ok(header) => header,
                                Err(e) => return Some(Err(e.into())),
                            };

                            Some(Ok((header, block_ref.into())))
                        }
                    }),
                )
            });

        Ok(StreamOf(Box::pin(headers)))
    }
}

impl<H: Hash + 'static> BlockRefT for follow_stream_unpin::BlockRef<H> {}
impl<H: Hash + 'static> From<follow_stream_unpin::BlockRef<H>> for BlockRef<H> {
    fn from(b: follow_stream_unpin::BlockRef<H>) -> Self {
        BlockRef::new(b.hash(), b)
    }
}

impl<T: Config> super::sealed::Sealed for ChainHeadBackend<T> {}

#[async_trait]
impl<T: Config + Send + Sync + 'static> Backend<T> for ChainHeadBackend<T> {
    async fn storage_fetch_values(
        &self,
        keys: Vec<Vec<u8>>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        retry(|| async {
            let queries = keys.iter().map(|key| StorageQuery {
                key: &**key,
                query_type: StorageQueryType::Value,
            });

            let storage_items =
                StorageItems::from_methods(queries, at, &self.follow_handle, self.methods.clone())
                    .await?;

            let stream = storage_items.filter_map(async |val| {
                let val = match val {
                    Ok(val) => val,
                    Err(e) => return Some(Err(e)),
                };

                let StorageResultType::Value(result) = val.result else {
                    return None;
                };
                Some(Ok(StorageResponse {
                    key: val.key.0,
                    value: result.0,
                }))
            });

            Ok(StreamOf(Box::pin(stream)))
        })
        .await
    }

    async fn storage_fetch_descendant_keys(
        &self,
        key: Vec<u8>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<Vec<u8>>, BackendError> {
        retry(|| async {
            // Ask for hashes, and then just ignore them and return the keys that come back.
            let query = StorageQuery {
                key: &*key,
                query_type: StorageQueryType::DescendantsHashes,
            };

            let storage_items = StorageItems::from_methods(
                std::iter::once(query),
                at,
                &self.follow_handle,
                self.methods.clone(),
            )
            .await?;

            let storage_result_stream = storage_items.map(|val| val.map(|v| v.key.0));
            Ok(StreamOf(Box::pin(storage_result_stream)))
        })
        .await
    }

    async fn storage_fetch_descendant_values(
        &self,
        key: Vec<u8>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        retry(|| async {
            let query = StorageQuery {
                key: &*key,
                query_type: StorageQueryType::DescendantsValues,
            };

            let storage_items = StorageItems::from_methods(
                std::iter::once(query),
                at,
                &self.follow_handle,
                self.methods.clone(),
            )
            .await?;

            let storage_result_stream = storage_items.filter_map(async |val| {
                let val = match val {
                    Ok(val) => val,
                    Err(e) => return Some(Err(e)),
                };

                let StorageResultType::Value(result) = val.result else {
                    return None;
                };
                Some(Ok(StorageResponse {
                    key: val.key.0,
                    value: result.0,
                }))
            });

            Ok(StreamOf(Box::pin(storage_result_stream)))
        })
        .await
    }

    async fn genesis_hash(&self) -> Result<HashFor<T>, BackendError> {
        retry(|| async {
            let genesis_hash = self.methods.chainspec_v1_genesis_hash().await?;
            Ok(genesis_hash)
        })
        .await
    }

    async fn block_header(&self, at: HashFor<T>) -> Result<Option<T::Header>, BackendError> {
        retry(|| async {
            let sub_id = get_subscription_id(&self.follow_handle).await?;
            let header = self.methods.chainhead_v1_header(&sub_id, at).await?;
            Ok(header)
        })
        .await
    }

    async fn block_body(&self, at: HashFor<T>) -> Result<Option<Vec<Vec<u8>>>, BackendError> {
        retry(|| async {
            let sub_id = get_subscription_id(&self.follow_handle).await?;

            // Subscribe to the body response and get our operationId back.
            let follow_events = self.follow_handle.subscribe().events();
            let status = self.methods.chainhead_v1_body(&sub_id, at).await?;
            let operation_id = match status {
                MethodResponse::LimitReached => return Err(RpcError::LimitReached.into()),
                MethodResponse::Started(s) => s.operation_id,
            };

            // Wait for the response to come back with the correct operationId.
            let mut exts_stream = follow_events.filter_map(|ev| {
                let FollowEvent::OperationBodyDone(body) = ev else {
                    return std::future::ready(None);
                };
                if body.operation_id != operation_id {
                    return std::future::ready(None);
                }
                let exts: Vec<_> = body.value.into_iter().map(|ext| ext.0).collect();
                std::future::ready(Some(exts))
            });

            Ok(exts_stream.next().await)
        })
        .await
    }

    async fn latest_finalized_block_ref(&self) -> Result<BlockRef<HashFor<T>>, BackendError> {
        let next_ref: Option<BlockRef<HashFor<T>>> = self
            .follow_handle
            .subscribe()
            .events()
            .filter_map(|ev| {
                let out = match ev {
                    FollowEvent::Initialized(init) => {
                        init.finalized_block_hashes.last().map(|b| b.clone().into())
                    }
                    _ => None,
                };
                std::future::ready(out)
            })
            .next()
            .await;

        next_ref.ok_or_else(|| RpcError::SubscriptionDropped.into())
    }

    async fn current_runtime_version(&self) -> Result<RuntimeVersion, BackendError> {
        // Just start a stream of version infos, and return the first value we get from it.
        let runtime_version = self.stream_runtime_version().await?.next().await;
        match runtime_version {
            None => Err(BackendError::Rpc(RpcError::SubscriptionDropped)),
            Some(Err(e)) => Err(e),
            Some(Ok(version)) => Ok(version),
        }
    }

    async fn stream_runtime_version(
        &self,
    ) -> Result<StreamOfResults<RuntimeVersion>, BackendError> {
        // Keep track of runtime details announced in new blocks, and then when blocks
        // are finalized, find the latest of these that has runtime details, and clear the rest.
        let mut runtimes = HashMap::new();
        let runtime_stream = self
            .follow_handle
            .subscribe()
            .events()
            .filter_map(move |ev| {
                let output = match ev {
                    FollowEvent::Initialized(ev) => {
                        for finalized_block in ev.finalized_block_hashes {
                            runtimes.remove(&finalized_block.hash());
                        }
                        ev.finalized_block_runtime
                    }
                    FollowEvent::NewBlock(ev) => {
                        if let Some(runtime) = ev.new_runtime {
                            runtimes.insert(ev.block_hash.hash(), runtime);
                        }
                        None
                    }
                    FollowEvent::Finalized(ev) => {
                        let next_runtime = {
                            let mut it = ev
                                .finalized_block_hashes
                                .iter()
                                .rev()
                                .filter_map(|h| runtimes.get(&h.hash()).cloned())
                                .peekable();

                            let next = it.next();

                            if it.peek().is_some() {
                                tracing::warn!(
                                    target: "subxt",
                                    "Several runtime upgrades in the finalized blocks but only the latest runtime upgrade is returned"
                                );
                            }

                            next
                        };

                        // Remove finalized and pruned blocks as valid runtime upgrades.
                        for block in ev
                            .finalized_block_hashes
                            .iter()
                            .chain(ev.pruned_block_hashes.iter())
                        {
                            runtimes.remove(&block.hash());
                        }

                        next_runtime
                    }
                    _ => None,
                };

                let runtime_event = match output {
                    None => return std::future::ready(None),
                    Some(ev) => ev,
                };

                let runtime_details = match runtime_event {
                    RuntimeEvent::Invalid(err) => {
                        return std::future::ready(Some(Err(BackendError::Other(format!("Invalid runtime error using chainHead RPCs: {}", err.error)))))
                    }
                    RuntimeEvent::Valid(ev) => ev,
                };

                let runtime_version = RuntimeVersion {
                    spec_version: runtime_details.spec.spec_version,
                    transaction_version: runtime_details.spec.transaction_version
                };
                std::future::ready(Some(Ok(runtime_version)))
            });

        Ok(StreamOf::new(Box::pin(runtime_stream)))
    }

    async fn stream_all_block_headers(
        &self,
        _hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        // TODO: https://github.com/paritytech/subxt/issues/1568
        //
        // It's possible that blocks may be silently missed if
        // a reconnection occurs because it's restarted by the unstable backend.
        self.stream_headers(|ev| match ev {
            FollowEvent::Initialized(init) => init.finalized_block_hashes,
            FollowEvent::NewBlock(ev) => {
                vec![ev.block_hash]
            }
            _ => vec![],
        })
        .await
    }

    async fn stream_best_block_headers(
        &self,
        _hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        // TODO: https://github.com/paritytech/subxt/issues/1568
        //
        // It's possible that blocks may be silently missed if
        // a reconnection occurs because it's restarted by the unstable backend.
        self.stream_headers(|ev| match ev {
            FollowEvent::Initialized(init) => init.finalized_block_hashes,
            FollowEvent::BestBlockChanged(ev) => vec![ev.best_block_hash],
            _ => vec![],
        })
        .await
    }

    async fn stream_finalized_block_headers(
        &self,
        _hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        self.stream_headers(|ev| match ev {
            FollowEvent::Initialized(init) => init.finalized_block_hashes,
            FollowEvent::Finalized(ev) => ev.finalized_block_hashes,
            _ => vec![],
        })
        .await
    }

    async fn submit_transaction(
        &self,
        extrinsic: &[u8],
    ) -> Result<StreamOfResults<TransactionStatus<HashFor<T>>>, BackendError> {
        // Submit a transaction. This makes no attempt to sync with follow events,
        async fn submit_transaction_ignoring_follow_events<T: Config>(
            extrinsic: &[u8],
            methods: &ChainHeadRpcMethods<T>,
        ) -> Result<StreamOfResults<TransactionStatus<HashFor<T>>>, BackendError> {
            let tx_progress = methods
                .transactionwatch_v1_submit_and_watch(extrinsic)
                .await?
                .map(|ev| {
                    ev.map(|tx_status| {
                        use subxt_rpcs::methods::chain_head::TransactionStatus as RpcTransactionStatus;
                        match tx_status {
                            RpcTransactionStatus::Validated => TransactionStatus::Validated,
                            RpcTransactionStatus::Broadcasted => TransactionStatus::Broadcasted,
                            RpcTransactionStatus::BestChainBlockIncluded { block: None } => {
                                TransactionStatus::NoLongerInBestBlock
                            },
                            RpcTransactionStatus::BestChainBlockIncluded { block: Some(block) } => {
                                TransactionStatus::InBestBlock { hash: BlockRef::from_hash(block.hash) }
                            },
                            RpcTransactionStatus::Finalized { block } => {
                                TransactionStatus::InFinalizedBlock { hash: BlockRef::from_hash(block.hash) }
                            },
                            RpcTransactionStatus::Error { error } => {
                                TransactionStatus::Error { message: error }
                            },
                            RpcTransactionStatus::Invalid { error } => {
                                TransactionStatus::Invalid { message: error }
                            },
                            RpcTransactionStatus::Dropped { error } => {
                                TransactionStatus::Dropped { message: error }
                            },
                        }
                    }).map_err(Into::into)
                });

            Ok(StreamOf(Box::pin(tx_progress)))
        }

        // Submit a transaction. This synchronizes with chainHead_follow events to ensure
        // that block hashes returned are ready to be queried.
        async fn submit_transaction_tracking_follow_events<T: Config>(
            extrinsic: &[u8],
            transaction_timeout_secs: u64,
            methods: &ChainHeadRpcMethods<T>,
            follow_handle: &FollowStreamDriverHandle<HashFor<T>>,
        ) -> Result<StreamOfResults<TransactionStatus<HashFor<T>>>, BackendError> {
            // We care about new and finalized block hashes.
            enum SeenBlockMarker {
                New,
                Finalized,
            }

            // First, subscribe to new blocks.
            let mut seen_blocks_sub = follow_handle.subscribe().events();

            // Then, submit the transaction.
            let mut tx_progress = methods
                .transactionwatch_v1_submit_and_watch(extrinsic)
                .await?;

            let mut seen_blocks = HashMap::new();
            let mut done = false;

            // If we see the finalized event, we start waiting until we find a finalized block that
            // matches, so we can guarantee to return a pinned block hash and be properly in sync
            // with chainHead_follow.
            let mut finalized_hash: Option<HashFor<T>> = None;

            // Record the start time so that we can time out if things appear to take too long.
            let start_instant = web_time::Instant::now();

            // A quick helper to return a generic error.
            let err_other = |s: &str| Some(Err(BackendError::Other(s.into())));

            // Now we can attempt to associate tx events with pinned blocks.
            let tx_stream = futures::stream::poll_fn(move |cx| {
                loop {
                    // Bail early if we're finished; nothing else to do.
                    if done {
                        return Poll::Ready(None);
                    }

                    // Bail if we exceed 4 mins; something very likely went wrong.
                    if start_instant.elapsed().as_secs() > transaction_timeout_secs {
                        return Poll::Ready(err_other(
                            "Timeout waiting for the transaction to be finalized",
                        ));
                    }

                    // Poll for a follow event, and error if the stream has unexpectedly ended.
                    let follow_ev_poll = match seen_blocks_sub.poll_next_unpin(cx) {
                        Poll::Ready(None) => {
                            return Poll::Ready(err_other(
                                "chainHead_follow stream ended unexpectedly",
                            ));
                        }
                        Poll::Ready(Some(follow_ev)) => Poll::Ready(follow_ev),
                        Poll::Pending => Poll::Pending,
                    };
                    let follow_ev_is_pending = follow_ev_poll.is_pending();

                    // If there was a follow event, then handle it and loop around to see if there are more.
                    // We want to buffer follow events until we hit Pending, so that we are as up-to-date as possible
                    // for when we see a BestBlockChanged event, so that we have the best change of already having
                    // seen the block that it mentions and returning a proper pinned block.
                    if let Poll::Ready(follow_ev) = follow_ev_poll {
                        match follow_ev {
                            FollowEvent::NewBlock(ev) => {
                                // Optimization: once we have a `finalized_hash`, we only care about finalized
                                // block refs now and can avoid bothering to save new blocks.
                                if finalized_hash.is_none() {
                                    seen_blocks.insert(
                                        ev.block_hash.hash(),
                                        (SeenBlockMarker::New, ev.block_hash),
                                    );
                                }
                            }
                            FollowEvent::Finalized(ev) => {
                                for block_ref in ev.finalized_block_hashes {
                                    seen_blocks.insert(
                                        block_ref.hash(),
                                        (SeenBlockMarker::Finalized, block_ref),
                                    );
                                }
                            }
                            FollowEvent::Stop => {
                                // If we get this event, we'll lose all of our existing pinned blocks and have a gap
                                // in which we may lose the finalized block that the TX is in. For now, just error if
                                // this happens, to prevent the case in which we never see a finalized block and wait
                                // forever.
                                return Poll::Ready(err_other(
                                    "chainHead_follow emitted 'stop' event during transaction submission",
                                ));
                            }
                            _ => {}
                        }
                        continue;
                    }

                    // If we have a finalized hash, we are done looking for tx events and we are just waiting
                    // for a pinned block with a matching hash (which must appear eventually given it's finalized).
                    if let Some(hash) = &finalized_hash {
                        if let Some((SeenBlockMarker::Finalized, block_ref)) =
                            seen_blocks.remove(hash)
                        {
                            // Found it! Hand back the event with a pinned block. We're done.
                            done = true;
                            let ev = TransactionStatus::InFinalizedBlock {
                                hash: block_ref.into(),
                            };
                            return Poll::Ready(Some(Ok(ev)));
                        } else {
                            // Not found it! If follow ev is pending, then return pending here and wait for
                            // a new one to come in, else loop around and see if we get another one immediately.
                            seen_blocks.clear();
                            if follow_ev_is_pending {
                                return Poll::Pending;
                            } else {
                                continue;
                            }
                        }
                    }

                    // If we don't have a finalized block yet, we keep polling for tx progress events.
                    let tx_progress_ev = match tx_progress.poll_next_unpin(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(None) => {
                            return Poll::Ready(err_other(
                                "No more transaction progress events, but we haven't seen a Finalized one yet",
                            ));
                        }
                        Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e.into()))),
                        Poll::Ready(Some(Ok(ev))) => ev,
                    };

                    // When we get one, map it to the correct format (or for finalized ev, wait for the pinned block):
                    use subxt_rpcs::methods::chain_head::TransactionStatus as RpcTransactionStatus;
                    let tx_progress_ev = match tx_progress_ev {
                        RpcTransactionStatus::Finalized { block } => {
                            // We'll wait until we have seen this hash, to try to guarantee
                            // that when we return this event, the corresponding block is
                            // pinned and accessible.
                            finalized_hash = Some(block.hash);
                            continue;
                        }
                        RpcTransactionStatus::BestChainBlockIncluded { block: Some(block) } => {
                            // Look up a pinned block ref if we can, else return a non-pinned
                            // block that likely isn't accessible. We have no guarantee that a best
                            // block on the node a tx was sent to will ever be known about on the
                            // chainHead_follow subscription.
                            let block_ref = match seen_blocks.get(&block.hash) {
                                Some((_, block_ref)) => block_ref.clone().into(),
                                None => BlockRef::from_hash(block.hash),
                            };
                            TransactionStatus::InBestBlock { hash: block_ref }
                        }
                        RpcTransactionStatus::BestChainBlockIncluded { block: None } => {
                            TransactionStatus::NoLongerInBestBlock
                        }
                        RpcTransactionStatus::Broadcasted => TransactionStatus::Broadcasted,
                        RpcTransactionStatus::Dropped { error, .. } => {
                            TransactionStatus::Dropped { message: error }
                        }
                        RpcTransactionStatus::Error { error } => {
                            TransactionStatus::Error { message: error }
                        }
                        RpcTransactionStatus::Invalid { error } => {
                            TransactionStatus::Invalid { message: error }
                        }
                        RpcTransactionStatus::Validated => TransactionStatus::Validated,
                    };
                    return Poll::Ready(Some(Ok(tx_progress_ev)));
                }
            });

            Ok(StreamOf(Box::pin(tx_stream)))
        }

        if self.submit_transactions_ignoring_follow_events {
            submit_transaction_ignoring_follow_events(extrinsic, &self.methods).await
        } else {
            submit_transaction_tracking_follow_events::<T>(
                extrinsic,
                self.transaction_timeout_secs as u64,
                &self.methods,
                &self.follow_handle,
            )
            .await
        }
    }

    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: HashFor<T>,
    ) -> Result<Vec<u8>, BackendError> {
        retry(|| async {
            let sub_id = get_subscription_id(&self.follow_handle).await?;

            // Subscribe to the body response and get our operationId back.
            let follow_events = self.follow_handle.subscribe().events();
            let call_parameters = call_parameters.unwrap_or(&[]);
            let status = self
                .methods
                .chainhead_v1_call(&sub_id, at, method, call_parameters)
                .await?;
            let operation_id = match status {
                MethodResponse::LimitReached => return Err(RpcError::LimitReached.into()),
                MethodResponse::Started(s) => s.operation_id,
            };

            // Wait for the response to come back with the correct operationId.
            let mut call_data_stream = follow_events.filter_map(|ev| {
                let FollowEvent::OperationCallDone(body) = ev else {
                    return std::future::ready(None);
                };
                if body.operation_id != operation_id {
                    return std::future::ready(None);
                }
                std::future::ready(Some(body.output.0))
            });

            call_data_stream
                .next()
                .await
                .ok_or_else(|| RpcError::SubscriptionDropped.into())
        })
        .await
    }
}

/// A helper to obtain a subscription ID.
async fn get_subscription_id<H: Hash>(
    follow_handle: &FollowStreamDriverHandle<H>,
) -> Result<String, BackendError> {
    let Some(sub_id) = follow_handle.subscribe().subscription_id().await else {
        return Err(RpcError::SubscriptionDropped.into());
    };

    Ok(sub_id)
}
