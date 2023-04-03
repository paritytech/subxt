// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub mod dispatch_error;
mod metadata_test_runner;
mod pallet_metadata_test_runner;

use frame_metadata::{
    v14::RuntimeMetadataV14, ExtrinsicMetadata, PalletMetadata, PalletStorageMetadata,
    RuntimeMetadataPrefixed, StorageEntryMetadata,
};
use scale_info::{meta_type, IntoPortable, TypeInfo};

pub use metadata_test_runner::MetadataTestRunner;
pub use pallet_metadata_test_runner::PalletMetadataTestRunner;

/// Given some pallet metadata, generate a [`RuntimeMetadataPrefixed`] struct.
/// We default to a useless extrinsic type, and register a fake `DispatchError`
/// type matching the generic type param provided.
pub fn generate_metadata_from_pallets_custom_dispatch_error<DispatchError: TypeInfo + 'static>(
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

    #[derive(TypeInfo)]
    struct Runtime;
    #[derive(TypeInfo)]
    enum RuntimeCall {}
    #[derive(TypeInfo)]
    enum RuntimeEvent {}

    let ty = registry.register_type(&meta_type::<Runtime>());
    registry.register_type(&meta_type::<RuntimeCall>());
    registry.register_type(&meta_type::<RuntimeEvent>());

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
pub fn generate_metadata_from_pallets(pallets: Vec<PalletMetadata>) -> RuntimeMetadataPrefixed {
    generate_metadata_from_pallets_custom_dispatch_error::<dispatch_error::ArrayDispatchError>(
        pallets,
    )
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
