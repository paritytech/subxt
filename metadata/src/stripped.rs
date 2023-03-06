// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use frame_metadata::{
    ExtrinsicMetadata,
    PalletMetadata,
    RuntimeMetadataV14,
    StorageEntryMetadata,
    StorageEntryType,
};
use scale_info::{
    form::PortableForm,
    interner::UntrackedSymbol,
    Field,
    PortableRegistry,
    Registry,
    Type,
    TypeDef,
    Variant,
};
use std::{
    any::TypeId,
    collections::{
        BTreeSet,
        HashSet,
    },
};

/// Strip Error.
#[derive(Debug)]
pub enum StripError {
    /// Pallet not present in the provided metadata.
    PalletNotFound,
    /// Pallet not present in the provided metadata.
    TypeNotFound(u32),
}

/// Collect the type IDs from the given pallet.
fn collect_pallet_types(pallet: &PalletMetadata<PortableForm>) -> BTreeSet<u32> {
    let mut type_ids = BTreeSet::new();

    println!("Collect");
    println!("Pallet is : {:?}", pallet);

    if let Some(storage) = &pallet.storage {
        println!("storage : {:?}", storage);

        for entry in storage.entries.iter() {
            println!("Entry : {:?}", entry);

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
        let ty = calls.ty;
        type_ids.insert(ty.id());
    }

    if let Some(event) = &pallet.event {
        let ty = event.ty;
        type_ids.insert(ty.id());
    }

    for constant in pallet.constants.iter() {
        let ty = constant.ty;
        type_ids.insert(ty.id());
    }

    if let Some(error) = &pallet.error {
        let ty = error.ty;
        type_ids.insert(ty.id());
    }

    type_ids
}

/// Recursively add all type IDs needed to express the given type ID.
fn extend_type_id(
    registry: &PortableRegistry,
    id: u32,
    result: &mut BTreeSet<u32>,
    visited: &mut HashSet<u32>,
) -> Result<(), StripError> {
    if visited.contains(&id) {
        return Ok(())
    }
    visited.insert(id);

    let ty = registry.resolve(id).ok_or(StripError::TypeNotFound(id))?;

    let mut extended = Vec::new();
    // Add generic type params.
    for param in ty.type_params() {
        if let Some(ty) = param.ty() {
            extended.push(ty.id());
        }
    }

    match ty.type_def() {
        TypeDef::Composite(composite) => {
            for field in composite.fields() {
                extended.push(field.ty().id());
            }
        }
        TypeDef::Variant(variant) => {
            for var in variant.variants() {
                for field in var.fields() {
                    extended.push(field.ty().id());
                }
            }
        }
        TypeDef::Sequence(sequence) => {
            extended.push(sequence.type_param().id());
        }
        TypeDef::Array(array) => {
            extended.push(array.type_param().id());
        }
        TypeDef::Tuple(tuple) => {
            for ty in tuple.fields() {
                extended.push(ty.id());
            }
        }
        TypeDef::Primitive(_) => (),
        TypeDef::Compact(compact) => {
            extended.push(compact.type_param().id());
        }
        TypeDef::BitSequence(bit_sequence) => {
            extended.push(bit_sequence.bit_store_type().id());
            extended.push(bit_sequence.bit_order_type().id());
        }
    }

    for ext in extended {
        result.insert(ext);
        extend_type_id(registry, ext, result, visited)?;
    }

    Ok(())
}

/// Keep only the pallet inside the metadata.
pub fn keep_pallet<T: AsRef<str>>(
    metadata: RuntimeMetadataV14,
    pallet_name: T,
) -> Result<RuntimeMetadataV14, StripError> {
    let pallet = metadata
        .pallets
        .iter()
        .find(|pallet| pallet.name == pallet_name.as_ref());

    let Some(pallet) = pallet else {
        return Err(StripError::PalletNotFound)
    };

    // Collect type ids from the pallet.
    let mut type_ids = collect_pallet_types(pallet);

    println!("TypeIDs {:#?}", type_ids);

    // println!("TypeIDs {:#?}", type_ids);

    // Extend the type IDs with their dependencies
    let registry = &metadata.types;

    let mut result = BTreeSet::new();
    let mut visited = HashSet::new();
    for id in type_ids.iter() {
        extend_type_id(registry, *id, &mut result, &mut visited)?;
    }

    type_ids.extend(result.iter());

    println!("TypeIDs Extended {:#?}", type_ids);
    println!(
        "Pallet [{}]\n   stripped {} vs full {}\n",
        pallet_name.as_ref(),
        type_ids.len(),
        metadata.types.types().len()
    );

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use codec::Decode;
    use frame_metadata::{
        RuntimeMetadata,
        RuntimeMetadataPrefixed,
        RuntimeMetadataV14,
    };
    use scale_info::meta_type;
    use std::{
        fs,
        path::Path,
    };

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
    fn strip_pallet() {
        let metadata = load_metadata();

        keep_pallet(metadata, "AuthorityDiscovery").unwrap();

        // for pallet in metadata.pallets.iter() {
        //     let metadata = load_metadata();
        //     keep_pallet(metadata, &pallet.name).unwrap();
        // }
    }
}
