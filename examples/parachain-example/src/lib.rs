//! # Tutorial: Subxt setup to interact with custom chains
//!
//! There are many parachains on Polkadot and Kusama. Using Subxt to connect to a parachain
//! (or any other custom, substrate based chain) requires 2 steps:
//!
//! 1. Fetching the chains metadata to generate a static interface via the subxt codegen.
//! 2. Creating a config struct that implements `subxt::Config` to give some type information
//! that is currently not covered by the metadata.
//!
//! This example shows you how to connect to the ["Statemint"](https://parachains.info/details/statemint) parachain,
//! also known as "Asset Hub", both locally and remotely.
//!
//! ### Example 1: Subscribe to blocks of the official ["Polkadot Asset Hub"](https://parachains.info/details/statemint)
//!
//! ```
//! cargo run --example fetch_blocks.rs
//! ```
//!
//! ### Example 2: Create an NFT on a local parachain
//!
//! This requires you to spin up a local asset hub parachain using Zombienet. Please see the instructions in the example.
//! ```
//! cargo run --example create_nft.rs
//! ```
//!
//! # How to get the parachains metadata
//!
//! To fetch the metadata for the Statemint parachain, we need to have the URL of an RPC node.
//! We can find the "Asset Hub" entry, by looking through the sidebar on [Polkadot.js](https://polkadot.js.org/apps/).
//! We connect to the node ("via Parity"), which leads us to [this page](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpolkadot-asset-hub-rpc.polkadot.io#/explorer).
//! In the URL of the page we can already see the URL of the RPC endpoint of the node as a query parameter. It is also printed
//! to the Javascript console of the browser: `WS endpoint= wss://polkadot-asset-hub-rpc.polkadot.io`.
//!
//! We can now get the metadata via the [subxt cli](https://crates.io/crates/subxt-cli) tool.
//! It is important to specify the port as `:443` like so:
//! ```txt
//! subxt metadata  --url wss://polkadot-asset-hub-rpc.polkadot.io:443 > statemint_metadata.scale
//! ```
//! The metadata is saved as `statemint_metadata.scale` and can be used to create the statically generated interface for the parachain:
//! ```
//! #[subxt::subxt(runtime_metadata_path = "statemint_metadata.scale")]
//! pub mod statemint {}
//! ```
//!

pub mod statemint_config_composed;
pub mod statemint_config_verbose;
pub mod statemint_config_with_subxt_types;

#[subxt::subxt(runtime_metadata_path = "statemint_metadata.scale")]
pub mod statemint {}
