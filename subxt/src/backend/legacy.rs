// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a legacy backend implementation, which relies
//! on the legacy RPC API methods.

mod descendant_streams;

use crate::backend::utils::{retry, retry_stream};
use crate::backend::{
    Backend, BlockRef, StorageResponse, StreamOf, StreamOfResults, TransactionStatus,
};
use crate::config::{Config, HashFor, Hasher, Header, RpcConfigFor};
use crate::error::BackendError;
use async_trait::async_trait;
use codec::Encode;
use descendant_streams::{StorageFetchDescendantKeysStream, StorageFetchDescendantValuesStream};
use futures::TryStreamExt;
use futures::{Future, Stream, StreamExt, future, future::Either, stream};
use subxt_rpcs::RpcClient;
use subxt_rpcs::methods::legacy::NumberOrHex;
use subxt_rpcs::methods::legacy::{LegacyRpcMethods, TransactionStatus as RpcTransactionStatus};

/// Configure and build an [`LegacyBackend`].
pub struct LegacyBackendBuilder<T> {
    storage_page_size: u32,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Config> Default for LegacyBackendBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Config> LegacyBackendBuilder<T> {
    /// Create a new [`LegacyBackendBuilder`].
    pub fn new() -> Self {
        Self {
            storage_page_size: 64,
            _marker: std::marker::PhantomData,
        }
    }

    /// Iterating over storage entries using the [`LegacyBackend`] requires
    /// fetching entries in batches. This configures the number of entries that
    /// we'll try to obtain in each batch (default: 64).
    pub fn storage_page_size(mut self, storage_page_size: u32) -> Self {
        self.storage_page_size = storage_page_size;
        self
    }

    /// Given an [`RpcClient`] to use to make requests, this returns a [`LegacyBackend`],
    /// which implements the [`Backend`] trait.
    pub fn build(self, client: impl Into<RpcClient>) -> LegacyBackend<T> {
        LegacyBackend {
            storage_page_size: self.storage_page_size,
            methods: LegacyRpcMethods::new(client.into()),
        }
    }
}

/// The legacy backend.
#[derive(Debug)]
pub struct LegacyBackend<T> {
    storage_page_size: u32,
    methods: LegacyRpcMethods<RpcConfigFor<T>>,
}

impl<T> Clone for LegacyBackend<T> {
    fn clone(&self) -> LegacyBackend<T> {
        LegacyBackend {
            storage_page_size: self.storage_page_size,
            methods: self.methods.clone(),
        }
    }
}

impl<T: Config> LegacyBackend<T> {
    /// Configure and construct an [`LegacyBackend`].
    pub fn builder() -> LegacyBackendBuilder<T> {
        LegacyBackendBuilder::new()
    }
}

impl<T: Config> super::sealed::Sealed for LegacyBackend<T> {}

#[async_trait]
impl<T: Config> Backend<T> for LegacyBackend<T> {
    async fn storage_fetch_values(
        &self,
        keys: Vec<Vec<u8>>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        fn get_entry<T: Config>(
            key: Vec<u8>,
            at: HashFor<T>,
            methods: LegacyRpcMethods<RpcConfigFor<T>>,
        ) -> impl Future<Output = Result<Option<StorageResponse>, BackendError>> {
            retry(move || {
                let methods = methods.clone();
                let key = key.clone();
                async move {
                    let res = methods.state_get_storage(&key, Some(at)).await?;
                    Ok(res.map(move |value| StorageResponse { key, value }))
                }
            })
        }

        let keys = keys.clone();
        let methods = self.methods.clone();

        // For each key, return it + a future to get the result.
        let iter = keys
            .into_iter()
            .map(move |key| get_entry(key, at, methods.clone()));

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
        at: HashFor<T>,
    ) -> Result<StreamOfResults<Vec<u8>>, BackendError> {
        let keys = StorageFetchDescendantKeysStream::new(
            self.methods.clone(),
            key,
            at,
            self.storage_page_size,
        );

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
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        let values_stream = StorageFetchDescendantValuesStream::new(
            self.methods.clone(),
            key,
            at,
            self.storage_page_size,
        );

        Ok(StreamOf(Box::pin(values_stream)))
    }

    async fn genesis_hash(&self) -> Result<HashFor<T>, BackendError> {
        retry(|| async {
            let hash = self.methods.genesis_hash().await?;
            Ok(hash)
        })
        .await
    }

    async fn block_number_to_hash(
        &self,
        number: u64,
    ) -> Result<Option<BlockRef<HashFor<T>>>, BackendError> {
        retry(|| async {
            let number_or_hash = NumberOrHex::Number(number);
            let hash = self
                .methods
                .chain_get_block_hash(Some(number_or_hash))
                .await?
                .map(BlockRef::from_hash);
            Ok(hash)
        })
        .await
    }

    async fn block_header(&self, at: HashFor<T>) -> Result<Option<T::Header>, BackendError> {
        retry(|| async {
            let header = self.methods.chain_get_header(Some(at)).await?;
            Ok(header)
        })
        .await
    }

    async fn block_body(&self, at: HashFor<T>) -> Result<Option<Vec<Vec<u8>>>, BackendError> {
        retry(|| async {
            let Some(details) = self.methods.chain_get_block(Some(at)).await? else {
                return Ok(None);
            };
            Ok(Some(
                details.block.extrinsics.into_iter().map(|b| b.0).collect(),
            ))
        })
        .await
    }

    async fn latest_finalized_block_ref(&self) -> Result<BlockRef<HashFor<T>>, BackendError> {
        retry(|| async {
            let hash = self.methods.chain_get_finalized_head().await?;
            Ok(BlockRef::from_hash(hash))
        })
        .await
    }

    async fn stream_all_block_headers(
        &self,
        hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        let methods = self.methods.clone();
        let retry_sub = retry_stream(move || {
            let methods = methods.clone();
            let hasher = hasher.clone();
            Box::pin(async move {
                let sub = methods.chain_subscribe_all_heads().await?;
                let sub = sub.map_err(|e| e.into()).map(move |r| {
                    r.map(|h| {
                        let hash = hasher.hash(&h.encode());
                        (h, BlockRef::from_hash(hash))
                    })
                });
                Ok(StreamOf(Box::pin(sub)))
            })
        })
        .await?;

        Ok(retry_sub)
    }

    async fn stream_best_block_headers(
        &self,
        hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        let methods = self.methods.clone();

        let retry_sub = retry_stream(move || {
            let methods = methods.clone();
            let hasher = hasher.clone();
            Box::pin(async move {
                let sub = methods.chain_subscribe_new_heads().await?;
                let sub = sub.map_err(|e| e.into()).map(move |r| {
                    r.map(|h| {
                        let hash = hasher.hash(&h.encode());
                        (h, BlockRef::from_hash(hash))
                    })
                });
                Ok(StreamOf(Box::pin(sub)))
            })
        })
        .await?;

        Ok(retry_sub)
    }

    async fn stream_finalized_block_headers(
        &self,
        hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        let this = self.clone();

        let retry_sub = retry_stream(move || {
            let this = this.clone();
            let hasher = hasher.clone();
            Box::pin(async move {
                let sub = this.methods.chain_subscribe_finalized_heads().await?;

                // Get the last finalized block immediately so that the stream will emit every finalized block after this.
                let last_finalized_block_ref = this.latest_finalized_block_ref().await?;
                let last_finalized_block_num = this
                    .block_header(last_finalized_block_ref.hash())
                    .await?
                    .map(|h| h.number().into());

                // Fill in any missing blocks, because the backend may not emit every finalized block; just the latest ones which
                // are finalized each time.
                let sub = subscribe_to_block_headers_filling_in_gaps(
                    this.methods.clone(),
                    sub,
                    last_finalized_block_num,
                );
                let sub = sub.map(move |r| {
                    r.map(|h| {
                        let hash = hasher.hash(&h.encode());
                        (h, BlockRef::from_hash(hash))
                    })
                });

                Ok(StreamOf(Box::pin(sub)))
            })
        })
        .await?;

        Ok(retry_sub)
    }

    async fn submit_transaction(
        &self,
        extrinsic: &[u8],
    ) -> Result<StreamOfResults<TransactionStatus<HashFor<T>>>, BackendError> {
        let sub = self
            .methods
            .author_submit_and_watch_extrinsic(extrinsic)
            .await?;

        let sub = sub.filter_map(|r| {
            let mapped = r
                .map_err(|e| e.into())
                .map(|tx| {
                    match tx {
                        // We ignore these because they don't map nicely to the new API. They don't signal "end states" so this should be fine.
                        RpcTransactionStatus::Future => None,
                        RpcTransactionStatus::Retracted(_) => None,
                        // These roughly map across:
                        RpcTransactionStatus::Ready => Some(TransactionStatus::Validated),
                        RpcTransactionStatus::Broadcast(_peers) => {
                            Some(TransactionStatus::Broadcasted)
                        }
                        RpcTransactionStatus::InBlock(hash) => {
                            Some(TransactionStatus::InBestBlock {
                                hash: BlockRef::from_hash(hash),
                            })
                        }
                        // These 5 mean that the stream will very likely end:
                        RpcTransactionStatus::FinalityTimeout(_) => {
                            Some(TransactionStatus::Dropped {
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

        Ok(StreamOf::new(Box::pin(sub)))
    }

    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: HashFor<T>,
    ) -> Result<Vec<u8>, BackendError> {
        retry(|| async {
            let res = self
                .methods
                .state_call(method, call_parameters, Some(at))
                .await?;
            Ok(res)
        })
        .await
    }
}

/// Note: This is exposed for testing but is not considered stable and may change
/// without notice in a patch release.
#[doc(hidden)]
pub fn subscribe_to_block_headers_filling_in_gaps<T, S, E>(
    methods: LegacyRpcMethods<RpcConfigFor<T>>,
    sub: S,
    mut last_block_num: Option<u64>,
) -> impl Stream<Item = Result<T::Header, BackendError>> + Send
where
    T: Config,
    S: Stream<Item = Result<T::Header, E>> + Send,
    E: Into<BackendError> + Send + 'static,
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
                    Ok::<_, BackendError>(header)
                }
            })
            .filter_map(async |h| h.transpose());

        // On the next iteration, we'll get details starting just after this end block.
        last_block_num = Some(end_block_num);

        // Return a combination of any previous headers plus the new header.
        Either::Right(previous_headers.chain(stream::once(async { Ok(header) })))
    })
}
