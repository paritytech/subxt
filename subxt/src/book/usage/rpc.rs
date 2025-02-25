// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # RPC calls
//!
//! The RPC interface is provided by the [`subxt_rpcs`] crate but re-exposed here. We have:
//!
//! - [`crate::backend::rpc::RpcClient`] and [`crate::backend::rpc::RpcClientT`]: the underlying type and trait
//!   which provides a basic RPC client.
//! - [`crate::backend::legacy::rpc_methods`] and [`crate::backend::chain_head::rpc_methods`]: RPc methods that
//!   can be instantiated with an RPC client.
//!
//! See [`subxt_rpcs`] or [`crate::ext::subxt_rpcs`] for more.
//!
//! # Example
//!
//! Here's an example which calls some legacy JSON-RPC methods, and reuses the same connection to run a full Subxt client
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/rpc_legacy.rs")]
//! ```
