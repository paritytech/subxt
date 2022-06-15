// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

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
