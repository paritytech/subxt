// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::Decode;
use core::marker::PhantomData;
use scale_encode::EncodeAsFields;
use scale_value::Composite;
use std::borrow::Cow;

use crate::dynamic::DecodedValueThunk;
use crate::{metadata::DecodeWithMetadata, Error, Metadata};

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
pub trait RuntimeApiPayload {
    /// The return type of the function call.
    // Note: `DecodeWithMetadata` is needed to decode the function call result
    // with the `subxt::Metadata.
    type ReturnType: DecodeWithMetadata;

    /// The runtime API function name.
    fn fn_name(&self) -> &str;

    /// Scale encode the arguments data.
    fn encode_args_to(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error>;

    /// Encode arguments data and return the output. This is a convenience
    /// wrapper around [`RuntimeApiPayload::encode_args_to`].
    fn encode_args(&self, metadata: &Metadata) -> Result<Vec<u8>, Error> {
        let mut v = Vec::new();
        self.encode_args_to(metadata, &mut v)?;
        Ok(v)
    }

    /// Returns the details needed to validate the runtime API call, which
    /// include a statically generated validation hash.
    fn validation_details(&self) -> Option<ValidationDetails> {
        None
    }
}

pub struct ValidationDetails {
    /// A hash (this is generated at compile time in our codegen)
    /// to compare against the runtime code.
    pub hash: [u8; 32],
}

/// A runtime API payload containing the generic argument data
/// and interpreting the result of the call as `ReturnTy`.
///
/// This can be created from static values (ie those generated
/// via the `subxt` macro) or dynamic values via [`dynamic`].
#[derive(Clone, Debug)]
pub struct Payload<ArgsData, ReturnTy> {
    fn_name: Cow<'static, str>,
    args_data: ArgsData,
    validation_hash: Option<[u8; 32]>,
    _marker: PhantomData<ReturnTy>,
}

impl<ReturnTy: DecodeWithMetadata> RuntimeApiPayload for StaticRuntimeApiPayload<ReturnTy> {
    type ReturnType = ReturnTy;

    fn fn_name(&self) -> &str {
        &self.fn_name
    }

    fn encode_args_to(&self, _metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error> {
        out.extend(self.args_data.clone());
        Ok(())
    }

    fn validation_details(&self) -> Option<ValidationDetails> {
        self.validation_hash.map(|hash| ValidationDetails { hash })
    }
}

impl RuntimeApiPayload for DynamicRuntimeApiPayload {
    type ReturnType = DecodedValueThunk;

    fn fn_name(&self) -> &str {
        &self.fn_name
    }

    fn encode_args_to(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error> {
        let fn_metadata = metadata.runtime_fn(&self.fn_name)?;

        self.args_data
            .encode_as_fields_to(fn_metadata.fields(), metadata.types(), out)?;

        Ok(())
    }
}

/// A static runtime API payload.
///
/// The payload already contains the scale-encoded argument bytes.
///
/// This is only expected to be used from codegen.
pub type StaticRuntimeApiPayload<ReturnTy> = Payload<Vec<u8>, ReturnTy>;

/// A dynamic runtime API payload.
pub type DynamicRuntimeApiPayload = Payload<Composite<()>, DecodedValueThunk>;

impl<ReturnTy: Decode, ArgsData> Payload<ArgsData, ReturnTy> {
    /// Create a new [`Payload`].
    pub fn new(
        fn_name: impl Into<String>,
        args_data: ArgsData,
        validation_hash: Option<[u8; 32]>,
    ) -> Self {
        Payload {
            fn_name: Cow::Owned(fn_name.into()),
            args_data,
            validation_hash,
            _marker: PhantomData,
        }
    }

    /// Do not validate this call prior to submitting it.
    pub fn unvalidated(self) -> Self {
        Self {
            validation_hash: None,
            ..self
        }
    }

    /// Returns the function name.
    pub fn fn_name(&self) -> &str {
        &self.fn_name
    }

    /// Returns the arguments data.
    pub fn args_data(&self) -> &ArgsData {
        &self.args_data
    }
}

impl<ReturnTy> StaticRuntimeApiPayload<ReturnTy> {
    /// Create a new [`StaticRuntimeApiPayload`] using static function name
    /// and scale-encoded argument data.
    ///
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(fn_name: &'static str, args_data: Vec<u8>, hash: [u8; 32]) -> Self {
        Self {
            fn_name: Cow::Borrowed(fn_name),
            args_data,
            validation_hash: Some(hash),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Create a new [`DynamicRuntimeApiPayload`].
pub fn dynamic(
    fn_name: impl Into<String>,
    args_data: impl Into<Composite<()>>,
    hash: Option<[u8; 32]>,
) -> DynamicRuntimeApiPayload {
    DynamicRuntimeApiPayload {
        fn_name: Cow::Owned(fn_name.into()),
        args_data: args_data.into(),
        validation_hash: hash,
        _marker: std::marker::PhantomData,
    }
}
