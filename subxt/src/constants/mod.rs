// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing constants.

mod constant_address;
mod constants_client;

pub use constant_address::{dynamic, Address, ConstantAddress, DynamicAddress};
pub use constants_client::ConstantsClient;
