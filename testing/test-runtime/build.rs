// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{Decode, Encode};
use std::{env, fs, path::Path};
use substrate_runner::{Error as SubstrateNodeError, SubstrateNode};

// This variable accepts a single binary name or comma separated list.
static SUBSTRATE_BIN_ENV_VAR: &str = "SUBSTRATE_NODE_PATH";

const V15_METADATA_VERSION: u32 = 15;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    // Select substrate binary to run based on env var.
    let substrate_bins: String =
        env::var(SUBSTRATE_BIN_ENV_VAR).unwrap_or_else(|_| "substrate-node,substrate".to_owned());
    let substrate_bins_vec: Vec<&str> = substrate_bins.split(',').map(|s| s.trim()).collect();

    let mut node_builder = SubstrateNode::builder();
    node_builder.binary_paths(substrate_bins_vec.iter());

    let node = match node_builder.spawn() {
        Ok(node) => node,
        Err(SubstrateNodeError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {
            panic!(
                "A substrate binary should be installed on your path for testing purposes. \
                 See https://github.com/paritytech/subxt/tree/master#integration-testing"
            )
        }
        Err(e) => {
            panic!("Cannot spawn substrate command from any of {substrate_bins_vec:?}: {e}")
        }
    };

    let port = node.ws_port();
    let out_dir_env_var = env::var_os("OUT_DIR");
    let out_dir = out_dir_env_var.as_ref().unwrap().to_str().unwrap();

    let stable_metadata_path =
        download_and_save_metadata(V15_METADATA_VERSION, port, out_dir, "v15")
            .await
            .unwrap_or_else(|e| panic!("Cannot download & save v15 metadata: {e}"));

    // Write out our expression to generate the runtime API to a file. Ideally, we'd just write this code
    // in lib.rs, but we must pass a string literal (and not `concat!(..)`) as an arg to `runtime_metadata_path`,
    // and so we need to spit it out here and include it verbatim instead.
    let runtime_api_contents = format!(
        r#"
        /// Generated types for the locally running Substrate node using V15 metadata.
        #[subxt::subxt(
            runtime_metadata_path = "{stable_metadata_path}",
            derive_for_all_types = "Eq, PartialEq",
        )]
        pub mod node_runtime {{}}
    "#
    );
    let runtime_path = Path::new(&out_dir).join("runtime.rs");
    fs::write(runtime_path, runtime_api_contents).expect("Couldn't write runtime rust output");

    for substrate_node_path in substrate_bins_vec {
        let Ok(full_path) = which::which(substrate_node_path) else {
            continue;
        };

        // Re-build if the substrate binary we're pointed to changes (mtime):
        println!("cargo:rerun-if-changed={}", full_path.to_string_lossy());
    }

    // Re-build if we point to a different substrate binary:
    println!("cargo:rerun-if-env-changed={SUBSTRATE_BIN_ENV_VAR}");
    // Re-build if this file changes:
    println!("cargo:rerun-if-changed=build.rs");
}

// Download metadata from binary. Avoid Subxt dep on `subxt::rpc::types::Bytes`and just impl here.
// This may at least prevent this script from running so often (ie whenever we change Subxt).
// If there's an error, we return a string for it.
async fn download_and_save_metadata(
    version: u32,
    port: u16,
    out_dir: &str,
    suffix: &str,
) -> Result<String, String> {
    // Encode version
    let bytes = version.encode();
    let version: String = format!("0x{}", hex::encode(&bytes));

    // Connect to the client and request metadata
    let raw: String = {
        use client::ClientT;
        client::build(&format!("ws://localhost:{port}"))
            .await
            .map_err(|e| format!("Failed to connect to node: {e}"))?
            .request(
                "state_call",
                client::rpc_params!["Metadata_metadata_at_version", &version],
            )
            .await
            .map_err(|e| format!("Failed to obtain metadata from node: {e}"))?
    };

    // Decode the raw metadata
    let raw_bytes = hex::decode(raw.trim_start_matches("0x"))
        .map_err(|e| format!("Failed to hex-decode metadata: {e}"))?;
    let bytes: Option<Vec<u8>> = Decode::decode(&mut &raw_bytes[..])
        .map_err(|e| format!("Failed to decode metadata bytes: {e}"))?;
    let metadata_bytes = bytes.ok_or_else(|| "Metadata version not found".to_string())?;

    // Save metadata to a file
    let metadata_path =
        Path::new(&out_dir).join(format!("test_node_runtime_metadata_{suffix}.scale"));
    fs::write(&metadata_path, metadata_bytes)
        .map_err(|e| format!("Couldn't write metadata output: {e}"))?;

    // Convert path to string and return
    metadata_path
        .to_str()
        .ok_or_else(|| "Path to metadata should be stringifiable".to_string())
        .map(|s| s.to_owned())
}

// Use jsonrpsee to obtain metadata from the node.
mod client {
    use jsonrpsee::client_transport::ws::EitherStream;
    pub use jsonrpsee::{
        client_transport::ws::{self, Url, WsTransportClientBuilder},
        core::client::{Client, Error},
    };
    use tokio_util::compat::Compat;

    pub use jsonrpsee::core::{client::ClientT, rpc_params};
    pub type Sender = ws::Sender<Compat<EitherStream>>;
    pub type Receiver = ws::Receiver<Compat<EitherStream>>;

    /// Build WS RPC client from URL
    pub async fn build(url: &str) -> Result<Client, Error> {
        let (sender, receiver) = ws_transport(url).await?;
        Ok(Client::builder().build_with_tokio(sender, receiver))
    }

    async fn ws_transport(url: &str) -> Result<(Sender, Receiver), Error> {
        let url = Url::parse(url).map_err(|e| Error::Transport(e.into()))?;
        WsTransportClientBuilder::default()
            .build(url)
            .await
            .map_err(|e| Error::Transport(e.into()))
    }
}
