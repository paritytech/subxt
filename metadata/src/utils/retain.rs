// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utility functions to generate a subset of the metadata.

use crate::utils::ordered_map::OrderedMap;
use crate::{
    ExtrinsicMetadata, Metadata, OuterEnumsMetadata, PalletMetadataInner, RuntimeApiMetadataInner,
    StorageEntryType,
};
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use hashbrown::HashSet;
use scale_info::{
    form::PortableForm, PortableType, TypeDef, TypeDefArray, TypeDefBitSequence, TypeDefCompact,
    TypeDefComposite, TypeDefSequence, TypeDefTuple, TypeDefVariant,
};

struct TypeSet {
    seen_ids: HashSet<u32>,
}

impl TypeSet {
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
}

fn update_types(metadata: &Metadata, map_ids: BTreeMap<u32, u32>, t: &PortableType) {
    let mut work_set = VecDeque::from([t]);
    while let Some(typ) = work_set.pop_front() {
        match &typ.ty.type_def {
            TypeDef::Composite(TypeDefComposite { fields }) => {
                for field in fields {
                    let ty = resolve_typ(metadata, field.ty.id);
                    work_set.push_back(ty);
                }
            }
            TypeDef::Variant(TypeDefVariant { variants }) => {
                for variant in variants {
                    for field in &variant.fields {
                        let ty = resolve_typ(metadata, field.ty.id);
                        work_set.push_back(ty);
                    }
                }
            }
            TypeDef::Array(TypeDefArray { len: _, type_param })
            | TypeDef::Sequence(TypeDefSequence { type_param })
            | TypeDef::Compact(TypeDefCompact { type_param }) => {
                let ty = resolve_typ(metadata, type_param.id);
                work_set.push_back(ty);
            }
            TypeDef::Tuple(TypeDefTuple { fields }) => {
                for field in fields {
                    let ty = resolve_typ(metadata, field.id);
                    work_set.push_back(ty);
                }
            }
            TypeDef::Primitive(_) => (),
            TypeDef::BitSequence(TypeDefBitSequence {
                bit_store_type,
                bit_order_type,
            }) => {
                for typ in [bit_order_type, bit_store_type] {
                    let ty = resolve_typ(metadata, typ.id);
                    work_set.push_back(ty);
                }
            }
        }
    }
}

fn resolve_typ(metadata: &Metadata, typ: u32) -> &PortableType {
    metadata
        .types
        .types
        .get(typ as usize)
        .expect("Metadata should contain enum type in registry")
}
/// Collect all type IDs needed to represent the provided pallet.
fn collect_pallet_types2(
    pallet: &PalletMetadataInner,
    seen_set: &mut TypeSet,
    metadata: &Metadata,
) {
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
        if seen_set.seen_ids.insert(id) {
            let typ = resolve_typ(metadata, id);
            seen_set.collect_types(metadata, typ)
        }
    }
}
fn update_filtered_pallet(pallet: &mut PalletMetadataInner, seen_set: &mut TypeSet) {
    pallet.storage.as_mut().and_then(|storage| {
        storage.retain_entries(|entry| match entry.entry_type {
            StorageEntryType::Plain(ty) => seen_set.seen_ids.contains(&ty),
            StorageEntryType::Map {
                key_ty, value_ty, ..
            } => seen_set.seen_ids.contains(&key_ty) && seen_set.seen_ids.contains(&value_ty),
        });
        if storage.entries().len() == 0 {
            None
        } else {
            Some(storage)
        }
    });
    pallet.storage = None;
    pallet.call_ty.and_then(|ty| {
        if seen_set.seen_ids.contains(&ty) {
            Some(ty)
        } else {
            None
        }
    });

    pallet.event_ty.and_then(|ty| {
        if seen_set.seen_ids.contains(&ty) {
            Some(ty)
        } else {
            None
        }
    });

    pallet.error_ty.and_then(|ty| {
        if seen_set.seen_ids.contains(&ty) {
            Some(ty)
        } else {
            None
        }
    });

    pallet
        .constants
        .retain(|value| seen_set.seen_ids.contains(&value.ty));

    pallet.constants = OrderedMap::new();
}

/// Collect all type IDs needed to represent the runtime APIs.
fn collect_runtime_api_types2(
    api: &RuntimeApiMetadataInner,
    seen_set: &mut TypeSet,
    metadata: &Metadata,
) {
    for method in api.methods.values() {
        for input in &method.inputs {
            if seen_set.seen_ids.insert(input.ty) {
                let typ = resolve_typ(metadata, input.ty);
                seen_set.collect_types(metadata, typ);
            }
        }
        if seen_set.seen_ids.insert(method.output_ty) {
            let typ = resolve_typ(metadata, method.output_ty);
            seen_set.collect_types(metadata, typ);
        }
    }
}
fn collect_extrinsic_types2(
    extrinsic: &ExtrinsicMetadata,
    seen_set: &mut TypeSet,
    metadata: &Metadata,
) {
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
        if seen_set.seen_ids.insert(id) {
            let typ = resolve_typ(metadata, id);
            seen_set.collect_types(metadata, typ)
        }
    }
}

/// Collect all type IDs needed to represent the provided pallet.
fn collect_pallet_types(pallet: &PalletMetadataInner, type_ids: &mut HashSet<u32>) {
    if let Some(storage) = &pallet.storage {
        for entry in storage.entries() {
            match entry.entry_type {
                StorageEntryType::Plain(ty) => {
                    type_ids.insert(ty);
                }
                StorageEntryType::Map {
                    key_ty, value_ty, ..
                } => {
                    type_ids.insert(key_ty);
                    type_ids.insert(value_ty);
                }
            }
        }
    }

    if let Some(ty) = pallet.call_ty {
        type_ids.insert(ty);
    }

    if let Some(ty) = pallet.event_ty {
        type_ids.insert(ty);
    }

    for constant in pallet.constants.values() {
        type_ids.insert(constant.ty);
    }

    if let Some(ty) = pallet.error_ty {
        type_ids.insert(ty);
    }
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

/// Collect all type IDs needed to represent the extrinsic metadata.
fn collect_extrinsic_types(extrinsic: &ExtrinsicMetadata, type_ids: &mut HashSet<u32>) {
    type_ids.insert(extrinsic.address_ty);
    type_ids.insert(extrinsic.call_ty);
    type_ids.insert(extrinsic.signature_ty);
    type_ids.insert(extrinsic.extra_ty);

    for signed in &extrinsic.signed_extensions {
        type_ids.insert(signed.extra_ty);
        type_ids.insert(signed.additional_ty);
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

/// Collect all type IDs needed to represent the runtime APIs.
fn collect_runtime_api_types(api: &RuntimeApiMetadataInner, type_ids: &mut HashSet<u32>) {
    for method in api.methods.values() {
        for input in &method.inputs {
            type_ids.insert(input.ty);
        }
        type_ids.insert(method.output_ty);
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

/// Collect the outer enums type IDs.
fn collect_outer_enums(enums: &OuterEnumsMetadata, type_ids: &mut HashSet<u32>) {
    type_ids.insert(enums.call_enum_ty);
    type_ids.insert(enums.event_enum_ty);
    type_ids.insert(enums.error_enum_ty);
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

/// Retain the enum type identified by ID and keep only the variants that
/// match the provided filter.
fn retain_variants_in_enum_type<F>(metadata: &mut Metadata, id: u32, mut filter: F)
where
    F: FnMut(&str) -> bool,
{
    let ty = metadata
        .types
        .types
        .get_mut(id as usize)
        .expect("Metadata should contain enum type in registry");

    let TypeDef::Variant(variant) = &mut ty.ty.type_def else {
        panic!("Metadata type is expected to be a variant type");
    };

    // Remove all variants from the type that aren't the pallet(s) we want to keep.
    variant.variants.retain(|v| filter(&v.name));
}

fn collect_variants_in_type2<F>(seen_set: &mut TypeSet, metadata: &Metadata, id: u32, mut filter: F)
where
    F: FnMut(&TypeDef<PortableForm>) -> (),
{
    let ty = metadata
        .types
        .types
        .get(id as usize)
        .expect("Metadata should contain enum type in registry");

    filter(&ty.ty.type_def);

    seen_set.seen_ids.insert(id);

    seen_set.collect_types(metadata, ty);
}

/// Strip any pallets out of the outer enum types that aren't the ones we want to keep.
fn retain_pallets_in_runtime_outer_types<F>(metadata: &mut Metadata, mut filter: F)
where
    F: FnMut(&str) -> bool,
{
    retain_variants_in_enum_type(metadata, metadata.outer_enums.call_enum_ty, &mut filter);
    retain_variants_in_enum_type(metadata, metadata.outer_enums.event_enum_ty, &mut filter);
    retain_variants_in_enum_type(metadata, metadata.outer_enums.error_enum_ty, &mut filter);
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
    let mut type_set = TypeSet {
        seen_ids: HashSet::new(),
    };
    collect_variants_in_type2(
        &mut type_set,
        &metadata,
        metadata.outer_enums.call_enum_ty,
        |type_def| {
            let TypeDef::Variant(_) = type_def else {
                panic!("Metadata type is expected to be a variant type");
            };
        },
    );
    collect_variants_in_type2(
        &mut type_set,
        &metadata,
        metadata.outer_enums.event_enum_ty,
        |type_def| {
            let TypeDef::Variant(_) = type_def else {
                panic!("Metadata type is expected to be a variant type");
            };
        },
    );
    collect_variants_in_type2(
        &mut type_set,
        &metadata,
        metadata.outer_enums.error_enum_ty,
        |type_def| {
            let TypeDef::Variant(_) = type_def else {
                panic!("Metadata type is expected to be a variant type");
            };
        },
    );

    for pallet in metadata.pallets.values() {
        let should_retain = pallets_filter(&pallet.name);
        if should_retain {
            collect_pallet_types2(pallet, &mut type_set, &metadata);
        }
    }

    for api in metadata.apis.values() {
        let should_retain = runtime_apis_filter(&api.name);
        if should_retain {
            collect_runtime_api_types2(api, &mut type_set, metadata);
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
    collect_extrinsic_types2(&metadata.extrinsic, &mut type_set, &metadata);

    for pallet in metadata.pallets.values_mut() {
        if !pallets_filter(&pallet.name) {
            update_filtered_pallet(pallet, &mut type_set);
        }
    }

    metadata.pallets.retain(|pallet| {
        let should_retain = pallets_filter(&pallet.name);
        if !should_retain {
            !matches!(
                pallet,
                PalletMetadataInner {
                    storage: None,
                    call_ty: None,
                    event_ty: None,
                    error_ty: None,
                    constants: _,
                    ..
                }
            )
        } else {
            should_retain
        }
    });
    metadata.apis.retain(|api| {
        let should_retain = runtime_apis_filter(&api.name);
        should_retain
    });

    metadata.pallets_by_index = metadata
        .pallets
        .values()
        .iter()
        .enumerate()
        .map(|(pos, p)| (p.index, pos))
        .collect();

    let map_ids = metadata.types.retain(|id| type_set.seen_ids.contains(&id));
    // for (k, v) in map_ids.iter() {
    //     if *v == 96 {
    //         dbg!(k);
    //         dbg!(metadata.types.resolve(*k));
    //         dbg!(metadata.types.resolve(*v));
    //     }
    // }

    update_outer_enums(&mut metadata.outer_enums, &map_ids);
    for pallets in metadata.pallets.values_mut() {
        update_pallet_types(pallets, &map_ids);
    }
    update_extrinsic_types(&mut metadata.extrinsic, &map_ids);
    update_type(&mut metadata.runtime_ty, &map_ids);
    update_runtime_api_types(metadata.apis.values_mut(), &map_ids);
}

pub fn retain_metadata_old<F, G>(
    metadata: &mut Metadata,
    mut pallets_filter: F,
    mut runtime_apis_filter: G,
) where
    F: FnMut(&str) -> bool,
    G: FnMut(&str) -> bool,
{
    let mut type_ids = HashSet::new();

    // There are special outer enum types that point to all pallets types (call, error, event) by default.
    // This brings in a significant chunk of types. We trim this down to only include variants
    // for the pallets we're retaining, to avoid this.
    retain_pallets_in_runtime_outer_types(metadata, &mut pallets_filter);

    // Collect the stripped outer enums.
    collect_outer_enums(&metadata.outer_enums, &mut type_ids);

    // Filter our pallet list to only those pallets we want to keep. Keep hold of all
    // type IDs in the pallets we're keeping. Retain all, if no filter specified.
    metadata.pallets.retain(|pallet| {
        let should_retain = pallets_filter(&pallet.name);
        if should_retain {
            collect_pallet_types(pallet, &mut type_ids);
        }
        should_retain
    });

    // We index pallets by their u8 index for easy access. Rebuild this index.
    metadata.pallets_by_index = metadata
        .pallets
        .values()
        .iter()
        .enumerate()
        .map(|(pos, p)| (p.index, pos))
        .collect();

    // Keep the extrinsic stuff referenced in our metadata.
    collect_extrinsic_types(&metadata.extrinsic, &mut type_ids);

    // Keep the "runtime" type ID, since it's referenced in our metadata.
    type_ids.insert(metadata.runtime_ty);

    // Keep only the runtime API types that the filter allows for. Keep hold of all
    // type IDs in the runtime apis we're keeping. Retain all, if no filter specified.
    metadata.apis.retain(|api| {
        let should_retain = runtime_apis_filter(&api.name);
        if should_retain {
            collect_runtime_api_types(api, &mut type_ids);
        }
        should_retain
    });

    // Additionally, subxt depends on the `DispatchError` type existing; we use the same
    // logic here that is used when building our `Metadata`.
    let dispatch_error_ty = metadata
        .types
        .types
        .iter()
        .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
        .expect("Metadata must contain sp_runtime::DispatchError");
    type_ids.insert(dispatch_error_ty.id);

    // Now, keep the type IDs we've asked for. This recursively keeps any types referenced from these.
    // This will return a map from old to new type ID, because IDs may change.
    let map_ids = metadata.types.retain(|id| type_ids.contains(&id));

    // And finally, we can go and update all of our type IDs in the metadata as a result of this:
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
    use assert_matches::assert_matches;
    use codec::{Decode, Encode};
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
            let mut metadata = metadata_cache.clone();
            retain_metadata(
                &mut metadata,
                |pallet_name| pallet_name == pallet.name(),
                |_| true,
            );

            assert_eq!(metadata.pallets.len(), 1);
            assert_eq!(
                &*metadata.pallets.get_by_index(0).unwrap().name,
                pallet.name()
            );

            let id = metadata.outer_enums().call_enum_ty;
            let ty = metadata.types.resolve(id).unwrap();
            let num_variants = if pallet.call_ty_id().is_some() { 1 } else { 0 };
            assert_matches!(&ty.type_def, TypeDef::Variant(variant) if variant.variants.len() == num_variants);

            let id = metadata.outer_enums().error_enum_ty;
            let ty = metadata.types.resolve(id).unwrap();
            let num_variants = if pallet.error_ty_id().is_some() { 1 } else { 0 };
            assert_matches!(&ty.type_def, TypeDef::Variant(variant) if variant.variants.len() == num_variants);

            let id = metadata.outer_enums().event_enum_ty;
            let ty = metadata.types.resolve(id).unwrap();
            let num_variants = if pallet.event_ty_id().is_some() { 1 } else { 0 };
            assert_matches!(&ty.type_def, TypeDef::Variant(variant) if variant.variants.len() == num_variants);
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

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let files = [(172, "full", "./test_data/metadata.scale.txt")];

        for (type_id, name, file) in files {
            println!("###################################");
            println!("Metadata: {name}");
            println!("###################################\n");

            let md_file = std::fs::read(file).expect("cannot read metadata");
            let md = Metadata::decode(&mut &*md_file).expect("cannot decode metadata");

            let outer_enum_hashes = crate::OuterEnumHashes::empty();
            let hash =
                crate::utils::validation::get_type_hash(md.types(), type_id, &outer_enum_hashes);
            let mut new_md = md.clone();
            retain_metadata(
                &mut new_md,
                |name| {
                    let list = "Balances,Timestamp,Contracts,ContractsEvm,System"
                        .split(",")
                        .collect::<Vec<&str>>();
                    list.iter().any(|s| *s == name)
                },
                |_| true,
            );
            let outer_enum_hashes = crate::OuterEnumHashes::empty();
            let new_hash = crate::utils::validation::get_type_hash(
                new_md.types(),
                type_id,
                &outer_enum_hashes,
            );

            // let mut old_md = md.clone();
            // retain_metadata_old(
            //     &mut old_md,
            //     |name| {
            //         let list = "Balances,Timestamp,Contracts,ContractsEvm,System"
            //             .split(",")
            //             .collect::<Vec<&str>>();
            //         list.iter().any(|s| *s == name)
            //     },
            //     |_| true,
            // );

            for typ in &new_md.types.types {
                assert_eq!(
                    new_md.type_hash(typ.id),
                    md.type_hash(typ.id),
                    "type_id {} {} \n type {:?} {:?}",
                    typ.id,
                    typ.ty.path.ident().unwrap(),
                    typ.ty.type_def,
                    md.types.resolve(typ.id).unwrap(),
                );
            }

            println!("\n{:?}\n{:?}\n", hash, new_hash);
        }

        Ok(())
    }
    // #[test]
    // fn retain_size() {
    //     let metadata_cache = load_metadata();

    //     // Retain one pallet at a time ensuring the test does not panic.
    //     for pallet in metadata_cache.pallets() {
    //         let mut metadata = metadata_cache.clone();
    //         let cloned_md = metadata.clone();
    //         retain_metadata(
    //             &mut metadata,
    //             |pallet_name| pallet_name == pallet.name(),
    //             |_| true,
    //         );
    //         (cloned_md.types.types.len(), metadata.types.types.len());
    //     }
    //     assert!(false)
    // }
}
