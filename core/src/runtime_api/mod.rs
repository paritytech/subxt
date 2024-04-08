// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types and functions for constructing runtime API requests.

// TODO: Like with storage entries:
// - put payload stuff in separate module and re-export here.
// - expose functions for encoding the request and decoding the response.
// - add example of this at the top.

pub mod payload;

use payload::PayloadT;
use crate::error::{Error,MetadataError};
use crate::metadata::{Metadata,DecodeWithMetadata};

/// Run the validation logic against some runtime API payload you'd like to use. Returns `Ok(())`
/// if the payload is valid (or if it's not possible to check since the payload has no validation hash).
/// Return an error if the payload was not valid or something went wrong trying to validate it (ie
/// the runtime API in question do not exist at all)
pub fn validate<Payload: PayloadT>(metadata: &Metadata, payload: &Payload) -> Result<(), Error> {
    let Some(static_hash) = payload.validation_hash() else {
        return Ok(())
    };

    let api_trait = metadata.runtime_api_trait_by_name_err(payload.trait_name())?;

    let Some(runtime_hash) = api_trait.method_hash(payload.method_name()) else {
        return Err(MetadataError::IncompatibleCodegen.into());
    };
    if static_hash != runtime_hash {
        return Err(MetadataError::IncompatibleCodegen.into());
    }
    Ok(())
}

/// Return the name of the runtime API call from the payload.
pub fn call_name<Payload: PayloadT>(payload: &Payload) -> String {
    format!("{}_{}", payload.trait_name(), payload.method_name())
}

/// Return the encoded call args given a runtime API payload.
pub fn call_args<Payload: PayloadT>(metadata: &Metadata, payload: &Payload) -> Result<Vec<u8>, Error> {
    payload.encode_args(&metadata)
}

/// Decode the value bytes at the location given by the provided runtime API payload.
pub fn decode_value<Payload: PayloadT>(
    metadata: &Metadata,
    payload: &Payload,
    bytes: &mut &[u8],
) -> Result<Payload::ReturnType, Error> {
    let api_method = metadata
        .runtime_api_trait_by_name_err(payload.trait_name())?
        .method_by_name(payload.method_name())
        .ok_or_else(|| {
            MetadataError::RuntimeMethodNotFound(payload.method_name().to_owned())
        })?;

    let val = <Payload::ReturnType as DecodeWithMetadata>::decode_with_metadata(
        &mut &bytes[..],
        api_method.output_ty(),
        &metadata,
    )?;

    Ok(val)
}