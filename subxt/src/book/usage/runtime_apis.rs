// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Runtime API interface
//!
//! The Runtime API interface allows Subxt to call runtime APIs exposed by certain pallets in order
//! to obtain information. Much like [`super::storage`] and [`super::extrinsics`], Making a runtime
//! call to a node and getting the response back takes the following steps:
//!
//! 1. [Constructing a runtime call](#constructing-a-runtime-call)
//! 2. [Submitting it to get back the response](#submitting-it)
//!
//! ## Constructing a runtime call
//!
//! We can use the statically generated interface to build runtime calls:
//!
//! ```rust,no_run
//! use sp_keyring::AccountKeyring;
//!
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
//! pub mod polkadot {}
//!
//! let runtime_call = polkadot::apis().metadata().metadata_versions();
//! ```
//!
//! Alternately, we can dynamically construct a runtime call:
//!
//! ```rust,no_run
//! use sp_keyring::AccountKeyring;
//! use subxt::dynamic::Value;
//!
//! let account = AccountKeyring::Alice.to_account_id();
//! let storage_query = subxt::dynamic::runtime_api_call("Metadata_metadata_versions", vec![], None)
//! ```
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
