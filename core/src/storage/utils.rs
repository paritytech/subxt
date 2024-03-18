// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! these utility methods complement the [`StorageAddress`] trait, but
//! aren't things that should ever be overridden, and so don't exist on
//! the trait itself.

use crate::error::MetadataError;
use crate::metadata::{DecodeWithMetadata, MetadataExt};
use alloc::vec::Vec;
use subxt_metadata::PalletMetadata;
use subxt_metadata::{StorageEntryMetadata, StorageHasher};

use super::StorageAddress;
use crate::{error::Error, metadata::Metadata};
use alloc::borrow::ToOwned;

/// Return the root of a given [`StorageAddress`]: hash the pallet name and entry name
/// and append those bytes to the output.
pub fn write_storage_address_root_bytes<Address: StorageAddress>(
    addr: &Address,
    out: &mut Vec<u8>,
) {
    out.extend(sp_core_hashing::twox_128(addr.pallet_name().as_bytes()));
    out.extend(sp_core_hashing::twox_128(addr.entry_name().as_bytes()));
}

/// Outputs the [`storage_address_root_bytes`] as well as any additional bytes that represent
/// a lookup in a storage map at that location.
pub fn storage_address_bytes<Address: StorageAddress>(
    addr: &Address,
    metadata: &Metadata,
) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    write_storage_address_root_bytes(addr, &mut bytes);
    addr.append_entry_bytes(metadata, &mut bytes)?;
    Ok(bytes)
}

/// Outputs a vector containing the bytes written by [`write_storage_address_root_bytes`].
pub fn storage_address_root_bytes<Address: StorageAddress>(addr: &Address) -> Vec<u8> {
    let mut bytes = Vec::new();
    write_storage_address_root_bytes(addr, &mut bytes);
    bytes
}

/// Take some SCALE encoded bytes and a [`StorageHasher`] and hash the bytes accordingly.
pub fn hash_bytes(input: &[u8], hasher: StorageHasher, bytes: &mut Vec<u8>) {
    match hasher {
        StorageHasher::Identity => bytes.extend(input),
        StorageHasher::Blake2_128 => bytes.extend(sp_core_hashing::blake2_128(input)),
        StorageHasher::Blake2_128Concat => {
            bytes.extend(sp_core_hashing::blake2_128(input));
            bytes.extend(input);
        }
        StorageHasher::Blake2_256 => bytes.extend(sp_core_hashing::blake2_256(input)),
        StorageHasher::Twox128 => bytes.extend(sp_core_hashing::twox_128(input)),
        StorageHasher::Twox256 => bytes.extend(sp_core_hashing::twox_256(input)),
        StorageHasher::Twox64Concat => {
            bytes.extend(sp_core_hashing::twox_64(input));
            bytes.extend(input);
        }
    }
}

/// Return details about the given storage entry.
pub fn lookup_entry_details<'a>(
    pallet_name: &str,
    entry_name: &str,
    metadata: &'a subxt_metadata::Metadata,
) -> Result<(PalletMetadata<'a>, &'a StorageEntryMetadata), Error> {
    let pallet_metadata = metadata.pallet_by_name_err(pallet_name)?;
    let storage_metadata = pallet_metadata
        .storage()
        .ok_or_else(|| MetadataError::StorageNotFoundInPallet(pallet_name.to_owned()))?;
    let storage_entry = storage_metadata
        .entry_by_name(entry_name)
        .ok_or_else(|| MetadataError::StorageEntryNotFound(entry_name.to_owned()))?;
    Ok((pallet_metadata, storage_entry))
}

/// Validate a storage address against the metadata.
pub fn validate_storage_address<Address: StorageAddress>(
    address: &Address,
    pallet: PalletMetadata<'_>,
) -> Result<(), Error> {
    if let Some(hash) = address.validation_hash() {
        validate_storage(pallet, address.entry_name(), hash)?;
    }
    Ok(())
}

/// Validate a storage entry against the metadata.
fn validate_storage(
    pallet: PalletMetadata<'_>,
    storage_name: &str,
    hash: [u8; 32],
) -> Result<(), Error> {
    let Some(expected_hash) = pallet.storage_hash(storage_name) else {
        return Err(MetadataError::IncompatibleCodegen.into());
    };
    if expected_hash != hash {
        return Err(MetadataError::IncompatibleCodegen.into());
    }
    Ok(())
}

/// Given some bytes, a pallet and storage name, decode the response.
pub fn decode_storage_with_metadata<T: DecodeWithMetadata>(
    bytes: &mut &[u8],
    metadata: &Metadata,
    storage_metadata: &StorageEntryMetadata,
) -> Result<T, Error> {
    let return_ty = storage_metadata.entry_type().value_ty();
    let val = T::decode_with_metadata(bytes, return_ty, metadata)?;
    Ok(val)
}
