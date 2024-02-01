use std::{
    fs::File,
    process::{Command, Stdio},
};

use substrate_runner::SubstrateNode;

/// A Script to generate artifacts that are used in the integration tests.
///
/// Run with `cargo run --bin artifacts` from the root of the repository.
fn main() {
    let mut node_builder = SubstrateNode::builder();
    node_builder.polkadot();

    // Spawn the node and retrieve a ws URL to it:
    let proc = node_builder
        .spawn()
        .map_err(|e| e.to_string())
        .expect("Could not spawn node");
    let node_url = format!("ws://127.0.0.1:{}", proc.ws_port());

    // Get the full metadata from the spawned substrate node
    run_cmd(
        &format!("cargo run --bin subxt metadata --version 15 --url {node_url}"),
        Some("artifacts/polkadot_metadata_full.scale"),
    );

    // Use it to generate polkadot.rs
    run_cmd(
        "cargo run --bin subxt codegen --file artifacts/polkadot_metadata_full.scale",
        Some("testing/integration-tests/src/full_client/codegen/polkadot.rs"),
    );
    run_cmd(
        "rustfmt testing/integration-tests/src/full_client/codegen/polkadot.rs",
        None,
    );

    // Generate a metadata file that only contains a few pallets that we need for our examples.
    run_cmd(
        "cargo run --bin subxt metadata --file artifacts/polkadot_metadata_full.scale --pallets Balances,Staking,System,Multisig,Timestamp,ParaInherent",
        Some("artifacts/polkadot_metadata_small.scale"),
    );

    // Generate a metadata file that contains no pallets
    run_cmd(
        "cargo run --bin subxt metadata --file artifacts/polkadot_metadata_full.scale --pallets \"\"",
        Some("artifacts/polkadot_metadata_tiny.scale"),
    );

    // Generate a metadata file that only contains some custom metadata
    run_cmd(
        "cargo run --bin generate-custom-metadata",
        Some("artifacts/metadata_with_custom_values.scale"),
    );

    // Generate the polkadot chain spec.
    run_cmd("cargo run --features chain-spec-pruning --bin subxt chain-spec --url wss://rpc.polkadot.io:443 --output-file artifacts/demo_chain_specs/polkadot.json --state-root-hash --remove-substitutes", None);
}

fn run_cmd(cmd: &str, out_path: Option<&str>) {
    println!("Running Command: {cmd}");
    // Note: simple space splitting, no fancy parsing of e.g. quotes surrounding whitespace.
    let mut parts = cmd.split(' ');
    let program = parts.next().expect("no program in command string");
    let mut command = Command::new(program);
    for e in parts {
        command.arg(e);
    }

    if let Some(out_path) = out_path {
        let file = File::create(out_path).unwrap();
        command.stdout(Stdio::from(file));
    }

    let status = command.spawn().unwrap().wait().unwrap();
    if !status.success() {
        panic!("Command `{cmd}` failed with status: {status}")
    }
}
