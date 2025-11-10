// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides the entry points to create dynamic
//! transactions, storage and constant lookups.

pub use scale_value::{At, Value};

// Submit dynamic transactions.
pub use crate::tx::payload::dynamic as tx;

// Lookup constants dynamically.
pub use crate::constants::address::dynamic as constant;

// Lookup storage values dynamically.
pub use crate::storage::address::dynamic as storage;

// Execute runtime API function call dynamically.
pub use crate::runtime_api::payload::dynamic as runtime_api_call;

// Execute View Function API function call dynamically.
pub use crate::view_functions::payload::dynamic as view_function_call;

/// Obtain a custom value from the metadata.
pub use crate::custom_values::address::dynamic as custom_value;
