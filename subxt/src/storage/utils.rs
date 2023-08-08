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

/// this should match the constants in the module `sp_core::storage::well_known_keys`
pub mod well_known_keys {
    use std::fmt::Formatter;
    use std::marker::PhantomData;

    /// Wasm code of the runtime.
    pub const CODE: WellKnownKey<Vec<u8>> = key(b":code");

    /// Number of wasm linear memory pages required for execution of the runtime.
    // note: currently untested, does not seem to work on polkadot
    const _HEAP_PAGES: WellKnownKey<u64> = key(b":heappages");

    /// Current extrinsic index (u32) is stored under this key.
    // note: currently untested, does not seem to work on polkadot
    const _EXTRINSIC_INDEX: WellKnownKey<u32> = key(b":extrinsic_index");

    /// Current intra-block entropy (a universally unique `[u8; 32]` value) is stored here.
    // note: currently untested, does not seem to work on polkadot
    const _INTRABLOCK_ENTROPY: WellKnownKey<[u8; 32]> = key(b":intrablock_entropy");

    const fn key<R>(key: &'static [u8]) -> WellKnownKey<R> {
        WellKnownKey {
            key,
            _phantom: PhantomData,
        }
    }

    /// a static key that every substrate node should contain a storage entry for.
    /// The static key bytes can be given to the `fetch_raw()` storage call directly, no hashing needed.
    /// The `R` type parameter is the return type of the storage entry. Should implement `Decode`.
    pub struct WellKnownKey<R> {
        key: &'static [u8],
        _phantom: PhantomData<R>,
    }
    impl<R> WellKnownKey<R> {
        /// the inner key (some static bytes)
        pub fn key(&self) -> &'static [u8] {
            self.key
        }
    }
    impl<R> std::fmt::Display for WellKnownKey<R> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(std::str::from_utf8(self.key).expect("only defined here in crate; qed"))
        }
    }
}
