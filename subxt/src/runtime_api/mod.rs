// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with executing runtime API calls.

#[cfg(feature = "std")]
mod runtime_client;

mod runtime_payload;
#[cfg(feature = "std")]
mod runtime_types;


#[cfg(feature = "std")]
pub use runtime_client::RuntimeApiClient;

pub use runtime_payload::{dynamic, DynamicRuntimeApiPayload, Payload, RuntimeApiPayload};

#[cfg(feature = "std")]
pub use runtime_types::RuntimeApi;
