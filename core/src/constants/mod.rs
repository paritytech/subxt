// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing constants.

mod constant_address;

pub use constant_address::{dynamic, Address, ConstantAddress, DynamicAddress};

use alloc::borrow::ToOwned;

use crate::{
    metadata::{DecodeWithMetadata, MetadataExt},
    Error, Metadata, MetadataError,
};

/// Run validation logic against some constant address you'd like to access. Returns `Ok(())`
/// if the address is valid (or if it's not possible to check since the address has no validation hash).
/// Return an error if the address was not valid or something went wrong trying to validate it (ie
/// the pallet or constant in question do not exist at all).
pub fn validate_constant<Address: ConstantAddress>(
    metadata: &subxt_metadata::Metadata,
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
