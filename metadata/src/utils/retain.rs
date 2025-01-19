// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utility functions to generate a subset of the metadata.

use crate::{
    ExtrinsicMetadata, Metadata, PalletMetadataInner, RuntimeApiMetadataInner, StorageEntryType,
};
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use scale_info::{
    PortableType, TypeDef, TypeDefArray, TypeDefBitSequence, TypeDefCompact, TypeDefComposite,
    TypeDefSequence, TypeDefTuple, TypeDefVariant,
};

#[derive(Clone)]
struct TypeSet {
    seen_ids: BTreeSet<u32>,
    pub work_set: Vec<u32>,
}

impl TypeSet {
    fn new() -> Self {
        Self {
            seen_ids: BTreeSet::new(),
            // Average work set size is around 30-50 elements, depending on the metadata size
            work_set: Vec::with_capacity(32),
        }
    }

    fn insert(&mut self, id: u32) -> bool {
        self.seen_ids.insert(id)
    }

    fn contains(&mut self, id: u32) -> bool {
        self.seen_ids.contains(&id)
    }

    fn push_to_workset(&mut self, id: u32) {
        // Check if wee hit a type we've already inserted; avoid infinite loops and stop.
        if self.insert(id) {
            self.work_set.push(id);
        }
    }

    /// This function will deeply traverse the initial type and it's dependencies to collect the relevant type_ids
    fn collect_types(&mut self, metadata: &Metadata, id: u32) {
        self.push_to_workset(id);
        while let Some(typ) = self.work_set.pop() {
            let typ = resolve_typ(metadata, typ);
            match &typ.ty.type_def {
                TypeDef::Composite(TypeDefComposite { fields }) => {
                    for field in fields {
                        self.push_to_workset(field.ty.id);
                    }
                }
                TypeDef::Variant(TypeDefVariant { variants }) => {
                    for variant in variants {
                        for field in &variant.fields {
                            self.push_to_workset(field.ty.id);
                        }
                    }
                }
                TypeDef::Array(TypeDefArray { len: _, type_param })
                | TypeDef::Sequence(TypeDefSequence { type_param })
                | TypeDef::Compact(TypeDefCompact { type_param }) => {
                    self.push_to_workset(type_param.id);
                }
                TypeDef::Tuple(TypeDefTuple { fields }) => {
                    for field in fields {
                        self.push_to_workset(field.id);
                    }
                }
                TypeDef::Primitive(_) => (),
                TypeDef::BitSequence(TypeDefBitSequence {
                    bit_store_type,
                    bit_order_type,
                }) => {
                    for typ in [bit_order_type, bit_store_type] {
                        self.push_to_workset(typ.id);
                    }
                }
            }
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
            self.collect_types(metadata, method.output_ty);
        }
    }

    /// Collect all type IDs needed to represent the provided pallet.
    fn collect_pallet_types(&mut self, pallet: &PalletMetadataInner, metadata: &Metadata) {
        if let Some(storage) = &pallet.storage {
            for entry in storage.entries() {
                match entry.entry_type {
                    StorageEntryType::Plain(ty) => {
                        self.collect_types(metadata, ty);
                    }
                    StorageEntryType::Map {
                        key_ty, value_ty, ..
                    } => {
                        self.collect_types(metadata, key_ty);
                        self.collect_types(metadata, value_ty);
                    }
                }
            }
        }

        for constant in pallet.constants.values() {
            self.collect_types(metadata, constant.ty);
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
    // 1. Delete pallets we don't want to keep.
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

    // 2. Delete runtime APIs we don't want to keep.
    metadata.apis.retain(|api| runtime_apis_filter(&api.name));

    // 3. For each outer enum type, strip it if possible, ie if it is not returned by any
    // of the things we're keeping (because if it is, we need to keep all of it so that we
    // can still decode values into it).
    let outer_enums = metadata.outer_enums();
    let mut find_type_id = keep_outer_enum(metadata, &mut pallets_filter, &mut runtime_apis_filter);
    for outer_enum_ty_id in [
        outer_enums.call_enum_ty(),
        outer_enums.error_enum_ty(),
        outer_enums.event_enum_ty(),
    ] {
        if !find_type_id(outer_enum_ty_id) {
            strip_variants_in_enum_type(metadata, &mut pallets_filter, outer_enum_ty_id);
        }
    }

    // 4. Collect all of the type IDs we still want to keep after deleting.
    let mut keep_these_type_ids: BTreeSet<u32> =
        iterate_metadata_types(metadata).map(|x| *x).collect();

    // 5. Additionally, subxt depends on the `DispatchError` type existing; we use the same
    // logic here that is used when building our `Metadata` to ensure we keep it too.
    let dispatch_error_ty = metadata
        .types
        .types
        .iter()
        .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
        .expect("Metadata must contain sp_runtime::DispatchError");

    keep_these_type_ids.insert(dispatch_error_ty.id);

    // 5. Strip all of the type IDs we no longer need, based on the above set.
    let map_ids = metadata
        .types
        .retain(|id| keep_these_type_ids.contains(&id));

    // 6. Now, update the type IDs referenced in our metadata to reflect this.
    for id in iterate_metadata_types(metadata) {
        if let Some(new_id) = map_ids.get(id) {
            *id = *new_id;
        } else {
            panic!("Type id {id} was not retained. This is a bug");
        }
    }
}

fn strip_variants_in_enum_type<F>(metadata: &mut Metadata, mut pallets_filter: F, id: u32)
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

    variant.variants.retain(|v| pallets_filter(&v.name));
}

/// Returns an iterator that allows modifying each type ID seen in the metadata (not recursively).
/// This will iterate over every type referenced in the metadata outside of `metadata.types`.
fn iterate_metadata_types(metadata: &mut Metadata) -> impl Iterator<Item = &mut u32> {
    let mut types = alloc::vec::Vec::new();

    // collect outer_enum top-level types
    let outer_enum = &mut metadata.outer_enums;
    types.push(&mut outer_enum.call_enum_ty);
    types.push(&mut outer_enum.event_enum_ty);
    types.push(&mut outer_enum.error_enum_ty);

    // collect pallet top-level type ids
    for pallet in metadata.pallets.values_mut() {
        if let Some(storage) = &mut pallet.storage {
            for entry in storage.entries.values_mut() {
                match &mut entry.entry_type {
                    StorageEntryType::Plain(ty) => {
                        types.push(ty);
                    }
                    StorageEntryType::Map {
                        key_ty, value_ty, ..
                    } => {
                        types.push(key_ty);
                        types.push(value_ty);
                    }
                }
            }
        };
        if let Some(ty) = &mut pallet.call_ty {
            types.push(ty);
        }

        if let Some(ty) = &mut pallet.event_ty {
            types.push(ty);
        }

        if let Some(ty) = &mut pallet.error_ty {
            types.push(ty);
        }

        for constant in pallet.constants.values_mut() {
            types.push(&mut constant.ty);
        }
    }

    // collect extrinsic type_ids
    for ty in [
        &mut metadata.extrinsic.extra_ty,
        &mut metadata.extrinsic.address_ty,
        &mut metadata.extrinsic.signature_ty,
        &mut metadata.extrinsic.call_ty,
    ] {
        types.push(ty);
    }

    for signed in &mut metadata.extrinsic.signed_extensions {
        types.push(&mut signed.extra_ty);
        types.push(&mut signed.additional_ty);
    }

    types.push(&mut metadata.runtime_ty);

    // collect runtime_api_types
    for api in metadata.apis.values_mut() {
        for method in api.methods.values_mut() {
            for input in &mut method.inputs.iter_mut() {
                types.push(&mut input.ty);
            }
            types.push(&mut method.output_ty);
        }
    }

    types.into_iter()
}

/// Look for a type ID anywhere that we can be given back, ie in constants, storage, extrinsics or runtime API return types.
/// This will recurse deeply into those type IDs to find them.
pub fn keep_outer_enum<F, G>(
    metadata: &Metadata,
    pallets_filter: &mut F,
    runtime_apis_filter: &mut G,
) -> impl FnMut(u32) -> bool
where
    F: FnMut(&str) -> bool,
    G: FnMut(&str) -> bool,
{
    let mut type_set = TypeSet::new();
    for pallet in metadata.pallets.values() {
        if pallets_filter(&pallet.name) {
            type_set.collect_pallet_types(pallet, metadata);
        }
    }
    for api in metadata.apis.values() {
        if runtime_apis_filter(&api.name) {
            type_set.collect_runtime_api_types(metadata, api);
        }
    }
    type_set.collect_extrinsic_types(&metadata.extrinsic);
    move |type_id| type_set.contains(type_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Metadata;
    use codec::Decode;
    use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
    use std::{fs, path::Path};

    fn load_metadata() -> Metadata {
        load_metadata_custom("../artifacts/polkadot_metadata_full.scale")
    }

    fn load_metadata_custom(path: impl AsRef<Path>) -> Metadata {
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
    fn issue_1659() {
        let full_metadata = load_metadata_custom("../artifacts/regressions/1659.scale");
        // Strip metadata to the pallets as described in the issue.
        let mut stripped_metadata = full_metadata.clone();
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
            full_metadata.type_hash(full_metadata.outer_enums.call_enum_ty)
        );

        // check that event_num did not change as it is referenced inside runtime_api
        assert_eq!(
            stripped_metadata.type_hash(stripped_metadata.outer_enums.event_enum_ty),
            full_metadata.type_hash(full_metadata.outer_enums.event_enum_ty)
        );
    }
}
