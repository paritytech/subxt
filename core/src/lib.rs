// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Subxt-core
//!
//! `#[no_std]` compatible core crate for subxt.

pub mod client;
pub mod config;
pub mod dynamic;
pub mod metadata;
pub mod tx;
pub mod utils;

pub use config::{
    BlockHash, Config, ExtrinsicParams, ExtrinsicParamsEncoder, PolkadotConfig,
    PolkadotExtrinsicParams, SubstrateConfig, SubstrateExtrinsicParams,
};

#[macro_use]
mod macros;
