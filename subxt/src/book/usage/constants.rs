// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Constants
//!
//! There are various constants stored in a node; the types and values of these are defined in a
//! runtime, and can only change when the runtime is updated. Much like [`super::storage`], we can
//! query these using Subxt by taking the following steps:
//!
//! 1. [Constructing a constant query](#constructing-a-query).
//! 2. [Submitting the query to get back the associated value](#submitting-it).
//!
//! ## Constructing a constant query
//!
//! We can use the statically generated interface to build constant queries:
//!
//! ```rust,no_run,standalone_crate
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale")]
//! pub mod polkadot {}
//!
//! let constant_query = polkadot::constants().system().block_length();
//! ```
//!
//! Alternately, we can dynamically construct a constant query. A dynamic query needs the return
//! type to be specified, where we can use [`crate::dynamic::Value`] if unsure:
//!
//! ```rust,no_run,standalone_crate
//! use subxt::dynamic::Value;
//!
//! let storage_query = subxt::dynamic::constant::<Value>("System", "BlockLength");
//! ```
//!
//! ## Submitting it
//!
//! Call [`crate::constants::ConstantsClient::at()`] to return and decode the constant into the
//! type given by the address, or [`crate::constants::ConstantsClient::bytes_at()`] to return the
//! raw bytes for some constant.
//!
//! Constant values are pulled directly out of the node metadata which Subxt has
//! already acquired, and so this function requires no network access and is available from a
//! [`crate::OfflineClient`].
//!
//! Here's an example using a static query:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/constants_static.rs")]
//! ```
//!
//! And here's one using a dynamic query:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/constants_dynamic.rs")]
//! ```
//!
