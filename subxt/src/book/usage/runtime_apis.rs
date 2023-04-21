// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Runtime API interface
//!
//! The Runtime API interface allows Subxt to call runtime APIs exposed by certain pallets in order
//! to obtain information.
//!
//! At the moment, this interface is simply a wrapper around the `state_call` RPC method. This means
//! that you need to know which runtime calls are available and how to encode their parameters (if
//! needed). Eventually, Subxt will be able to generate an interface to the Runtime APIs exposed
//! here to make this as easy to do as constructing extrinsics or storage queries.
//!
//! ## Example
//!
//! Downloading node metadata via the Metadata runtime API interface:
//!
//!
//! ```rust,ignore
#![doc = include_str!("../../../../examples/examples/runtime_apis_raw.rs")]
//! ```
//!
