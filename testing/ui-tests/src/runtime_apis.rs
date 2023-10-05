// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use frame_metadata::{
    v15::{RuntimeApiMetadata, RuntimeApiMethodMetadata, RuntimeApiMethodParamMetadata},
    RuntimeMetadataPrefixed,
};

use crate::utils::generate_metadata_from_runtime_apis;

/// Generate metadata which contains a `Map` storage entry with no hashers/values.
/// This is a bit of an odd case, but it was raised in https://github.com/paritytech/subxt/issues/552,
/// and this test will fail before the fix and should pass once the fix is applied.
pub fn metadata_runtime_api_underscore_method_name() -> RuntimeMetadataPrefixed {
    generate_metadata_from_runtime_apis(vec![RuntimeApiMetadata {
        name: "MyApi".to_owned(),
        docs: vec![],
        methods: vec![RuntimeApiMethodMetadata {
            name: "my_method".to_owned(),
            inputs: vec![RuntimeApiMethodParamMetadata {
                name: "_".to_owned(), // The important bit we're testing.
                ty: 0.into(),         // we don't care what type this is.
            }],
            output: 0.into(), // we don't care what type this is.
            docs: vec![],
        }],
    }])
}
