// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::sync::Arc;
use std::time::Duration;
use substrate_runner::SubstrateNode;
use subxt::backend::rpc::reconnecting_rpc_client::{ExponentialBackoff, RpcClientBuilder};
use subxt::{
    backend::{legacy, rpc, unstable},
    Config, OnlineClient,
};

// The URL that we'll connect to for our tests comes from SUBXT_TEXT_HOST env var,
// defaulting to localhost if not provided. If the env var is set, we won't spawn
// a binary. Note though that some tests expect and modify a fresh state, and so will
// fail. Fo a similar reason wyou should also use `--test-threads 1` when running tests
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
    unstable_client: RefCell<Option<OnlineClient<R>>>,
    legacy_client: RefCell<Option<OnlineClient<R>>>,

    rpc_client: rpc::RpcClient,
    client: OnlineClient<R>,
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

    pub async fn restart(mut self) -> Self {
        tokio::task::spawn_blocking(move || {
            if let Some(ref mut proc) = &mut self.proc {
                proc.restart().unwrap();
            }
            self
        })
        .await
        .expect("to succeed")
    }

    /// Hand back an RPC client connected to the test node which exposes the legacy RPC methods.
    pub async fn legacy_rpc_methods(&self) -> legacy::LegacyRpcMethods<R> {
        let rpc_client = self.rpc_client.clone();
        legacy::LegacyRpcMethods::new(rpc_client)
    }

    /// Hand back an RPC client connected to the test node which exposes the unstable RPC methods.
    pub async fn unstable_rpc_methods(&self) -> unstable::UnstableRpcMethods<R> {
        let rpc_client = self.rpc_client.clone();
        unstable::UnstableRpcMethods::new(rpc_client)
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
    pub fn client(&self) -> OnlineClient<R> {
        self.client.clone()
    }

    /// Returns the rpc client connected to the node
    pub fn rpc_client(&self) -> rpc::RpcClient {
        self.rpc_client.clone()
    }
}

/// Kind of rpc client to use in tests
pub enum RpcClientKind {
    Legacy,
    UnstableReconnecting,
}

/// Construct a test node process.
pub struct TestNodeProcessBuilder {
    node_paths: Vec<OsString>,
    authority: Option<String>,
    rpc_client: RpcClientKind,
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
            rpc_client: RpcClientKind::Legacy,
        }
    }

    /// Set the testRunner to use a preferred RpcClient impl, ie Legacy or Unstable
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
    pub async fn spawn<R>(self) -> Result<TestNodeProcess<R>, String>
    where
        R: Config,
    {
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
            RpcClientKind::UnstableReconnecting => build_unstable_rpc_client(&ws_url).await,
        }
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
    let rpc_client = rpc::RpcClient::from_insecure_url(ws_url)
        .await
        .map_err(|e| format!("Cannot construct RPC client: {e}"))?;

    Ok(rpc_client)
}

async fn build_unstable_rpc_client(ws_url: &str) -> Result<rpc::RpcClient, String> {
    let client = RpcClientBuilder::new()
        .retry_policy(ExponentialBackoff::from_millis(100).max_delay(Duration::from_secs(10)))
        .build(ws_url.to_string())
        .await
        .map_err(|e| format!("Cannot construct RPC client: {e}"))?;

    Ok(rpc::RpcClient::new(client))
}

async fn build_legacy_client<T: Config>(
    rpc_client: rpc::RpcClient,
) -> Result<OnlineClient<T>, String> {
    let backend = legacy::LegacyBackend::builder().build(rpc_client);
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
async fn build_light_client<T: Config>(
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

    // Wait for a few blocks to be produced using the subxt client.
    let client = OnlineClient::<T>::from_url(ws_url.clone())
        .await
        .map_err(|err| format!("Failed to connect to node rpc at {ws_url}: {err}"))?;

    // Wait for at least a few blocks before starting the light client.
    // Otherwise, the lightclient might error with
    // `"Error when retrieving the call proof: No node available for call proof query"`.
    super::wait_for_number_of_blocks(&client, 5).await;

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
    build_unstable_client(rpc.into()).await
}
