//! # Connecting to a parachain with Subxt
//!
//! In this example we connect to a parachain and subscribe to its blocks.
//! There are many parachains on Polkadot and Kusama. Connecting to a parachain (or any other substrate based blockchain) with subxt requires 2 steps:
//!
//! 1. Fetching the chains metadata to generate a static interface via the subxt codegen.
//! 2. Creating a config struct that implements `subxt::Config` to give some type information
//! that is currently not covered by the metadata.
//!
//! We now show these steps in detail. As an example we use the
//! ["Ajuna"](https://parachains.info/details/ajuna_network/) parachain that is currently (2023-06-26)
//! deployed on Polkadot and [Kusama (as "Bajun")](https://parachains.info/details/bajun_network).
//!
//!
//! ## 1. Fetch the Metadata
//!
//! To fetch the metadata for the Ajuna network, we need to have the URL of an RPC node.
//! We can find "Ajuna Network" by looking through the sidebar on [Polkadot.js](https://polkadot.js.org/apps/).
//! We connect to the node, which leads us to [this page](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc-parachain.ajuna.network#/explorer).
//! In the url we can already see the url of the RPC endpoint of the node. It is also printed to the Javascript console of the browser:
//! `WS endpoint= wss://rpc-parachain.ajuna.network`.
//!
//! ```
//! subxt metadata --url ajuna_metadata.scale
//! ```
//!
//!
//!
//! In order to connect to a
//! chain we first need to fetch its metadata. This requires us to know a valid
//! RPC endpoint of the chain.
//! The [Substrate Chains Runtime Api Doc Center](https://substrate.rs/index.html) lists parachains
//! on Polkadot and Kusama and gives us a list of RPC endpoints for each parachain. Alternatively
//! we can also look for the RPC endpoint of a chain via [Polkadot.js](https://polkadot.js.org/apps/).
//! In the side menu, you can select a Polkadot/Kusama parachain. The hit the "Switch" button and
//! let the page load the chains data. The RPC endpoint url can be found as part of the url.
//! You can also open the console, to see the RPC endpoint url printed without URL-encoded special characters.
//!
//! Let's go through an example for the "Ajuna" parachain on Polkadot.
//! Selecting it in the [Polkadot.js](https://polkadot.js.org/apps/) menu, then opening the console
//! lets us observe the following output: `WS endpoint= wss://rpc-parachain.ajuna.network`
//!
//! We can now get the metadata of the chain via the subxt-cli tool.
//! It is important to note that we need to add a `:433` as the port number here:
//! ```txt
//! metadata --url wss://rpc-parachain.ajuna.network:443 > artifacts/ajuna_metadata.scale
//! ```
//!
//! This allows us to construct a module with the ajuna types, calls,
//!
//!

use subxt::{Config, OnlineClient, PolkadotConfig, SubstrateConfig};

/// Default set of commonly used types by Polkadot nodes.
pub enum AjunaConfig {}

impl Config for AjunaConfig {
    type Index = <SubstrateConfig as Config>::Index;
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = <SubstrateConfig as Config>::ExtrinsicParams;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<AjunaConfig>::new().await?;

    Ok(())
}
