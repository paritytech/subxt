// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains the trait and types used to represent
//! runtime API calls that can be made.

use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;
use derive_where::derive_where;
use scale_encode::EncodeAsFields;
use scale_value::Composite;

use crate::Error;
use crate::dynamic::DecodedValueThunk;
use crate::error::MetadataError;

use crate::metadata::{DecodeWithMetadata, Metadata};

/// This represents a runtime API payload that can call into the runtime of node.
///
/// # Components
///
/// - associated return type
///
/// Resulting bytes of the call are interpreted into this type.
///
/// - runtime function name
///
/// The function name of the runtime API call. This is obtained by concatenating
/// the runtime trait name with the trait's method.
///
/// For example, the substrate runtime trait [Metadata](https://github.com/paritytech/substrate/blob/cb954820a8d8d765ce75021e244223a3b4d5722d/primitives/api/src/lib.rs#L745)
/// contains the `metadata_at_version` function. The corresponding runtime function
/// is `Metadata_metadata_at_version`.
///
/// - encoded arguments
///
/// Each argument of the runtime function must be scale-encoded.
pub trait Payload {
    /// The return type of the function call.
    // Note: `DecodeWithMetadata` is needed to decode the function call result
    // with the `subxt::Metadata.
    type ReturnType: DecodeWithMetadata;

    /// The runtime API trait name.
    fn trait_name(&self) -> &str;

    /// The runtime API method name.
    fn method_name(&self) -> &str;

    /// Scale encode the arguments data.
    fn encode_args_to(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error>;

    /// Encode arguments data and return the output. This is a convenience
    /// wrapper around [`Payload::encode_args_to`].
    fn encode_args(&self, metadata: &Metadata) -> Result<Vec<u8>, Error> {
        let mut v = Vec::new();
        self.encode_args_to(metadata, &mut v)?;
        Ok(v)
    }

    /// Returns the statically generated validation hash.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

/// A runtime API payload containing the generic argument data
/// and interpreting the result of the call as `ReturnTy`.
///
/// This can be created from static values (ie those generated
/// via the `subxt` macro) or dynamic values via [`dynamic`].
#[derive_where(Clone, Debug, Eq, Ord, PartialEq, PartialOrd; ArgsData)]
pub struct DefaultPayload<ArgsData, ReturnTy> {
    trait_name: Cow<'static, str>,
    method_name: Cow<'static, str>,
    args_data: ArgsData,
    validation_hash: Option<[u8; 32]>,
    _marker: PhantomData<ReturnTy>,
}

/// A statically generated runtime API payload.
pub type StaticPayload<ArgsData, ReturnTy> = DefaultPayload<ArgsData, ReturnTy>;
/// A dynamic runtime API payload.
pub type DynamicPayload = DefaultPayload<Composite<()>, DecodedValueThunk>;

impl<ArgsData: EncodeAsFields, ReturnTy: DecodeWithMetadata> Payload
    for DefaultPayload<ArgsData, ReturnTy>
{
    type ReturnType = ReturnTy;

    fn trait_name(&self) -> &str {
        &self.trait_name
    }

    fn method_name(&self) -> &str {
        &self.method_name
    }

    fn encode_args_to(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error> {
        let api_method = metadata
            .runtime_api_trait_by_name_err(&self.trait_name)?
            .method_by_name(&self.method_name)
            .ok_or_else(|| MetadataError::RuntimeMethodNotFound((*self.method_name).to_owned()))?;
        let mut fields = api_method
            .inputs()
            .map(|input| scale_encode::Field::named(input.ty, &input.name));

        self.args_data
            .encode_as_fields_to(&mut fields, metadata.types(), out)?;
        Ok(())
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.validation_hash
    }
}

impl<ReturnTy, ArgsData> DefaultPayload<ArgsData, ReturnTy> {
    /// Create a new [`DefaultPayload`].
    pub fn new(
        trait_name: impl Into<String>,
        method_name: impl Into<String>,
        args_data: ArgsData,
    ) -> Self {
        DefaultPayload {
            trait_name: Cow::Owned(trait_name.into()),
            method_name: Cow::Owned(method_name.into()),
            args_data,
            validation_hash: None,
            _marker: PhantomData,
        }
    }

    /// Create a new static [`DefaultPayload`] using static function name
    /// and scale-encoded argument data.
    ///
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        trait_name: &'static str,
        method_name: &'static str,
        args_data: ArgsData,
        hash: [u8; 32],
    ) -> DefaultPayload<ArgsData, ReturnTy> {
        DefaultPayload {
            trait_name: Cow::Borrowed(trait_name),
            method_name: Cow::Borrowed(method_name),
            args_data,
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

    /// Returns the arguments data.
    pub fn args_data(&self) -> &ArgsData {
        &self.args_data
    }
}

/// Create a new [`DynamicPayload`].
pub fn dynamic(
    trait_name: impl Into<String>,
    method_name: impl Into<String>,
    args_data: impl Into<Composite<()>>,
) -> DynamicPayload {
    DefaultPayload::new(trait_name, method_name, args_data.into())
}
