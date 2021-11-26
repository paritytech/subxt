// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use serde_json::json;
use std::{
    env,
    fs,
    net::TcpListener,
    path::Path,
    process::Command,
    sync::atomic::{
        AtomicU16,
        Ordering,
    },
    thread,
    time,
};

static SUBSTRATE_BIN_ENV_VAR: &'static str = "SUBSTRATE_NODE_PATH";

fn main() {
    // Select substrate binary to run based on env var.
    let substrate_bin = env::var(SUBSTRATE_BIN_ENV_VAR).unwrap_or("substrate".to_owned());

    // Run binary, waiting for RPC to start.
    let port = next_open_port()
        .expect("Cannot spawn substrate: no available ports in the given port range");
    let cmd = Command::new(&substrate_bin)
        .arg("--dev")
        .arg("--tmp")
        .arg(format!("--rpc-port={}", port))
        .spawn();
    let mut cmd = match cmd {
        Ok(cmd) => cmd,
        Err(e) => {
            panic!(
                "Cannot spawn substrate command '{}': {}",
                SUBSTRATE_BIN_ENV_VAR, e
            )
        }
    };

    // Download metadata from binary; retry until successful, or a limit is hit.
    let res: serde_json::Value = {
        const MAX_RETRIES: usize = 20;
        let mut retries = 0;
        let mut wait_secs = 1;
        loop {
            if retries >= MAX_RETRIES {
                panic!("Cannot connect to substrate node after {} retries", retries);
            }
            let res = reqwest::blocking::Client::new()
                .post(format!("http://localhost:{}", port))
                .json(&json!({
                   "id": 1,
                   "jsonrpc": "2.0",
                   "method": "state_getMetadata",
                }))
                .send();
            match res {
                Ok(res) if res.status().is_success() => {
                    println!("RESPONSE: {:?}", res);
                    let _ = cmd.kill();
                    break res
                        .json()
                        .expect("valid JSON response from substrate node expected")
                }
                _ => {
                    thread::sleep(time::Duration::from_secs(wait_secs));
                    retries += 1;
                    wait_secs += 1;
                }
            };
        }
    };
    let metadata_hex = res["result"]
        .as_str()
        .expect("Metadata should be returned as a string of hex encoded SCALE bytes");
    let metadata_bytes = hex::decode(&metadata_hex.trim_start_matches("0x")).unwrap();

    // Save metadata to a file:
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let metadata_path = Path::new(&out_dir).join("metadata.scale");
    fs::write(&metadata_path, metadata_bytes).expect("Couldn't write metadata output");

    // Write out our expression to generate the runtime API to a file. Ideally, we'd just write this code
    // in lib.rs, but we must pass a string literal (and not `concat!(..)`) as an arg to runtime_metadata_path,
    // and so we need to split it out here and include it verbatim instead.
    let runtime_api_contents = format!(
        "
        #[subxt::subxt(
            runtime_metadata_path = \"{}\",
            generated_type_derives = \"Debug, Eq, PartialEq\"
        )]
        pub mod node_runtime {{
            #[subxt(substitute_type = \"sp_arithmetic::per_things::Perbill\")]
            use sp_runtime::Perbill;
        }}
    ",
        metadata_path
            .to_str()
            .expect("Path to metadata should be stringifiable")
    );

    let runtime_path = Path::new(&out_dir).join("runtime.rs");
    fs::write(&runtime_path, runtime_api_contents)
        .expect("Couldn't write runtime rust output");

    // Re-build if we point to a different substrate binary:
    println!("cargo:rerun-if-env-changed={}", SUBSTRATE_BIN_ENV_VAR);
    // Re-build if this file changes:
    println!("cargo:rerun-if-changed=build.rs");
}

/// Returns the next open port, or None if no port found in range.
fn next_open_port() -> Option<u16> {
    /// The start of the port range to scan.
    const START_PORT: u16 = 9900;
    /// The end of the port range to scan.
    const END_PORT: u16 = 10000;
    /// The maximum number of ports to scan before giving up.
    const MAX_PORTS: u16 = 1000;

    let next_port: AtomicU16 = AtomicU16::new(START_PORT);
    let mut ports_scanned = 0u16;
    loop {
        // Loop back from the beginning if needed
        let _ = next_port.compare_exchange(
            END_PORT,
            START_PORT,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
        let next = next_port.fetch_add(1, Ordering::SeqCst);
        if TcpListener::bind(("0.0.0.0", next)).is_ok() {
            return Some(next)
        }
        ports_scanned += 1;
        if ports_scanned == MAX_PORTS {
            return None
        }
    }
}
