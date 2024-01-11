# This script is to be run from the root of the repository: `scripts/artifacts.sh`
#
# It expects a local polkadot node to be running a JSON-RPC HTTP server at 127.0.0.1:9944
# A local polkadot node can be run via:
# ```
# git clone https://github.com/paritytech/polkadot.git
# cd polkadot 
# cargo build --release
# ./target/release/polkadot --dev --tmp
# ```
#
# Instead of this script you can also use the binary crate "scripts/artifacts",
# which you can run with `cargo run --bin artifacts` from the root of the repository. 
# It will spawn up a substrate node and generate the artifacts (This is to be preferred for CI jobs)
#

# get the full metadata
cargo run --bin subxt metadata --version 15 > artifacts/polkadot_metadata_full.scale
# use it to generate polkadot.rs
cargo run --bin subxt codegen --file artifacts/polkadot_metadata_full.scale | rustfmt > testing/integration-tests/src/full_client/codegen/polkadot.rs
# generate a metadata file that only contains a few pallets that we need for our examples.
cargo run --bin subxt metadata --file artifacts/polkadot_metadata_full.scale --pallets "Balances,Staking,System,Multisig,Timestamp,ParaInherent" > artifacts/polkadot_metadata_small.scale
# generate a metadata file that contains no pallets
cargo run --bin subxt metadata --file artifacts/polkadot_metadata_full.scale --pallets "" > artifacts/polkadot_metadata_tiny.scale
# generate a metadata file that only contains some custom metadata
cargo run --bin generate-custom-metadata > artifacts/metadata_with_custom_values.scale 

# Generate the polkadot chain spec.
cargo run --features chain-spec-pruning --bin subxt chain-spec --url wss://rpc.polkadot.io:443 --output-file artifacts/demo_chain_specs/polkadot.json --state-root-hash --remove-substitutes
