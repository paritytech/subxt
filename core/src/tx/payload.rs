// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains the trait and types used to represent
//! transactions that can be submitted.

use crate::Error;
use crate::error::MetadataError;
use crate::metadata::Metadata;
use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::string::String;

use alloc::vec::Vec;
use codec::Encode;
use scale_encode::EncodeAsFields;
use scale_value::{Composite, Value, ValueDef, Variant};

/// This represents a transaction payload that can be submitted
/// to a node.
pub trait Payload {
    /// Encode call data to the provided output.
    fn encode_call_data_to(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error>;

    /// Encode call data and return the output. This is a convenience
    /// wrapper around [`Payload::encode_call_data_to`].
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

macro_rules! boxed_payload {
    ($ty:path) => {
        impl<T: Payload + ?Sized> Payload for $ty {
            fn encode_call_data_to(
                &self,
                metadata: &Metadata,
                out: &mut Vec<u8>,
            ) -> Result<(), Error> {
                self.as_ref().encode_call_data_to(metadata, out)
            }
            fn encode_call_data(&self, metadata: &Metadata) -> Result<Vec<u8>, Error> {
                self.as_ref().encode_call_data(metadata)
            }
            fn validation_details(&self) -> Option<ValidationDetails<'_>> {
                self.as_ref().validation_details()
            }
        }
    };
}

boxed_payload!(Box<T>);
#[cfg(feature = "std")]
boxed_payload!(std::sync::Arc<T>);
#[cfg(feature = "std")]
boxed_payload!(std::rc::Rc<T>);

/// Details required to validate the shape of a transaction payload against some metadata.
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DefaultPayload<CallData> {
    pallet_name: Cow<'static, str>,
    call_name: Cow<'static, str>,
    call_data: CallData,
    validation_hash: Option<[u8; 32]>,
}

/// The payload type used by static codegen.
pub type StaticPayload<Calldata> = DefaultPayload<Calldata>;
/// The type of a payload typically used for dynamic transaction payloads.
pub type DynamicPayload = DefaultPayload<Composite<()>>;

impl<CallData> DefaultPayload<CallData> {
    /// Create a new [`DefaultPayload`].
    pub fn new(
        pallet_name: impl Into<String>,
        call_name: impl Into<String>,
        call_data: CallData,
    ) -> Self {
        DefaultPayload {
            pallet_name: Cow::Owned(pallet_name.into()),
            call_name: Cow::Owned(call_name.into()),
            call_data,
            validation_hash: None,
        }
    }

    /// Create a new [`DefaultPayload`] using static strings for the pallet and call name.
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        call_name: &'static str,
        call_data: CallData,
        validation_hash: [u8; 32],
    ) -> Self {
        DefaultPayload {
            pallet_name: Cow::Borrowed(pallet_name),
            call_name: Cow::Borrowed(call_name),
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

    /// Returns the pallet name.
    pub fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    /// Returns the call name.
    pub fn call_name(&self) -> &str {
        &self.call_name
    }
}

impl DefaultPayload<Composite<()>> {
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

impl<CallData: EncodeAsFields> Payload for DefaultPayload<CallData> {
    fn encode_call_data_to(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error> {
        let pallet = metadata.pallet_by_name_err(&self.pallet_name)?;
        let call = pallet
            .call_variant_by_name(&self.call_name)
            .ok_or_else(|| MetadataError::CallNameNotFound((*self.call_name).to_owned()))?;

        let pallet_index = pallet.index();
        let call_index = call.index;

        pallet_index.encode_to(out);
        call_index.encode_to(out);

        let mut fields = call
            .fields
            .iter()
            .map(|f| scale_encode::Field::new(f.ty.id, f.name.as_deref()));

        self.call_data
            .encode_as_fields_to(&mut fields, metadata.types(), out)
            .expect("The fields are valid types from the metadata, qed;");
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

/// Construct a transaction at runtime; essentially an alias to [`DefaultPayload::new()`]
/// which provides a [`Composite`] value for the call data.
pub fn dynamic(
    pallet_name: impl Into<String>,
    call_name: impl Into<String>,
    call_data: impl Into<Composite<()>>,
) -> DynamicPayload {
    DefaultPayload::new(pallet_name, call_name, call_data.into())
}
