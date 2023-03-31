// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains the trait and types used to represent
//! transactions that can be submitted.

use crate::{dynamic::Value, error::Error, metadata::Metadata};
use codec::Encode;
use scale_encode::EncodeAsFields;
use scale_value::{Composite, ValueDef, Variant};
use std::{borrow::Cow, sync::Arc};

/// This represents a transaction payload that can be submitted
/// to a node.
pub trait TxPayload {
    /// Encode call data to the provided output.
    fn encode_call_data_to(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error>;

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

/// A transaction payload containing some generic `CallData`.
#[derive(Clone, Debug)]
pub struct Payload<CallData> {
    pallet_name: Cow<'static, str>,
    call_name: Cow<'static, str>,
    call_data: CallData,
    validation_hash: Option<[u8; 32]>,
}

/// A boxed transaction payload.
// Dev Note: Arc used to enable easy cloning (given that we can't have dyn Clone).
pub type BoxedPayload = Payload<Arc<dyn EncodeAsFields + Send + Sync + 'static>>;

/// The type of a payload typically used for dynamic transaction payloads.
pub type DynamicPayload = Payload<Composite<()>>;

impl<CallData> Payload<CallData> {
    /// Create a new [`Payload`].
    pub fn new(
        pallet_name: impl Into<String>,
        call_name: impl Into<String>,
        call_data: CallData,
    ) -> Self {
        Payload {
            pallet_name: Cow::Owned(pallet_name.into()),
            call_name: Cow::Owned(call_name.into()),
            call_data,
            validation_hash: None,
        }
    }

    /// Create a new [`Payload`] using static strings for the pallet and call name.
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        call_name: &'static str,
        call_data: CallData,
        validation_hash: [u8; 32],
    ) -> Self {
        Payload {
            pallet_name: Cow::Borrowed(pallet_name),
            call_name: Cow::Borrowed(call_name),
            call_data,
            validation_hash: Some(validation_hash),
        }
    }

    /// Box the payload.
    pub fn boxed(self) -> BoxedPayload
    where
        CallData: EncodeAsFields + Send + Sync + 'static,
    {
        BoxedPayload {
            pallet_name: self.pallet_name,
            call_name: self.call_name,
            call_data: Arc::new(self.call_data),
            validation_hash: self.validation_hash,
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

impl Payload<Composite<()>> {
    /// Convert the dynamic `Composite` payload into a [`Value`].
    /// This is useful if you want to use this as an argument for a
    /// larger dynamic call that wants to use this as a nested call.
    pub fn into_value(self) -> Value<()> {
        let call = Value {
            context: (),
            value: ValueDef::Variant(Variant {
                name: self.call_name.into_owned(),
                values: self.call_data,
            }),
        };

        Value::unnamed_variant(self.pallet_name, [call])
    }
}

impl<CallData: EncodeAsFields> TxPayload for Payload<CallData> {
    fn encode_call_data_to(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error> {
        let pallet = metadata.pallet(&self.pallet_name)?;
        let call = pallet.call(&self.call_name)?;

        let pallet_index = pallet.index();
        let call_index = call.index();

        pallet_index.encode_to(out);
        call_index.encode_to(out);

        self.call_data
            .encode_as_fields_to(call.fields(), metadata.types(), out)?;
        Ok(())
    }

    fn validation_details(&self) -> Option<ValidationDetails<'_>> {
        self.validation_hash.map(|hash| ValidationDetails {
            pallet_name: &self.pallet_name,
            call_name: &self.call_name,
            hash,
        })
    }
}

/// Construct a transaction at runtime; essentially an alias to [`Payload::new()`]
/// which provides a [`Composite`] value for the call data.
pub fn dynamic(
    pallet_name: impl Into<String>,
    call_name: impl Into<String>,
    call_data: impl Into<Composite<()>>,
) -> DynamicPayload {
    Payload::new(pallet_name, call_name, call_data.into())
}
