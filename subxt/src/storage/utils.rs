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
        *address_bytes = &address_bytes[16..];
        Ok(())
    } else {
        Err(StorageAddressError::UnexpectedAddressBytes)
    }
}

/// Strips the first few bytes off a hash to possibly skip to the plan key value,
/// if [`hash_contains_unhashed_value()`] for this StorageHasher.
///
/// Returns `Err(..)` if there are not enough bytes.
/// Returns `Ok(())` otherwise
pub fn strip_storage_hash_bytes(
    hash: &mut &[u8],
    hasher: &StorageHasher,
) -> Result<(), StorageAddressError> {
    let bytes_to_strip = hasher.hash_bytes_before_unhashed_key();
    if hash.len() < bytes_to_strip {
        return Err(StorageAddressError::UnexpectedAddressBytes);
    }
    *hash = &hash[bytes_to_strip..];
    Ok(())
}

/// This value is contained within the hash for concat-stle hashers
/// ([`StorageHasher::Identity`] or [`StorageHasher::Identity`]) and the
/// identity hash function ([`StorageHasher::Identity`]).
pub fn hash_contains_unhashed_value(hasher: &StorageHasher) -> bool {
    matches!(
        hasher,
        StorageHasher::Blake2_128Concat | StorageHasher::Twox64Concat | StorageHasher::Identity
    )
}
