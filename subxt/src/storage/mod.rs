// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing and working with storage items.

mod storage_client;
mod storage_type;

pub use storage_client::StorageClient;
pub use storage_type::{Storage, StorageKeyValuePair};
pub use subxt_core::storage::address::{
    dynamic, Address, AddressT, DynamicAddress, StaticStorageKey, StorageKey,
};
