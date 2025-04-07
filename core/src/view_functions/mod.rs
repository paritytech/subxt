// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Encode View Function payloads, decode the associated values returned from them, and validate
//! static View Function payloads.

pub mod payload;

use crate::error::{Error, MetadataError};
use crate::metadata::{DecodeWithMetadata, Metadata};
use alloc::vec::Vec;
use payload::Payload;

/// Run the validation logic against some View Function payload you'd like to use. Returns `Ok(())`
/// if the payload is valid (or if it's not possible to check since the payload has no validation hash).
/// Return an error if the payload was not valid or something went wrong trying to validate it (ie
/// the View Function in question do not exist at all)
pub fn validate<P: Payload>(payload: &P, metadata: &Metadata) -> Result<(), Error> {
    let Some(static_hash) = payload.validation_hash() else {
        return Ok(());
    };

    let view_function = metadata
        .view_function_by_query_id(payload.query_id())
        .ok_or_else(|| MetadataError::ViewFunctionNotFound(*payload.query_id()))?;
    if static_hash != view_function.hash() {
        return Err(MetadataError::IncompatibleCodegen.into());
    }

    Ok(())
}

/// Return the name of the Runtime API call which can execute
pub fn call_name() -> &'static str {
    "RuntimeViewFunction_execute_view_function"
}

/// Encode the bytes that will be passed to the "execute_view_function" Runtime API call,
/// to execute the View Function represented by the given payload.
pub fn call_args<P: Payload>(payload: &P, metadata: &Metadata) -> Result<Vec<u8>, Error> {
    let mut call_args = Vec::with_capacity(32);
    call_args.extend_from_slice(payload.query_id());

    let mut call_arg_params = vec![];
    payload.encode_args_to(metadata, &mut call_arg_params)?;

    use codec::Encode;
    call_arg_params.encode_to(&mut call_args);

    Ok(call_args)
}

/// Decode the value bytes at the location given by the provided View Function payload.
pub fn decode_value<P: Payload>(
    bytes: &mut &[u8],
    payload: &P,
    metadata: &Metadata,
) -> Result<P::ReturnType, Error> {
    let view_function = metadata
        .view_function_by_query_id(payload.query_id())
        .ok_or_else(|| MetadataError::ViewFunctionNotFound(*payload.query_id()))?;

    let val = <P::ReturnType as DecodeWithMetadata>::decode_with_metadata(
        &mut &bytes[..],
        view_function.output_ty(),
        metadata,
    )?;

    Ok(val)
}
