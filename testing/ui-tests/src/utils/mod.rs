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

pub mod dispatch_error;
mod metadata_test_runner;

use frame_metadata::{
    v14::RuntimeMetadataV14,
    ExtrinsicMetadata,
    PalletMetadata,
    PalletStorageMetadata,
    RuntimeMetadataPrefixed,
    StorageEntryMetadata,
};
use scale_info::{
    meta_type,
    IntoPortable,
    TypeInfo,
};

pub use metadata_test_runner::MetadataTestRunner;

/// Given some pallet metadata, generate a [`RuntimeMetadataPrefixed`] struct.
/// We default to a useless extrinsic type, and register a fake `DispatchError`
/// type matching the generic type param provided.
pub fn generate_metadata_from_pallets_custom_dispatch_error<
    DispatchError: TypeInfo + 'static,
>(
    pallets: Vec<PalletMetadata>,
) -> RuntimeMetadataPrefixed {
    // We don't care about the extrinsic type.
    let extrinsic = ExtrinsicMetadata {
        ty: meta_type::<()>(),
        version: 0,
        signed_extensions: vec![],
    };

    // Construct metadata manually from our types (See `RuntimeMetadataV14::new()`).
    // Add any extra types we need to the registry.
    let mut registry = scale_info::Registry::new();
    let pallets = registry.map_into_portable(pallets);
    let extrinsic = extrinsic.into_portable(&mut registry);
    let ty = registry.register_type(&meta_type::<()>());

    // Metadata needs to contain this DispatchError, since codegen looks for it.
    registry.register_type(&meta_type::<DispatchError>());

    let metadata = RuntimeMetadataV14 {
        types: registry.into(),
        pallets,
        extrinsic,
        ty,
    };

    RuntimeMetadataPrefixed::from(metadata)
}

/// Given some pallet metadata, generate a [`RuntimeMetadataPrefixed`] struct.
/// We default to a useless extrinsic type, and register a fake `DispatchError`
/// type so that codegen is happy with the metadata generated.
pub fn generate_metadata_from_pallets(
    pallets: Vec<PalletMetadata>,
) -> RuntimeMetadataPrefixed {
    generate_metadata_from_pallets_custom_dispatch_error::<
        dispatch_error::ArrayDispatchError,
    >(pallets)
}

/// Given some storage entries, generate a [`RuntimeMetadataPrefixed`] struct.
/// We default to a useless extrinsic type, mock a pallet out, and register a
/// fake `DispatchError` type so that codegen is happy with the metadata generated.
pub fn generate_metadata_from_storage_entries(
    storage_entries: Vec<StorageEntryMetadata>,
) -> RuntimeMetadataPrefixed {
    let storage = PalletStorageMetadata {
        prefix: "System",
        entries: storage_entries,
    };

    let pallet = PalletMetadata {
        index: 0,
        name: "System",
        storage: Some(storage),
        constants: vec![],
        calls: None,
        event: None,
        error: None,
    };

    generate_metadata_from_pallets(vec![pallet])
}
