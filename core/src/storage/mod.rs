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
//! use subxt_core::metadata;
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
//! let metadata = metadata::decode_from(&metadata_bytes[..]).unwrap();
//!
//! // Build a storage query to access account information.
//! let account = dev::alice().public_key().into();
//! let address = polkadot::storage().system().account(account);
//!
//! // We can validate that the address is compatible with the given metadata.
//! storage::validate(&address, &metadata).unwrap();
//!
//! // Encode the address to bytes. These can be sent to a node to query the value.
//! storage::get_address_bytes(&address, &metadata).unwrap();
//!
//! // If we were to obtain a value back from the node at that address, we could
//! // then decode it using the same address and metadata like so:
//! let value_bytes = hex::decode("00000000000000000100000000000000000064a7b3b6e00d0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080").unwrap();
//! let value = storage::decode_value(&mut &*value_bytes, &address, &metadata).unwrap();
//!
//! println!("Alice's account info: {value:?}");
//! ```

mod prefix_of;
mod storage_entry;
mod storage_value;
mod storage_key;
mod storage_key_value;

pub mod address;

use crate::{Metadata, error::StorageError};
use address::Address;

pub use storage_key::{key, StorageKey, StorageKeyPart, StorageHasher};
pub use storage_entry::{entry, StorageEntry};
pub use storage_value::{value, StorageValue};
pub use storage_key_value::{key_value, StorageKeyValue};
pub use prefix_of::{ EqualOrPrefixOf, PrefixOf };

/// When the provided `address` is statically generated via the `#[subxt]` macro, this validates
/// that the shape of the storage value is the same as the shape expected by the static address.
///
/// When the provided `address` is dynamic (and thus does not come with any expectation of the
/// shape of the constant value), this just returns `Ok(())`
pub fn validate<Addr: Address>(address: &Addr, metadata: &Metadata) -> Result<(), StorageError> {
    let Some(hash) = address.validation_hash() else {
        return Ok(());
    };

    let pallet_name = address.pallet_name();
    let entry_name = address.entry_name();

    let pallet_metadata = metadata.pallet_by_name(pallet_name)
        .ok_or_else(|| StorageError::PalletNameNotFound(pallet_name.to_string()))?;
    let storage_hash = pallet_metadata.storage_hash(entry_name)
        .ok_or_else(|| StorageError::StorageEntryNotFound {
            pallet_name: pallet_name.to_string(),
            entry_name: entry_name.to_string(),
        })?;

    if storage_hash != hash {
        Err(StorageError::IncompatibleCodegen)
    } else {
        Ok(())
    }
}
