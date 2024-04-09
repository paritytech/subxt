// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # subxt-core
//!
//! A `#[no_std]` compatible subset of the functionality provided in the `subxt` crate. This
//! contains the core logic for encoding and decoding things, but nothing related to networking.
//!
//! Here's an overview of the main things exposed here:
//!
//! - [`blocks`]: decode and explore block bodies.
//! - [`constants`]: access and validate the constant addresses in some metadata.
//! - [`custom_values`]: access and validate the custom value addresses in some metadata.
//! - [`metadata`]: decode bytes into the metadata used throughout this library.
//! - [`storage`]: construct storage request payloads and decode the results you'd get back.
//! - [`tx`]: construct and sign transactions (extrinsics).
//! - [`runtime_api`]: construct runtime API request payloads and decode the results you'd get back.
//! - [`events`]: decode and explore events.
//!

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
pub extern crate alloc;

#[macro_use]
mod macros;

pub mod blocks;
pub mod client;
pub mod config;
pub mod constants;
pub mod custom_values;
pub mod dynamic;
pub mod error;
pub mod events;
pub mod metadata;
pub mod runtime_api;
pub mod storage;
pub mod tx;
pub mod utils;

pub use config::Config;
pub use error::Error;
pub use metadata::Metadata;

/// Re-exports of some of the key external crates.
pub mod ext {
    pub use codec;
    pub use scale_decode;
    pub use scale_encode;
    pub use scale_value;

    cfg_substrate_compat! {
        pub use sp_runtime;
        pub use sp_core;
    }
}
