// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::sync::Arc;
use parking_lot::RwLock;
use super::{
    OfflineClient,
};
use futures::future;
use crate::{
    Config,
    Call,
    Encoded,
    rpc::{
        Rpc,
        RpcClient,
        RuntimeVersion
    },
    error::{
        BasicError,
    },
    extrinsic::{
        Signer,
        ExtrinsicParams,
    },
    Metadata,
};
use codec::{
    Compact,
    Encode,
};
use derivative::Derivative;

/// A client capable of perfomring offline or online operations. This
/// builds on [`OfflineClient`] to provide connectivity to a node.
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct OnlineClient<T: Config> {
    inner: Arc<RwLock<OfflineClient<T>>>,
    rpc: Rpc<T>,
}

impl <T: Config> OnlineClient<T> {
    /// Construct a new [`OnlineClient`] using default settings which
    /// point to a locally running node on `ws://127.0.0.1:9944`.
    pub async fn new() -> Result<OnlineClient<T>, BasicError> {
        let url = "ws://127.0.0.1:9944";
        OnlineClient::from_url(url).await
    }

    /// Construct a new [`OnlineClient`], providing a URL to connect to.
    pub async fn from_url(url: impl AsRef<str>) -> Result<OnlineClient<T>, BasicError> {
        let client = crate::rpc::ws_client(url.as_ref()).await?;
        OnlineClient::from_rpc_client(client).await
    }

    /// Construct a new [`OnlineClient`] by providing the underlying [`RpcClient`]
    /// to use to drive the connection.
    pub async fn from_rpc_client(rpc_client: impl Into<RpcClient>) -> Result<OnlineClient<T>, BasicError> {
        let rpc = Rpc::new(rpc_client.into());

        let (genesis_hash, runtime_version, metadata) = future::join3(
            rpc.genesis_hash(),
            rpc.runtime_version(None),
            rpc.metadata()
        )
        .await;

        Ok(OnlineClient {
            inner: Arc::new(RwLock::new(OfflineClient::new(
                genesis_hash?,
                runtime_version?,
                metadata?,
            ))),
            rpc
        })
    }

    /// Return an [`OfflineClient`] to use.
    pub fn offline(&self) -> OfflineClient<T> {
        let inner = self.inner.read();
        // This is fairly cheap:
        (*inner).clone()
    }

    /// Return the [`Metadata`] used in this client.
    pub fn metadata(&self) -> Metadata {
        let inner = self.inner.read();
        inner.metadata().clone()
    }

    /// Return the genesis hash.
    pub fn genesis_hash(&self) -> T::Hash {
        let inner = self.inner.read();
        *inner.genesis_hash()
    }

    /// Return the runtime version.
    pub fn runtime_version(&self) -> RuntimeVersion {
        let inner = self.inner.read();
        inner.runtime_version().clone()
    }

    /// Return the RPC interface.
    pub fn rpc(&self) -> &Rpc<T> {
        &self.rpc
    }
}

// These allow implementations to take in something like
// `impl Into<OfflineClient>` and have either an online or
// offline client (or references to) "just work", for ergonomics.
impl <T: Config> From<OnlineClient<T>> for OfflineClient<T> {
    fn from(client: OnlineClient<T>) -> Self {
        client.offline()
    }
}
impl <'a, T: Config> From<&'a OnlineClient<T>> for OfflineClient<T> {
    fn from(client: &'a OnlineClient<T>) -> Self {
        client.offline()
    }
}
impl <'a, T: Config> From<&'a OnlineClient<T>> for OnlineClient<T> {
    fn from(client: &'a OnlineClient<T>) -> Self {
        client.clone()
    }
}