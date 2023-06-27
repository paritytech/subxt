// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Low level light client implementation for RPC method and
//! subscriptions requests.
//!
//! The client implementation supports both native and wasm
//! environments.
//!
//! This leverages the smoldot crate to connect to the chain.

#![deny(
    missing_docs,
    unused_crate_dependencies,
    unused_extern_crates,
    clippy::all
)]

#[cfg(any(
    all(feature = "web", feature = "native"),
    not(any(feature = "web", feature = "native"))
))]
compile_error!("subxt: exactly one of the 'web' and 'native' features should be used.");

mod background;
mod platform;
mod client;

// Used to enable the js feature for wasm.
#[cfg(feature = "web")]
#[allow(unused_imports)]
pub use getrandom as _;

pub use client::LightClientRpc;

/// Light client error.
#[derive(Debug, thiserror::Error)]
pub enum LightClientRpcError {
    /// Error encountered while adding the chain to the light-client.
    #[error("Failed to add the chain to the light client: {0}.")]
    AddChainError(String),
    /// Error originated while trying to submit a RPC request.
    #[error("RPC request cannot be sent: {0}.")]
    Request(String),
}
