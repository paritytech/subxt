// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! these utility methods complement the [`StorageAddress`] trait, but
//! aren't things that should ever be overridden, and so don't exist on
//! the trait itself.

use subxt_metadata::StorageHasher;

use super::StorageAddress;
use crate::{error::Error, metadata::Metadata};

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

/// Tries to recover an encoded value from a concat-style hash.
pub fn recover_value_from_hash<V: codec::Encode + codec::Decode>(
    hash: &[u8],
    hasher: &StorageHasher,
) -> Option<Result<V, Error>> {
    let value_bytes = value_bytes_from_hash_bytes(hash, hasher)?;
    let value = match V::decode(&mut &value_bytes[..]) {
        Ok(value) => value,
        Err(err) => return Some(Err(err.into())),
    };
    Some(Ok(value))
}

/// Tries to recover from the hash, the bytes of the value that was originially hashed.
/// Note: this only returns `Some(..)` for concat-style hashers.
fn value_bytes_from_hash_bytes<'a>(hash: &'a [u8], hasher: &StorageHasher) -> Option<&'a [u8]> {
    match hasher {
        StorageHasher::Blake2_128Concat => {
            if hash.len() > 16 {
                Some(&hash[16..])
            } else {
                None
            }
        }
        StorageHasher::Twox64Concat => {
            if hash.len() > 8 {
                Some(&hash[8..])
            } else {
                None
            }
        }
        StorageHasher::Blake2_128
        | StorageHasher::Blake2_256
        | StorageHasher::Twox128
        | StorageHasher::Twox256
        | StorageHasher::Identity => None,
    }
}
