/// In this example we want to submit a transaction to the asset hub parachain.
/// Doing so on the official polkadot network, would require us to have an account with sufficient funds.
/// That is why we spawn a local asset hub parachain instead, that comes with pre-funded dev accounts.
///
/// # Setting up the local parachain
///
/// We use [zombienet](https://github.com/paritytech/zombienet) to start up a local asset hub.
///
/// ## 1. Install necessary tools
///
/// To setup the local parachain we need to have 3 binaries installed in our path:
///
/// ### 1. `zombienet`
///
/// Zombienet is a tool for quickly spinning up a (local) blockchain network. Please follow the install guide in the [zombienet github repo](https://github.com/paritytech/zombienet).
///
/// ### 2. `polkadot`
///
/// Build the polkadot binary from the [polkadot github repo](https://github.com/paritytech/polkadot) and install it in your path:
/// ```txt
/// git clone https://github.com/paritytech/polkadot.git
/// cd polkadot
/// cargo build --release
/// cargo install --path .
/// ```
///
/// ### 3. `polkadot-parachain`
///
/// The polkadot asset hub is part of the [cumulus github repo](https://github.com/paritytech/cumulus), an SDK for developing parachains.
/// Building the cumulus workspace produces a binary called `polkadot-parachain` that has the capability and configuration data to run the asset hub.
/// ```txt
/// git clone https://github.com/paritytech/cumulus.git
/// cd cumulus
/// cargo build --release
/// cargo install --path .
/// ```
///
/// ## 2. Run the parachain locally
///
/// Zombienet can now spawn the parachain locally from a configuration file, `asset-hub-zombienet.toml` in this case.
/// We need to have at least 2 validator nodes running (via the `polkadot` binary),
/// while the `polkadot-parachain` binary starts the asset hub parachain and gets it registered with the validator nodes.
/// To do that, run:
/// ```txt
/// zombienet -p native spawn asset-hub-zombienet.toml
/// ```
/// Zombienet uses Kubernetes by default, but we can use it without Kubernetes, by providing the `-p native` flag.
///
/// # Running the example
///
/// After you have the network running, you should see [https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:41955#/explorer]
/// as the polkadot.js link to the parachain (e.g. "collator01") in the zombienet output.
///
/// To run this example:
/// ```txt
/// cargo run --bin submit_transaction
/// ```
#[tokio::main]
pub async fn main() {
    todo!()
}
