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
