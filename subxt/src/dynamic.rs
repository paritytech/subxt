// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module ex-exports various helpers for constructing dynamic payloads/queries/addresses.

pub use scale_value::{At, Value, value};

// Submit dynamic transactions.
pub use crate::transactions::dynamic as transaction;
pub use crate::transactions::dynamic as tx;

// Lookup constants dynamically.
pub use crate::constants::dynamic as constant;

// Lookup storage values dynamically.
pub use crate::storage::dynamic as storage;

// Execute runtime API function call dynamically.
pub use crate::runtime_apis::dynamic as runtime_api_call;

// Execute View Function API function call dynamically.
pub use crate::view_functions::dynamic as view_function_call;

/// Obtain a custom value from the metadata.
pub use crate::custom_values::dynamic as custom_value;
