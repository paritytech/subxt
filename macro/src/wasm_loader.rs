// Copyright 2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::{borrow::Cow, path::Path};

use codec::{Decode, Encode};
use sc_executor::{WasmExecutionMethod, WasmExecutor};
use sc_executor_common::runtime_blob::RuntimeBlob;
use sp_maybe_compressed_blob::{self, CODE_BLOB_BOMB_LIMIT};
use subxt_codegen::{CodegenError, Metadata};

static SUPPORTED_METADATA_VERSIONS: [u32; 2] = [14, 15];

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
    let mut executor = Executor::new(&wasm_file)?;

    if let Ok(versions) = executor.versions() {
        let version = versions
            .into_iter()
            .max()
            .expect("This is checked earlier and can't fail.");

        executor.load_metadata_at_version(version)
    } else {
        executor.load_legacy_metadata()
    }
}

fn decode(encoded_metadata: Vec<u8>) -> WasmMetadataResult<Metadata> {
    Metadata::decode(&mut encoded_metadata.as_ref()).map_err(Into::into)
}

fn maybe_decompress(file_contents: Vec<u8>) -> WasmMetadataResult<Vec<u8>> {
    sp_maybe_compressed_blob::decompress(file_contents.as_ref(), CODE_BLOB_BOMB_LIMIT)
        .map_err(|e| CodegenError::Wasm(e.to_string()))
        .map(Cow::into_owned)
}

struct Executor {
    runtime_blob: RuntimeBlob,
    executor: WasmExecutor<sp_io::SubstrateHostFunctions>,
    externalities: sp_state_machine::BasicExternalities,
}

impl Executor {
    fn new(wasm_file: &[u8]) -> WasmMetadataResult<Self> {
        let externalities: sp_state_machine::BasicExternalities = Default::default();

        let executor: WasmExecutor<sp_io::SubstrateHostFunctions> = WasmExecutor::builder()
            .with_execution_method(WasmExecutionMethod::default())
            .with_offchain_heap_alloc_strategy(sc_executor::HeapAllocStrategy::Dynamic {
                maximum_pages: Some(64),
            })
            .with_max_runtime_instances(1)
            .with_runtime_cache_size(1)
            .build();

        let runtime_blob =
            RuntimeBlob::new(wasm_file).map_err(|e| CodegenError::Wasm(e.to_string()))?;

        Ok(Self {
            runtime_blob,
            executor,
            externalities,
        })
    }

    fn versions(&mut self) -> WasmMetadataResult<Vec<u32>> {
        let version = self
            .executor
            .uncached_call(
                self.runtime_blob.clone(),
                &mut self.externalities,
                true,
                "Metadata_metadata_versions",
                &[],
            )
            .map_err(|_| {
                CodegenError::Wasm("method \"Metadata_metadata_versions\" doesnt exist".to_owned())
            })?;
        let versions = <Vec<u32>>::decode(&mut &version[..])
            .map_err(CodegenError::Decode)
            .map(|x| {
                x.into_iter()
                    .filter(|version| SUPPORTED_METADATA_VERSIONS.contains(version))
                    .collect::<Vec<u32>>()
            })?;

        if versions.is_empty() {
            return Err(CodegenError::Other(
                "No supported metadata versions were returned".to_owned(),
            ));
        }

        Ok(versions)
    }

    fn load_legacy_metadata(&mut self) -> WasmMetadataResult<Metadata> {
        let encoded_metadata = self
            .executor
            .uncached_call(
                self.runtime_blob.clone(),
                &mut self.externalities,
                false,
                "Metadata_metadata",
                &[],
            )
            .map_err(|e| {
                CodegenError::Wasm(format!(
                    "Failed to call \"Metadata_metadata\" on WASM runtime. Cause: {e}"
                ))
            })?;
        let encoded_metadata =
            <Vec<u8>>::decode(&mut &encoded_metadata[..]).map_err(CodegenError::Decode)?;
        decode(encoded_metadata)
    }

    fn load_metadata_at_version(&mut self, version: u32) -> WasmMetadataResult<Metadata> {
        let encoded_metadata = self
            .executor
            .uncached_call(
                self.runtime_blob.clone(),
                &mut self.externalities,
                false,
                "Metadata_metadata_at_version",
                &version.encode(),
            )
            .map_err(|e| {
                CodegenError::Wasm(format!(
                    "Failed to call \"Metadata_metadata_at_version\" on WASM runtime. Cause: {e}"
                ))
            })?;
        let Some(encoded_metadata) =
            <Option<Vec<u8>>>::decode(&mut &encoded_metadata[..]).map_err(CodegenError::Decode)?
        else {
            return Err(CodegenError::Other(
                format!("Received empty metadata at version: v{version}").to_owned(),
            ));
        };
        decode(encoded_metadata)
    }
}
