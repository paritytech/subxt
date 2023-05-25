// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Light Clien
//!
//! The light client vision is to help the decentralization of blockchains.
//! They connect to the p2p network, behaving similarly to a full node without needing excessive synchronization.
//!
//! The `unstable-light-client` feature flag enables this functionality.
//!
//! To connect to a blockchain network, the light client requires a trusted sync state of the network.
//!
//! ```bash
//! {
//!    "bootNodes": [
//!      "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
//!    ],
//!    "genesis": { ... },
//! }
//! ```
//!
//! ## Example
//!
//! Here's an example which connects to a local chain and submits a transaction:
//!
//! ```rust,ignore
#![doc = include_str!("../../../../examples/examples/unstable_light_client_tx_basic.rs")]
//! ```
//!
//! ## Example live chain
//!
//! The following example connects to the polkadot live chain and performs
//! several queries using the light client.
//!
//! ```rust,ignore
#![doc = include_str!("../../../../examples/examples/unstable_light_client.rs")]
//! ```
//!
