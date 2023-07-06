// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub mod dispatch_error;
mod metadata_test_runner;

use frame_metadata::{
    v15::{
        CustomMetadata, ExtrinsicMetadata, OuterEnums, PalletMetadata, PalletStorageMetadata,
        RuntimeMetadataV15, StorageEntryMetadata,
    },
    RuntimeMetadataPrefixed,
};
use scale_info::{meta_type, IntoPortable, TypeInfo};

pub use metadata_test_runner::MetadataTestRunner;

/// Given some pallet metadata, generate a [`RuntimeMetadataPrefixed`] struct.
/// We default to a useless extrinsic type, and register a fake `DispatchError`
/// type matching the generic type param provided.
pub fn generate_metadata_from_pallets_custom_dispatch_error<DispatchError: TypeInfo + 'static>(
    pallets: Vec<PalletMetadata>,
) -> RuntimeMetadataPrefixed {
    // We don't care about the extrinsic type.
    let extrinsic = ExtrinsicMetadata {
        version: 0,
        signed_extensions: vec![],
        address_ty: meta_type::<()>(),
        call_ty: meta_type::<()>(),
        signature_ty: meta_type::<()>(),
        extra_ty: meta_type::<()>(),
    };

    // Construct metadata manually from our types (See `RuntimeMetadataV15::new()`).
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
    #[derive(TypeInfo)]
    enum RuntimeError {}

    let ty = registry.register_type(&meta_type::<Runtime>());
    let runtime_call = registry.register_type(&meta_type::<RuntimeCall>());
    let runtime_event = registry.register_type(&meta_type::<RuntimeEvent>());
    let runtime_error = registry.register_type(&meta_type::<RuntimeError>());

    // Metadata needs to contain this DispatchError, since codegen looks for it.
    registry.register_type(&meta_type::<DispatchError>());

    let metadata = RuntimeMetadataV15 {
        types: registry.into(),
        pallets,
        extrinsic,
        ty,
        apis: vec![],
        outer_enums: OuterEnums {
            call_enum_ty: runtime_call,
            event_enum_ty: runtime_event,
            error_enum_ty: runtime_error,
        },
        custom: CustomMetadata {
            map: Default::default(),
        },
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
        docs: vec![],
    };

    generate_metadata_from_pallets(vec![pallet])
}
