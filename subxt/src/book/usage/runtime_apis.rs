// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Runtime API interface
//!
//! The Runtime API interface allows Subxt to call runtime APIs exposed by certain pallets in order
//! to obtain information. Much like [`super::storage`] and [`super::transactions`], Making a runtime
//! call to a node and getting the response back takes the following steps:
//!
//! 1. [Constructing a runtime call](#constructing-a-runtime-call)
//! 2. [Submitting it to get back the response](#submitting-it)
//!
//! **Note:** Runtime APIs are only available when using V15 metadata, which is currently unstable.
//! You'll need to use `subxt metadata --version unstable` command to download the unstable V15 metadata,
//! and activate the `unstable-metadata` feature in Subxt for it to also use this metadata from a node. The
//! metadata format is unstable because it may change and break compatibility with Subxt at any moment, so
//! use at your own risk.
//!
//! ## Constructing a runtime call
//!
//! We can use the statically generated interface to build runtime calls:
//!
//! ```rust,no_run
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
//! pub mod polkadot {}
//!
//! let runtime_call = polkadot::apis().metadata().metadata_versions();
//! ```
//!
//! Alternately, we can dynamically construct a runtime call:
//!
//! ```rust,no_run
//! use subxt::dynamic::Value;
//!
//! let runtime_call = subxt::dynamic::runtime_api_call(
//!     "Metadata",
//!     "metadata_versions",
//!     Vec::<Value<()>>::new()
//! );
//! ```
//!
//! All valid runtime calls implement [`crate::runtime_api::RuntimeApiPayload`], a trait which
//! describes how to encode the runtime call arguments and what return type to decode from the
//! response.
//!
//! ## Submitting it
//!
//! Runtime calls can be handed to [`crate::runtime_api::RuntimeApi::call()`], which will submit
//! them and hand back the associated response.
//!
//! ### Making a static Runtime API call
//!
//! The easiest way to make a runtime API call is to use the statically generated interface.
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/runtime_apis_static.rs")]
//! ```
//!
//! ### Making a dynamic Runtime API call
//!
//! If you'd prefer to construct the call at runtime, you can do this using the
//! [`crate::dynamic::runtime_api_call`] method.
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/runtime_apis_dynamic.rs")]
//! ```
//!
//! ### Making a raw call
//!
//! This is generally discouraged in favour of one of the above, but may be necessary (especially if
//! the node you're talking to does not yet serve V15 metadata). Here, you must manually encode
//! the argument bytes and manually provide a type for the response bytes to be decoded into.
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/runtime_apis_raw.rs")]
//! ```
//!
