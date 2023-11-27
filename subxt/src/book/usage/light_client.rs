// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Light Client
//!
//! The light client based interface uses _Smoldot_ to connect to a _chain_, rather than an individual
//! node. This means that you don't have to trust a specific node when interacting with some chain.
//!
//! This feature is currently unstable. Use the `unstable-light-client` feature flag to enable it.
//! To use this in WASM environments, also enable the `web` feature flag.
//!
//! To connect to a blockchain network, the Light Client requires a trusted sync state of the network,
//! known as a _chain spec_. One way to obtain this is by making a `sync_state_genSyncSpec` RPC call to a
//! trusted node belonging to the chain that you wish to interact with.
//!
//! The following is an example of fetching the chain spec from a local running node on port 9933:
//!
//! ```bash
//! curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "sync_state_genSyncSpec", "params":[true]}' http://localhost:9933/ | jq .result > chain_spec.json
//! ```
//!
//! Alternately, you can have the `LightClient` download the chain spec from a trusted node when it
//! initializes, which is not recommended in production but is useful for examples and testing, as below.
//!
//! ## Examples
//!
//! ### Basic Example
//!
//! This example connects to a local chain and submits a transaction. To run this, you first need
//! to have a local polkadot node running using the following command:
//!
//! ```text
//! polkadot --dev --node-key 0000000000000000000000000000000000000000000000000000000000000001
//! ```
//!
//! Leave that running for a minute, and then you can run the example using the following command
//! in the `subxt` crate:
//!
//! ```bash
//! cargo run --example light_client_tx_basic --features=unstable-light-client
//! ```
//!
//! This is the code that will be executed:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/light_client_tx_basic.rs")]
//! ```
//!
//! ### Connecting to a parachain
//!
//! This example connects to a parachain using the light client. Currently, it's quite verbose to do this.
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/light_client_parachains.rs")]
//! ```
