// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{Decode, Encode};
use std::{env, fs, path::Path};
use substrate_runner::{Error as SubstrateNodeError, SubstrateNode};

static SUBSTRATE_BIN_ENV_VAR: &str = "SUBSTRATE_NODE_PATH";

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    // Select substrate binary to run based on env var.
    let substrate_bin = env::var(SUBSTRATE_BIN_ENV_VAR).unwrap_or_else(|_| "substrate".to_owned());

    let mut node_builder = SubstrateNode::builder();
    node_builder.binary_path(substrate_bin.clone());

    let node = match node_builder.spawn() {
        Ok(node) => node,
        Err(SubstrateNodeError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {
            panic!(
                "A substrate binary should be installed on your path for testing purposes. \
                 See https://github.com/paritytech/subxt/tree/master#integration-testing"
            )
        }
        Err(e) => {
            panic!("Cannot spawn substrate command '{substrate_bin}': {e}")
        }
    };

    let port = node.ws_port();

    // Download metadata from binary. Avoid Subxt dep on `subxt::rpc::types::Bytes`and just impl here.
    // This may at least prevent this script from running so often (ie whenever we change Subxt).
    const V15_METADATA_VERSION: u32 = 15;
    let bytes = V15_METADATA_VERSION.encode();
    let version: String = format!("0x{}", hex::encode(&bytes));
    let raw: String = {
        use client::ClientT;
        client::build(&format!("ws://localhost:{port}"))
            .await
            .unwrap_or_else(|e| panic!("Failed to connect to node: {e}"))
            .request(
                "state_call",
                client::rpc_params!["Metadata_metadata_at_version", &version],
            )
            .await
            .unwrap_or_else(|e| panic!("Failed to obtain metadata from node: {e}"))
    };
    let raw_bytes = hex::decode(raw.trim_start_matches("0x"))
        .unwrap_or_else(|e| panic!("Failed to hex-decode metadata: {e}"));
    let bytes: Option<Vec<u8>> = Decode::decode(&mut &raw_bytes[..])
        .unwrap_or_else(|e| panic!("Failed to decode metadata bytes: {e}"));
    let metadata_bytes = bytes.expect("Metadata version not found");

    // Save metadata to a file:
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let metadata_path = Path::new(&out_dir).join("metadata.scale");
    fs::write(&metadata_path, metadata_bytes).expect("Couldn't write metadata output");

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
