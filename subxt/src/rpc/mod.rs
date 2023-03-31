// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! RPC types and client for interacting with a substrate node.
//!
//! These is used behind the scenes by various `subxt` APIs, but can
//! also be used directly.
//!
//! - [`Rpc`] is the highest level wrapper, and the one you will run into
//!   first. It contains the higher level methods for interacting with a node.
//! - [`RpcClient`] is what [`Rpc`] uses to actually talk to a node, offering
//!   a [`RpcClient::request`] and [`RpcClient::subscribe`] method to do so.
//! - [`RpcClientT`] is the underlying dynamic RPC implementation. This provides
//!   the low level [`RpcClientT::request_raw`] and [`RpcClientT::subscribe_raw`]
//!   methods. This can be swapped out for a custom implementation, but by default
//!   we'll rely on `jsonrpsee` for this.
//!
//! # Example
//!
//! Fetching storage keys
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() {
//! use subxt::{ PolkadotConfig, OnlineClient, storage::StorageKey };
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
//! pub mod polkadot {}
//!
//! let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
//!
//! let genesis_hash = api
//!     .rpc()
//!     .genesis_hash()
//!     .await
//!     .unwrap();
//!
//! println!("{genesis_hash}");
//! # }
//! ```

// Allow an `rpc.rs` file in the `rpc` folder to align better
// with other file names for their types.
#![allow(clippy::module_inception)]

#[cfg(feature = "jsonrpsee")]
mod jsonrpsee_impl;

mod rpc;
mod rpc_client;
mod rpc_client_t;

// Expose our RPC types here.
pub mod types;

// Expose the `Rpc` struct.
pub use rpc::*;

pub use rpc_client_t::{
    RawValue, RpcClientT, RpcFuture, RpcSubscription, RpcSubscriptionId, RpcSubscriptionStream,
};

pub use rpc_client::{rpc_params, RpcClient, RpcParams, Subscription};
