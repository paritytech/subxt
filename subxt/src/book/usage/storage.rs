// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Storage
//!
//! A Substrate based chain can be seen as a key/value database which starts off at some initial
//! state, and is modified by the extrinsics in each block. This database is referred to as the
//! node storage. With Subxt, you can query this key/value storage with the following steps:
//!
//! 1. [Constructing a storage query](#constructing-a-storage-query).
//! 2. [Submitting the query to get back the associated entry](#submitting-it).
//! 3. [Fetching](#fetching-storage-entries) or [iterating](#iterating-storage-entries) over that
//!    entry to retrieve the value or values within it.
//!
//! ## Constructing a storage query
//!
//! We can use the statically generated interface to build storage queries:
//!
//! ```rust,no_run,standalone_crate
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
//! pub mod polkadot {}
//!
//! let storage_query = polkadot::storage().system().account();
//! ```
//!
//! Alternately, we can dynamically construct a storage query. A dynamic query needs the input
//! and return value types to be specified, where we can use [`crate::dynamic::Value`] if unsure.
//!
//! ```rust,no_run,standalone_crate
//! use subxt::dynamic::Value;
//!
//! let storage_query = subxt::dynamic::storage::<(Value,), Value>("System", "Account");
//! ```
//!
//! ## Submitting it
//!
//! Storage queries can be handed to various functions in [`crate::storage::StorageClientAt`] in order to
//! obtain the associated values (also referred to as storage entries) back.
//!
//! The core API here is [`crate::storage::StorageClientAt::entry()`], which takes a query and looks up the
//! corresponding storage entry, from which you can then fetch or iterate over the values contained within.
//! [`crate::storage::StorageClientAt::fetch()`] and [`crate::storage::StorageClientAt::iter()`] are shorthand
//! for this.
//!
//! When you wish to manually query some entry, [`crate::storage::StorageClientAt::fetch_raw()`] exists to take
//! in raw bytes pointing at some storage value, and return the value bytes if possible. [`crate::storage::StorageClientAt::storage_version()`]
//! and [`crate::storage::StorageClientAt::runtime_wasm_code()`] use this to retrieve the version of some storage API
//! and the current Runtime WASM blob respectively.
//!
//! ### Fetching storage entries
//!
//! The simplest way to access storage entries is to construct a query and then call either
//! [`crate::storage::StorageClientAt::fetch()`]:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/storage_fetch.rs")]
//! ```
//!
//! For completeness, below is an example using a dynamic query instead. Dynamic queries can define the types that
//! they wish to accept inputs and decode the return value into ([`crate::dynamic::Value`] can be used here anywhere we
//! are not sure of the specific types).
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/storage_fetch_dynamic.rs")]
//! ```
//!
//! ### Iterating storage entries
//!
//! Many storage entries are maps of values; as well as fetching individual values, it's possible to
//! iterate over all of the values stored at that location:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/storage_iterating.rs")]
//! ```
//!
//! Here's the same logic but using dynamically constructed values instead:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/storage_iterating_dynamic.rs")]
//! ```
//!
