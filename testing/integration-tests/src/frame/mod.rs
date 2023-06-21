// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Test interactions with some built-in FRAME pallets.

mod balances;
mod staking;
mod sudo;
mod system;
mod timestamp;

#[cfg(not(feature = "unstable-light-client"))]
mod contracts;
