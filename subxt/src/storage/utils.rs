// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! these utility methods complement the [`StorageAddress`] trait, but
//! aren't things that should ever be overridden, and so don't exist on
//! the trait itself.

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
