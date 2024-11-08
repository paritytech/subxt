// Copyright 2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::{borrow::Cow, path::Path};

use codec::{Decode, Encode};
use polkadot_sdk::{
    sc_executor::{self, WasmExecutionMethod, WasmExecutor},
    sc_executor_common::runtime_blob::RuntimeBlob,
    sp_io,
    sp_maybe_compressed_blob::{self, CODE_BLOB_BOMB_LIMIT},
    sp_state_machine,
};
use subxt_codegen::{CodegenError, Metadata};

/// Result type shorthand
pub type WasmMetadataResult<A> = Result<A, CodegenError>;

/// Uses wasm artifact produced by compiling the runtime to generate metadata
pub fn from_wasm_file(wasm_file_path: &Path) -> WasmMetadataResult<Metadata> {
    let wasm_file = subxt_utils_fetchmetadata::from_file_blocking(wasm_file_path)
        .map_err(|e| CodegenError::Other(e.to_string()))
        .and_then(maybe_decompress)?;
    call_and_decode(wasm_file)
}

fn call_and_decode(wasm_file: Vec<u8>) -> WasmMetadataResult<Metadata> {
    let mut ext: sp_state_machine::BasicExternalities = Default::default();

    let executor: WasmExecutor<sp_io::SubstrateHostFunctions> = WasmExecutor::builder()
        .with_execution_method(WasmExecutionMethod::default())
        .with_offchain_heap_alloc_strategy(sc_executor::HeapAllocStrategy::Dynamic {
            maximum_pages: Some(64),
        })
        .with_max_runtime_instances(1)
        .with_runtime_cache_size(1)
        .build();

    let runtime_blob =
        RuntimeBlob::new(&wasm_file).map_err(|e| CodegenError::Wasm(e.to_string()))?;

    let version = executor
        .uncached_call(
            runtime_blob.clone(),
            &mut ext,
            true,
            "Metadata_metadata_versions",
            &[],
        )
        .map_err(|_| {
            CodegenError::Wasm("method \"Metadata_metadata_versions\" doesnt exist".to_owned())
        })?;
    let mut versions = <Vec<u32>>::decode(&mut &version[..]).map_err(CodegenError::Decode)?;

    // Highest version will always be the last one in the vec
    versions.sort();

    let version = versions
        .last()
        .ok_or(CodegenError::Other(
            "No metadata versions were returned".to_owned(),
        ))
        .map(|v| v.encode())?;

    let encoded_metadata = executor
        .uncached_call(
            runtime_blob,
            &mut ext,
            false,
            "Metadata_metadata_at_version",
            &version,
        )
        .map_err(|e| {
            dbg!(e);
            CodegenError::Wasm("method \"Metadata_metadata_at_version\" doesnt exist".to_owned())
        })?;

    decode(encoded_metadata)
}

fn decode(encoded_metadata: Vec<u8>) -> WasmMetadataResult<Metadata> {
    // We slice the first byte from the metadata because it's wrapped inside an option and we know that its always `Some`
    let metadata = <Vec<u8>>::decode(&mut &encoded_metadata[1..]).map_err(CodegenError::Decode)?;
    Metadata::decode(&mut metadata.as_ref()).map_err(Into::into)
}

fn maybe_decompress(file_contents: Vec<u8>) -> WasmMetadataResult<Vec<u8>> {
    sp_maybe_compressed_blob::decompress(file_contents.as_ref(), CODE_BLOB_BOMB_LIMIT)
        .map_err(|e| CodegenError::Wasm(e.to_string()))
        .map(Cow::into_owned)
}
