// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a legacy backend implementation, which relies
//! on the legacy RPC API methods.

pub mod rpc_methods;

use async_trait::async_trait;
use std::sync::Arc;
use futures::StreamExt;
use crate::backend::{
    Backend,
    BlockRef,
    StreamOfResults,
    RuntimeVersion,
    TransactionStatus,
    rpc::{RpcClient, RpcClientT}
};
use crate::{ Config, Error, config::Header };
use self::rpc_methods::TransactionStatus as RpcTransactionStatus;


/// The legacy backend.
pub struct LegacyBackend<T: Config + Send + Sync + 'static> {
    client: RpcClient<T>
}

impl <T: Config + Send + Sync + 'static> LegacyBackend<T> {
    pub fn new<R: RpcClientT>(client: Arc<R>) -> Self {
        Self {
            client: RpcClient::new(client)
        }
    }
}

#[async_trait]
impl <T: Config + Send + Sync + 'static> Backend<T> for LegacyBackend<T> {
    async fn storage_fetch_value(
        &self,
        key: &[u8],
        at: Option<T::Hash>,
    ) -> Result<Option<Vec<u8>>, Error> {
        rpc_methods::state_get_storage(&self.client, key, at).await
    }

    async fn storage_fetch_keys(
        &self,
        key: &[u8],
        count: u32,
        start_key: Option<&[u8]>,
        at: Option<T::Hash>,
    ) -> Result<Vec<Vec<u8>>, Error> {
        rpc_methods::state_get_keys_paged(&self.client, key, count, start_key, at).await
    }

    async fn genesis_hash(&self) -> Result<T::Hash, Error> {
        rpc_methods::genesis_hash(&self.client).await
    }

    async fn block_header(&self, at: Option<T::Hash>) -> Result<Option<T::Header>, Error> {
        rpc_methods::chain_get_header(&self.client, at).await
    }

    async fn block_body(&self, at: Option<T::Hash>) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let Some(details) = rpc_methods::chain_get_block(&self.client, at).await? else {
            return Ok(None)
        };
        Ok(Some(details.block.extrinsics.into_iter().map(|b| b.0).collect()))
    }

    async fn latest_finalized_block_hash(&self) -> Result<BlockRef<T::Hash>, Error> {
        let hash = rpc_methods::chain_get_finalized_head(&self.client).await?;
        Ok(BlockRef::from_hash(hash))
    }

    async fn latest_best_block_hash(&self) -> Result<BlockRef<T::Hash>, Error> {
        let hash = rpc_methods::chain_get_block_hash(&self.client, None)
            .await?
            .ok_or_else(|| Error::Other("Latest best block doesn't exist".into()))?;
        Ok(BlockRef::from_hash(hash))
    }

    async fn current_runtime_version(&self) -> Result<RuntimeVersion, Error> {
        let details = rpc_methods::state_get_runtime_version(&self.client, None).await?;
        Ok(RuntimeVersion { spec_version: details.spec_version, transaction_version: details.transaction_version })
    }

    async fn stream_runtime_version(&self) -> Result<StreamOfResults<RuntimeVersion>, Error> {
        let sub = rpc_methods::state_subscribe_runtime_version(&self.client).await?;
        let sub = sub.map(|r| r.map(|v| RuntimeVersion { spec_version: v.spec_version, transaction_version: v.transaction_version }));
        Ok(Box::pin(sub))
    }

    async fn stream_all_block_headers(&self) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error> {
        let sub = rpc_methods::chain_subscribe_all_heads(&self.client).await?;
        let sub = sub.map(|r| r.map(|h| (h, BlockRef::from_hash(h.hash()))));
        Ok(Box::pin(sub))
    }

    async fn stream_best_block_headers(&self) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error> {
        let sub = rpc_methods::chain_subscribe_new_heads(&self.client).await?;
        let sub = sub.map(|r| r.map(|h| (h, BlockRef::from_hash(h.hash()))));
        Ok(Box::pin(sub))
    }

    async fn stream_finalized_block_headers(&self) -> Result<StreamOfResults<(T::Header, BlockRef<T::Hash>)>, Error> {
        let sub: super::rpc::Subscription<<T as Config>::Header> = rpc_methods::chain_subscribe_finalized_heads(&self.client).await?;
        let sub = sub.map(|r| r.map(|h| (h, BlockRef::from_hash(h.hash()))));
        Ok(Box::pin(sub))
    }

    async fn submit_transaction(&self, extrinsic: &[u8]) -> Result<StreamOfResults<TransactionStatus<T::Hash>>, Error> {
        let sub = rpc_methods::author_submit_and_watch_extrinsic(&self.client, extrinsic).await?;
        let sub = sub.map(|r| r.map(|tx| {
            // Dev note: When the new backend is implemented, I expect the shape or the output status to change, so
            // we'll have to adapt this old API version into the new API version here.
            match tx {
                RpcTransactionStatus::Future => TransactionStatus::Future,
                RpcTransactionStatus::Ready => TransactionStatus::Ready,
                RpcTransactionStatus::Broadcast(s) => TransactionStatus::Broadcast(s),
                RpcTransactionStatus::InBlock(s) => TransactionStatus::InBlock(s),
                RpcTransactionStatus::Retracted(s) => TransactionStatus::Retracted(s),
                RpcTransactionStatus::FinalityTimeout(s) => TransactionStatus::FinalityTimeout(s),
                RpcTransactionStatus::Finalized(s) => TransactionStatus::Finalized(s),
                RpcTransactionStatus::Usurped(s) => TransactionStatus::Usurped(s),
                RpcTransactionStatus::Dropped => TransactionStatus::Dropped,
                RpcTransactionStatus::Invalid => TransactionStatus::Invalid,
            }
        }));
        Ok(Box::pin(sub))
    }

    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: Option<T::Hash>,
    ) -> Result<Vec<u8>, Error> {
        rpc_methods::state_call(&self.client, method, call_parameters, at).await
    }
}