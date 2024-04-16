// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Test interactions with some built-in FRAME pallets.

mod balances;
mod staking;
mod system;
mod timestamp;

#[cfg(fullclient)]
mod contracts;
#[cfg(fullclient)]
mod sudo;
