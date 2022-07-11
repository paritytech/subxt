// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing and working with storage items.

mod storage_client;
mod storage_address;

pub use storage_client::{
    StorageClient,
    KeyIter,
    SignedExtension,
};

/// Types representing an address which describes where a storage
/// entry lives and how to properly decode it.
pub mod address {
    pub use super::storage_address::{
        StorageAddress,
        StorageEntryKey,
        StorageMapKey,
        StorageHasher,
        AddressHasDefaultValue,
        AddressIsIterable,
    };
}
