// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::time::Duration;
use std::sync::Arc;
use substrate_runner::SubstrateNode;
use subxt::{
    client::OnlineClient,
    backend::{ChainHeadBackend, LegacyBackend},
    config::{Config, RpcConfigFor},
    ext::subxt_rpcs::{
        methods::{chain_head, legacy},
        client::{RpcClient, reconnecting_rpc_client::{ExponentialBackoff, RpcClientBuilder}},
    }
};

// The URL that we'll connect to for our tests comes from SUBXT_TEXT_HOST env var,
// defaulting to localhost if not provided. If the env var is set, we won't spawn
// a binary. Note though that some tests expect and modify a fresh state, and so will
// fail. For a similar reason you should also use `--test-threads 1` when running tests
// to reduce the number of conflicts between state altering tests.
const URL_ENV_VAR: &str = "SUBXT_TEST_URL";
fn is_url_provided() -> bool {
    std::env::var(URL_ENV_VAR).is_ok()
}
fn get_url(port: Option<u16>) -> String {
    match (std::env::var(URL_ENV_VAR).ok(), port) {
        (Some(host), None) => host,
        (None, Some(port)) => format!("ws://127.0.0.1:{port}"),
        (Some(_), Some(_)) => {
            panic!("{URL_ENV_VAR} and port provided: only one or the other should exist")
        }
        (None, None) => {
            panic!("No {URL_ENV_VAR} or port was provided, so we don't know where to connect to")
        }
    }
}

/// Spawn a local substrate node for testing subxt.
pub struct TestNodeProcess<R: Config> {
    // Keep a handle to the node; once it's dropped the node is killed.
    proc: Option<SubstrateNode>,

    // Lazily construct these when asked for.
    chainhead_backend: RefCell<Option<OnlineClient<R>>>,
    legacy_backend: RefCell<Option<OnlineClient<R>>>,

    rpc_client: RpcClient,
    client: OnlineClient<R>,
    config: R,
}

impl<R> TestNodeProcess<R>
where
    R: Config,
{
    /// Construct a builder for spawning a test node process.
    pub fn build<P>(config: R, paths: &[P]) -> TestNodeProcessBuilder<R>
    where
        P: AsRef<OsStr> + Clone,
    {
        TestNodeProcessBuilder::new(config, paths)
    }

    pub async fn restart(mut self) -> Self {
        tokio::task::spawn_blocking(move || {
            if let Some(proc) = &mut self.proc {
                proc.restart().unwrap();
            }
            self
        })
        .await
        .expect("to succeed")
    }

    /// Hand back an RPC client connected to the test node which exposes the legacy RPC methods.
    pub async fn legacy_rpc_methods(&self) -> legacy::LegacyRpcMethods<RpcConfigFor<R>> {
        let rpc_client = self.rpc_client.clone();
        legacy::LegacyRpcMethods::new(rpc_client)
    }

    /// Hand back an RPC client connected to the test node which exposes the unstable RPC methods.
    pub async fn chainhead_rpc_methods(&self) -> chain_head::ChainHeadRpcMethods<RpcConfigFor<R>> {
        let rpc_client = self.rpc_client.clone();
        chain_head::ChainHeadRpcMethods::new(rpc_client)
    }

    /// Always return a client using the chainhead backend.
    /// Only use for comparing backends; use [`TestNodeProcess::client()`] normally,
    /// which enables us to run each test against both backends.
    pub async fn chainhead_backend(&self) -> OnlineClient<R> {
        if self.chainhead_backend.borrow().is_none() {
            let c = build_chainhead_backend(self.config.clone(), self.rpc_client.clone())
                .await
                .unwrap();
            self.chainhead_backend.replace(Some(c));
        }
        self.chainhead_backend.borrow().as_ref().unwrap().clone()
    }

    /// Always return a client using the legacy backend.
    /// Only use for comparing backends; use [`TestNodeProcess::client()`] normally,
    /// which enables us to run each test against both backends.
    pub async fn legacy_backend(&self) -> OnlineClient<R> {
        if self.legacy_backend.borrow().is_none() {
            let c = build_legacy_backend(self.config.clone(), self.rpc_client.clone()).await.unwrap();
            self.legacy_backend.replace(Some(c));
        }
        self.legacy_backend.borrow().as_ref().unwrap().clone()
    }

    /// Returns the subxt client connected to the running node. This client
    /// will use the legacy backend by default or the chainhead backend if the
    /// "chainhead-backend" feature is enabled, so that we can run each
    /// test against both.
    pub fn client(&self) -> OnlineClient<R> {
        self.client.clone()
    }

    /// Returns the rpc client connected to the node
    pub fn rpc_client(&self) -> RpcClient {
        self.rpc_client.clone()
    }
}

/// Kind of rpc client to use in tests
pub enum RpcClientKind {
    Legacy,
    Reconnecting,
}

/// Construct a test node process.
pub struct TestNodeProcessBuilder<T: Config> {
    config: T,
    node_paths: Vec<OsString>,
    authority: Option<String>,
    rpc_client: RpcClientKind,
}

impl <T: Config> TestNodeProcessBuilder<T> {
    pub fn new<P>(config: T, node_paths: &[P]) -> TestNodeProcessBuilder<T>
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
            config,
            node_paths: paths,
            authority: None,
            rpc_client: RpcClientKind::Legacy,
        }
    }

    /// Set the testRunner to use a preferred RpcClient impl, ie Legacy or Reconnecting.
    pub fn with_rpc_client_kind(&mut self, rpc_client_kind: RpcClientKind) -> &mut Self {
        self.rpc_client = rpc_client_kind;
        self
    }

    /// Set the authority dev account for a node in validator mode e.g. --alice.
    pub fn with_authority(&mut self, account: String) -> &mut Self {
        self.authority = Some(account);
        self
    }

    /// Spawn the substrate node at the given path, and wait for rpc to be initialized.
    pub async fn spawn(self) -> Result<TestNodeProcess<T>, String> {
        // Only spawn a process if a URL to target wasn't provided as an env var.
        let proc = if !is_url_provided() {
            let mut node_builder = SubstrateNode::builder();
            node_builder.binary_paths(&self.node_paths);

            if let Some(authority) = &self.authority {
                node_builder.arg(authority.to_lowercase());
            }

            Some(node_builder.spawn().map_err(|e| e.to_string())?)
        } else {
            None
        };

        let ws_url = get_url(proc.as_ref().map(|p| p.ws_port()));
        let rpc_client = match self.rpc_client {
            RpcClientKind::Legacy => build_rpc_client(&ws_url).await,
            RpcClientKind::Reconnecting => build_reconnecting_rpc_client(&ws_url).await,
        }
        .map_err(|e| format!("Failed to connect to node at {ws_url}: {e}"))?;

        // Cache whatever client we build, and None for the other.
        #[allow(unused_assignments, unused_mut)]
        let mut chainhead_backend = None;
        #[allow(unused_assignments, unused_mut)]
        let mut legacy_backend = None;

        #[cfg(lightclient)]
        let client = build_light_client(self.config.clone(), &proc).await?;

        #[cfg(chainhead_backend)]
        let client = {
            let client = build_chainhead_backend(self.config.clone(), rpc_client.clone()).await?;
            chainhead_backend = Some(client.clone());
            client
        };

        #[cfg(all(not(lightclient), legacy_backend))]
        let client = {
            let client = build_legacy_backend(self.config.clone(), rpc_client.clone()).await?;
            legacy_backend = Some(client.clone());
            client
        };

        Ok(TestNodeProcess {
            proc,
            client,
            config: self.config,
            legacy_backend: RefCell::new(legacy_backend),
            chainhead_backend: RefCell::new(chainhead_backend),
            rpc_client,
        })
    }
}

async fn build_rpc_client(ws_url: &str) -> Result<RpcClient, String> {
    let rpc_client = RpcClient::from_insecure_url(ws_url)
        .await
        .map_err(|e| format!("Cannot construct RPC client: {e}"))?;

    Ok(rpc_client)
}

async fn build_reconnecting_rpc_client(ws_url: &str) -> Result<RpcClient, String> {
    let client = RpcClientBuilder::new()
        .retry_policy(ExponentialBackoff::from_millis(100).max_delay(Duration::from_secs(10)))
        .build(ws_url.to_string())
        .await
        .map_err(|e| format!("Cannot construct RPC client: {e}"))?;

    Ok(RpcClient::new(client))
}

async fn build_legacy_backend<T: Config>(
    config: T,
    rpc_client: RpcClient,
) -> Result<OnlineClient<T>, String> {
    let backend = LegacyBackend::<T>::builder().build(rpc_client);
    let client = OnlineClient::from_backend(config, Arc::new(backend))
        .await
        .map_err(|e| format!("Cannot construct OnlineClient from backend: {e}"))?;

    Ok(client)
}

async fn build_chainhead_backend<T: Config>(
    config: T,
    rpc_client: RpcClient,
) -> Result<OnlineClient<T>, String> {
    let backend = ChainHeadBackend::builder().build_with_background_driver(rpc_client);
    let client = OnlineClient::from_backend(config, Arc::new(backend))
        .await
        .map_err(|e| format!("Cannot construct OnlineClient from backend: {e}"))?;

    Ok(client)
}

#[cfg(lightclient)]
async fn build_light_client<T: Config>(
    config: T,
    maybe_proc: &Option<SubstrateNode>,
) -> Result<OnlineClient<T>, String> {
    use subxt::lightclient::{ChainConfig, LightClient};

    let proc = if let Some(proc) = maybe_proc {
        proc
    } else {
        return Err("Cannot build light client: no substrate node is running (you can't start a light client when pointing to an external node)".into());
    };

    // RPC endpoint. Only localhost works.
    let ws_url = format!("ws://127.0.0.1:{}", proc.ws_port());

    // Wait for a few blocks to be produced. We instantiate a Subxt client for
    // this for simplicity, but then throw it away.
    {
        let client = OnlineClient::<T>::from_url(config.clone(), ws_url.clone())
            .await
            .map_err(|err| format!("Failed to connect to node rpc at {ws_url}: {err}"))?;
    
        // Wait for at least a few blocks before starting the light client.
        // Otherwise, the lightclient might error with
        // `"Error when retrieving the call proof: No node available for call proof query"`.
        super::wait_for_number_of_blocks(&client, 5).await;
    }

    // Now, configure a light client; fetch the chain spec and modify the bootnodes.
    let bootnode = format!(
        "/ip4/127.0.0.1/tcp/{}/p2p/{}",
        proc.p2p_port(),
        proc.p2p_address()
    );

    let chain_spec = subxt::utils::fetch_chainspec_from_rpc_node(ws_url.as_str())
        .await
        .map_err(|e| format!("Failed to obtain chain spec from local machine: {e}"))?;

    let chain_config = ChainConfig::chain_spec(chain_spec.get())
        .set_bootnodes([bootnode.as_str()])
        .map_err(|e| format!("Light client: cannot update boot nodes: {e}"))?;

    // Instantiate the light client.
    let (_lightclient, rpc) = LightClient::relay_chain(chain_config)
        .map_err(|e| format!("Light client: cannot add relay chain: {e}"))?;

    // Instantiate subxt client from this.
    build_chainhead_backend(rpc.into()).await
}
