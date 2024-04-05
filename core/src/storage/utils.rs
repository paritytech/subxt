// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! these utility methods complement the [`StorageAddress`] trait, but
//! aren't things that should ever be overridden, and so don't exist on
//! the trait itself.

use alloc::vec::Vec;
use subxt_metadata::{StorageHasher, PalletMetadata, StorageEntryMetadata};
use crate::error::{Error, MetadataError};
use crate::metadata::Metadata;

use super::StorageAddress;

/// Return the root of a given [`StorageAddress`]: hash the pallet name and entry name
/// and append those bytes to the output.
pub fn write_storage_address_root_bytes<Address: StorageAddress>(
    addr: &Address,
    out: &mut Vec<u8>,
) {
    out.extend(sp_crypto_hashing::twox_128(addr.pallet_name().as_bytes()));
    out.extend(sp_crypto_hashing::twox_128(addr.entry_name().as_bytes()));
}

/// Take some SCALE encoded bytes and a [`StorageHasher`] and hash the bytes accordingly.
pub fn hash_bytes(input: &[u8], hasher: StorageHasher, bytes: &mut Vec<u8>) {
    match hasher {
        StorageHasher::Identity => bytes.extend(input),
        StorageHasher::Blake2_128 => bytes.extend(sp_crypto_hashing::blake2_128(input)),
        StorageHasher::Blake2_128Concat => {
            bytes.extend(sp_crypto_hashing::blake2_128(input));
            bytes.extend(input);
        }
        StorageHasher::Blake2_256 => bytes.extend(sp_crypto_hashing::blake2_256(input)),
        StorageHasher::Twox128 => bytes.extend(sp_crypto_hashing::twox_128(input)),
        StorageHasher::Twox256 => bytes.extend(sp_crypto_hashing::twox_256(input)),
        StorageHasher::Twox64Concat => {
            bytes.extend(sp_crypto_hashing::twox_64(input));
            bytes.extend(input);
        }
    }
}

/// Return details about the given storage entry.
pub fn lookup_storage_entry_details<'a>(
    pallet_name: &str,
    entry_name: &str,
    metadata: &'a Metadata,
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