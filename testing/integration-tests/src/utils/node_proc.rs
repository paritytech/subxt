// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::sync::Arc;
use substrate_runner::SubstrateNode;
use subxt::{
    backend::{legacy, rpc, unstable},
    Config, OnlineClient,
};

#[cfg(lightclient)]
use subxt::client::{LightClient, LightClientBuilder};

/// Spawn a local substrate node for testing subxt.
pub struct TestNodeProcess<R: Config> {
    // Keep a handle to the node; once it's dropped the node is killed.
    proc: SubstrateNode,

    // Lazily construct these when asked for.
    unstable_client: RefCell<Option<OnlineClient<R>>>,
    legacy_client: RefCell<Option<OnlineClient<R>>>,

    rpc_client: rpc::RpcClient,

    #[cfg(fullclient)]
    client: OnlineClient<R>,

    #[cfg(lightclient)]
    client: LightClient<R>,
}

impl<R> TestNodeProcess<R>
where
    R: Config,
{
    /// Construct a builder for spawning a test node process.
    pub fn build<P>(paths: &[P]) -> TestNodeProcessBuilder
    where
        P: AsRef<OsStr> + Clone,
    {
        TestNodeProcessBuilder::new(paths)
    }

    /// Hand back an RPC client connected to the test node which exposes the legacy RPC methods.
    pub async fn legacy_rpc_methods(&self) -> legacy::LegacyRpcMethods<R> {
        let rpc_client = self.rpc_client().await;
        legacy::LegacyRpcMethods::new(rpc_client)
    }

    /// Hand back an RPC client connected to the test node which exposes the unstable RPC methods.
    pub async fn unstable_rpc_methods(&self) -> unstable::UnstableRpcMethods<R> {
        let rpc_client = self.rpc_client().await;
        unstable::UnstableRpcMethods::new(rpc_client)
    }

    /// Hand back an RPC client connected to the test node.
    pub async fn rpc_client(&self) -> rpc::RpcClient {
        let url = format!("ws://127.0.0.1:{}", self.proc.ws_port());
        rpc::RpcClient::from_url(url)
            .await
            .expect("Unable to connect RPC client to test node")
    }

    /// Always return a client using the unstable backend.
    /// Only use for comparing backends; use [`TestNodeProcess::client()`] normally,
    /// which enables us to run each test against both backends.
    pub async fn unstable_client(&self) -> OnlineClient<R> {
        if self.unstable_client.borrow().is_none() {
            let c = build_unstable_client(self.rpc_client.clone())
                .await
                .unwrap();
            self.unstable_client.replace(Some(c));
        }
        self.unstable_client.borrow().as_ref().unwrap().clone()
    }

    /// Always return a client using the legacy backend.
    /// Only use for comparing backends; use [`TestNodeProcess::client()`] normally,
    /// which enables us to run each test against both backends.
    pub async fn legacy_client(&self) -> OnlineClient<R> {
        if self.legacy_client.borrow().is_none() {
            let c = build_legacy_client(self.rpc_client.clone()).await.unwrap();
            self.legacy_client.replace(Some(c));
        }
        self.legacy_client.borrow().as_ref().unwrap().clone()
    }

    /// Returns the subxt client connected to the running node. This client
    /// will use the legacy backend by default or the unstable backend if the
    /// "unstable-backend-client" feature is enabled, so that we can run each
    /// test against both.
    #[cfg(fullclient)]
    pub fn client(&self) -> OnlineClient<R> {
        self.client.clone()
    }

    /// Returns the subxt client connected to the running node.
    #[cfg(lightclient)]
    pub fn client(&self) -> LightClient<R> {
        self.client.clone()
    }
}

/// Construct a test node process.
pub struct TestNodeProcessBuilder {
    node_paths: Vec<OsString>,
    authority: Option<String>,
}

impl TestNodeProcessBuilder {
    pub fn new<P>(node_paths: &[P]) -> TestNodeProcessBuilder
    where
        P: AsRef<OsStr>,
    {
        // Check that paths are valid and build up vec.
        let mut paths = Vec::new();
        for path in node_paths {
            let path = path.as_ref();
            paths.push(path.to_os_string())
        }

        Self {
            node_paths: paths,
            authority: None,
        }
    }

    /// Set the authority dev account for a node in validator mode e.g. --alice.
    pub fn with_authority(&mut self, account: String) -> &mut Self {
        self.authority = Some(account);
        self
    }

    /// Spawn the substrate node at the given path, and wait for rpc to be initialized.
    pub async fn spawn<R>(self) -> Result<TestNodeProcess<R>, String>
    where
        R: Config,
    {
        let mut node_builder = SubstrateNode::builder();

        node_builder.binary_paths(&self.node_paths);

        if let Some(authority) = &self.authority {
            node_builder.arg(authority.to_lowercase());
        }

        // Spawn the node and retrieve a URL to it:
        let proc = node_builder.spawn().map_err(|e| e.to_string())?;
        let ws_url = format!("ws://127.0.0.1:{}", proc.ws_port());
        let rpc_client = build_rpc_client(&ws_url)
            .await
            .map_err(|e| format!("Failed to connect to node at {ws_url}: {e}"))?;

        // Cache whatever client we build, and None for the other.
        #[allow(unused_assignments, unused_mut)]
        let mut unstable_client = None;
        #[allow(unused_assignments, unused_mut)]
        let mut legacy_client = None;

        #[cfg(lightclient)]
        let client = build_light_client(&proc).await?;

        #[cfg(feature = "unstable-backend-client")]
        let client = {
            let client = build_unstable_client(rpc_client.clone()).await?;
            unstable_client = Some(client.clone());
            client
        };

        #[cfg(all(not(lightclient), not(feature = "unstable-backend-client")))]
        let client = {
            let client = build_legacy_client(rpc_client.clone()).await?;
            legacy_client = Some(client.clone());
            client
        };

        Ok(TestNodeProcess {
            proc,
            client,
            legacy_client: RefCell::new(legacy_client),
            unstable_client: RefCell::new(unstable_client),
            rpc_client,
        })
    }
}

async fn build_rpc_client(ws_url: &str) -> Result<rpc::RpcClient, String> {
    let rpc_client = rpc::RpcClient::from_url(ws_url)
        .await
        .map_err(|e| format!("Cannot construct RPC client: {e}"))?;

    Ok(rpc_client)
}

async fn build_legacy_client<T: Config>(
    rpc_client: rpc::RpcClient,
) -> Result<OnlineClient<T>, String> {
    let backend = legacy::LegacyBackend::new(rpc_client);
    let client = OnlineClient::from_backend(Arc::new(backend))
        .await
        .map_err(|e| format!("Cannot construct OnlineClient from backend: {e}"))?;

    Ok(client)
}

async fn build_unstable_client<T: Config>(
    rpc_client: rpc::RpcClient,
) -> Result<OnlineClient<T>, String> {
    let (backend, mut driver) = unstable::UnstableBackend::builder().build(rpc_client);

    // The unstable backend needs driving:
    tokio::spawn(async move {
        use futures::StreamExt;
        while let Some(val) = driver.next().await {
            if let Err(e) = val {
                // This is a test; bail if something does wrong and try to
                // ensure that the message makes it to some logs.
                eprintln!("Error driving unstable backend in tests (will panic): {e}");
                panic!("Error driving unstable backend in tests: {e}");
            }
        }
    });

    let client = OnlineClient::from_backend(Arc::new(backend))
        .await
        .map_err(|e| format!("Cannot construct OnlineClient from backend: {e}"))?;

    Ok(client)
}

#[cfg(lightclient)]
async fn build_light_client<T: Config>(proc: &SubstrateNode) -> Result<LightClient<T>, String> {
    // RPC endpoint.
    let ws_url = format!("ws://127.0.0.1:{}", proc.ws_port());

    // Step 1. Wait for a few blocks to be produced using the subxt client.
    let client = OnlineClient::<T>::from_url(ws_url.clone())
        .await
        .map_err(|err| format!("Failed to connect to node rpc at {ws_url}: {err}"))?;

    super::wait_for_blocks(&client).await;

    // Step 2. Construct the light client.
    // P2p bootnode.
    let bootnode = format!(
        "/ip4/127.0.0.1/tcp/{}/p2p/{}",
        proc.p2p_port(),
        proc.p2p_address()
    );

    LightClientBuilder::new()
        .bootnodes([bootnode.as_str()])
        .build_from_url(ws_url.as_str())
        .await
        .map_err(|e| format!("Failed to construct light client {}", e))
}
