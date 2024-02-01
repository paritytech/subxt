// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides the entry points to create dynamic
//! transactions, storage and constant lookups.

use crate::error::Error;

use scale_decode::DecodeAsType;
use subxt_core::metadata::{DecodeWithMetadata, Metadata};

pub use scale_value::{At, Value};

/// A [`scale_value::Value`] type endowed with contextual information
/// regarding what type was used to decode each part of it. This implements
/// [`subxt_core::metadata::DecodeWithMetadata`], and is used as a return type
/// for dynamic requests.
pub type DecodedValue = scale_value::Value<scale_value::scale::TypeId>;

// Submit dynamic transactions.
pub use crate::tx::dynamic as tx;

// Lookup constants dynamically.
pub use crate::constants::dynamic as constant;

// Lookup storage values dynamically.
pub use crate::storage::dynamic as storage;

// Execute runtime API function call dynamically.
pub use crate::runtime_api::dynamic as runtime_api_call;
