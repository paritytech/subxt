// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::{
    env, fs,
    net::TcpListener,
    ops::{Deref, DerefMut},
    path::Path,
    process::Command,
    thread, time,
};

static SUBSTRATE_BIN_ENV_VAR: &str = "SUBSTRATE_NODE_PATH";

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    // Select substrate binary to run based on env var.
    let substrate_bin = env::var(SUBSTRATE_BIN_ENV_VAR).unwrap_or_else(|_| "substrate".to_owned());

    // Run binary.
    let port = next_open_port().expect("Cannot spawn substrate: no available ports");
    let cmd = Command::new(&substrate_bin)
        .arg("--dev")
        .arg("--tmp")
        .arg(format!("--ws-port={port}"))
        .spawn();
    let mut cmd = match cmd {
        Ok(cmd) => KillOnDrop(cmd),
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            panic!(
                "A substrate binary should be installed on your path for testing purposes. \
            See https://github.com/paritytech/subxt/tree/master#integration-testing"
            )
        }
        Err(e) => {
            panic!("Cannot spawn substrate command '{substrate_bin}': {e}")
        }
    };

    // Download metadata from binary; retry until successful, or a limit is hit.
    let metadata_bytes: subxt::rpc::types::Bytes = {
        const MAX_RETRIES: usize = 6;
        let mut retries = 0;

        loop {
            if retries >= MAX_RETRIES {
                panic!("Cannot connect to substrate node after {retries} retries");
            }

            // It might take a while for substrate node that spin up the RPC server.
            // Thus, the connection might get rejected a few times.
            use client::ClientT;
            let res = match client::build(&format!("ws://localhost:{port}")).await {
                Ok(c) => c.request("state_getMetadata", client::rpc_params![]).await,
                Err(e) => Err(e),
            };

            match res {
                Ok(res) => {
                    let _ = cmd.kill();
                    break res;
                }
                _ => {
                    thread::sleep(time::Duration::from_secs(1 << retries));
                    retries += 1;
                }
            };
        }
    };

    // Save metadata to a file:
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let metadata_path = Path::new(&out_dir).join("metadata.scale");
    fs::write(&metadata_path, metadata_bytes.0).expect("Couldn't write metadata output");

    // Write out our expression to generate the runtime API to a file. Ideally, we'd just write this code
    // in lib.rs, but we must pass a string literal (and not `concat!(..)`) as an arg to `runtime_metadata_path`,
    // and so we need to spit it out here and include it verbatim instead.
    let runtime_api_contents = format!(
        r#"
        #[subxt::subxt(
            runtime_metadata_path = "{}",
            derive_for_all_types = "Eq, PartialEq",
        )]
        pub mod node_runtime {{}}
    "#,
        metadata_path
            .to_str()
            .expect("Path to metadata should be stringifiable")
    );
    let runtime_path = Path::new(&out_dir).join("runtime.rs");
    fs::write(runtime_path, runtime_api_contents).expect("Couldn't write runtime rust output");

    let substrate_path =
        which::which(substrate_bin).expect("Cannot resolve path to substrate binary");

    // Re-build if the substrate binary we're pointed to changes (mtime):
    println!(
        "cargo:rerun-if-changed={}",
        substrate_path.to_string_lossy()
    );
    // Re-build if we point to a different substrate binary:
    println!("cargo:rerun-if-env-changed={SUBSTRATE_BIN_ENV_VAR}");
    // Re-build if this file changes:
    println!("cargo:rerun-if-changed=build.rs");
}

/// Returns the next open port, or None if no port found.
fn next_open_port() -> Option<u16> {
    match TcpListener::bind(("127.0.0.1", 0)) {
        Ok(listener) => {
            if let Ok(address) = listener.local_addr() {
                Some(address.port())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// If the substrate process isn't explicitly killed on drop,
/// it seems that panics that occur while the command is running
/// will leave it running and block the build step from ever finishing.
/// Wrapping it in this prevents this from happening.
struct KillOnDrop(std::process::Child);

impl Deref for KillOnDrop {
    type Target = std::process::Child;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for KillOnDrop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Drop for KillOnDrop {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

// Use jsonrpsee to obtain metadata from the node.
mod client {
    pub use jsonrpsee::{
        client_transport::ws::{InvalidUri, Receiver, Sender, Uri, WsTransportClientBuilder},
        core::{
            client::{Client, ClientBuilder},
            Error,
        },
    };

    pub use jsonrpsee::core::{client::ClientT, rpc_params};

    /// Build WS RPC client from URL
    pub async fn build(url: &str) -> Result<Client, Error> {
        let (sender, receiver) = ws_transport(url).await?;
        Ok(ClientBuilder::default().build_with_tokio(sender, receiver))
    }

    async fn ws_transport(url: &str) -> Result<(Sender, Receiver), Error> {
        let url: Uri = url
            .parse()
            .map_err(|e: InvalidUri| Error::Transport(e.into()))?;
        WsTransportClientBuilder::default()
            .build(url)
            .await
            .map_err(|e| Error::Transport(e.into()))
    }
}
