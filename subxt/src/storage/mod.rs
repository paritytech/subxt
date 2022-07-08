// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types associated with accessing and working with storage items.

mod storage_client;

pub use storage_client::{
    KeyIter,
    SignedExtension,
    StorageAddress,
    StorageClient,
    StorageEntryKey,
    StorageMapKey,
};