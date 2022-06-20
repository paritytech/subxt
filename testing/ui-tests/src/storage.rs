// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use frame_metadata::{
    RuntimeMetadataPrefixed,
    StorageEntryMetadata,
    StorageEntryModifier,
    StorageEntryType,
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
