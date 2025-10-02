// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Encode View Function payloads, decode the associated values returned from them, and validate
//! static View Function payloads.

pub mod payload;

use crate::error::ViewFunctionError;
use crate::Metadata;
use alloc::vec::Vec;
use payload::Payload;
use scale_decode::IntoVisitor;

/// Run the validation logic against some View Function payload you'd like to use. Returns `Ok(())`
/// if the payload is valid (or if it's not possible to check since the payload has no validation hash).
/// Return an error if the payload was not valid or something went wrong trying to validate it (ie
/// the View Function in question do not exist at all)
pub fn validate<P: Payload>(payload: &P, metadata: &Metadata) -> Result<(), ViewFunctionError> {
    let Some(hash) = payload.validation_hash() else {
        return Ok(());
    };

    let pallet_name = payload.pallet_name();
    let function_name = payload.function_name();

    let view_function = metadata.pallet_by_name(pallet_name)
        .ok_or_else(|| ViewFunctionError::PalletNotFound(pallet_name.to_string()))?
        .view_function_by_name(function_name)
        .ok_or_else(|| ViewFunctionError::ViewFunctionNotFound {
            pallet_name: pallet_name.to_string(),
            function_name: function_name.to_string()
        })?;

    if hash != view_function.hash() {
        Err(ViewFunctionError::IncompatibleCodegen)
    } else {
        Ok(())
    }
}

/// The name of the Runtime API call which can execute
pub const CALL_NAME: &str = "RuntimeViewFunction_execute_view_function";

/// Encode the bytes that will be passed to the "execute_view_function" Runtime API call,
/// to execute the View Function represented by the given payload.
pub fn call_args<P: Payload>(payload: &P, metadata: &Metadata) -> Result<Vec<u8>, ViewFunctionError> {
    let inputs = frame_decode::view_functions::encode_view_function_inputs(
        payload.pallet_name(),
        payload.function_name(),
        payload.args(),
        metadata,
        metadata.types()
    ).map_err(ViewFunctionError::CouldNotEncodeInputs)?;

    Ok(inputs)
}

/// Decode the value bytes at the location given by the provided View Function payload.
pub fn decode_value<P: Payload>(
    bytes: &mut &[u8],
    payload: &P,
    metadata: &Metadata,
) -> Result<P::ReturnType, ViewFunctionError> {
    let value = frame_decode::view_functions::decode_view_function_response(
        payload.pallet_name(),
        payload.function_name(),
        bytes,
        metadata,
        metadata.types(),
        P::ReturnType::into_visitor()
    ).map_err(ViewFunctionError::CouldNotDecodeResponse)?;

    Ok(value)
}
