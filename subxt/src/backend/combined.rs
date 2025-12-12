// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a backend implementation which will lookup the methods available
//! to it from the RPC client and call methods accordingly.

use crate::backend::chain_head::ChainHeadBackendDriver;
use crate::backend::{
    Backend, BlockRef, StorageResponse, StreamOfResults, TransactionStatus,
    archive::ArchiveBackend, chain_head::ChainHeadBackend, legacy::LegacyBackend,
};
use crate::config::{Config, HashFor};
use crate::error::{BackendError, CombinedBackendError};
use async_trait::async_trait;
use futures::Stream;
use futures::StreamExt;
use std::task::Poll;
use subxt_rpcs::RpcClient;

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

impl<T: Config> CombinedBackendBuilder<T> {
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
    pub async fn build(
        self,
        rpc_client: impl Into<RpcClient>,
    ) -> Result<(CombinedBackend<T>, CombinedBackendDriver<T>), CombinedBackendError> {
        let rpc_client = rpc_client.into();

        // What does the thing wer're talking to actually know about?
        #[derive(serde::Deserialize)]
        struct Methods {
            methods: Vec<String>,
        }
        let methods: Methods = rpc_client
            .request("rpc_methods", subxt_rpcs::rpc_params![])
            .await
            .map_err(CombinedBackendError::CouldNotObtainRpcMethodList)?;
        let methods = methods.methods;

        let has_archive_methods = methods.iter().any(|m| m.starts_with("archive_v1_"));
        let has_chainhead_methods = methods.iter().any(|m| m.starts_with("chainHead_v1"));

        let mut combined_driver = CombinedBackendDriver {
            chainhead_driver: None,
        };

        let archive = if has_archive_methods {
            match self.archive {
                BackendChoice::Use(b) => Some(b),
                BackendChoice::UseDefault => Some(ArchiveBackend::new(rpc_client.clone())),
                BackendChoice::DontUse => None,
            }
        } else {
            None
        };

        let chainhead = if has_chainhead_methods {
            match self.chainhead {
                BackendChoice::Use(b) => Some(b),
                BackendChoice::UseDefault => {
                    let (chainhead, chainhead_driver) =
                        ChainHeadBackend::builder().build(rpc_client.clone());
                    combined_driver.chainhead_driver = Some(chainhead_driver);
                    Some(chainhead)
                }
                BackendChoice::DontUse => None,
            }
        } else {
            None
        };

        let legacy = match self.legacy {
            BackendChoice::Use(b) => Some(b),
            BackendChoice::UseDefault => Some(LegacyBackend::builder().build(rpc_client.clone())),
            BackendChoice::DontUse => None,
        };

        let combined = CombinedBackend {
            archive,
            chainhead,
            legacy,
        };

        Ok((combined, combined_driver))
    }

    /// An API to build the backend and driver which will run in the background until completion
    /// on the default runtime.
    ///
    /// - On non-wasm targets, this will spawn the driver on `tokio`.
    /// - On wasm targets, this will spawn the driver on `wasm-bindgen-futures`.
    #[cfg(feature = "runtime")]
    pub async fn build_with_background_driver(
        self,
        client: impl Into<RpcClient>,
    ) -> Result<CombinedBackend<T>, CombinedBackendError> {
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
    chainhead_driver: Option<ChainHeadBackendDriver<T>>,
}

impl<T: Config> CombinedBackendDriver<T> {
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
            None => Poll::Ready(None),
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

impl<T: Config> CombinedBackend<T> {
    /// Configure and construct a [`CombinedBackend`].
    pub fn builder() -> CombinedBackendBuilder<T> {
        CombinedBackendBuilder::new()
    }

    fn archive(&self) -> Option<&dyn Backend<T>> {
        self.archive.as_ref().map(|a| {
            let a: &dyn Backend<T> = a;
            a
        })
    }

    fn chainhead(&self) -> Option<&dyn Backend<T>> {
        self.chainhead.as_ref().map(|a| {
            let a: &dyn Backend<T> = a;
            a
        })
    }

    fn legacy(&self) -> Option<&dyn Backend<T>> {
        self.legacy.as_ref().map(|a| {
            let a: &dyn Backend<T> = a;
            a
        })
    }
}

impl<T: Config> super::sealed::Sealed for CombinedBackend<T> {}

// Our default behaviour:
// - Try the archive backend first if it's available. Why? It has all block headers/bodies
//   etc so it's mroe likely to succeed than chainHead backend and give back things that won't
//   expire.
// - If archive calls aren't available, fall back to the chainHead backend. Blocks given back
//   by this are more likely to expire.
// - If neither exists / works, we fall back to the legacy methods. These have some limits on
//   what is available (often fewer limits than chainHead though) but tend to do the job. We'd
//   rather not use these as they are old and should go away, but until then they are a good
//   fallback.
#[async_trait]
impl<T: Config> Backend<T> for CombinedBackend<T> {
    async fn storage_fetch_values(
        &self,
        keys: Vec<Vec<u8>>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        try_backends(
            &[self.archive(), self.chainhead(), self.legacy()],
            async |b: &dyn Backend<T>| b.storage_fetch_values(keys.clone(), at).await,
        )
        .await
    }

    async fn storage_fetch_descendant_keys(
        &self,
        key: Vec<u8>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<Vec<u8>>, BackendError> {
        try_backends(
            &[self.archive(), self.chainhead(), self.legacy()],
            async |b: &dyn Backend<T>| b.storage_fetch_descendant_keys(key.clone(), at).await,
        )
        .await
    }

    async fn storage_fetch_descendant_values(
        &self,
        key: Vec<u8>,
        at: HashFor<T>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        try_backends(
            &[self.archive(), self.chainhead(), self.legacy()],
            async |b: &dyn Backend<T>| b.storage_fetch_descendant_values(key.clone(), at).await,
        )
        .await
    }

    async fn genesis_hash(&self) -> Result<HashFor<T>, BackendError> {
        try_backends(
            &[self.archive(), self.chainhead(), self.legacy()],
            async |b: &dyn Backend<T>| b.genesis_hash().await,
        )
        .await
    }

    async fn block_number_to_hash(
        &self,
        number: u64,
    ) -> Result<Option<BlockRef<HashFor<T>>>, BackendError> {
        try_backends(
            &[
                self.archive(),
                self.legacy(),
                // chainHead last as it cannot handle this request and will fail, so it's here
                // just to hand back a more relevant error in case the above two backends aren't
                // enabled or have some issue.
                self.chainhead(),
            ],
            async |b: &dyn Backend<T>| b.block_number_to_hash(number).await,
        )
        .await
    }

    async fn block_header(&self, at: HashFor<T>) -> Result<Option<T::Header>, BackendError> {
        try_backends(
            &[self.archive(), self.chainhead(), self.legacy()],
            async |b: &dyn Backend<T>| b.block_header(at).await,
        )
        .await
    }

    async fn block_body(&self, at: HashFor<T>) -> Result<Option<Vec<Vec<u8>>>, BackendError> {
        try_backends(
            &[self.archive(), self.chainhead(), self.legacy()],
            async |b: &dyn Backend<T>| b.block_body(at).await,
        )
        .await
    }

    async fn latest_finalized_block_ref(&self) -> Result<BlockRef<HashFor<T>>, BackendError> {
        try_backends(
            &[
                // Prioritize chainHead backend since it's streaming these things; save another call.
                self.chainhead(),
                self.archive(),
                self.legacy(),
            ],
            async |b: &dyn Backend<T>| b.latest_finalized_block_ref().await,
        )
        .await
    }

    async fn stream_all_block_headers(
        &self,
        hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        try_backends(
            &[
                // Ignore archive backend; it doesn't support this.
                self.chainhead(),
                self.legacy(),
            ],
            async |b: &dyn Backend<T>| b.stream_all_block_headers(hasher.clone()).await,
        )
        .await
    }

    async fn stream_best_block_headers(
        &self,
        hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        try_backends(
            &[
                // Ignore archive backend; it doesn't support this.
                self.chainhead(),
                self.legacy(),
            ],
            async |b: &dyn Backend<T>| b.stream_best_block_headers(hasher.clone()).await,
        )
        .await
    }

    async fn stream_finalized_block_headers(
        &self,
        hasher: T::Hasher,
    ) -> Result<StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>, BackendError> {
        try_backends(
            &[
                // Ignore archive backend; it doesn't support this.
                self.chainhead(),
                self.legacy(),
            ],
            async |b: &dyn Backend<T>| b.stream_finalized_block_headers(hasher.clone()).await,
        )
        .await
    }

    async fn submit_transaction(
        &self,
        extrinsic: &[u8],
    ) -> Result<StreamOfResults<TransactionStatus<HashFor<T>>>, BackendError> {
        try_backends(
            &[
                // chainHead first as it does the same as the archive backend, but with better
                // guarantees around the block handed back being pinned & ready to access.
                self.chainhead(),
                self.legacy(),
                // archive last just incase chainHead & legacy fail or aren't provided for some
                // reason.
                self.archive(),
            ],
            async |b: &dyn Backend<T>| b.submit_transaction(extrinsic).await,
        )
        .await
    }

    async fn call(
        &self,
        method: &str,
        call_parameters: Option<&[u8]>,
        at: HashFor<T>,
    ) -> Result<Vec<u8>, BackendError> {
        try_backends(
            &[self.archive(), self.chainhead(), self.legacy()],
            async |b: &dyn Backend<T>| b.call(method, call_parameters, at).await,
        )
        .await
    }
}

/// Call one backend after the other in the list until we get a successful result back.
async fn try_backends<'s, 'b, T, Func, Fut, O>(
    backends: &'s [Option<&'b dyn Backend<T>>],
    mut f: Func,
) -> Result<O, BackendError>
where
    'b: 's,
    T: Config,
    Func: FnMut(&'b dyn Backend<T>) -> Fut,
    Fut: Future<Output = Result<O, BackendError>> + 'b,
{
    static NO_AVAILABLE_BACKEND: &str =
        "None of the configured backends are capable of handling this request";
    let mut err = BackendError::other(NO_AVAILABLE_BACKEND);

    for backend in backends.into_iter().filter_map(|b| *b) {
        match f(backend).await {
            Ok(res) => return Ok(res),
            Err(e) => err = e,
        }
    }

    Err(err)
}
