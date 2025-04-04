// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with executing View Function calls.

mod view_functions_client;
mod view_function_types;

pub use view_functions_client::ViewFunctionsClient;
pub use view_function_types::ViewFunctionsApi;
pub use subxt_core::view_functions::payload::{
    dynamic, DefaultPayload, DynamicPayload, Payload, StaticPayload,
};
