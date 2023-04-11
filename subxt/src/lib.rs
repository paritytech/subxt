// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Subxt is a library for interacting with Substrate based nodes. Using it looks something like this:
//!
//! ```rust,ignore
#![doc = include_str!("../../examples/examples/balance_transfer_basic.rs")]
//! ```
//!
//! Take a look at [the Subxt guide](book) to learn more about how to use Subxt.

#![deny(
    bad_style,
    improper_ctypes,
    missing_docs,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    trivial_casts,
    trivial_numeric_casts,
    unused_crate_dependencies,
    unused_extern_crates,
    clippy::all
)]
#![allow(clippy::type_complexity)]

// The guide is here.
pub mod book;

// Suppress an unused dependency warning because tokio is
// only used in example code snippets at the time of writing.
#[cfg(test)]
use tokio as _;

pub use subxt_macro::subxt;

// Used to enable the js feature for wasm.
#[cfg(target_arch = "wasm32")]
pub use getrandom as _;

#[cfg(all(feature = "jsonrpsee-ws", feature = "jsonrpsee-web"))]
std::compile_error!(
    "Both the features `jsonrpsee-ws` and `jsonrpsee-web` are enabled which are mutually exclusive"
);

pub mod blocks;
pub mod client;
pub mod config;
pub mod constants;
pub mod dynamic;
pub mod error;
pub mod events;
pub mod metadata;
pub mod rpc;
pub mod runtime_api;
pub mod storage;
pub mod tx;
pub mod utils;

// Expose a few of the most common types at root,
// but leave most types behind their respective modules.
pub use crate::{
    client::{OfflineClient, OnlineClient},
    config::{Config, PolkadotConfig, SubstrateConfig},
    error::Error,
    metadata::Metadata,
};

/// Re-export external crates that are made use of in the subxt API.
pub mod ext {
    pub use codec;
    pub use frame_metadata;
    pub use scale_bits;
    pub use scale_decode;
    pub use scale_encode;
    pub use scale_value;
    #[cfg(feature = "substrate-compat")]
    pub use sp_core;
    #[cfg(feature = "substrate-compat")]
    pub use sp_runtime;
}
