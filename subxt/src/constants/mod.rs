// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing constants.

mod constants_client;

pub use constants_client::ConstantsClient;
pub use subxt_core::constants::address::{
    dynamic, Address, DefaultAddress, DynamicAddress, StaticAddress,
};
