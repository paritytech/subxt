// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with executing View Function calls.

mod view_function_types;
mod view_functions_client;

pub use subxt_core::view_functions::payload::{DynamicPayload, Payload, StaticPayload, dynamic};
pub use view_function_types::ViewFunctionsApi;
pub use view_functions_client::ViewFunctionsClient;
