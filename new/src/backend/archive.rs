// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a backend implementation based on the new APIs
//! described at <https://github.com/paritytech/json-rpc-interface-spec/>. See
//! [`rpc_methods`] for the raw API calls.
//!
//! Specifically, the focus here is on the `archive` methods. These can only be used
//! to interact with archive nodes, but are less restrictive than the `chainHead` methods
//! in terms of the allowed operations.

mod storage_stream;

use subxt_rpcs::methods::ChainHeadRpcMethods;
use crate::backend::{
    Backend, BlockRef, StorageResponse, StreamOf, StreamOfResults,
    TransactionStatus, utils::retry,
};
use crate::config::{Config, HashFor, RpcConfigFor};
use crate::error::BackendError;
use async_trait::async_trait;
use futures::StreamExt;
use subxt_rpcs::RpcClient;
use subxt_rpcs::methods::chain_head::{
    ArchiveStorageQuery, ArchiveCallResult, StorageQueryType,
};
use storage_stream::ArchiveStorageStream;

/// The archive backend.
#[derive(Debug, Clone)]
pub struct ArchiveBackend<T: Config> {
    // RPC methods we'll want to call:
    methods: ChainHeadRpcMethods<RpcConfigFor<T>>,
}

impl<T: Config> ArchiveBackend<T> {
    /// Configure and construct an [`ArchiveBackend`] and the associated [`ChainHeadBackendDriver`].
    pub fn new(client: impl Into<RpcClient>,) -> ArchiveBackend<T> {
        let methods = ChainHeadRpcMethods::new(client.into());

        ArchiveBackend { methods }
    }
}

#[async_trait]
impl<T: Config> Backend<T> for ArchiveBackend<T> {
    async fn storage_fetch_values(
        &self,
        keys: Vec<Vec<u8>>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        let queries = keys.into_iter()
            .map(|key| ArchiveStorageQuery {
                key: key,
                query_type: StorageQueryType::Value,
                pagination_start_key: None,
            })
            .collect();

        let stream = ArchiveStorageStream::new(at, self.methods.clone(), queries).map(|item| {
            match item {
                Err(e) => Some(Err(e)),
                Ok(item) => item.value.map(|val| Ok(StorageResponse { key: item.key.0, value: val.0 }))
            }
        }).filter_map(async |item| item);

        Ok(StreamOf(Box::pin(stream)))
    }

    async fn storage_fetch_descendant_keys(
        &self,
        key: Vec<u8>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<Vec<u8>>, BackendError> {
        let queries = std::iter::once(ArchiveStorageQuery {
                key: key,
                // Just ask for the hash and then ignore it and return keys
                query_type: StorageQueryType::DescendantsHashes,
                pagination_start_key: None,
            })
            .collect();

        let stream = ArchiveStorageStream::new(at, self.methods.clone(), queries).map(|item| {
            match item {
                Err(e) => Err(e),
                Ok(item) => Ok(item.key.0)
            }
        });

        Ok(StreamOf(Box::pin(stream)))
    }

    async fn storage_fetch_descendant_values(
        &self,
        key: Vec<u8>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        let queries = std::iter::once(ArchiveStorageQuery {
                key: key,
                query_type: StorageQueryType::DescendantsValues,
                pagination_start_key: None,
            })
            .collect();

        let stream = ArchiveStorageStream::new(at, self.methods.clone(), queries).map(|item| {
            match item {
                Err(e) => Some(Err(e)),
                Ok(item) => item.value.map(|val| Ok(StorageResponse { key: item.key.0, value: val.0 }))
            }
        }).filter_map(async |item| item);

        Ok(StreamOf(Box::pin(stream)))
    }

    async fn genesis_hash(&self) -> Result<HashFor<T>, BackendError> {
        retry(|| async {
            let hash = self.methods.archive_v1_genesis_hash().await?;
            Ok(hash)
        })
        .await
    }

    async fn block_number_to_hash(&self, number: u64) -> Result<Option<BlockRef<HashFor<T>>>, BackendError> {
        retry(|| async {
            let mut hashes = self.methods.archive_v1_hash_by_height(number as usize).await?;
            if let (Some(hash), None) = (hashes.pop(), hashes.pop()) {
                // One hash; return it.
                Ok(Some(BlockRef::from_hash(hash)))
            } else {
                // More than one; return None.
                Ok(None)
            }
        }).await
    }

    async fn block_header(&self, at: HashFor<T>) -> Result<Option<T::Header>, BackendError> {
        retry(|| async {
            let header = self.methods.archive_v1_header(at).await?;
            Ok(header)
        })
        .await
    }

    async fn block_body(&self, at: HashFor<T>) -> Result<Option<Vec<Vec<u8>>>, BackendError> {
        retry(|| async {
            let Some(exts) = self.methods.archive_v1_body(at).await? else {
                return Ok(None);
            };
            Ok(Some(
                exts.into_iter().map(|ext| ext.0).collect()
            ))
        })
        .await
    }

    async fn latest_finalized_block_ref(&self) -> Result<BlockRef<HashFor<T>>, BackendError> {
        retry(|| async {
            let height = self.methods.archive_v1_finalized_height().await?;
            let mut hashes = self.methods.archive_v1_hash_by_height(height).await?;
            let Some(hash) = hashes.pop() else {
                return Err(BackendError::Other("Multiple hashes not expected at a finalized height".into()))
            };
            Ok(BlockRef::from_hash(hash))
        })
        .await
    }

    async fn stream_all_block_headers(
        &self,
        _hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        Err(BackendError::Other("The archive backend cannot stream block headers".into()))
    }

    async fn stream_best_block_headers(
        &self,
        _hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        Err(BackendError::Other("The archive backend cannot stream block headers".into()))
    }

    async fn stream_finalized_block_headers(
        &self,
        _hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        Err(BackendError::Other("The archive backend cannot stream block headers".into()))
    }

    async fn submit_transaction(
        &self,
        extrinsic: &[u8],
    ) -> Result<StreamOfResults<TransactionStatus<HashFor<T>>>, BackendError> {
        // This chainHead impl does not use chainHead_follow and so is suitable here too.
        super::chain_head::submit_transaction_ignoring_follow_events(extrinsic, &self.methods).await
    }

    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: HashFor<T>,
    ) -> Result<Vec<u8>, BackendError> {
        let res = self.methods.archive_v1_call(at, method, call_parameters.unwrap_or(&[])).await?;
        match res {
            ArchiveCallResult::Success(bytes) => Ok(bytes.0),
            ArchiveCallResult::Error(e) => Err(BackendError::other(e)),
        }
    }
}

 impl<T: Config> crate::backend::sealed::Sealed for ArchiveBackend<T> {}

