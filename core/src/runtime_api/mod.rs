// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Encode runtime API payloads, decode the associated values returned from them, and validate
//! static runtime API payloads.
//!
//! # Example
//!
//! ```rust
//! use subxt_macro::subxt;
//! use subxt_core::runtime_api;
//! use subxt_core::metadata;
//!
//! // If we generate types without `subxt`, we need to point to `::subxt_core`:
//! #[subxt(
//!     crate = "::subxt_core",
//!     runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale",
//! )]
//! pub mod polkadot {}
//!
//! // Some metadata we'll use to work with storage entries:
//! let metadata_bytes = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");
//! let metadata = metadata::decode_from(&metadata_bytes[..]).unwrap();
//!
//! // Build a storage query to access account information.
//! let payload = polkadot::apis().metadata().metadata_versions();
//!
//! // We can validate that the payload is compatible with the given metadata.
//! runtime_api::validate(&payload, &metadata).unwrap();
//!
//! // Encode the payload name and arguments to hand to a node:
//! let _call_name = runtime_api::call_name(&payload);
//! let _call_args = runtime_api::call_args(&payload, &metadata).unwrap();
//!
//! // If we were to obtain a value back from the node, we could
//! // then decode it using the same payload and metadata like so:
//! let value_bytes = hex::decode("080e0000000f000000").unwrap();
//! let value = runtime_api::decode_value(&mut &*value_bytes, &payload, &metadata).unwrap();
//!
//! println!("Available metadata versions: {value:?}");
//! ```

pub mod payload;

use crate::error::RuntimeApiError;
use crate::Metadata;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use payload::Payload;
use scale_decode::IntoVisitor;

/// Run the validation logic against some runtime API payload you'd like to use. Returns `Ok(())`
/// if the payload is valid (or if it's not possible to check since the payload has no validation hash).
/// Return an error if the payload was not valid or something went wrong trying to validate it (ie
/// the runtime API in question do not exist at all)
pub fn validate<P: Payload>(payload: &P, metadata: &Metadata) -> Result<(), RuntimeApiError> {
    let Some(hash) = payload.validation_hash() else {
        return Ok(());
    };

    let trait_name = payload.trait_name();
    let method_name = payload.method_name();

    let api_trait = metadata.runtime_api_trait_by_name(trait_name)
        .ok_or_else(|| RuntimeApiError::TraitNotFound(trait_name.to_string()))?;
    let api_method = api_trait.method_by_name(method_name)
        .ok_or_else(|| RuntimeApiError::MethodNotFound { 
            trait_name: trait_name.to_string(), 
            method_name: method_name.to_string() 
        })?;

    if hash != api_method.hash() {
        Err(RuntimeApiError::IncompatibleCodegen)
    } else {
        Ok(())
    }
}

/// Return the name of the runtime API call from the payload.
pub fn call_name<P: Payload>(payload: &P) -> String {
    format!("{}_{}", payload.trait_name(), payload.method_name())
}

/// Return the encoded call args given a runtime API payload.
pub fn call_args<P: Payload>(payload: &P, metadata: &Metadata) -> Result<Vec<u8>, RuntimeApiError> {
    let value = frame_decode::runtime_apis::encode_runtime_api_inputs(
        payload.trait_name(),
        payload.method_name(),
        payload.args(),
        metadata,
        metadata.types()
    ).map_err(RuntimeApiError::CouldNotEncodeInputs)?;

    Ok(value)
}

/// Decode the value bytes at the location given by the provided runtime API payload.
pub fn decode_value<P: Payload>(
    bytes: &mut &[u8],
    payload: &P,
    metadata: &Metadata,
) -> Result<P::ReturnType, RuntimeApiError> {
    let value = frame_decode::runtime_apis::decode_runtime_api_response(
        payload.trait_name(),
        payload.method_name(),
        bytes,
        metadata,
        metadata.types(),
        P::ReturnType::into_visitor()
    ).map_err(RuntimeApiError::CouldNotDecodeResponse)?;

    Ok(value)
}
