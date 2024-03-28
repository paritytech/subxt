// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Functions and types associated with accessing constants.

mod constant_address;

pub use constant_address::{dynamic, Address, ConstantAddress, DynamicAddress};

use alloc::borrow::ToOwned;

use crate::{
    error::MetadataError,
    metadata::DecodeWithMetadata,
    Error, Metadata,
};

/// When the provided `address` is statically generated via the `#[subxt]` macro, this validates
/// that the shape of the constant value is the same as the shape expected by the static address.
///
/// When the provided `address` is dynamic (and thus does not come with any expectation of the
/// shape of the constant value), this just returns `Ok(())`
pub fn validate_constant<Address: ConstantAddress>(
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
pub fn get_constant<Address: ConstantAddress>(
    metadata: &Metadata,
    address: &Address,
) -> Result<Address::Target, Error> {
    // 1. Validate constant shape if hash given:
    validate_constant(metadata, address)?;

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
