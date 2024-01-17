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
    node_builder.binary_paths(["substrate-node", "substrate"]);

    // Spawn the node and retrieve a ws URL to it:
    let proc = node_builder
        .spawn()
        .map_err(|e| e.to_string())
        .expect("Could not spawn node");
    let node_url = format!("ws://127.0.0.1:{}", proc.ws_port());

    // Get the full metadata from the spawned substrate node
    Command::make(&format!(
        "cargo run --bin subxt metadata --version 15 --url {node_url}"
    ))
    .out("artifacts/polkadot_metadata_full.scale");

    // Use it to generate polkadot.rs
    Command::make("cargo run --bin subxt codegen --file artifacts/polkadot_metadata_full.scale")
        .pipe("rustfmt")
        .out("testing/integration-tests/src/full_client/codegen/polkadot.rs");

    // Generate a metadata file that only contains a few pallets that we need for our examples.
    Command::make(r#"cargo run --bin subxt metadata --file artifacts/polkadot_metadata_full.scale --pallets "Balances,Staking,System,Multisig,Timestamp,ParaInherent""#)
    .out("artifacts/polkadot_metadata_small.scale");

    // Generate a metadata file that contains no pallets
    Command::make(r#"cargo run --bin subxt metadata --file artifacts/polkadot_metadata_full.scale --pallets """#)
    .out("artifacts/polkadot_metadata_tiny.scale");

    // Generate a metadata file that only contains some custom metadata
    Command::make("cargo run --bin generate-custom-metadata")
        .out("artifacts/metadata_with_custom_values.scale");

    // Generate the polkadot chain spec.
    Command::make("cargo run --features chain-spec-pruning --bin subxt chain-spec --url wss://rpc.polkadot.io:443 --output-file artifacts/demo_chain_specs/polkadot.json --state-root-hash --remove-substitutes").spawn().unwrap().wait().unwrap();
}

trait CommandT {
    /// Creates a new command, parsing the arg_string provided.
    fn make(arg_string: &str) -> Self;

    /// Pipes the output of the current command to the next command.
    fn pipe(self, arg_string: &str) -> Self;

    /// Writes bytes from stdout to a new file at path.
    fn out(self, path: &str);
}

impl CommandT for Command {
    fn make(arg_string: &str) -> Self {
        // Note: simple space splitting, no fancy parsing of e.g. quotes surrounding whitespace.
        let mut parts = arg_string.split(' ');
        let program = parts.next().expect("no program in command string");
        let mut command = Command::new(program);
        for e in parts {
            command.arg(e);
        }
        command
    }

    fn pipe(mut self, arg_string: &str) -> Self {
        // execute self
        let old_cmd = self.stdout(Stdio::piped()).spawn().unwrap();
        let mut next_cmd = Self::make(arg_string);
        next_cmd.stdin(Stdio::from(old_cmd.stdout.unwrap()));
        next_cmd
    }

    fn out(mut self, path: &str) {
        dbg!(path);
        let file = File::create(path).unwrap();
        self.stdout(Stdio::from(file))
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
