// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Subxt-core
//!
//! `#[no_std]` compatible core crate for subxt.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod client;
pub mod config;
pub mod dynamic;
pub mod metadata;
pub mod prelude;
pub mod tx;
pub mod utils;

pub use config::{
    BlockHash, Config, ExtrinsicParams, ExtrinsicParamsEncoder, PolkadotConfig,
    PolkadotExtrinsicParams, SubstrateConfig, SubstrateExtrinsicParams,
};

pub use metadata::Metadata;

#[macro_use]
mod macros;
