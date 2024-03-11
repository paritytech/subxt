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
pub mod custom_values;
pub mod dynamic;
mod error;
pub mod events;
pub mod metadata;
pub mod runtime_api;
pub mod signer;
pub mod storage;
pub mod tx;
pub mod utils;

pub use client::{ClientMetadata, RuntimeVersion};
pub use config::{
    BlockHash, Config, ExtrinsicParams, ExtrinsicParamsEncoder, Hasher, Header, PolkadotConfig,
    PolkadotExtrinsicParams, RefineParams, RefineParamsData, SubstrateConfig,
    SubstrateExtrinsicParams,
};
pub use error::{Error, ExtrinsicParamsError, MetadataError, StorageAddressError};
pub use events::{EventDetails, Phase, StaticEvent};
pub use metadata::Metadata;
pub use signer::Signer;

pub use utils::{
    to_hex, AccountId32, Encoded, Era, MultiAddress, MultiSignature, Static, UncheckedExtrinsic,
    WrapperKeepOpaque, Yes, H160, H256, H512,
};

#[macro_use]
mod macros;
