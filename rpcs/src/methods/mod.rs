// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! RPC methods are defined in this module. At the moment we have:
//!
//! - [`ChainHeadRpcMethods`] (and the types in [`chain_head`]): these methods
//!   implement the RPC spec at <https://paritytech.github.io/json-rpc-interface-spec/api/chainHead.html>
//!
//! We also have (although their use is not advised):
//!
//! - [`LegacyRpcMethods`] (and the types in [`legacy`]): a collection of legacy RPCs.
//!   These are not well specified and may change in implementations without warning,
//!   but for those methods we expose, we make a best effort to work against latest Substrate versions.

pub mod chain_head;
pub mod legacy;

pub use chain_head::ChainHeadRpcMethods;
pub use legacy::LegacyRpcMethods;
