// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use futures::future;
use lazy_static::lazy_static;
use sp_keyring::AccountKeyring;
use std::{
    ffi::{OsStr, OsString},
    io::{BufRead, BufReader, Read},
    process,
    sync::Arc,
};
use subxt::{
    client::default_rpc_client,
    rpc::{types::RuntimeVersion, Rpc},
    Config, Metadata, OnlineClient, SubstrateConfig,
};
use tokio::sync::Mutex;

/// Spawn a local substrate node for testing subxt.
pub struct TestNodeProcess<R: Config> {
    proc: process::Child,
    client: OnlineClient<R>,
}

impl<R> Drop for TestNodeProcess<R>
where
    R: Config,
{
    fn drop(&mut self) {
        let _ = self.kill();
    }
}

impl<R> TestNodeProcess<R>
where
    R: Config,
{
    /// Construct a builder for spawning a test node process.
    pub fn build<S>(program: S) -> TestNodeProcessBuilder
    where
        S: AsRef<OsStr> + Clone,
    {
        TestNodeProcessBuilder::new(program)
    }

    /// Attempt to kill the running substrate process.
    pub fn kill(&mut self) -> Result<(), String> {
        tracing::info!("Killing node process {}", self.proc.id());
        if let Err(err) = self.proc.kill() {
            let err = format!("Error killing node process {}: {}", self.proc.id(), err);
            tracing::error!("{}", err);
            return Err(err);
        }

        Ok(())
    }

    /// Returns the subxt client connected to the running node.
    pub fn client(&self) -> OnlineClient<R> {
        self.client.clone()
    }
}

/// Construct a test node process.
pub struct TestNodeProcessBuilder {
    node_path: OsString,
    authority: Option<AccountKeyring>,
}

impl TestNodeProcessBuilder {
    pub fn new<P>(node_path: P) -> TestNodeProcessBuilder
    where
        P: AsRef<OsStr>,
    {
        Self {
            node_path: node_path.as_ref().into(),
            authority: None,
        }
    }

    /// Set the authority dev account for a node in validator mode e.g. --alice.
    pub fn with_authority(&mut self, account: AccountKeyring) -> &mut Self {
        self.authority = Some(account);
        self
    }

    async fn fetch_cache_data(ws_url: String) -> Result<(RuntimeVersion, Metadata), String> {
        let rpc_client = default_rpc_client(ws_url)
            .await
            .map_err(|err| format!("Cannot build default rpc client: {:?}", err))?;

        let rpc = Rpc::<SubstrateConfig>::new(Arc::new(rpc_client));

        let (runtime_version, metadata) =
            future::join(rpc.runtime_version(None), rpc.metadata(None)).await;
        Ok((
            runtime_version.map_err(|err| format!("Cannot fetch runtime version: {:?}", err))?,
            metadata.map_err(|err| format!("Cannot fetch metadata {:?}", err))?,
        ))
    }

    async fn get_cache<R: Config>(
        ws_url: String,
    ) -> Result<(R::Hash, RuntimeVersion, Metadata), String> {
        lazy_static! {
            // Cannot place the `<R as Config>::Hash` associated type from the outer function into the static mutex cache.
            static ref CACHE: Mutex<Option<(RuntimeVersion, Metadata)>> = Mutex::new(None);
        }

        let mut cache = CACHE.lock().await;

        let rpc_client = default_rpc_client(ws_url.clone())
            .await
            .expect("Cannot build default rpc client");

        let rpc = Rpc::<R>::new(Arc::new(rpc_client));

        // Fetch the genesis hash to avoid a `mem::transmute`, or to limit the `TestNodeProcess` to `SubstrateConfig` only.
        let genesis = rpc
            .genesis_hash()
            .await
            .map_err(|err| format!("Cannot fetch genesis: {:?}", err))?;

        match &mut *cache {
            Some((runtime, metadata)) => Ok((genesis, runtime.clone(), metadata.clone())),
            None => {
                let (runtime_version, metadata) =
                    TestNodeProcessBuilder::fetch_cache_data(ws_url).await?;

                *cache = Some((runtime_version.clone(), metadata.clone()));

                Ok((genesis, runtime_version, metadata))
            }
        }
    }

    /// Spawn the substrate node at the given path, and wait for rpc to be initialized.
    pub async fn spawn<R>(&self) -> Result<TestNodeProcess<R>, String>
    where
        R: Config,
    {
        let mut cmd = process::Command::new(&self.node_path);
        cmd.env("RUST_LOG", "info")
            .arg("--dev")
            .arg("--tmp")
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .arg("--port=0")
            .arg("--rpc-port=0")
            .arg("--ws-port=0");

        if let Some(authority) = self.authority {
            let authority = format!("{authority:?}");
            let arg = format!("--{}", authority.as_str().to_lowercase());
            cmd.arg(arg);
        }

        let mut proc = cmd.spawn().map_err(|e| {
            format!(
                "Error spawning substrate node '{}': {}",
                self.node_path.to_string_lossy(),
                e
            )
        })?;

        // Wait for RPC port to be logged (it's logged to stderr):
        let stderr = proc.stderr.take().unwrap();
        let ws_port = find_substrate_port_from_output(stderr);
        let ws_url = format!("ws://127.0.0.1:{ws_port}");

        let (genesis, runtime, metadata) =
            TestNodeProcessBuilder::get_cache::<R>(ws_url.clone()).await?;

        let rpc_client = default_rpc_client(ws_url.clone())
            .await
            .map_err(|err| err.to_string())?;

        // Connect to the node with a subxt client:
        let client =
            OnlineClient::from_rpc_client_with(genesis, runtime, metadata, Arc::new(rpc_client));
        match client {
            Ok(client) => Ok(TestNodeProcess { proc, client }),
            Err(err) => {
                let err = format!("Failed to connect to node rpc at {ws_url}: {err}");
                tracing::error!("{}", err);
                proc.kill().map_err(|e| {
                    format!("Error killing substrate process '{}': {}", proc.id(), e)
                })?;
                Err(err)
            }
        }
    }
}

// Consume a stderr reader from a spawned substrate command and
// locate the port number that is logged out to it.
fn find_substrate_port_from_output(r: impl Read + Send + 'static) -> u16 {
    BufReader::new(r)
        .lines()
        .find_map(|line| {
            let line = line.expect("failed to obtain next line from stdout for port discovery");

            // does the line contain our port (we expect this specific output from substrate).
            let line_end = line
                .rsplit_once("Listening for new connections on 127.0.0.1:")
                .or_else(|| line.rsplit_once("Running JSON-RPC WS server: addr=127.0.0.1:"))
                .map(|(_, port_str)| port_str)?;

            // trim non-numeric chars from the end of the port part of the line.
            let port_str = line_end.trim_end_matches(|b: char| !b.is_ascii_digit());

            // expect to have a number here (the chars after '127.0.0.1:') and parse them into a u16.
            let port_num = port_str
                .parse()
                .unwrap_or_else(|_| panic!("valid port expected for log line, got '{port_str}'"));

            Some(port_num)
        })
        .expect("We should find a port before the reader ends")
}
