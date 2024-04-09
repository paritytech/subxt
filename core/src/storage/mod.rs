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
//! let address = polkadot::storage().system().account(&account);
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

mod storage_key;
mod utils;

pub mod address;

use crate::{error::MetadataError, metadata::DecodeWithMetadata, Error, Metadata};
use address::AddressT;
use alloc::vec::Vec;

// This isn't a part of the public API, but expose here because it's useful in Subxt.
#[doc(hidden)]
pub use utils::lookup_storage_entry_details;

/// When the provided `address` is statically generated via the `#[subxt]` macro, this validates
/// that the shape of the storage value is the same as the shape expected by the static address.
///
/// When the provided `address` is dynamic (and thus does not come with any expectation of the
/// shape of the constant value), this just returns `Ok(())`
pub fn validate<Address: AddressT>(address: &Address, metadata: &Metadata) -> Result<(), Error> {
    let Some(hash) = address.validation_hash() else {
        return Ok(());
    };

    let pallet_name = address.pallet_name();
    let entry_name = address.entry_name();

    let pallet_metadata = metadata.pallet_by_name_err(pallet_name)?;

    let Some(expected_hash) = pallet_metadata.storage_hash(entry_name) else {
        return Err(MetadataError::IncompatibleCodegen.into());
    };
    if expected_hash != hash {
        return Err(MetadataError::IncompatibleCodegen.into());
    }
    Ok(())
}

/// Given a storage address and some metadata, this encodes the address into bytes which can be
/// handed to a node to retrieve the corresponding value.
pub fn get_address_bytes<Address: AddressT>(
    address: &Address,
    metadata: &Metadata,
) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    utils::write_storage_address_root_bytes(address, &mut bytes);
    address.append_entry_bytes(metadata, &mut bytes)?;
    Ok(bytes)
}

/// Given a storage address and some metadata, this encodes the root of the address (ie the pallet
/// and storage entry part) into bytes. If the entry being addressed is inside a map, this returns
/// the bytes needed to iterate over all of the entries within it.
pub fn get_address_root_bytes<Address: AddressT>(address: &Address) -> Vec<u8> {
    let mut bytes = Vec::new();
    utils::write_storage_address_root_bytes(address, &mut bytes);
    bytes
}

/// Given some storage value that we've retrieved from a node, the address used to retrieve it, and
/// metadata from the node, this function attempts to decode the bytes into the target value specified
/// by the address.
pub fn decode_value<Address: AddressT>(
    bytes: &mut &[u8],
    address: &Address,
    metadata: &Metadata,
) -> Result<Address::Target, Error> {
    let pallet_name = address.pallet_name();
    let entry_name = address.entry_name();

    let (_, entry_metadata) =
        utils::lookup_storage_entry_details(pallet_name, entry_name, metadata)?;
    let value_ty_id = match entry_metadata.entry_type() {
        subxt_metadata::StorageEntryType::Plain(ty) => *ty,
        subxt_metadata::StorageEntryType::Map { value_ty, .. } => *value_ty,
    };

    let val = Address::Target::decode_with_metadata(bytes, value_ty_id, metadata)?;
    Ok(val)
}

/// Return the default value at a given storage address if one is available, or an error otherwise.
pub fn default_value<Address: AddressT>(
    address: &Address,
    metadata: &Metadata,
) -> Result<Address::Target, Error> {
    let pallet_name = address.pallet_name();
    let entry_name = address.entry_name();

    let (_, entry_metadata) =
        utils::lookup_storage_entry_details(pallet_name, entry_name, metadata)?;
    let value_ty_id = match entry_metadata.entry_type() {
        subxt_metadata::StorageEntryType::Plain(ty) => *ty,
        subxt_metadata::StorageEntryType::Map { value_ty, .. } => *value_ty,
    };

    let default_bytes = entry_metadata.default_bytes();
    let val = Address::Target::decode_with_metadata(&mut &*default_bytes, value_ty_id, metadata)?;
    Ok(val)
}
