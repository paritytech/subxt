// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! these utility methods complement the [`StorageAddress`] trait, but
//! aren't things that should ever be overridden, and so don't exist on
//! the trait itself.

use subxt_metadata::StorageHasher;

use super::StorageAddress;
use crate::{
    error::{Error, StorageAddressError},
    metadata::Metadata,
};

/// Return the root of a given [`StorageAddress`]: hash the pallet name and entry name
/// and append those bytes to the output.
pub(crate) fn write_storage_address_root_bytes<Address: StorageAddress>(
    addr: &Address,
    out: &mut Vec<u8>,
) {
    out.extend(sp_core_hashing::twox_128(addr.pallet_name().as_bytes()));
    out.extend(sp_core_hashing::twox_128(addr.entry_name().as_bytes()));
}

/// Outputs the [`storage_address_root_bytes`] as well as any additional bytes that represent
/// a lookup in a storage map at that location.
pub(crate) fn storage_address_bytes<Address: StorageAddress>(
    addr: &Address,
    metadata: &Metadata,
) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    write_storage_address_root_bytes(addr, &mut bytes);
    addr.append_entry_bytes(metadata, &mut bytes)?;
    Ok(bytes)
}

/// Outputs a vector containing the bytes written by [`write_storage_address_root_bytes`].
pub(crate) fn storage_address_root_bytes<Address: StorageAddress>(addr: &Address) -> Vec<u8> {
    let mut bytes = Vec::new();
    write_storage_address_root_bytes(addr, &mut bytes);
    bytes
}

/// Strips the first 16 bytes (8 for the pallet hash, 8 for the entry hash) off some storage address bytes.
pub(crate) fn strip_storage_addess_root_bytes(
    address_bytes: &mut &[u8],
) -> Result<(), StorageAddressError> {
    if address_bytes.len() >= 16 {
        *address_bytes = &mut &address_bytes[16..];
        Ok(())
    } else {
        Err(StorageAddressError::UnexpectedAddressBytes)
    }
}

/// Strips the first few bytes off a hash produced by a concat hasher.
/// Returns None(..) if the hasher provided is not a concat hasher.
/// Returns Some(Err(..)) if there are not enough bytes.
/// Returns Some(Ok(..)) if the stripping was successful.
pub fn strip_concat_hash_bytes(
    hash: &mut &[u8],
    hasher: &StorageHasher,
) -> Option<Result<(), StorageAddressError>> {
    match hasher {
        StorageHasher::Blake2_128Concat => {
            if hash.len() >= 16 {
                *hash = &mut &hash[16..];
                Some(Ok(()))
            } else {
                Some(Err(StorageAddressError::UnexpectedAddressBytes))
            }
        }
        StorageHasher::Twox64Concat => {
            if hash.len() >= 8 {
                *hash = &mut &hash[8..];
                Some(Ok(()))
            } else {
                Some(Err(StorageAddressError::UnexpectedAddressBytes))
            }
        }
        StorageHasher::Blake2_128
        | StorageHasher::Blake2_256
        | StorageHasher::Twox128
        | StorageHasher::Twox256
        | StorageHasher::Identity => None,
    }
}
