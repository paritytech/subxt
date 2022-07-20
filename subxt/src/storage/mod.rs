// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing and working with storage items.

mod storage_address;
mod storage_client;
mod storage_map_key;

pub mod utils;

pub use storage_client::{
    KeyIter,
    StorageClient,
};

// Re-export as this is used in the public API:
pub use sp_core::storage::StorageKey;

/// Types representing an address which describes where a storage
/// entry lives and how to properly decode it.
pub mod address {
    pub use super::{
        storage_address::{
            dynamic,
            dynamic_root,
            DynamicStorageAddress,
            StaticStorageAddress,
            StorageAddress,
            Yes,
        },
        storage_map_key::{
            StorageHasher,
            StorageMapKey,
        },
    };
}

// For consistency with other modules, also expose
// the basic address stuff at the root of the module.
pub use storage_address::{
    dynamic,
    dynamic_root,
    DynamicStorageAddress,
    StaticStorageAddress,
    StorageAddress,
};
