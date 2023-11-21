// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # RPC calls
//!
//! Subxt exposes low level interfaces that can be used to make RPC requests; [`crate::backend::legacy::rpc_methods`]
//! and [`crate::backend::unstable::rpc_methods`].
//!
//! These interfaces cannot be accessed directly through an [`crate::OnlineClient`]; this is so that the high level
//! Subxt APIs can target either the "legacy" or the more modern "unstable" sets of RPC methods by selecting an appropriate
//! [`crate::backend::Backend`]. It also means that there could exist a backend in the future that doesn't use JSON-RPC at all.
//!
//! # Example
//!
//! Here's an example which calls some legacy JSON-RPC methods, and reuses the same connection to run a full Subxt client
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/rpc_legacy.rs")]
//! ```