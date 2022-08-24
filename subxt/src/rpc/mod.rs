// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! RPC types and client for interacting with a substrate node.
//!
//! This is used behind the scenes by various `subxt` APIs, but can
//! also be used directly.
//!
//! # Example
//!
//! Fetching storage keys
//!
//! ```no_run
//! use subxt::{ PolkadotConfig, OnlineClient, storage::StorageKey };
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
//! pub mod polkadot {}
//!
//! # #[tokio::main]
//! # async fn main() {
//! let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
//!
//! let key = polkadot::storage()
//!     .xcm_pallet()
//!     .version_notifiers_root()
//!     .to_bytes();
//!
//! // Fetch up to 10 keys.
//! let keys = api
//!     .rpc()
//!     .storage_keys_paged(&key, 10, None, None)
//!     .await
//!     .unwrap();
//!
//! for key in keys.iter() {
//!     println!("Key: 0x{}", hex::encode(&key));
//! }
//! # }
//! ```

#[cfg(feature = "jsonrpsee")]
mod jsonrpsee_impl;

mod methods;
mod rpc_client;
mod rpc_client_t;

// Expose the `Rpc` struct and any associated types.
pub use methods::*;

// The underlying client used to talk to a node.
pub use rpc_client_t::{
    RpcClientT,
    RpcResponse,
    RpcSubscription,
    RpcSubscriptionStream,
};

// A wrapper around the above; this is what is generally exposed.
pub use rpc_client::{
    RpcClient,
    Subscription,
};