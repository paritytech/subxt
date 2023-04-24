// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes the types and such necessary for working with extrinsics.
//! The two main entry points into events are [`crate::OnlineClient::events()`]
//! and calls like [crate::tx::TxProgress::wait_for_finalized_success()].

mod extrinsics_client;
mod extrinsics_type;

pub use extrinsics_client::ExtrinsicsClient;
pub use extrinsics_type::{ExtrinsicDetails, Extrinsics};
