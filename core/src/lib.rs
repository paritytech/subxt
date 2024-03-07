// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Subxt-core
//!
//! `#[no_std]` compatible core crate for subxt.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod client;
pub mod config;
pub mod constants;
pub mod dynamic;
mod error;
pub mod metadata;
pub mod utils;
pub mod runtime_api;
pub mod tx;
pub mod signer;
pub mod custom_values;
pub mod storage;

pub use client::{ClientMetadata, RuntimeVersion};
pub use config::{
    BlockHash, Config, ExtrinsicParams, ExtrinsicParamsEncoder, PolkadotConfig,
    PolkadotExtrinsicParams, SubstrateConfig, SubstrateExtrinsicParams,
};
pub use error::{Error, ExtrinsicParamsError, MetadataError, StorageAddressError};
pub use metadata::Metadata;
pub use utils::{to_hex, AccountId32, MultiAddress, MultiSignature, Yes, H160, H256, H512};
pub use signer::Signer;

#[macro_use]
mod macros;
