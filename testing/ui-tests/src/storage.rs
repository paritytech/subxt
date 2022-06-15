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
    PalletMetadata,
    PalletStorageMetadata,
    RuntimeMetadataPrefixed,
    StorageEntryMetadata,
    StorageEntryModifier,
    StorageEntryType,
};
use scale_info::meta_type;

use crate::utils::{
    generate_metadata_from_pallets,
    MetadataTestRunner,
};

/// Generate metadata which contains a `Map` storage entry with no hashers/values.
/// This is a bit of an odd case, but it was raised in https://github.com/paritytech/subxt/issues/552,
/// and this test will fail before the fix and should pass once the fix is applied.
fn metadata_storage_item_no_values() -> RuntimeMetadataPrefixed {
    let storage = PalletStorageMetadata {
        prefix: "System".into(),
        entries: vec![StorageEntryMetadata {
            name: "Map".into(),
            modifier: StorageEntryModifier::Optional,
            ty: StorageEntryType::Map {
                hashers: vec![],
                key: meta_type::<()>(),
                value: meta_type::<u32>(),
            },
            default: vec![0],
            docs: vec![],
        }],
    };

    let pallet = PalletMetadata {
        index: 0,
        name: "System".into(),
        storage: Some(storage),
        constants: vec![],
        calls: None,
        event: None,
        error: None,
    };

    generate_metadata_from_pallets(vec![pallet])
}

#[test]
fn ui_tests() {
    let mut m = MetadataTestRunner::default();
    let t = trybuild::TestCases::new();

    t.pass(&m.path_to_ui_test_for_metadata(metadata_storage_item_no_values()));
}
