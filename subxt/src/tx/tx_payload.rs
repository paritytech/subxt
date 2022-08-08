// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains the trait and types used to represent
//! transactions that can be submitted.

use crate::{
    dynamic::Value,
    error::{
        Error,
        MetadataError,
    },
    metadata::Metadata,
};
use codec::Encode;
use std::borrow::Cow;

/// This represents a transaction payload that can be submitted
/// to a node.
pub trait TxPayload {
    /// The name of the pallet that the call lives under.
    fn pallet_name(&self) -> &str;

    /// The name of the call.
    fn call_name(&self) -> &str;

    /// Encode call data to the provided output.
    fn encode_call_data(
        &self,
        metadata: &Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), Error>;

    /// An optional validation hash that can be provided
    /// to verify that the shape of the call on the node
    /// aligns with our expectations.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

/// This represents a statically generated transaction payload.
pub struct StaticTxPayload<CallData> {
    pallet_name: &'static str,
    call_name: &'static str,
    call_data: CallData,
    validation_hash: Option<[u8; 32]>,
}

impl<CallData> StaticTxPayload<CallData> {
    /// Create a new [`StaticTxPayload`] from static data.
    pub fn new(
        pallet_name: &'static str,
        call_name: &'static str,
        call_data: CallData,
        validation_hash: [u8; 32],
    ) -> Self {
        StaticTxPayload {
            pallet_name,
            call_name,
            call_data,
            validation_hash: Some(validation_hash),
        }
    }

    /// Do not validate this call prior to submitting it.
    pub fn unvalidated(self) -> Self {
        Self {
            validation_hash: None,
            ..self
        }
    }
}

impl<CallData: Encode> TxPayload for StaticTxPayload<CallData> {
    fn pallet_name(&self) -> &str {
        self.pallet_name
    }

    fn call_name(&self) -> &str {
        self.call_name
    }

    fn encode_call_data(
        &self,
        metadata: &Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let pallet = metadata.pallet(self.pallet_name)?;
        let pallet_index = pallet.index();
        let call_index = pallet.call_index(self.call_name)?;

        pallet_index.encode_to(out);
        call_index.encode_to(out);
        self.call_data.encode_to(out);
        Ok(())
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

/// This represents a dynamically generated transaction payload.
pub struct DynamicTxPayload<'a> {
    pallet_name: Cow<'a, str>,
    call_name: Cow<'a, str>,
    fields: Vec<Value<()>>,
}

/// Construct a new dynamic transaction payload to submit to a node.
pub fn dynamic<'a>(
    pallet_name: impl Into<Cow<'a, str>>,
    call_name: impl Into<Cow<'a, str>>,
    fields: Vec<Value<()>>,
) -> DynamicTxPayload<'a> {
    DynamicTxPayload {
        pallet_name: pallet_name.into(),
        call_name: call_name.into(),
        fields,
    }
}

impl<'a> TxPayload for DynamicTxPayload<'a> {
    fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    fn call_name(&self) -> &str {
        &self.call_name
    }

    fn encode_call_data(
        &self,
        metadata: &Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let pallet = metadata.pallet(&self.pallet_name)?;
        let call_id = pallet.call_ty_id().ok_or(MetadataError::CallNotFound)?;
        let call_value =
            Value::unnamed_variant(self.call_name.to_owned(), self.fields.clone());

        pallet.index().encode_to(out);
        scale_value::scale::encode_as_type(&call_value, call_id, metadata.types(), out)?;
        Ok(())
    }
}
