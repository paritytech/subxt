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
#![cfg(test)]

mod storage;
mod utils;

use crate::utils::{
    dispatch_error::{
        ArrayDispatchError,
        LegacyDispatchError,
        NamedFieldDispatchError,
    },
    generate_metadata_from_pallets_custom_dispatch_error,
    MetadataTestRunner,
};

// Each of these tests leads to some rust code being compiled and
// executed to test that compilation is successful (or errors in the
// way that we'd expect).
#[test]
fn ui_tests() {
    let mut m = MetadataTestRunner::default();
    let t = trybuild::TestCases::new();

    // Check that storage maps with no keys are handled properly.
    t.pass(&m.path_to_ui_test_for_metadata(storage::metadata_storage_map_no_keys()));

    // Test that the codegen can handle the different types of DispatchError.
    t.pass(&m.path_to_ui_test_for_metadata(
        generate_metadata_from_pallets_custom_dispatch_error::<ArrayDispatchError>(
            vec![],
        ),
    ));
    t.pass(&m.path_to_ui_test_for_metadata(
        generate_metadata_from_pallets_custom_dispatch_error::<LegacyDispatchError>(
            vec![],
        ),
    ));
    t.pass(&m.path_to_ui_test_for_metadata(
        generate_metadata_from_pallets_custom_dispatch_error::<NamedFieldDispatchError>(
            vec![],
        ),
    ));
}
