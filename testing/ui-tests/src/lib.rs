// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.
#![cfg(test)]

//! UI test set uses [`trybuild`](https://docs.rs/trybuild/latest/trybuild/index.html) to
//! check whether expected valid examples of code compile correctly, and for incorrect ones
//! errors are helpful and valid (e.g. have correct spans).
//!
//!
//! Use with `TRYBUILD=overwrite` after updating codebase (see `trybuild` docs for more details on that)
//! to automatically regenerate `stderr` files, but don't forget to check that new files make sense.

mod dispatch_errors;
mod storage;
mod utils;

use crate::utils::{MetadataTestRunner, PalletMetadataTestRunner};

// Each of these tests leads to some rust code being compiled and
// executed to test that compilation is successful (or errors in the
// way that we'd expect).
#[test]
fn ui_tests() {
    let mut m = MetadataTestRunner::default();
    let mut p = PalletMetadataTestRunner::new();
    let t = trybuild::TestCases::new();

    t.pass("src/correct/*.rs");

    // Check that storage maps with no keys are handled properly.
    t.pass(m.path_to_ui_test_for_metadata(
        "storage_map_no_keys",
        storage::metadata_storage_map_no_keys(),
    ));

    // Test that the codegen can handle the different types of DispatchError.
    t.pass(m.path_to_ui_test_for_metadata(
        "named_field_dispatch_error",
        dispatch_errors::metadata_named_field_dispatch_error(),
    ));
    t.pass(m.path_to_ui_test_for_metadata(
        "legacy_dispatch_error",
        dispatch_errors::metadata_legacy_dispatch_error(),
    ));
    t.pass(m.path_to_ui_test_for_metadata(
        "array_dispatch_error",
        dispatch_errors::metadata_array_dispatch_error(),
    ));

    // Ensure the generate per pallet metadata compiles.
    while let Some(path) = p.path_to_next_ui_test() {
        t.pass(path);
    }
}

#[test]
fn ui_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/incorrect/*.rs");
}
