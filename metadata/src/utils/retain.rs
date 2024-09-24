// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utility functions to generate a subset of the metadata.

use alloc::collections::BTreeSet;

use crate::{
    ExtrinsicMetadata, Metadata, OuterEnumsMetadata, PalletMetadataInner, RuntimeApiMetadataInner,
    StorageEntryType,
};
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use scale_info::{
    PortableType, TypeDef, TypeDefArray, TypeDefBitSequence, TypeDefCompact, TypeDefComposite,
    TypeDefSequence, TypeDefTuple, TypeDefVariant,
};

use super::variant_index::VariantIndex;

#[derive(Clone)]
struct TypeSet {
    seen_ids: BTreeSet<u32>,
}

impl TypeSet {
    fn new() -> Self {
        Self {
            seen_ids: BTreeSet::new(),
        }
    }

    fn collect_types(&mut self, metadata: &Metadata, t: &PortableType) {
        let mut work_set = VecDeque::from([t]);
        while let Some(typ) = work_set.pop_front() {
            match &typ.ty.type_def {
                TypeDef::Composite(TypeDefComposite { fields }) => {
                    for field in fields {
                        if self.seen_ids.insert(field.ty.id) {
                            let ty = resolve_typ(metadata, field.ty.id);
                            work_set.push_back(ty);
                        }
                    }
                }
                TypeDef::Variant(TypeDefVariant { variants }) => {
                    for variant in variants {
                        for field in &variant.fields {
                            if self.seen_ids.insert(field.ty.id) {
                                let ty = resolve_typ(metadata, field.ty.id);
                                work_set.push_back(ty);
                            }
                        }
                    }
                }
                TypeDef::Array(TypeDefArray { len: _, type_param })
                | TypeDef::Sequence(TypeDefSequence { type_param })
                | TypeDef::Compact(TypeDefCompact { type_param }) => {
                    if self.seen_ids.insert(type_param.id) {
                        let ty = resolve_typ(metadata, type_param.id);
                        work_set.push_back(ty);
                    }
                }
                TypeDef::Tuple(TypeDefTuple { fields }) => {
                    for field in fields {
                        if self.seen_ids.insert(field.id) {
                            let ty = resolve_typ(metadata, field.id);
                            work_set.push_back(ty);
                        }
                    }
                }
                TypeDef::Primitive(_) => (),
                TypeDef::BitSequence(TypeDefBitSequence {
                    bit_store_type,
                    bit_order_type,
                }) => {
                    for typ in [bit_order_type, bit_store_type] {
                        if self.seen_ids.insert(typ.id) {
                            let ty = resolve_typ(metadata, typ.id);
                            work_set.push_back(ty);
                        }
                    }
                }
            }
        }
    }

    fn insert_and_collect_types(&mut self, metadata: &Metadata, ty: &PortableType) {
        if self.seen_ids.insert(ty.id) {
            self.collect_types(metadata, ty);
        }
    }

    fn collect_extrinsic_types(&mut self, extrinsic: &ExtrinsicMetadata) {
        let mut ids = Vec::from([
            extrinsic.address_ty,
            extrinsic.call_ty,
            extrinsic.signature_ty,
            extrinsic.extra_ty,
        ]);

        for signed in &extrinsic.signed_extensions {
            ids.push(signed.extra_ty);
            ids.push(signed.additional_ty);
        }
        for id in ids {
            self.seen_ids.insert(id);
        }
    }

    /// Collect all type IDs needed to represent the runtime APIs.
    fn collect_runtime_api_types(&mut self, api: &RuntimeApiMetadataInner) {
        for method in api.methods.values() {
            for input in &method.inputs {
                self.seen_ids.insert(input.ty);
            }
            self.seen_ids.insert(method.output_ty);
        }
    }

    /// Collect all type IDs needed to represent the provided pallet.
    fn collect_pallet_types(&mut self, pallet: &PalletMetadataInner, metadata: &Metadata) {
        let mut type_ids = Vec::new();
        if let Some(storage) = &pallet.storage {
            for entry in storage.entries() {
                match entry.entry_type {
                    StorageEntryType::Plain(ty) => {
                        type_ids.push(ty);
                    }
                    StorageEntryType::Map {
                        key_ty, value_ty, ..
                    } => {
                        type_ids.push(key_ty);
                        type_ids.push(value_ty);
                    }
                }
            }
        }

        if let Some(ty) = pallet.call_ty {
            type_ids.push(ty);
        }

        if let Some(ty) = pallet.event_ty {
            type_ids.push(ty);
        }

        for constant in pallet.constants.values() {
            type_ids.push(constant.ty);
        }

        if let Some(ty) = pallet.error_ty {
            type_ids.push(ty);
        }
        for id in type_ids {
            let typ = resolve_typ(metadata, id);
            self.insert_and_collect_types(metadata, typ);
        }
    }
    /// Strips pallets that we need to keep around for their types
    fn update_filtered_pallet(
        &mut self,
        pallet: &mut PalletMetadataInner,
        retained_set: &mut TypeSet,
    ) {
        let entry_fn = |entry: &crate::StorageEntryMetadata| match entry.entry_type {
            StorageEntryType::Plain(ty) => {
                self.seen_ids.contains(&ty) && retained_set.seen_ids.remove(&ty)
            }
            StorageEntryType::Map {
                key_ty, value_ty, ..
            } => {
                if self.seen_ids.contains(&key_ty) && self.seen_ids.contains(&value_ty) {
                    retained_set.seen_ids.remove(&key_ty) && retained_set.seen_ids.remove(&value_ty)
                } else {
                    false
                }
            }
        };
        let new_storage = match pallet.storage.as_mut() {
            Some(storage) => {
                // check if the both types in the seen_set and keep the entry if types were not retained already
                storage.retain_entries(entry_fn);
                // if the storage list is empty - drop it completetely
                if storage.entries().is_empty() {
                    None
                } else {
                    Some(storage)
                }
            }
            None => None,
        };

        pallet.storage = new_storage.cloned();

        // Helpers
        let mut check_opt_and_retain = |option: Option<u32>| -> Option<u32> {
            match option {
                Some(ty) if self.seen_ids.contains(&ty) && retained_set.seen_ids.remove(&ty) => {
                    Some(ty)
                }
                _ => None,
            }
        };
        fn reset_variant_index(variant_index: &mut VariantIndex, opt: Option<u32>) {
            if opt.is_none() {
                *variant_index = VariantIndex::empty()
            }
        }

        pallet.call_ty = check_opt_and_retain(pallet.call_ty);
        reset_variant_index(&mut pallet.call_variant_index, pallet.call_ty);

        pallet.event_ty = check_opt_and_retain(pallet.event_ty);
        reset_variant_index(&mut pallet.event_variant_index, pallet.event_ty);

        pallet.error_ty = check_opt_and_retain(pallet.error_ty);
        reset_variant_index(&mut pallet.error_variant_index, pallet.error_ty);

        pallet.constants.retain(|value| {
            self.seen_ids.contains(&value.ty) && retained_set.seen_ids.remove(&value.ty)
        });
    }

    // Collect types referenced inside outer enum
    fn collect_variants_in_type<F>(&mut self, metadata: &mut Metadata, id: u32, mut name_filter: F)
    where
        F: FnMut(&str) -> bool,
    {
        let m = metadata.clone();

        let ty = {
            metadata
                .types
                .types
                .get_mut(id as usize)
                .expect("Metadata should contain enum type in registry")
        };

        let mut for_mut_ty = ty.clone();

        let TypeDef::Variant(variant) = &mut for_mut_ty.ty.type_def else {
            panic!("Metadata type is expected to be a variant type");
        };

        // Remove all variants from the cloned type that aren't the pallet(s) we want to keep.
        variant.variants.retain(|v| name_filter(&v.name));

        // traverse the enum and collect the types
        self.collect_types(&m, &for_mut_ty);

        // Redo the thing above but keep filtered out variants if they reference types that we intend to keep
        let TypeDef::Variant(variant) = &mut ty.ty.type_def else {
            panic!("Metadata type is expected to be a variant type");
        };

        variant.variants.retain(|v| {
            name_filter(&v.name) || {
                v.fields
                    .iter()
                    .all(|field| self.seen_ids.contains(&field.ty.id))
            }
        });

        self.seen_ids.insert(id);

        self.collect_types(&m, ty);
    }
}

fn resolve_typ(metadata: &Metadata, typ: u32) -> &PortableType {
    metadata
        .types
        .types
        .get(typ as usize)
        .expect("Metadata should contain enum type in registry")
}

/// Update all type IDs of the provided pallet using the new type IDs from the portable registry.
fn update_pallet_types(pallet: &mut PalletMetadataInner, map_ids: &BTreeMap<u32, u32>) {
    if let Some(storage) = &mut pallet.storage {
        for entry in storage.entries.values_mut() {
            match &mut entry.entry_type {
                StorageEntryType::Plain(ty) => {
                    update_type(ty, map_ids);
                }
                StorageEntryType::Map {
                    key_ty, value_ty, ..
                } => {
                    update_type(key_ty, map_ids);
                    update_type(value_ty, map_ids);
                }
            }
        }
    }
    if let Some(ty) = &mut pallet.call_ty {
        update_type(ty, map_ids);
    }

    if let Some(ty) = &mut pallet.event_ty {
        update_type(ty, map_ids);
    }

    if let Some(ty) = &mut pallet.error_ty {
        update_type(ty, map_ids);
    }

    for constant in pallet.constants.values_mut() {
        update_type(&mut constant.ty, map_ids);
    }
}

/// Update all type IDs of the provided extrinsic metadata using the new type IDs from the portable registry.
fn update_extrinsic_types(extrinsic: &mut ExtrinsicMetadata, map_ids: &BTreeMap<u32, u32>) {
    update_type(&mut extrinsic.address_ty, map_ids);
    update_type(&mut extrinsic.call_ty, map_ids);
    update_type(&mut extrinsic.signature_ty, map_ids);
    update_type(&mut extrinsic.extra_ty, map_ids);

    for signed in &mut extrinsic.signed_extensions {
        update_type(&mut signed.extra_ty, map_ids);
        update_type(&mut signed.additional_ty, map_ids);
    }
}

/// Update all type IDs of the provided runtime APIs metadata using the new type IDs from the portable registry.
fn update_runtime_api_types(apis: &mut [RuntimeApiMetadataInner], map_ids: &BTreeMap<u32, u32>) {
    for api in apis {
        for method in api.methods.values_mut() {
            for input in &mut method.inputs {
                update_type(&mut input.ty, map_ids);
            }
            update_type(&mut method.output_ty, map_ids);
        }
    }
}

/// Update all the type IDs for outer enums.
fn update_outer_enums(enums: &mut OuterEnumsMetadata, map_ids: &BTreeMap<u32, u32>) {
    update_type(&mut enums.call_enum_ty, map_ids);
    update_type(&mut enums.event_enum_ty, map_ids);
    update_type(&mut enums.error_enum_ty, map_ids);
}

/// Update the given type using the new type ID from the portable registry.
///
/// # Panics
///
/// Panics if the [`scale_info::PortableRegistry`] did not retain all needed types.
fn update_type(ty: &mut u32, map_ids: &BTreeMap<u32, u32>) {
    let old_id = *ty;
    let new_id = map_ids
        .get(&old_id)
        .copied()
        .unwrap_or_else(|| panic!("PortableRegistry did not retain type id {old_id}. This is a bug. Please open an issue."));
    *ty = new_id;
}

/// Generate a subset of the metadata that contains only the
/// types needed to represent the provided pallets and runtime APIs.
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
pub fn retain_metadata<F, G>(
    metadata: &mut Metadata,
    mut pallets_filter: F,
    mut runtime_apis_filter: G,
) where
    F: FnMut(&str) -> bool,
    G: FnMut(&str) -> bool,
{
    // Types specifically referenced inside pallets that we keep
    let mut retained_set = TypeSet::new();

    for pallet in metadata.pallets.values() {
        let should_retain = pallets_filter(&pallet.name);
        if should_retain {
            retained_set.collect_pallet_types(pallet, metadata);
        }
    }

    // all types that we intend to keep
    let mut type_set = retained_set.clone();

    for api in metadata.apis.values() {
        let should_retain = runtime_apis_filter(&api.name);
        if should_retain {
            type_set.collect_runtime_api_types(api);
        }
    }

    // Additionally, subxt depends on the `DispatchError` type existing; we use the same
    // logic here that is used when building our `Metadata`.
    let dispatch_error_ty = metadata
        .types
        .types
        .iter()
        .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
        .expect("Metadata must contain sp_runtime::DispatchError");
    type_set.seen_ids.insert(dispatch_error_ty.id);
    type_set.seen_ids.insert(metadata.runtime_ty);
    type_set.collect_extrinsic_types(&metadata.extrinsic);

    // Collect the outer enums type IDs.
    for typ in [
        metadata.outer_enums.call_enum_ty,
        metadata.outer_enums.event_enum_ty,
        metadata.outer_enums.error_enum_ty,
    ] {
        type_set.collect_variants_in_type(metadata, typ, &mut pallets_filter);
    }

    let mut retained_set = TypeSet {
        seen_ids: type_set
            .seen_ids
            .difference(&retained_set.seen_ids)
            .copied()
            .collect(),
    };
    // Strip down Pallets we dont need and only keep types that are not yet included in the retained set.
    for pallet in metadata.pallets.values_mut() {
        if !pallets_filter(&pallet.name) {
            type_set.update_filtered_pallet(pallet, &mut retained_set);
        }
    }

    // Filter out unnecesary pallets that have no entries
    metadata.pallets.retain(|pallet| {
        pallets_filter(&pallet.name)
            || !matches!(
                pallet,
                PalletMetadataInner {
                    storage: None,
                    call_ty: None,
                    event_ty: None,
                    error_ty: None,
                    constants: map,
                    ..
                } if map.is_empty()
            )
    });

    // Retain the apis
    metadata.apis.retain(|api| runtime_apis_filter(&api.name));

    metadata.pallets_by_index = metadata
        .pallets
        .values()
        .iter()
        .enumerate()
        .map(|(pos, p)| (p.index, pos))
        .collect();

    let map_ids = metadata.types.retain(|id| type_set.seen_ids.contains(&id));

    update_outer_enums(&mut metadata.outer_enums, &map_ids);
    for pallets in metadata.pallets.values_mut() {
        update_pallet_types(pallets, &map_ids);
    }
    update_extrinsic_types(&mut metadata.extrinsic, &map_ids);
    update_type(&mut metadata.runtime_ty, &map_ids);
    update_runtime_api_types(metadata.apis.values_mut(), &map_ids);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Metadata;
    use codec::Decode;
    use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
    use std::{fs, path::Path};

    fn load_metadata() -> Metadata {
        let bytes = fs::read(Path::new("../artifacts/polkadot_metadata_full.scale"))
            .expect("Cannot read metadata blob");
        let meta: RuntimeMetadataPrefixed =
            Decode::decode(&mut &*bytes).expect("Cannot decode scale metadata");

        match meta.1 {
            RuntimeMetadata::V14(v14) => v14.try_into().unwrap(),
            RuntimeMetadata::V15(v15) => v15.try_into().unwrap(),
            _ => panic!("Unsupported metadata version {:?}", meta.1),
        }
    }

    #[test]
    fn retain_one_pallet() {
        let metadata_cache = load_metadata();

        // Retain one pallet at a time ensuring the test does not panic.
        for pallet in metadata_cache.pallets() {
            let original_meta = metadata_cache.clone();
            let mut metadata = metadata_cache.clone();
            retain_metadata(
                &mut metadata,
                |pallet_name| pallet_name == pallet.name(),
                |_| true,
            );

            assert!(
                metadata.pallets.len() < original_meta.pallets.len(),
                "Stripped metadata must have less pallets than the non-stripped one: stripped amount {}, original amount {}",
                metadata.pallets.len(), original_meta.pallets.len()
            );

            assert!(
                metadata.types.types.len() < original_meta.types.types.len(),
                "Stripped metadata must have less retained types than the non-stripped one: stripped amount {}, original amount {}",
                metadata.types.types.len(), original_meta.types.types.len()
            )
        }
    }

    #[test]
    fn retain_one_runtime_api() {
        let metadata_cache = load_metadata();

        // Retain one runtime API at a time ensuring the test does not panic.
        for runtime_api in metadata_cache.runtime_api_traits() {
            let mut metadata = metadata_cache.clone();
            retain_metadata(
                &mut metadata,
                |_| true,
                |runtime_api_name| runtime_api_name == runtime_api.name(),
            );

            assert_eq!(metadata.apis.len(), 1);
            assert_eq!(
                &*metadata.apis.get_by_index(0).unwrap().name,
                runtime_api.name()
            );
        }
    }
}
