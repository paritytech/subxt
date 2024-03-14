// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Light Client
//!
//! The light client based interface uses _Smoldot_ to connect to a _chain_, rather than an individual
//! node. This means that you don't have to trust a specific node when interacting with some chain.
//!
//! This feature is currently unstable. Use the `unstable-light-client` feature flag to enable it.
//! To use this in WASM environments, enable the `web` feature flag and disable the "native" one.
//!
//! To connect to a blockchain network, the Light Client requires a trusted sync state of the network,
//! known as a _chain spec_. One way to obtain this is by making a `sync_state_genSyncSpec` RPC call to a
//! trusted node belonging to the chain that you wish to interact with.
//!
//! Subxt exposes a utility method to obtain the chain spec: [`subxt::utils::fetch_chainspec_from_rpc_node()`].
//! Alternately, you can manually make an RPC call to `sync_state_genSyncSpec` like do (assuming a node running
//! locally on port 9933):
//!
//! ```bash
//! curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "sync_state_genSyncSpec", "params":[true]}' http://localhost:9933/ | jq .result > chain_spec.json
//! ```
//!
//! ## Examples
//!
//! ### Basic Example
//!
//! This basic example uses some already-known chain specs to connect to a relay chain and parachain
//! and stream information about their finalized blocks:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/light_client_basic.rs")]
//! ```
//!
//! ### Connecting to a local node
//!
//! This example connects to a local chain and submits a transaction. To run this, you first need
//! to have a local polkadot node running using the following command:
//!
//! ```text
//! polkadot --dev --node-key 0000000000000000000000000000000000000000000000000000000000000001
//! ```
//!
//! Then, the following code will download a chain spec from this local node, alter the bootnodes
//! to point only to the local node, and then submit a transaction through it.
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/light_client_local_node.rs")]
//! ```
//!
