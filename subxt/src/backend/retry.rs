//! Retry-able backend.

use super::{
    utils::{retry, BlockSubscription, RuntimeVersionSubscription, SubmitTransactionSubscription},
    Backend, BlockRef, RuntimeVersion, StorageResponse, StreamOfResults,
};
use crate::{Config, Error};
use async_trait::async_trait;

/// Retry-able rpc backend.
pub struct RetryBackend<T>(Box<dyn Backend<T>>);

impl<T> From<Box<dyn Backend<T>>> for RetryBackend<T> {
    fn from(backend: Box<dyn Backend<T>>) -> Self {
        Self(backend)
    }
}

impl<T: Config> super::sealed::Sealed for RetryBackend<T> {}

#[async_trait]
impl<T: Config + Send + Sync + 'static> Backend<T> for RetryBackend<T> {
    async fn storage_fetch_values(
        &self,
        keys: Vec<Vec<u8>>,
        at: T::Hash,
    ) -> Result<StreamOfResults<StorageResponse>, Error> {
        retry(|| self.0.storage_fetch_values(keys.clone(), at)).await
    }

    async fn storage_fetch_descendant_keys(
        &self,
        key: Vec<u8>,
        at: T::Hash,
    ) -> Result<StreamOfResults<Vec<u8>>, Error> {
        retry(|| self.0.storage_fetch_descendant_keys(key.clone(), at)).await
    }

    async fn storage_fetch_descendant_values(
        &self,
        key: Vec<u8>,
        at: T::Hash,
    ) -> Result<StreamOfResults<StorageResponse>, Error> {
        retry(|| self.0.storage_fetch_descendant_values(key.clone(), at)).await
    }

    async fn genesis_hash(&self) -> Result<T::Hash, Error> {
        retry(|| self.0.genesis_hash()).await
    }

    async fn block_header(&self, at: T::Hash) -> Result<Option<T::Header>, Error> {
        retry(|| self.0.block_header(at)).await
    }

    async fn block_body(&self, at: T::Hash) -> Result<Option<Vec<Vec<u8>>>, Error> {
        retry(|| self.0.block_body(at)).await
    }

    async fn latest_finalized_block_ref(&self) -> Result<BlockRef<T::Hash>, Error> {
        retry(|| self.0.latest_finalized_block_ref()).await
    }

    async fn current_runtime_version(&self) -> Result<RuntimeVersion, Error> {
        retry(|| self.0.current_runtime_version()).await
    }

    async fn stream_runtime_version(&self) -> Result<RuntimeVersionSubscription<T>, Error> {
        retry(|| self.0.stream_runtime_version()).await
    }

    async fn stream_all_block_headers(&self) -> Result<BlockSubscription<T>, Error> {
        retry(|| self.0.stream_all_block_headers()).await
    }

    async fn stream_best_block_headers(&self) -> Result<BlockSubscription<T>, Error> {
        retry(|| self.0.stream_best_block_headers()).await
    }

    async fn stream_finalized_block_headers(&self) -> Result<BlockSubscription<T>, Error> {
        retry(|| self.0.stream_finalized_block_headers()).await
    }

    async fn submit_transaction(
        &self,
        extrinsic: &[u8],
    ) -> Result<SubmitTransactionSubscription<T>, Error> {
        retry(|| self.0.submit_transaction(extrinsic)).await
    }

    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: T::Hash,
    ) -> Result<Vec<u8>, Error> {
        retry(|| self.0.call(method, call_parameters, at)).await
    }
}
