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
use scale_value::{
    Composite,
    ValueDef,
    Variant,
};
use std::borrow::Cow;

/// This represents a transaction payload that can be submitted
/// to a node.
pub trait TxPayload {
    /// Encode call data to the provided output.
    fn encode_call_data_to(
        &self,
        metadata: &Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), Error>;

    /// Encode call data and return the output. This is a convenience
    /// wrapper around [`TxPayload::encode_call_data_to`].
    fn encode_call_data(&self, metadata: &Metadata) -> Result<Vec<u8>, Error> {
        let mut v = Vec::new();
        self.encode_call_data_to(metadata, &mut v)?;
        Ok(v)
    }

    /// Returns the details needed to validate the call, which
    /// include a statically generated hash, the pallet name,
    /// and the call name.
    fn validation_details(&self) -> Option<ValidationDetails<'_>> {
        None
    }
}

pub struct ValidationDetails<'a> {
    /// The pallet name.
    pub pallet_name: &'a str,
    /// The call name.
    pub call_name: &'a str,
    /// A hash (this is generated at compile time in our codegen)
    /// to compare against the runtime code.
    pub hash: [u8; 32],
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

    /// Returns the call data.
    pub fn call_data(&self) -> &CallData {
        &self.call_data
    }
}

impl<CallData: Encode> TxPayload for StaticTxPayload<CallData> {
    fn encode_call_data_to(
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

    fn validation_details(&self) -> Option<ValidationDetails<'_>> {
        self.validation_hash.map(|hash| {
            ValidationDetails {
                pallet_name: self.pallet_name,
                call_name: self.call_name,
                hash,
            }
        })
    }
}

/// This represents a dynamically generated transaction payload.
pub struct DynamicTxPayload<'a> {
    pallet_name: Cow<'a, str>,
    call_name: Cow<'a, str>,
    fields: Composite<()>,
}

impl<'a> DynamicTxPayload<'a> {
    /// Return the pallet name.
    pub fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    /// Return the call name.
    pub fn call_name(&self) -> &str {
        &self.call_name
    }

    /// Convert the dynamic payload into a [`Value`]. This is useful
    /// if you need to submit this as part of a larger call.
    pub fn into_value(self) -> Value<()> {
        let call = Value {
            context: (),
            value: ValueDef::Variant(Variant {
                name: self.call_name.into_owned(),
                values: self.fields,
            }),
        };

        Value::unnamed_variant(self.pallet_name, [call])
    }
}

/// Construct a new dynamic transaction payload to submit to a node.
pub fn dynamic<'a>(
    pallet_name: impl Into<Cow<'a, str>>,
    call_name: impl Into<Cow<'a, str>>,
    fields: impl Into<Composite<()>>,
) -> DynamicTxPayload<'a> {
    DynamicTxPayload {
        pallet_name: pallet_name.into(),
        call_name: call_name.into(),
        fields: fields.into(),
    }
}

impl<'a> TxPayload for DynamicTxPayload<'a> {
    fn encode_call_data_to(
        &self,
        metadata: &Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let pallet = metadata.pallet(&self.pallet_name)?;
        let call_id = pallet.call_ty_id().ok_or(MetadataError::CallNotFound)?;
        let call_value = Value {
            context: (),
            value: ValueDef::Variant(Variant {
                name: self.call_name.to_string(),
                values: self.fields.clone(),
            }),
        };

        pallet.index().encode_to(out);
        scale_value::scale::encode_as_type(&call_value, call_id, metadata.types(), out)?;
        Ok(())
    }
}
