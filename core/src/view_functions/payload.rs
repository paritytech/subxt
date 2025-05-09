// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains the trait and types used to represent
//! View Function calls that can be made.

use alloc::vec::Vec;
use core::marker::PhantomData;
use derive_where::derive_where;
use scale_encode::EncodeAsFields;
use scale_value::Composite;

use crate::Error;
use crate::dynamic::DecodedValueThunk;
use crate::error::MetadataError;

use crate::metadata::{DecodeWithMetadata, Metadata};

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
    /// The return type of the function call.
    // Note: `DecodeWithMetadata` is needed to decode the function call result
    // with the `subxt::Metadata.
    type ReturnType: DecodeWithMetadata;

    /// The payload target.
    fn query_id(&self) -> &[u8; 32];

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

/// A View Function payload containing the generic argument data
/// and interpreting the result of the call as `ReturnTy`.
///
/// This can be created from static values (ie those generated
/// via the `subxt` macro) or dynamic values via [`dynamic`].
#[derive_where(Clone, Debug, Eq, Ord, PartialEq, PartialOrd; ArgsData)]
pub struct DefaultPayload<ArgsData, ReturnTy> {
    query_id: [u8; 32],
    args_data: ArgsData,
    validation_hash: Option<[u8; 32]>,
    _marker: PhantomData<ReturnTy>,
}

/// A statically generated View Function payload.
pub type StaticPayload<ArgsData, ReturnTy> = DefaultPayload<ArgsData, ReturnTy>;
/// A dynamic View Function payload.
pub type DynamicPayload = DefaultPayload<Composite<()>, DecodedValueThunk>;

impl<ArgsData: EncodeAsFields, ReturnTy: DecodeWithMetadata> Payload
    for DefaultPayload<ArgsData, ReturnTy>
{
    type ReturnType = ReturnTy;

    fn query_id(&self) -> &[u8; 32] {
        &self.query_id
    }

    fn encode_args_to(&self, metadata: &Metadata, out: &mut Vec<u8>) -> Result<(), Error> {
        let view_function = metadata
            .view_function_by_query_id(&self.query_id)
            .ok_or(MetadataError::ViewFunctionNotFound(self.query_id))?;
        let mut fields = view_function
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
    /// Create a new [`DefaultPayload`] for a View Function call.
    pub fn new(query_id: [u8; 32], args_data: ArgsData) -> Self {
        DefaultPayload {
            query_id,
            args_data,
            validation_hash: None,
            _marker: PhantomData,
        }
    }

    /// Create a new static [`DefaultPayload`] for a View Function call
    /// using static function name and scale-encoded argument data.
    ///
    /// This is only expected to be used from codegen.
    #[doc(hidden)]
    pub fn new_static(
        query_id: [u8; 32],
        args_data: ArgsData,
        hash: [u8; 32],
    ) -> DefaultPayload<ArgsData, ReturnTy> {
        DefaultPayload {
            query_id,
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

    /// Returns the arguments data.
    pub fn args_data(&self) -> &ArgsData {
        &self.args_data
    }
}

/// Create a new [`DynamicPayload`] to call a View Function.
pub fn dynamic(query_id: [u8; 32], args_data: impl Into<Composite<()>>) -> DynamicPayload {
    DefaultPayload::new(query_id, args_data.into())
}
