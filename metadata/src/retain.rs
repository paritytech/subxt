// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utility functions to generate a subset of the metadata.

use frame_metadata::{PalletMetadata, RuntimeMetadataV14, StorageEntryType};
use scale_info::{form::PortableForm, interner::UntrackedSymbol};
use std::{
    any::TypeId,
    collections::{BTreeMap, HashSet},
};

/// Collect all type IDs needed to represent the provided pallet.
fn collect_pallet_types(pallet: &PalletMetadata<PortableForm>, type_ids: &mut HashSet<u32>) {
    if let Some(storage) = &pallet.storage {
        for entry in &storage.entries {
            match entry.ty {
                StorageEntryType::Plain(ty) => {
                    type_ids.insert(ty.id());
                }
                StorageEntryType::Map { key, value, .. } => {
                    type_ids.insert(key.id());
                    type_ids.insert(value.id());
                }
            }
        }
    }

    if let Some(calls) = &pallet.calls {
        type_ids.insert(calls.ty.id());
    }

    if let Some(event) = &pallet.event {
        type_ids.insert(event.ty.id());
    }

    for constant in &pallet.constants {
        type_ids.insert(constant.ty.id());
    }

    if let Some(error) = &pallet.error {
        type_ids.insert(error.ty.id());
    }
}

/// Update the given type using the new type ID from the portable registry.
///
/// # Panics
///
/// Panics if the [`scale_info::PortableRegistry`] did not retain all needed types.
fn update_type(ty: &mut UntrackedSymbol<TypeId>, map_ids: &BTreeMap<u32, u32>) {
    let old_id = ty.id();
    let new_id = map_ids
        .get(&old_id)
        .unwrap_or_else(|| panic!("PortableRegistry did not retain type id {old_id}. This is a bug. Please open an issue."));
    *ty = (*new_id).into();
}

/// Update all type IDs of the provided pallet using the new type IDs from the portable registry.
fn update_pallet_types(pallet: &mut PalletMetadata<PortableForm>, map_ids: &BTreeMap<u32, u32>) {
    if let Some(storage) = &mut pallet.storage {
        for entry in &mut storage.entries {
            match &mut entry.ty {
                StorageEntryType::Plain(ty) => {
                    update_type(ty, map_ids);
                }
                StorageEntryType::Map { key, value, .. } => {
                    update_type(key, map_ids);
                    update_type(value, map_ids);
                }
            }
        }
    }

    if let Some(calls) = &mut pallet.calls {
        update_type(&mut calls.ty, map_ids);
    }

    if let Some(event) = &mut pallet.event {
        update_type(&mut event.ty, map_ids);
    }

    for constant in &mut pallet.constants {
        update_type(&mut constant.ty, map_ids);
    }

    if let Some(error) = &mut pallet.error {
        update_type(&mut error.ty, map_ids);
    }
}

/// Generate a subset of the metadata that contains only the
/// types needed to represent the provided pallets.
///
/// # Note
///
/// Used to strip metadata of unneeded information and to reduce the
/// binary size.
///
/// # Panics
///
/// Panics if the [`scale_info::PortableRegistry`] did not retain all needed types,
/// or the metadata does not contain the "sp_runtime::DispatchError" type.
pub fn retain_metadata_pallets<F>(metadata: &mut RuntimeMetadataV14, mut filter: F)
where
    F: FnMut(&PalletMetadata<PortableForm>) -> bool,
{
    let mut type_ids = HashSet::new();

    // Collect all types needed to represent the provided pallets.
    metadata.pallets.retain(|pallet| {
        if filter(pallet) {
            collect_pallet_types(pallet, &mut type_ids);
            true
        } else {
            false
        }
    });

    // Additionally, subxt depends on the `RuntimeError` and `DispatchError`.
    let dispatch_error_ty = metadata
        .types
        .types()
        .iter()
        .find(|ty| ty.ty().path().segments() == ["sp_runtime", "DispatchError"])
        .expect("Metadata must contain sp_runtime::DispatchError");

    type_ids.insert(dispatch_error_ty.id());

    // Keep only the needed IDs in the portable registry.
    let map_ids = metadata.types.retain(|id| type_ids.contains(&id));

    // Update the metadata types to their new IDs in the registry.
    for pallets in &mut metadata.pallets {
        update_pallet_types(pallets, &map_ids);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codec::Decode;
    use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed, RuntimeMetadataV14};
    use std::{fs, path::Path};

    fn load_metadata() -> RuntimeMetadataV14 {
        let bytes = fs::read(Path::new("../artifacts/polkadot_metadata.scale"))
            .expect("Cannot read metadata blob");
        let meta: RuntimeMetadataPrefixed =
            Decode::decode(&mut &*bytes).expect("Cannot decode scale metadata");

        match meta.1 {
            RuntimeMetadata::V14(v14) => v14,
            _ => panic!("Unsupported metadata version {:?}", meta.1),
        }
    }

    #[test]
    fn retain_one_pallet() {
        let metadata_cache = load_metadata();

        // Retain one pallet at a time ensuring the test does not panic.
        for pallet in &metadata_cache.pallets {
            let mut metadata = metadata_cache.clone();
            retain_metadata_pallets(&mut metadata, |filter_pallet| {
                filter_pallet.name == pallet.name
            });

            assert_eq!(metadata.pallets.len(), 1);
            assert_eq!(metadata.pallets.get(0).unwrap().name, pallet.name);
        }
    }
}
