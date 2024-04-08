// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing constants.

mod constants_client;

pub use constants_client::ConstantsClient;
pub use subxt_core::constants::address::{Address, AddressT, DynamicAddress, dynamic};
