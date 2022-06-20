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
#![cfg(test)]

mod dispatch_errors;
mod storage;
mod utils;

use crate::utils::MetadataTestRunner;

// Each of these tests leads to some rust code being compiled and
// executed to test that compilation is successful (or errors in the
// way that we'd expect).
#[test]
fn ui_tests() {
    let mut m = MetadataTestRunner::default();
    let t = trybuild::TestCases::new();

    // Check that storage maps with no keys are handled properly.
    t.pass(&m.path_to_ui_test_for_metadata(
        "storage_map_no_keys",
        storage::metadata_storage_map_no_keys(),
    ));

    // Test that the codegen can handle the different types of DispatchError.
    t.pass(&m.path_to_ui_test_for_metadata(
        "named_field_dispatch_error",
        dispatch_errors::metadata_named_field_dispatch_error(),
    ));
    t.pass(&m.path_to_ui_test_for_metadata(
        "legacy_dispatch_error",
        dispatch_errors::metadata_legacy_dispatch_error(),
    ));
    t.pass(&m.path_to_ui_test_for_metadata(
        "array_dispatch_error",
        dispatch_errors::metadata_array_dispatch_error(),
    ));
}
