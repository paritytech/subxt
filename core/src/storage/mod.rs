// Copyright 2019-2024 Parity Technologies (UK) Ltd.
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

pub mod address;

use crate::{Metadata, error::StorageError};
use address::Address;
use alloc::vec::Vec;
use frame_decode::storage::StorageTypeInfo;
use scale_decode::IntoVisitor;

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

/// Given a storage address and some metadata, this encodes the address into bytes which can be
/// handed to a node to retrieve the corresponding value.
pub fn get_address_bytes<Addr: Address, Keys: EqualOrPrefixOf<Addr::KeyParts>>(
    address: &Addr,
    metadata: &Metadata,
    keys: Keys,
) -> Result<Vec<u8>, StorageError> {
    frame_decode::storage::encode_storage_key(
        address.pallet_name(),
        address.entry_name(),
        &keys,
        metadata,
        metadata.types(),
    )
    .map_err(|e| StorageError::StorageKeyEncodeError(e).into())
}

/// Given a storage address and some metadata, this encodes the root of the address (ie the pallet
/// and storage entry part) into bytes. If the entry being addressed is inside a map, this returns
/// the bytes needed to iterate over all of the entries within it.
pub fn get_address_root_bytes<Addr: Address>(address: &Addr) -> [u8; 32] {
    frame_decode::storage::encode_storage_key_prefix(
        address.pallet_name(), 
        address.entry_name()
    )
}

/// Given some storage value that we've retrieved from a node, the address used to retrieve it, and
/// metadata from the node, this function attempts to decode the bytes into the target value specified
/// by the address.
pub fn decode_value<Addr: Address>(
    bytes: &mut &[u8],
    address: &Addr,
    metadata: &Metadata,
) -> Result<Addr::Value, StorageError> {
    frame_decode::storage::decode_storage_value(
        address.pallet_name(),
        address.entry_name(),
        bytes,
        metadata,
        metadata.types(),
        Addr::Value::into_visitor(),
    )
    .map_err(|e| StorageError::StorageValueDecodeError(e).into())
}

/// Return the default value at a given storage address if one is available, or None otherwise.
pub fn default_value<Addr: Address>(
    address: &Addr,
    metadata: &Metadata,
) -> Result<Option<Addr::Value>, StorageError> {
    let storage_info = metadata
        .storage_info(address.pallet_name(), address.entry_name())
        .map_err(|e| StorageError::StorageInfoError(e.into_owned()))?;

    let value = frame_decode::storage::decode_default_storage_value_with_info(
        &storage_info,
        metadata.types(),
        Addr::Value::into_visitor(),
    )
    .map_err(|e| StorageError::StorageValueDecodeError(e))?;

    Ok(value)
}
