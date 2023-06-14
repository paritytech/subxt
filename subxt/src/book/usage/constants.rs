// Copyright 2019-2023 Parity Technologies (UK) Ltd.
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
//! ```rust,no_run
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale")]
//! pub mod polkadot {}
//!
//! let constant_query = polkadot::constants().system().block_length();
//! ```
//!
//! Alternately, we can dynamically construct a constant query:
//!
//! ```rust,no_run
//! use subxt::dynamic::Value;
//!
//! let storage_query = subxt::dynamic::constant("System", "BlockLength");
//! ```
//!
//! Static queries also have a static return type, so the constant is decoded appropriately. In
//! addition, they are validated at runtime to ensure that they align with the current node state.
//! Dynamic queries must be decoded into some static type manually, or into the dynamic
//! [`crate::dynamic::Value`] type.
//!
//! ## Submitting it
//!
//! Constant queries are handed to Subxt via [`crate::constants::ConstantsClient::at()`]. It's worth
//! noting that constant values are pulled directly out of the node metadata which Subxt has
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
