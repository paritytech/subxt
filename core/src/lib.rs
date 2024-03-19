// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Subxt-core
//!
//! `#[no_std]` compatible core crate for subxt.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

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
pub mod signer;
pub mod storage;
pub mod tx;
pub mod utils;

pub use config::Config;
pub use error::Error;
pub use metadata::Metadata;
pub use signer::Signer;

pub use alloc::boxed::Box;
pub use alloc::vec::Vec;

#[macro_use]
mod macros;

pub mod ext {
    pub use codec;
    pub use scale_decode;
    pub use scale_encode;

    cfg_substrate_compat! {
        pub use sp_runtime;
        pub use sp_core;
    }
}
