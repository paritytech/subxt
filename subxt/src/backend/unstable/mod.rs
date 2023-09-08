// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module will expose a backend implementation based on the new APIs
//! described at <https://github.com/paritytech/json-rpc-interface-spec/>. See
//! [`rpc_methods`] for the raw API calls.
//!
//! # Warning
//!
//! Everything in this module is **unstable**, meaning that it could change without
//! warning at any time.

pub mod rpc_methods;

pub use rpc_methods::UnstableRpcMethods;
