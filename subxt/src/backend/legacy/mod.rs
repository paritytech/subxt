// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a legacy backend implementation, which relies
//! on the legacy RPC API methods.

pub mod rpc_methods;

use self::rpc_methods::TransactionStatus as RpcTransactionStatus;
use crate::backend::{
    rpc::RpcClient, Backend, BlockRef, RuntimeVersion, StorageResponse, StreamOf, StreamOfResults,
    TransactionStatus,
};
use crate::{config::Header, Config, Error};
use async_trait::async_trait;
use futures::{future, future::Either, stream, Future, FutureExt, Stream, StreamExt};
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};

// Expose the RPC methods.
pub use rpc_methods::LegacyRpcMethods;

/// The legacy backend.
#[derive(Debug, Clone)]
pub struct LegacyBackend<T> {
    methods: LegacyRpcMethods<T>,
}

impl<T: Config> LegacyBackend<T> {
    /// Instantiate a new backend which uses the legacy API methods.
    pub fn new(client: RpcClient) -> Self {
        Self {
            methods: LegacyRpcMethods::new(client),
        }
    }
}

impl<T: Config> super::sealed::Sealed for LegacyBackend<T> {}

#[async_trait]
impl<T: Config + Send + Sync + 'static> Backend<T> for LegacyBackend<T> {
    async fn storage_fetch_values(
        &self,
        keys: Vec<Vec<u8>>,
        at: T::Hash,
    ) -> Result<StreamOfResults<StorageResponse>, Error> {
        let methods = self.methods.clone();

        // For each key, return it + a future to get the result.
        let iter = keys.into_iter().map(move |key| {
            let methods = methods.clone();
            async move {
                let res = methods.state_get_storage(&key, Some(at)).await?;
                Ok(res.map(|value| StorageResponse { key, value }))
            }
        });

        let s = stream::iter(iter)
            // Resolve the future
            .then(|fut| fut)
            // Filter any Options out (ie if we didn't find a value at some key we return nothing for it).
            .filter_map(|r| future::ready(r.transpose()));

        Ok(StreamOf(Box::pin(s)))
    }

    async fn storage_fetch_descendant_keys(
        &self,
        key: Vec<u8>,
        at: T::Hash,
    ) -> Result<StreamOfResults<Vec<u8>>, Error> {
        let keys = StorageFetchDescendantKeysStream {
            at,
            key,
            methods: self.methods.clone(),
            done: Default::default(),
            keys_fut: Default::default(),
            pagination_start_key: None,
        };

        let keys = keys.flat_map(|keys| {
            match keys {
                Err(e) => {
                    // If there's an error, return that next:
                    Either::Left(stream::iter(std::iter::once(Err(e))))
                }
                Ok(keys) => {
                    // Or, stream each "ok" value:
                    Either::Right(stream::iter(keys.into_iter().map(Ok)))
                }
            }
        });

        Ok(StreamOf(Box::pin(keys)))
    }

    async fn storage_fetch_descendant_values(
        &self,
        key: Vec<u8>,
        at: T::Hash,
    ) -> Result<StreamOfResults<StorageResponse>, Error> {
        let keys_stream = StorageFetchDescendantKeysStream {
            at,
            key,
            methods: self.methods.clone(),
            done: Default::default(),
            keys_fut: Default::default(),
            pagination_start_key: None,
        };

        Ok(StreamOf(Box::pin(StorageFetchDescendantValuesStream {
            keys: keys_stream,
            results_fut: None,
            results: Default::default(),
        })))
    }

    async fn genesis_hash(&self) -> Result<T::Hash, Error> {
        self.methods.genesis_hash().await
    }

    async fn block_header(&self, at: T::Hash) -> Result<Option<T::Header>, Error> {
        self.methods.chain_get_header(Some(at)).await
    }

    async fn block_body(&self, at: T::Hash) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let Some(details) = self.methods.chain_get_block(Some(at)).await? else {
            return Ok(None);
        };
        Ok(Some(
            details.block.extrinsics.into_iter().map(|b| b.0).collect(),
        ))
    }

    async fn latest_finalized_block_ref(&self) -> Result<BlockRef<T::Hash>, Error> {
        let hash = self.methods.chain_get_finalized_head().await?;
        Ok(BlockRef::from_hash(hash))
    }

    async fn current_runtime_version(&self) -> Result<RuntimeVersion, Error> {
        let details = self.methods.state_get_runtime_version(None).await?;
        Ok(RuntimeVersion {
            spec_version: details.spec_version,
            transaction_version: details.transaction_version,
        })
    }

    async fn stream_runtime_version(&self) -> Result<StreamOfResults<RuntimeVersion>, Error> {
        let sub = self.methods.state_subscribe_runtime_version().await?;
        let sub = sub.map(|r| {
            r.map(|v| RuntimeVersion {
                spec_version: v.spec_version,
                transaction_version: v.transaction_version,
            })
        });
        Ok(StreamOf(Box::pin(sub)))
    }

    async fn stream_all_block_headers(
        &self,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error> {
        let sub = self.methods.chain_subscribe_all_heads().await?;
        let sub = sub.map(|r| {
            r.map(|h| {
                let hash = h.hash();
                (h, BlockRef::from_hash(hash))
            })
        });
        Ok(StreamOf(Box::pin(sub)))
    }

    async fn stream_best_block_headers(
        &self,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error> {
        let sub = self.methods.chain_subscribe_new_heads().await?;
        let sub = sub.map(|r| {
            r.map(|h| {
                let hash = h.hash();
                (h, BlockRef::from_hash(hash))
            })
        });
        Ok(StreamOf(Box::pin(sub)))
    }

    async fn stream_finalized_block_headers(
        &self,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error> {
        let sub: super::rpc::RpcSubscription<<T as Config>::Header> =
            self.methods.chain_subscribe_finalized_heads().await?;

        // Get the last finalized block immediately so that the stream will emit every finalized block after this.
        let last_finalized_block_ref = self.latest_finalized_block_ref().await?;
        let last_finalized_block_num = self
            .block_header(last_finalized_block_ref.hash())
            .await?
            .map(|h| h.number().into());

        // Fill in any missing blocks, because the backend may not emit every finalized block; just the latest ones which
        // are finalized each time.
        let sub = subscribe_to_block_headers_filling_in_gaps(
            self.methods.clone(),
            sub,
            last_finalized_block_num,
        );
        let sub = sub.map(|r| {
            r.map(|h| {
                let hash = h.hash();
                (h, BlockRef::from_hash(hash))
            })
        });
        Ok(StreamOf(Box::pin(sub)))
    }

    async fn submit_transaction(
        &self,
        extrinsic: &[u8],
    ) -> Result<StreamOfResults<TransactionStatus<T::Hash>>, Error> {
        let sub = self
            .methods
            .author_submit_and_watch_extrinsic(extrinsic)
            .await?;
        let sub = sub.filter_map(|r| {
            let mapped = r
                .map(|tx| {
                    match tx {
                        // We ignore these because they don't map nicely to the new API. They don't signal "end states" so this should be fine.
                        RpcTransactionStatus::Future => None,
                        RpcTransactionStatus::Retracted(_) => None,
                        // These roughly map across:
                        RpcTransactionStatus::Ready => Some(TransactionStatus::Validated),
                        RpcTransactionStatus::Broadcast(peers) => {
                            Some(TransactionStatus::Broadcasted {
                                num_peers: peers.len() as u32,
                            })
                        }
                        RpcTransactionStatus::InBlock(hash) => {
                            Some(TransactionStatus::InBestBlock {
                                hash: BlockRef::from_hash(hash),
                            })
                        }
                        // These 5 mean that the stream will very likely end:
                        RpcTransactionStatus::FinalityTimeout(_) => {
                            Some(TransactionStatus::Invalid {
                                message: "Finality timeout".into(),
                            })
                        }
                        RpcTransactionStatus::Finalized(hash) => {
                            Some(TransactionStatus::InFinalizedBlock {
                                hash: BlockRef::from_hash(hash),
                            })
                        }
                        RpcTransactionStatus::Usurped(_) => Some(TransactionStatus::Invalid {
                            message: "Transaction was usurped by another with the same nonce"
                                .into(),
                        }),
                        RpcTransactionStatus::Dropped => Some(TransactionStatus::Dropped {
                            message: "Transaction was dropped".into(),
                        }),
                        RpcTransactionStatus::Invalid => Some(TransactionStatus::Invalid {
                            message:
                                "Transaction is invalid (eg because of a bad nonce, signature etc)"
                                    .into(),
                        }),
                    }
                })
                .transpose();

            future::ready(mapped)
        });
        Ok(StreamOf(Box::pin(sub)))
    }

    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: T::Hash,
    ) -> Result<Vec<u8>, Error> {
        self.methods
            .state_call(method, call_parameters, Some(at))
            .await
    }
}

/// Note: This is exposed for testing but is not considered stable and may change
/// without notice in a patch release.
#[doc(hidden)]
pub fn subscribe_to_block_headers_filling_in_gaps<T, S, E>(
    methods: LegacyRpcMethods<T>,
    sub: S,
    mut last_block_num: Option<u64>,
) -> impl Stream<Item = Result<T::Header, Error>> + Send
where
    T: Config,
    S: Stream<Item = Result<T::Header, E>> + Send,
    E: Into<Error> + Send + 'static,
{
    sub.flat_map(move |s| {
        // Get the header, or return a stream containing just the error.
        let header = match s {
            Ok(header) => header,
            Err(e) => return Either::Left(stream::once(async { Err(e.into()) })),
        };

        // We want all previous details up to, but not including this current block num.
        let end_block_num = header.number().into();

        // This is one after the last block we returned details for last time.
        let start_block_num = last_block_num.map(|n| n + 1).unwrap_or(end_block_num);

        // Iterate over all of the previous blocks we need headers for, ignoring the current block
        // (which we already have the header info for):
        let methods = methods.clone();
        let previous_headers = stream::iter(start_block_num..end_block_num)
            .then(move |n| {
                let methods = methods.clone();
                async move {
                    let hash = methods.chain_get_block_hash(Some(n.into())).await?;
                    let header = methods.chain_get_header(hash).await?;
                    Ok::<_, Error>(header)
                }
            })
            .filter_map(|h| async { h.transpose() });

        // On the next iteration, we'll get details starting just after this end block.
        last_block_num = Some(end_block_num);

        // Return a combination of any previous headers plus the new header.
        Either::Right(previous_headers.chain(stream::once(async { Ok(header) })))
    })
}

/// How many keys/values to fetch at once.
const STORAGE_PAGE_SIZE: u32 = 32;

/// This provides a stream of values given some prefix `key`. It
/// internally manages pagination and such.
pub struct StorageFetchDescendantKeysStream<T: Config> {
    methods: LegacyRpcMethods<T>,
    key: Vec<u8>,
    at: T::Hash,
    // What key do we start paginating from? None = from the beginning.
    pagination_start_key: Option<Vec<u8>>,
    // Keys, future and cached:
    keys_fut: Option<Pin<Box<dyn Future<Output = Result<Vec<Vec<u8>>, Error>> + Send + 'static>>>,
    // Set to true when we're done:
    done: bool,
}

impl<T: Config> std::marker::Unpin for StorageFetchDescendantKeysStream<T> {}

impl<T: Config> Stream for StorageFetchDescendantKeysStream<T> {
    type Item = Result<Vec<Vec<u8>>, Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();
        loop {
            // We're already done.
            if this.done {
                return Poll::Ready(None);
            }

            // Poll future to fetch next keys.
            if let Some(mut keys_fut) = this.keys_fut.take() {
                let Poll::Ready(keys) = keys_fut.poll_unpin(cx) else {
                    this.keys_fut = Some(keys_fut);
                    return Poll::Pending;
                };

                match keys {
                    Ok(keys) => {
                        if keys.is_empty() {
                            // No keys left; we're done!
                            this.done = true;
                            return Poll::Ready(None);
                        }
                        // The last key is where we want to paginate from next time.
                        this.pagination_start_key = keys.last().cloned();
                        // return all of the keys from this run.
                        return Poll::Ready(Some(Ok(keys)));
                    }
                    Err(e) => {
                        // Error getting keys? Return it.
                        return Poll::Ready(Some(Err(e)));
                    }
                }
            }

            // Else, we don't have a fut to get keys yet so start one going.
            let methods = this.methods.clone();
            let key = this.key.clone();
            let at = this.at;
            let pagination_start_key = this.pagination_start_key.take();
            let keys_fut = async move {
                methods
                    .state_get_keys_paged(
                        &key,
                        STORAGE_PAGE_SIZE,
                        pagination_start_key.as_deref(),
                        Some(at),
                    )
                    .await
            };
            this.keys_fut = Some(Box::pin(keys_fut));
        }
    }
}

/// This provides a stream of values given some stream of keys.
pub struct StorageFetchDescendantValuesStream<T: Config> {
    // Stream of keys.
    keys: StorageFetchDescendantKeysStream<T>,
    // Then we track the future to get the values back for each key:
    results_fut: Option<
        Pin<
            Box<
                dyn Future<Output = Result<Option<VecDeque<(Vec<u8>, Vec<u8>)>>, Error>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    // And finally we return each result back one at a time:
    results: VecDeque<(Vec<u8>, Vec<u8>)>,
}

impl<T: Config> Stream for StorageFetchDescendantValuesStream<T> {
    type Item = Result<StorageResponse, Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();
        loop {
            // If we have results back, return them one by one
            if let Some((key, value)) = this.results.pop_front() {
                let res = StorageResponse { key, value };
                return Poll::Ready(Some(Ok(res)));
            }

            // If we're waiting on the next results then poll that future:
            if let Some(mut results_fut) = this.results_fut.take() {
                match results_fut.poll_unpin(cx) {
                    Poll::Ready(Ok(Some(results))) => {
                        this.results = results;
                        continue;
                    }
                    Poll::Ready(Ok(None)) => {
                        // No values back for some keys? Skip.
                        continue;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e))),
                    Poll::Pending => {
                        this.results_fut = Some(results_fut);
                        return Poll::Pending;
                    }
                }
            }

            match this.keys.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(keys))) => {
                    let methods = this.keys.methods.clone();
                    let at = this.keys.at;
                    let results_fut = async move {
                        let keys = keys.iter().map(|k| &**k);
                        let values = methods.state_query_storage_at(keys, Some(at)).await?;
                        let values: VecDeque<_> = values
                            .into_iter()
                            .flat_map(|v| {
                                v.changes.into_iter().filter_map(|(k, v)| {
                                    let v = v?;
                                    Some((k.0, v.0))
                                })
                            })
                            .collect();
                        Ok(Some(values))
                    };

                    this.results_fut = Some(Box::pin(results_fut));
                    continue;
                }
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e))),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
