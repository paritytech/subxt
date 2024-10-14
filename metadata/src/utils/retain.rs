// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utility functions to generate a subset of the metadata.

use crate::{
    ExtrinsicMetadata, Metadata, OuterEnumsMetadata, PalletMetadataInner, RuntimeApiMetadataInner,
    StorageEntryType,
};
use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use scale_info::{
    PortableType, TypeDef, TypeDefArray, TypeDefBitSequence, TypeDefCompact, TypeDefComposite,
    TypeDefSequence, TypeDefTuple, TypeDefVariant,
};

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

    fn insert(&mut self, id: u32) -> bool {
        self.seen_ids.insert(id)
    }

    fn contains(&mut self, id: u32) -> bool {
        self.seen_ids.contains(&id)
    }

    /// This function will deeply traverse the inital type and it's dependencies to collect the relevant type_ids
    fn collect_types(&mut self, metadata: &Metadata, t: &PortableType) {
        let mut work_set = VecDeque::from([t]);
        while let Some(typ) = work_set.pop_front() {
            match &typ.ty.type_def {
                TypeDef::Composite(TypeDefComposite { fields }) => {
                    for field in fields {
                        if self.insert(field.ty.id) {
                            let ty = resolve_typ(metadata, field.ty.id);
                            work_set.push_back(ty);
                        }
                    }
                }
                TypeDef::Variant(TypeDefVariant { variants }) => {
                    for variant in variants {
                        for field in &variant.fields {
                            if self.insert(field.ty.id) {
                                let ty = resolve_typ(metadata, field.ty.id);
                                work_set.push_back(ty);
                            }
                        }
                    }
                }
                TypeDef::Array(TypeDefArray { len: _, type_param })
                | TypeDef::Sequence(TypeDefSequence { type_param })
                | TypeDef::Compact(TypeDefCompact { type_param }) => {
                    if self.insert(type_param.id) {
                        let ty = resolve_typ(metadata, type_param.id);
                        work_set.push_back(ty);
                    }
                }
                TypeDef::Tuple(TypeDefTuple { fields }) => {
                    for field in fields {
                        if self.insert(field.id) {
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
                        if self.insert(typ.id) {
                            let ty = resolve_typ(metadata, typ.id);
                            work_set.push_back(ty);
                        }
                    }
                }
            }
        }
    }

    fn insert_and_collect(&mut self, metadata: &Metadata, id: u32) {
        if self.insert(id) {
            let t = resolve_typ(metadata, id);
            self.collect_types(metadata, t);
        }
    }

    fn collect_extrinsic_types(&mut self, extrinsic: &ExtrinsicMetadata) {
        for ty in [
            extrinsic.address_ty,
            extrinsic.call_ty,
            extrinsic.signature_ty,
            extrinsic.extra_ty,
        ] {
            self.insert(ty);
        }

        for signed in &extrinsic.signed_extensions {
            self.insert(signed.extra_ty);
            self.insert(signed.additional_ty);
        }
    }

    /// Collect all type IDs needed to represent the runtime APIs.
    fn collect_runtime_api_types(&mut self, metadata: &Metadata, api: &RuntimeApiMetadataInner) {
        for method in api.methods.values() {
            for input in &method.inputs {
                self.insert_and_collect(metadata, input.ty);
            }
            self.insert_and_collect(metadata, method.output_ty);
        }
    }

    /// Collect all type IDs needed to represent the provided pallet.
    fn collect_pallet_types(&mut self, pallet: &PalletMetadataInner, metadata: &Metadata) {
        if let Some(storage) = &pallet.storage {
            for entry in storage.entries() {
                match entry.entry_type {
                    StorageEntryType::Plain(ty) => {
                        self.insert_and_collect(metadata, ty);
                    }
                    StorageEntryType::Map {
                        key_ty, value_ty, ..
                    } => {
                        self.insert_and_collect(metadata, key_ty);
                        self.insert_and_collect(metadata, value_ty);
                    }
                }
            }
        }

        if let Some(ty) = pallet.call_ty {
            self.insert_and_collect(metadata, ty);
        }

        if let Some(ty) = pallet.event_ty {
            self.insert_and_collect(metadata, ty);
        }

        for constant in pallet.constants.values() {
            self.insert_and_collect(metadata, constant.ty);
        }

        if let Some(ty) = pallet.error_ty {
            self.insert_and_collect(metadata, ty);
        }
    }

    // Collect the types in outerEnums
    // If the type wasn't previously collected we can safely strip some of the variants
    fn collect_variants_in_type<F>(&mut self, metadata: &mut Metadata, id: u32, mut name_filter: F)
    where
        F: FnMut(&str) -> bool,
    {
        let ty = {
            metadata
                .types
                .types
                .get_mut(id as usize)
                .expect("Metadata should contain enum type in registry")
        };

        let TypeDef::Variant(variant) = &mut ty.ty.type_def else {
            panic!("Metadata type is expected to be a variant type");
        };

        // If the type was not referenced earlier we can safely strip some of the variants
        if self.insert(id) {
            variant.variants.retain(|v| name_filter(&v.name));
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
    // all types that we intend to keep
    let mut type_set = TypeSet::new();

    // Do a deep traversal over the pallet types first
    for pallet in metadata.pallets.values() {
        let should_retain = pallets_filter(&pallet.name);
        if should_retain {
            type_set.collect_pallet_types(pallet, metadata);
        }
    }

    // Do a deep traversal over the `Runtime apis` input/output types
    for api in metadata.apis.values() {
        let should_retain = runtime_apis_filter(&api.name);
        if should_retain {
            type_set.collect_runtime_api_types(metadata, api);
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
    type_set.insert(dispatch_error_ty.id);
    type_set.insert(metadata.runtime_ty);
    type_set.collect_extrinsic_types(&metadata.extrinsic);

    // Collect the outer enums type IDs.
    for typ in [
        metadata.outer_enums.call_enum_ty,
        metadata.outer_enums.event_enum_ty,
        metadata.outer_enums.error_enum_ty,
    ] {
        type_set.collect_variants_in_type(metadata, typ, &mut pallets_filter);
    }

    // Retain the apis
    metadata.apis.retain(|api| runtime_apis_filter(&api.name));

    // Filter out unnecesary pallets that have no entries
    // and then re-index pallets after removing some of them above
    metadata
        .pallets
        .retain(|pallet| pallets_filter(&pallet.name));
    metadata.pallets_by_index = metadata
        .pallets
        .values()
        .iter()
        .enumerate()
        .map(|(pos, p)| (p.index, pos))
        .collect();

    let map_ids = metadata.types.retain(|id| type_set.contains(id));

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

    fn load_metadata_polkadot() -> Metadata {
        load_metadata("../artifacts/polkadot_metadata_full.scale")
    }

    fn load_metadata(path: impl AsRef<Path>) -> Metadata {
        let bytes = fs::read(path).expect("Cannot read metadata blob");
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
        let metadata_cache = load_metadata_polkadot();

        // Retain one pallet at a time ensuring the test does not panic.
        for pallet in metadata_cache.pallets() {
            let original_meta = metadata_cache.clone();
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

            assert!(
                metadata.types.types.len() < original_meta.types.types.len(),
                "Stripped metadata must have less retained types than the non-stripped one: stripped amount {}, original amount {}",
                metadata.types.types.len(), original_meta.types.types.len()
            );
        }
    }

    #[test]
    fn retain_one_runtime_api() {
        let metadata_cache = load_metadata_polkadot();

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
    fn issue_1659() {
        let metadata_cache = load_metadata("../artifacts/regressions/1659.scale");

        // Strip metadata to the pallets as described in the issue.
        let mut stripped_metadata = metadata_cache.clone();
        retain_metadata(
            &mut stripped_metadata,
            {
                let set = "Balances,Timestamp,Contracts,ContractsEvm,System"
                    .split(",")
                    .collect::<BTreeSet<&str>>();
                move |s| set.contains(&s)
            },
            |_| true,
        );

        // check that call_enum did not change as it is referenced inside runtime_api
        assert_eq!(
            stripped_metadata.type_hash(stripped_metadata.outer_enums.call_enum_ty),
            metadata_cache.type_hash(metadata_cache.outer_enums.call_enum_ty)
        );

        // check that event_num did not change as it is referenced inside runtime_api
        assert_eq!(
            stripped_metadata.type_hash(stripped_metadata.outer_enums.event_enum_ty),
            metadata_cache.type_hash(metadata_cache.outer_enums.event_enum_ty)
        );
    }
}
