// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains the trait and types used to represent
//! transactions that can be submitted.

use scale_encode::EncodeAsFields;
use scale_value::{Composite, Value, ValueDef, Variant};
use std::borrow::Cow;

/// This represents a transaction payload that can be submitted
/// to a node.
pub trait Payload {
    /// The call data
    type CallData: EncodeAsFields;

    /// The pallet name
    fn pallet_name(&self) -> &str;

    /// The call name
    fn call_name(&self) -> &str;

    /// The call data
    fn call_data(&self) -> &Self::CallData;

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
            type CallData = T::CallData;

            fn pallet_name(&self) -> &str {
                self.as_ref().pallet_name()
            }
            fn call_name(&self) -> &str {
                self.as_ref().call_name()
            }
            fn call_data(&self) -> &Self::CallData {
                self.as_ref().call_data()
            }
            fn validation_details(&self) -> Option<ValidationDetails<'_>> {
                self.as_ref().validation_details()
            }
        }
    };
}

boxed_payload!(Box<T>);
boxed_payload!(std::sync::Arc<T>);
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
pub struct StaticPayload<CallData> {
    pallet_name: Cow<'static, str>,
    call_name: Cow<'static, str>,
    call_data: CallData,
    validation_hash: Option<[u8; 32]>,
}

/// The type of a payload typically used for dynamic transaction payloads.
pub type DynamicPayload<CallData> = StaticPayload<CallData>;

impl<CallData> StaticPayload<CallData> {
    /// Create a new [`StaticPayload`].
    pub fn new(
        pallet_name: impl Into<String>,
        call_name: impl Into<String>,
        call_data: CallData,
    ) -> Self {
        StaticPayload {
            pallet_name: Cow::Owned(pallet_name.into()),
            call_name: Cow::Owned(call_name.into()),
            call_data,
            validation_hash: None,
        }
    }

    /// Create a new [`StaticPayload`] using static strings for the pallet and call name.
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        call_name: &'static str,
        call_data: CallData,
        validation_hash: [u8; 32],
    ) -> Self {
        StaticPayload {
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

impl StaticPayload<Composite<()>> {
    /// Convert the `Composite` payload into a [`Value`].
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

impl<CD: EncodeAsFields> Payload for StaticPayload<CD> {
    type CallData = CD;

    fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    fn call_name(&self) -> &str {
        &self.call_name
    }

    fn call_data(&self) -> &Self::CallData {
        &self.call_data
    }

    fn validation_details(&self) -> Option<ValidationDetails<'_>> {
        self.validation_hash.map(|hash| ValidationDetails {
            pallet_name: &self.pallet_name,
            call_name: &self.call_name,
            hash,
        })
    }
}

/// Construct a transaction at runtime; essentially an alias to [`DynamicPayload::new()`].
pub fn dynamic<CallData>(
    pallet_name: impl Into<String>,
    call_name: impl Into<String>,
    call_data: CallData,
) -> DynamicPayload<CallData> {
    StaticPayload::new(pallet_name, call_name, call_data)
}
