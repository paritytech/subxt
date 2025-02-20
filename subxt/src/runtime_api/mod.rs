// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with executing runtime API calls.

mod runtime_client;
mod runtime_types;

pub use runtime_client::RuntimeApiClient;
pub use runtime_types::RuntimeApi;
pub use subxt_core::runtime_api::payload::{
    dynamic, DefaultPayload, DynamicPayload, Payload, StaticPayload,
};
