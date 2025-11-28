// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a backend implementation which will lookup the methods available
//! to it from the RPC client and call methods accordingly.

use crate::backend::chain_head::ChainHeadBackendDriver;
use crate::backend::{
    legacy::LegacyBackend,
    chain_head::{ChainHeadBackend, },
    archive::ArchiveBackend,
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
use futures::Stream;
use std::task::Poll;

pub struct CombinedBackendBuilder<T: Config> {
    archive: BackendChoice<ArchiveBackend<T>>,
    chainhead: BackendChoice<ChainHeadBackend<T>>,
    legacy: BackendChoice<LegacyBackend<T>>,
}

enum BackendChoice<V> {
    Use(V),
    DontUse,
    UseDefault,
}

impl <T: Config> CombinedBackendBuilder<T> {
    /// Create a new [`CombinedBackendBuilder`].
    pub fn new() -> Self {
        CombinedBackendBuilder {
            archive: BackendChoice::UseDefault,
            chainhead: BackendChoice::UseDefault,
            legacy: BackendChoice::UseDefault,
        }
    }

    /// Use the given [`ArchiveBackend`] where applicable.
    pub fn with_archive_backend(mut self, backend: ArchiveBackend<T>) -> Self {
        self.archive = BackendChoice::Use(backend);
        self
    }

    /// Use the given [`ChainHeadBackend`] where applicable.
    pub fn with_chainhead_backend(mut self, backend: ChainHeadBackend<T>) -> Self {
        self.chainhead = BackendChoice::Use(backend);
        self
    }

    /// Use the given [`LegacyBackend`] where applicable.
    pub fn with_legacy_backend(mut self, backend: LegacyBackend<T>) -> Self {
        self.legacy = BackendChoice::Use(backend);
        self
    }

    /// Don't use any default backends; only use what is explicitly configured via
    /// [`CombinedBackendBuilder::with_archive_backend`],
    /// [`CombinedBackendBuilder::with_chainhead_backend`] and
    /// [`CombinedBackendBuilder::with_legacy_backend`].
    pub fn no_default_backends(mut self) -> Self {
        if matches!(self.legacy, BackendChoice::UseDefault) {
            self.legacy = BackendChoice::DontUse;
        }
        if matches!(self.archive, BackendChoice::UseDefault) {
            self.archive = BackendChoice::DontUse;
        }
        if matches!(self.chainhead, BackendChoice::UseDefault) {
            self.chainhead = BackendChoice::DontUse;
        }
        self
    }

    /// A low-level API to build the backend and driver which requires polling the driver for the backend
    /// to make progress.
    ///
    /// This is useful if you want to manage the driver yourself, for example if you want to run it in on
    /// a specific runtime.
    ///
    /// If you just want to run the driver in the background until completion in on the default runtime,
    /// use [`CombinedBackendBuilder::build_with_background_driver`] instead.
    pub async fn build(self, rpc_client: impl Into<RpcClient>) -> Result<(CombinedBackend<T>, CombinedBackendDriver<T>), BackendError> {
        let rpc_client = rpc_client.into();
    
        // What does the thing wer're talking to actually know about?
        let methods: Vec<String> = rpc_client
            .request("rpc_methods", subxt_rpcs::rpc_params![])
            .await?;

        let has_archive_methods = methods.iter().any(|m| m.starts_with("archive_v1_"));
        let has_chainhead_methods = methods.iter().any(|m| m.starts_with("chainHead_v1"));

        let mut combined_driver = CombinedBackendDriver { chainhead_driver: None };

        let archive = if has_archive_methods {
            match self.archive {
                BackendChoice::Use(b) => Some(b),
                BackendChoice::UseDefault => Some(ArchiveBackend::new(rpc_client.clone())),
                BackendChoice::DontUse => None,
            }
        } else { None };

        let chainhead = if has_chainhead_methods {
            match self.chainhead {
                BackendChoice::Use(b) => Some(b),
                BackendChoice::UseDefault => {
                    let (chainhead, chainhead_driver) = ChainHeadBackend::builder().build(rpc_client.clone());
                    combined_driver.chainhead_driver = Some(chainhead_driver);
                    Some(chainhead)
                },
                BackendChoice::DontUse => None,
            }
        } else { None };

        let legacy = match self.legacy {
            BackendChoice::Use(b) => Some(b),
            BackendChoice::UseDefault => Some(LegacyBackend::builder().build(rpc_client.clone())),
            BackendChoice::DontUse => None,
        };

        let combined = CombinedBackend {
            archive,
            chainhead,
            legacy
        };

        Ok((combined, combined_driver))
    }

    /// An API to build the backend and driver which will run in the background until completion
    /// on the default runtime.
    ///
    /// - On non-wasm targets, this will spawn the driver on `tokio`.
    /// - On wasm targets, this will spawn the driver on `wasm-bindgen-futures`.
    #[cfg(feature = "runtime")]
    pub async fn build_with_background_driver(self, client: impl Into<RpcClient>) -> Result<CombinedBackend<T>, BackendError> {
        let (backend, mut driver) = self.build(client).await?;

        super::utils::spawn(async move {
            // NOTE: we need to poll the driver until it's done i.e returns None
            // to ensure that the backend is shutdown properly.
            while let Some(res) = driver.next().await {
                if let Err(err) = res {
                    tracing::debug!(target: "subxt", "chainHead backend error={err}");
                }
            }

            tracing::debug!(target: "subxt", "combined backend was closed");
        });

        Ok(backend)
    }
}

/// Driver for the [`CombinedBackend`]. This needs to be polled to ensure
/// that the [`CombinedBackend`] can make progress. It does not need polling
/// if [`CombinedBackendDriver::needs_polling`] returns `false`.
pub struct CombinedBackendDriver<T: Config> {
    chainhead_driver: Option<ChainHeadBackendDriver<T>>
}

impl <T: Config> CombinedBackendDriver<T> {
    pub fn needs_polling(&self) -> bool {
        self.chainhead_driver.is_some()
    }
}

impl<T: Config> Stream for CombinedBackendDriver<T> {
    type Item = <ChainHeadBackendDriver<T> as Stream>::Item;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match &mut self.chainhead_driver {
            Some(driver) => driver.poll_next_unpin(cx),
            None => Poll::Ready(None)
        }
    }
}

/// A combined backend. This selects which RPC calls to use based on the `rpc_methods`
/// available from the given RPC client we're given.
pub struct CombinedBackend<T: Config> {
    archive: Option<ArchiveBackend<T>>,
    chainhead: Option<ChainHeadBackend<T>>,
    legacy: Option<LegacyBackend<T>>,
}

impl <T: Config> CombinedBackend<T> {
    /// Configure and construct a [`CombinedBackend`].
    pub fn builder() -> CombinedBackendBuilder<T> {
        CombinedBackendBuilder::new()
    }
}

impl<T: Config> super::sealed::Sealed for CombinedBackend<T> {}

static NO_AVAILABLE_BACKEND: &str = "No available RPC methods to use. `no_default_backends` was used, but no applicable backends were then provided.";

macro_rules! call_backends {
    ({$($backend_name:ident)|+}. $method_name:ident ( $($arg:expr),* )) => {{
        let mut err = BackendError::other(NO_AVAILABLE_BACKEND);
        
        $(
            if let Some(backend) = &self.$backend_name {
                err = match backend.$method_name($( $arg, )*).await {
                    Ok(res) => return Ok(res),
                    Err(e) => e
                }
            }
        )+

        return Err(err)
    }}
}

#[async_trait]
impl<T: Config> Backend<T> for CombinedBackend<T> {
    async fn storage_fetch_values(
        &self,
        keys: Vec<Vec<u8>>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        call_backends!({archive|chainhead|legacy}.storage_fetch_values(keys, at))
    }

    async fn storage_fetch_descendant_keys(
        &self,
        key: Vec<u8>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<Vec<u8>>, BackendError> {

    }

    async fn storage_fetch_descendant_values(
        &self,
        key: Vec<u8>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {

    }

    async fn genesis_hash(&self) -> Result<HashFor<T>, BackendError> {

    }

    async fn block_header(&self, at: HashFor<T>) -> Result<Option<T::Header>, BackendError> {

    }

    async fn block_body(&self, at: HashFor<T>) -> Result<Option<Vec<Vec<u8>>>, BackendError> {

    }

    async fn latest_finalized_block_ref(&self) -> Result<BlockRef<HashFor<T>>, BackendError> {

    }

    async fn stream_all_block_headers(
        &self,
        _hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {

    }

    async fn stream_best_block_headers(
        &self,
        _hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {

    }

    async fn stream_finalized_block_headers(
        &self,
        _hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
    
    }

    async fn submit_transaction(
        &self,
        extrinsic: &[u8],
    ) -> Result<StreamOfResults<TransactionStatus<HashFor<T>>>, BackendError> {

    }

    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: HashFor<T>,
    ) -> Result<Vec<u8>, BackendError> {

    }
}
