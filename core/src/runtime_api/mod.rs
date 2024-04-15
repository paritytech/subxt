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

use crate::error::{Error, MetadataError};
use crate::metadata::{DecodeWithMetadata, Metadata};
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use payload::PayloadT;

/// Run the validation logic against some runtime API payload you'd like to use. Returns `Ok(())`
/// if the payload is valid (or if it's not possible to check since the payload has no validation hash).
/// Return an error if the payload was not valid or something went wrong trying to validate it (ie
/// the runtime API in question do not exist at all)
pub fn validate<Payload: PayloadT>(payload: &Payload, metadata: &Metadata) -> Result<(), Error> {
    let Some(static_hash) = payload.validation_hash() else {
        return Ok(());
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
pub fn call_args<Payload: PayloadT>(
    payload: &Payload,
    metadata: &Metadata,
) -> Result<Vec<u8>, Error> {
    payload.encode_args(metadata)
}

/// Decode the value bytes at the location given by the provided runtime API payload.
pub fn decode_value<Payload: PayloadT>(
    bytes: &mut &[u8],
    payload: &Payload,
    metadata: &Metadata,
) -> Result<Payload::ReturnType, Error> {
    let api_method = metadata
        .runtime_api_trait_by_name_err(payload.trait_name())?
        .method_by_name(payload.method_name())
        .ok_or_else(|| MetadataError::RuntimeMethodNotFound(payload.method_name().to_owned()))?;

    let val = <Payload::ReturnType as DecodeWithMetadata>::decode_with_metadata(
        &mut &bytes[..],
        api_method.output_ty(),
        metadata,
    )?;

    Ok(val)
}
