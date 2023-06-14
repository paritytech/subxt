// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Storage
//!
//! A Substrate based chain can be seen as a key/value database which starts off at some initial
//! state, and is modified by the extrinsics in each block. This database is referred to as the
//! node storage. With Subxt, you can query this key/value storage with the following steps:
//!
//! 1. [Constructing a storage query](#constructing-a-storage-query).
//! 2. [Submitting the query to get back the associated values](#submitting-it).
//!
//! ## Constructing a storage query
//!
//! We can use the statically generated interface to build storage queries:
//!
//! ```rust,no_run
//! use subxt_signer::sr25519::dev;
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
//! pub mod polkadot {}
//!
//! let account = dev::alice().public_key().into();
//! let storage_query = polkadot::storage().system().account(&account);
//! ```
//!
//! Alternately, we can dynamically construct a storage query. This will not be type checked or
//! validated until it's submitted:
//!
//! ```rust,no_run
//! use subxt_signer::sr25519::dev;
//! use subxt::dynamic::Value;
//!
//! let account = dev::alice().public_key();
//! let storage_query = subxt::dynamic::storage("System", "Account", vec![
//!     Value::from_bytes(account)
//! ]);
//! ```
//!
//! As well as accessing specific entries, some storage locations can also be iterated over (such as
//! the map of account information). To do this, suffix `_root` onto the query constructor (this
//! will only be available on static constructors when iteration is actually possible):
//!
//! ```rust,no_run
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
//! pub mod polkadot {}
//!
//! // A static query capable of iterating over accounts:
//! let storage_query = polkadot::storage().system().account_root();
//! // A dynamic query to do the same:
//! let storage_query = subxt::dynamic::storage_root("System", "Account");
//! ```
//!
//! All valid storage queries implement [`crate::storage::StorageAddress`]. As well as describing
//! how to build a valid storage query, this trait also has some associated types that determine the
//! shape of the result you'll get back, and determine what you can do with it (ie, can you iterate
//! over storage entries using it).
//!
//! Static queries set appropriate values for these associated types, and can therefore only be used
//! where it makes sense. Dynamic queries don't know any better and can be used in more places, but
//! may fail at runtime instead if they are invalid in those places.
//!
//! ## Submitting it
//!
//! Storage queries can be handed to various functions in [`crate::storage::Storage`] in order to
//! obtain the associated values (also referred to as storage entries) back.
//!
//! ### Fetching storage entries
//!
//! The simplest way to access storage entries is to construct a query and then call either
//! [`crate::storage::Storage::fetch()`] or [`crate::storage::Storage::fetch_or_default()`] (the
//! latter will only work for storage queries that have a default value when empty):
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/storage_fetch.rs")]
//! ```
//!
//! For completeness, below is an example using a dynamic query instead. The return type from a
//! dynamic query is a [`crate::dynamic::DecodedValueThunk`], which can be decoded into a
//! [`crate::dynamic::Value`], or else the raw bytes can be accessed instead.
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
//! ### Advanced
//!
//! For more advanced use cases, have a look at [`crate::storage::Storage::fetch_raw`] and
//! [`crate::storage::Storage::fetch_keys`]. Both of these take raw bytes as arguments, which can be
//! obtained from a [`crate::storage::StorageAddress`] by using
//! [`crate::storage::StorageClient::address_bytes()`] or
//! [`crate::storage::StorageClient::address_root_bytes()`].
//!
