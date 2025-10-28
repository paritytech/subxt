// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains the trait and types used to represent
//! runtime API calls that can be made.

use alloc::borrow::Cow;
use alloc::string::String;
use core::marker::PhantomData;
use derive_where::derive_where;
use frame_decode::runtime_apis::IntoEncodableValues;
use scale_decode::DecodeAsType;

/// This represents a runtime API payload that can be used to call a Runtime API on
/// a chain and decode the response.
pub trait Payload {
    /// Type of the arguments.
    type ArgsType: IntoEncodableValues;
    /// The return type of the function call.
    type ReturnType: DecodeAsType;

    /// The runtime API trait name.
    fn trait_name(&self) -> &str;

    /// The runtime API method name.
    fn method_name(&self) -> &str;

    /// The input arguments.
    fn args(&self) -> &Self::ArgsType;

    /// Returns the statically generated validation hash.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

// Any reference to a payload is a valid payload.
impl<P: Payload + ?Sized> Payload for &'_ P {
    type ArgsType = P::ArgsType;
    type ReturnType = P::ReturnType;

    fn trait_name(&self) -> &str {
        P::trait_name(*self)
    }

    fn method_name(&self) -> &str {
        P::method_name(*self)
    }

    fn args(&self) -> &Self::ArgsType {
        P::args(*self)
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        P::validation_hash(*self)
    }
}

/// A runtime API payload containing the generic argument data
/// and interpreting the result of the call as `ReturnTy`.
///
/// This can be created from static values (ie those generated
/// via the `subxt` macro) or dynamic values via [`dynamic`].
#[derive_where(Clone, Debug, Eq, Ord, PartialEq, PartialOrd; ArgsType)]
pub struct StaticPayload<ArgsType, ReturnType> {
    trait_name: Cow<'static, str>,
    method_name: Cow<'static, str>,
    args: ArgsType,
    validation_hash: Option<[u8; 32]>,
    _marker: PhantomData<ReturnType>,
}

/// A dynamic runtime API payload.
pub type DynamicPayload<ArgsType, ReturnType> = StaticPayload<ArgsType, ReturnType>;

impl<ArgsType: IntoEncodableValues, ReturnType: DecodeAsType> Payload
    for StaticPayload<ArgsType, ReturnType>
{
    type ArgsType = ArgsType;
    type ReturnType = ReturnType;

    fn trait_name(&self) -> &str {
        &self.trait_name
    }

    fn method_name(&self) -> &str {
        &self.method_name
    }

    fn args(&self) -> &Self::ArgsType {
        &self.args
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

impl<ArgsType, ReturnTy> StaticPayload<ArgsType, ReturnTy> {
    /// Create a new [`StaticPayload`].
    pub fn new(
        trait_name: impl Into<String>,
        method_name: impl Into<String>,
        args: ArgsType,
    ) -> Self {
        StaticPayload {
            trait_name: trait_name.into().into(),
            method_name: method_name.into().into(),
            args,
            validation_hash: None,
            _marker: PhantomData,
        }
    }

    /// Create a new static [`StaticPayload`] using static function name
    /// and scale-encoded argument data.
    ///
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        trait_name: &'static str,
        method_name: &'static str,
        args: ArgsType,
        hash: [u8; 32],
    ) -> StaticPayload<ArgsType, ReturnTy> {
        StaticPayload {
            trait_name: Cow::Borrowed(trait_name),
            method_name: Cow::Borrowed(method_name),
            args,
            validation_hash: Some(hash),
            _marker: core::marker::PhantomData,
        }
    }

    /// Do not validate this call prior to submitting it.
    pub fn unvalidated(self) -> Self {
        Self {
            validation_hash: None,
            ..self
        }
    }

    /// Returns the trait name.
    pub fn trait_name(&self) -> &str {
        &self.trait_name
    }

    /// Returns the method name.
    pub fn method_name(&self) -> &str {
        &self.method_name
    }
}

/// Create a new [`DynamicPayload`].
pub fn dynamic<ArgsType, ReturnType>(
    trait_name: impl Into<String>,
    method_name: impl Into<String>,
    args_data: ArgsType,
) -> DynamicPayload<ArgsType, ReturnType> {
    DynamicPayload::new(trait_name, method_name, args_data)
}
