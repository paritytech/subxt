// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing constants.

mod constants_client;
mod constant_address;

pub use constants_client::{
    ConstantsClient,
};
pub use constant_address::{
    ConstantAddress,
};