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

use crate::utils::MetadataTestRunner;

// Each of these tests leads to some rust code being compiled and
// executed to test that compilation is successful (or errors in the
// way that we'd expect).
#[test]
fn ui_tests() {
    let mut m = MetadataTestRunner::default();
    let t = trybuild::TestCases::new();

    t.pass("src/correct/*.rs");

    // Check that storage maps with no keys are handled properly.
    t.pass(
        m.new_test_case()
            .name("storage_map_no_keys")
            .build(storage::metadata_storage_map_no_keys()),
    );

    // Test that the codegen can handle the different types of DispatchError.
    t.pass(
        m.new_test_case()
            .name("named_field_dispatch_error")
            .build(dispatch_errors::metadata_named_field_dispatch_error()),
    );
    t.pass(
        m.new_test_case()
            .name("legacy_dispatch_error")
            .build(dispatch_errors::metadata_legacy_dispatch_error()),
    );
    t.pass(
        m.new_test_case()
            .name("array_dispatch_error")
            .build(dispatch_errors::metadata_array_dispatch_error()),
    );

    // Test retaining only specific pallets and ensure that works.
    for pallet in ["Babe", "Claims", "Grandpa", "Balances"] {
        let mut metadata = MetadataTestRunner::load_metadata();
        metadata.retain(|p| p == pallet, |_| true);

        t.pass(
            m.new_test_case()
                .name(format!("retain_pallet_{pallet}"))
                .build(metadata),
        );
    }

    // Test retaining only specific runtime APIs to ensure that works.
    for runtime_api in ["Core", "Metadata"] {
        let mut metadata = MetadataTestRunner::load_metadata();
        metadata.retain(|_| true, |r| r == runtime_api);

        t.pass(
            m.new_test_case()
                .name(format!("retain_runtime_api_{runtime_api}"))
                .build(metadata),
        );
    }

    // Validation should succeed when metadata we codegen from is stripped and
    // client metadata is full:
    {
        let mut metadata = MetadataTestRunner::load_metadata();
        metadata.retain(
            |p| ["Babe", "Claims"].contains(&p),
            |r| ["Core", "Metadata"].contains(&r),
        );

        t.pass(
            m.new_test_case()
                .name("stripped_metadata_validates_against_full")
                .validation_metadata(MetadataTestRunner::load_metadata())
                .build(metadata),
        );
    }

    // Finally as a sanity check, codegen against stripped metadata should
    // _not_ compare valid against client with differently stripped metadata.
    {
        let mut codegen_metadata = MetadataTestRunner::load_metadata();
        codegen_metadata.retain(
            |p| ["Babe", "Claims"].contains(&p),
            |r| ["Core", "Metadata"].contains(&r),
        );
        let mut validation_metadata = MetadataTestRunner::load_metadata();
        validation_metadata.retain(|p| p != "Claims", |r| r != "Metadata");

        t.pass(
            m.new_test_case()
                .name("stripped_metadata_doesnt_validate_against_different")
                .validation_metadata(validation_metadata)
                .expects_invalid()
                .build(codegen_metadata),
        );
    }
}

#[test]
fn ui_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/incorrect/*.rs");
}
