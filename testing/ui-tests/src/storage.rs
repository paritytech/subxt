// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use frame_metadata::{
    RuntimeMetadataPrefixed, StorageEntryMetadata, StorageEntryModifier, StorageEntryType,
};
use scale_info::meta_type;

use crate::utils::generate_metadata_from_storage_entries;

/// Generate metadata which contains a `Map` storage entry with no hashers/values.
/// This is a bit of an odd case, but it was raised in https://github.com/paritytech/subxt/issues/552,
/// and this test will fail before the fix and should pass once the fix is applied.
pub fn metadata_storage_map_no_keys() -> RuntimeMetadataPrefixed {
    generate_metadata_from_storage_entries(vec![StorageEntryMetadata {
        name: "MapWithNoKeys",
        modifier: StorageEntryModifier::Optional,
        ty: StorageEntryType::Map {
            hashers: vec![],
            key: meta_type::<()>(),
            value: meta_type::<u32>(),
        },
        default: vec![0],
        docs: vec![],
    }])
}
