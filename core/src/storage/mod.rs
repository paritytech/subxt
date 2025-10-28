// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Encode storage keys, decode storage values, and validate static storage addresses.
//!
//! # Example
//!
//! ```rust
//! use subxt_signer::sr25519::dev;
//! use subxt_macro::subxt;
//! use subxt_core::storage;
//! use subxt_core::Metadata;
//!
//! // If we generate types without `subxt`, we need to point to `::subxt_core`:
//! #[subxt(
//!     crate = "::subxt_core",
//!     runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale",
//! )]
//! pub mod polkadot {}
//!
//! // Some metadata we'll use to work with storage entries:
//! let metadata_bytes = include_bytes!("../../../artifacts/polkadot_metadata_small.scale");
//! let metadata = Metadata::decode_from(&metadata_bytes[..]).unwrap();
//!
//! // Build a storage query to access account information.
//! let address = polkadot::storage().system().account();
//!
//! // We can validate that the address is compatible with the given metadata.
//! storage::validate(&address, &metadata).unwrap();
//!
//! // We can fetch details about the storage entry associated with this address:
//! let entry = storage::entry(address, &metadata).unwrap();
//!
//! // .. including generating a key to fetch the entry with:
//! let fetch_key = entry.fetch_key((dev::alice().public_key().into(),)).unwrap();
//!
//! // .. or generating a key to iterate over entries with at a given depth:
//! let iter_key = entry.iter_key(()).unwrap();
//!
//! // Given a value, we can decode it:
//! let value_bytes = hex::decode("00000000000000000100000000000000000064a7b3b6e00d0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080").unwrap();
//! let value = entry.value(value_bytes).decode().unwrap();
//!
//! println!("Alice's account info: {value:?}");
//! ```

mod prefix_of;
mod storage_entry;
mod storage_key;
mod storage_key_value;
mod storage_value;

pub mod address;

use crate::{Metadata, error::StorageError};
use address::Address;
use alloc::string::ToString;

pub use prefix_of::{EqualOrPrefixOf, PrefixOf};
pub use storage_entry::{StorageEntry, entry};
pub use storage_key::{StorageHasher, StorageKey, StorageKeyPart};
pub use storage_key_value::StorageKeyValue;
pub use storage_value::StorageValue;

/// When the provided `address` is statically generated via the `#[subxt]` macro, this validates
/// that the shape of the storage value is the same as the shape expected by the static address.
///
/// When the provided `address` is dynamic (and thus does not come with any expectation of the
/// shape of the constant value), this just returns `Ok(())`
pub fn validate<Addr: Address>(address: Addr, metadata: &Metadata) -> Result<(), StorageError> {
    let Some(hash) = address.validation_hash() else {
        return Ok(());
    };

    let pallet_name = address.pallet_name();
    let entry_name = address.entry_name();

    let pallet_metadata = metadata
        .pallet_by_name(pallet_name)
        .ok_or_else(|| StorageError::PalletNameNotFound(pallet_name.to_string()))?;
    let storage_hash = pallet_metadata.storage_hash(entry_name).ok_or_else(|| {
        StorageError::StorageEntryNotFound {
            pallet_name: pallet_name.to_string(),
            entry_name: entry_name.to_string(),
        }
    })?;

    if storage_hash != hash {
        Err(StorageError::IncompatibleCodegen)
    } else {
        Ok(())
    }
}
