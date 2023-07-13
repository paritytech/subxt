// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Light Client
//!
//! The Light Client aims to contribute to the decentralization of blockchains by providing connectivity
//! to the P2P network and behaving similarly to a full node.
//!
//! To enable this functionality, the unstable-light-client feature flag needs to be enabled.
//!
//! To connect to a blockchain network, the Light Client requires a trusted sync state of the network, named "chain spec".
//! This can be obtained by making a `sync_state_genSyncSpec` RPC call to a trusted node.
//!
//! The following is an example of fetching the chain spec from a local running onde on port 9933.
//!
//! ```bash
//! curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "sync_state_genSyncSpec", "params":[true]}' http://localhost:9933/ | jq .result > chain_spec.json
//! ```
//!
//! ## Example
//!
//! You can construct a Light Client from a trusted chain spec stored on disk.
//! Similary, the Light Client can fetch the chain spec from a running node and
//! overwrite the bootNodes section. The `jsonrpsee` feature flag exposes the
//! `build_from_url` method.
//!
//! ```rust,ignore
//! let light_client = LightClientBuilder::new()
//!     .bootnodes(
//!         ["/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"]
//!     )
//!     .build_from_url("ws://127.0.0.1:9944")
//!     .await?;
//! ```
//!
//! Here's an example which connects to a local chain and submits a transaction.
//!
//! You can run the example using the following command:
//!
//! ```bash
//! cargo run --example unstable_light_client_tx_basic --features="unstable-light-client jsonrpsee"
//! ```
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/unstable_light_client_tx_basic.rs")]
//! ```
//!
