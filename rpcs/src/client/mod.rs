// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! RPC types and client for interacting with a substrate node.
//!
//! An RPC client is instantiated and then used to create some methods, for instance
//! [`crate::methods::ChainHeadRpcMethods`], which defines the calls that can be made with it.
//! The core RPC client bits are:
//!
//! - [`RpcClientT`] is the underlying dynamic RPC implementation. This provides
//!   the low level [`RpcClientT::request_raw`] and [`RpcClientT::subscribe_raw`]
//!   methods.
//! - [`RpcClient`] is the higher level wrapper around this, offering
//!   the [`RpcClient::request`] and [`RpcClient::subscribe`] methods.
//!
//! We then expose implementations here (depending on which features are enabled)
//! which implement [`RpcClientT`] and can therefore be used to construct [`RpcClient`]s.
//!
//! - **jsonrpsee**: Enable an RPC client based on `jsonrpsee`.
//! - **unstable-light-client**: Enable an RPC client which uses the Smoldot light client under
//!   the hood to communicate with the network of choice.
//! - **reconnecting-rpc-client**: Enable an RPC client based on `jsonrpsee` which handles
//!   reconnecting automatically in the event of network issues.
//! - **mock-rpc-client**: Enable a mock RPC client that can be used in tests.
//!

crate::macros::cfg_jsonrpsee! {
    mod jsonrpsee_impl;
    pub use jsonrpsee::core::client::Client as JsonrpseeRpcClient;
}

crate::macros::cfg_unstable_light_client! {
    mod lightclient_impl;
    pub use subxt_lightclient::LightClientRpc as LightClientRpcClient;
}

crate::macros::cfg_reconnecting_rpc_client! {
   pub mod reconnecting_rpc_client;
   pub use reconnecting_rpc_client::RpcClient as ReconnectingRpcClient;
}

crate::macros::cfg_mock_rpc_client! {
    pub mod mock_rpc_client;
    pub use mock_rpc_client::MockRpcClient;
}

mod rpc_client;
mod rpc_client_t;

pub use rpc_client::{rpc_params, RpcClient, RpcParams, RpcSubscription};
pub use rpc_client_t::{RawRpcFuture, RawRpcSubscription, RawValue, RpcClientT};
