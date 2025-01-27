// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! RPC types and client for interacting with a substrate node.
//!
//! These are used behind the scenes by Subxt backend implementations, for
//! example [`crate::backend::legacy::LegacyBackend`]. If you need an RPC client,
//! then you can manually instantiate one, and then hand it to Subxt if you'd like
//! to re-use it for the Subxt connection.
//!
//! - [`RpcClientT`] is the underlying dynamic RPC implementation. This provides
//!   the low level [`RpcClientT::request_raw`] and [`RpcClientT::subscribe_raw`]
//!   methods.
//! - [`RpcClient`] is the higher level wrapper around this, offering
//!   the [`RpcClient::request`] and [`RpcClient::subscribe`] methods.
//!
//! # Example
//!
//! Fetching the genesis hash.
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() {
//! use subxt::{
//!     client::OnlineClient,
//!     config::SubstrateConfig,
//!     backend::rpc::RpcClient,
//!     backend::legacy::LegacyRpcMethods,
//! };
//!
//! // Instantiate a default RPC client pointing at some URL.
//! let rpc_client = RpcClient::from_url("ws://localhost:9944")
//!     .await
//!     .unwrap();
//!
//! // Instantiate the legacy RPC interface, providing an appropriate
//! // config so that it uses the correct types for your chain.
//! let rpc_methods = LegacyRpcMethods::<SubstrateConfig>::new(rpc_client.clone());
//!
//! // Use it to make RPC calls, here using the legacy genesis_hash method.
//! let genesis_hash = rpc_methods
//!     .genesis_hash()
//!     .await
//!     .unwrap();
//!
//! println!("{genesis_hash}");
//!
//! // Instantiate the Subxt interface using the same client and config if you
//! // want to reuse the same connection:
//! let client = OnlineClient::<SubstrateConfig>::from_rpc_client(rpc_client);
//! # }
//! ```

// Allow an `rpc.rs` file in the `rpc` folder to align better
// with other file names for their types.
#![allow(clippy::module_inception)]

crate::macros::cfg_jsonrpsee! {
    mod jsonrpsee_impl;
    pub use jsonrpsee::core::client::Client as JsonrpseeRpcClient; 
}

crate::macros::cfg_unstable_light_client! {
    mod lightclient_impl;
    pub use lightclient_impl::LightClientRpc as LightClientRpcClient; 
}

crate::macros::cfg_reconnecting_rpc_client! {
   pub mod reconnecting_rpc_client;
   pub use reconnecting_rpc_client::RpcClient as ReconnectingRpcClient;
}

mod rpc_client;
mod rpc_client_t;

pub use rpc_client::{rpc_params, RpcClient, RpcParams, RpcSubscription};
pub use rpc_client_t::{RawRpcFuture, RawRpcSubscription, RawValue, RpcClientT};
