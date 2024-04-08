// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Access constants from metadata.
//!
//! Use [`get`] to retrieve a constant from some metadata, or [`validate`] to check that a static
//! constant address lines up with the value seen in the metadata.
//!
//! # Example
//!
//! ```rust
//! use subxt_macro::subxt;
//! use subxt_core::constants;
//! use subxt_core::metadata;
//!
//! // If we generate types without `subxt`, we need to point to `::subxt_core`:
//! #[subxt(
//!     crate = "::subxt_core",
//!     runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale",
//! )]
//! pub mod polkadot {}
//!
//! // Some metadata we'd like to access constants in:
//! let metadata_bytes = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");
//! let metadata = metadata::decode_from(&metadata_bytes[..]).unwrap();
//!
//! // We can use a static address to obtain some constant:
//! let address = polkadot::constants().balances().existential_deposit();
//!
//! // This validates that the address given is in line with the metadata
//! // we're trying to access the constant in:
//! constants::validate(&metadata, &address).expect("is valid");
//!
//! // This acquires the constant (and internally also validates it):
//! let ed = constants::get(&metadata, &address).expect("can decode constant");
//!
//! assert_eq!(ed, 33_333_333);
//! ```

pub mod address;

use address::AddressT;
use alloc::borrow::ToOwned;

use crate::{error::MetadataError, metadata::DecodeWithMetadata, Error, Metadata};

/// When the provided `address` is statically generated via the `#[subxt]` macro, this validates
/// that the shape of the constant value is the same as the shape expected by the static address.
///
/// When the provided `address` is dynamic (and thus does not come with any expectation of the
/// shape of the constant value), this just returns `Ok(())`
pub fn validate<Address: AddressT>(
    metadata: &Metadata,
    address: &Address,
) -> Result<(), Error> {
    if let Some(actual_hash) = address.validation_hash() {
        let expected_hash = metadata
            .pallet_by_name_err(address.pallet_name())?
            .constant_hash(address.constant_name())
            .ok_or_else(|| {
                MetadataError::ConstantNameNotFound(address.constant_name().to_owned())
            })?;
        if actual_hash != expected_hash {
            return Err(MetadataError::IncompatibleCodegen.into());
        }
    }
    Ok(())
}

/// Fetch a constant out of the metadata given a constant address. If the `address` has been
/// statically generated, this will validate that the constant shape is as expected, too.
pub fn get<Address: AddressT>(
    metadata: &Metadata,
    address: &Address,
) -> Result<Address::Target, Error> {
    // 1. Validate constant shape if hash given:
    validate(metadata, address)?;

    // 2. Attempt to decode the constant into the type given:
    let constant = metadata
        .pallet_by_name_err(address.pallet_name())?
        .constant_by_name(address.constant_name())
        .ok_or_else(|| MetadataError::ConstantNameNotFound(address.constant_name().to_owned()))?;
    let value = <Address::Target as DecodeWithMetadata>::decode_with_metadata(
        &mut constant.value(),
        constant.ty(),
        metadata,
    )?;
    Ok(value)
}
