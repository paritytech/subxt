// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module provides the entry points to create dynamic
//! transactions, storage and constant lookups.

use crate::{
    error::Error,
    metadata::{DecodeWithMetadata, Metadata},
};
use scale_decode::DecodeAsType;

pub use scale_value::Value;

/// A [`scale_value::Value`] type endowed with contextual information
/// regarding what type was used to decode each part of it. This implements
/// [`crate::metadata::DecodeWithMetadata`], and is used as a return type
/// for dynamic requests.
pub type DecodedValue = scale_value::Value<scale_value::scale::TypeId>;

// Submit dynamic transactions.
pub use crate::tx::dynamic as tx;

// Lookup constants dynamically.
pub use crate::constants::dynamic as constant;

// Lookup storage values dynamically.
pub use crate::storage::{dynamic as storage, dynamic_root as storage_root};

/// This is the result of making a dynamic request to a node. From this,
/// we can return the raw SCALE bytes that we were handed back, or we can
/// complete the decoding of the bytes into a [`DecodedValue`] type.
pub struct DecodedValueThunk {
    type_id: u32,
    metadata: Metadata,
    scale_bytes: Vec<u8>,
}

impl DecodeWithMetadata for DecodedValueThunk {
    fn decode_with_metadata(
        bytes: &mut &[u8],
        type_id: u32,
        metadata: &Metadata,
    ) -> Result<Self, Error> {
        let mut v = Vec::with_capacity(bytes.len());
        v.extend_from_slice(bytes);
        *bytes = &[];
        Ok(DecodedValueThunk {
            type_id,
            metadata: metadata.clone(),
            scale_bytes: v,
        })
    }
}

impl DecodedValueThunk {
    /// Return the SCALE encoded bytes handed back from the node.
    pub fn into_encoded(self) -> Vec<u8> {
        self.scale_bytes
    }
    /// Return the SCALE encoded bytes handed back from the node without taking ownership of them.
    pub fn encoded(&self) -> &[u8] {
        &self.scale_bytes
    }
    /// Decode the SCALE encoded storage entry into a dynamic [`DecodedValue`] type.
    pub fn to_value(&self) -> Result<DecodedValue, Error> {
        let val = DecodedValue::decode_as_type(
            &mut &*self.scale_bytes,
            self.type_id,
            self.metadata.types(),
        )?;
        Ok(val)
    }
}
