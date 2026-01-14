// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains the trait and types used to represent
//! View Function calls that can be made.

use core::marker::PhantomData;
use derive_where::derive_where;
use frame_decode::view_functions::IntoEncodableValues;
use scale_decode::DecodeAsType;
use std::borrow::Cow;

/// This represents a View Function payload that can call into the runtime of node.
///
/// # Components
///
/// - associated return type
///
/// Resulting bytes of the call are interpreted into this type.
///
/// - query ID
///
/// The ID used to identify in the runtime which view function to call.
///
/// - encoded arguments
///
/// Each argument of the View Function must be scale-encoded.
pub trait Payload {
    /// Type of the arguments for this call.
    type ArgsType: IntoEncodableValues;
    /// The return type of the function call.
    type ReturnType: DecodeAsType;

    /// The View Function pallet name.
    fn pallet_name(&self) -> &str;

    /// The View Function function name.
    fn function_name(&self) -> &str;

    /// The arguments.
    fn args(&self) -> &Self::ArgsType;

    /// Returns the statically generated validation hash.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

// A reference to a payload is a valid payload.
impl<P: Payload + ?Sized> Payload for &'_ P {
    type ArgsType = P::ArgsType;
    type ReturnType = P::ReturnType;

    fn pallet_name(&self) -> &str {
        P::pallet_name(*self)
    }

    fn function_name(&self) -> &str {
        P::function_name(*self)
    }

    fn args(&self) -> &Self::ArgsType {
        P::args(*self)
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        P::validation_hash(*self)
    }
}

/// A View Function payload containing the generic argument data
/// and interpreting the result of the call as `ReturnType`.
///
/// This can be created from static values (ie those generated
/// via the `subxt` macro) or dynamic values via [`dynamic`].
#[derive_where(Clone, Debug, Eq, Ord, PartialEq, PartialOrd; ArgsType)]
pub struct StaticPayload<ArgsType, ReturnType> {
    pallet_name: Cow<'static, str>,
    function_name: Cow<'static, str>,
    args: ArgsType,
    validation_hash: Option<[u8; 32]>,
    _marker: PhantomData<ReturnType>,
}

/// A dynamic View Function payload.
pub type DynamicPayload<ArgsType, ReturnType> = StaticPayload<ArgsType, ReturnType>;

impl<ArgsType: IntoEncodableValues, ReturnType: DecodeAsType> Payload
    for StaticPayload<ArgsType, ReturnType>
{
    type ArgsType = ArgsType;
    type ReturnType = ReturnType;

    fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    fn function_name(&self) -> &str {
        &self.function_name
    }

    fn args(&self) -> &Self::ArgsType {
        &self.args
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

impl<ReturnTy, ArgsType> StaticPayload<ArgsType, ReturnTy> {
    /// Create a new [`StaticPayload`] for a View Function call.
    pub fn new(
        pallet_name: impl Into<String>,
        function_name: impl Into<String>,
        args: ArgsType,
    ) -> Self {
        StaticPayload {
            pallet_name: pallet_name.into().into(),
            function_name: function_name.into().into(),
            args,
            validation_hash: None,
            _marker: PhantomData,
        }
    }

    /// Create a new static [`StaticPayload`] for a View Function call
    /// using static function name and scale-encoded argument data.
    ///
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        function_name: &'static str,
        args: ArgsType,
        hash: [u8; 32],
    ) -> StaticPayload<ArgsType, ReturnTy> {
        StaticPayload {
            pallet_name: Cow::Borrowed(pallet_name),
            function_name: Cow::Borrowed(function_name),
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
}

/// Create a new [`DynamicPayload`] to call a View Function.
pub fn dynamic<ArgsType, ReturnType>(
    pallet_name: impl Into<String>,
    function_name: impl Into<String>,
    args: ArgsType,
) -> DynamicPayload<ArgsType, ReturnType> {
    DynamicPayload::new(pallet_name, function_name, args)
}
