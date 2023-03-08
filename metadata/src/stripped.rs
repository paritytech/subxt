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
        HashMap,
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

    // println!("Collect");
    // println!("Pallet is : {:?}", pallet);

    if let Some(storage) = &pallet.storage {
        // println!("storage : {:?}", storage);

        for entry in storage.entries.iter() {
            // println!("Entry : {:?}", entry);

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
    for ty in metadata.types.types() {
        let Some(ident) = ty.ty().path().ident() else {
            continue
        };
        // Collect custom type IDs.
        if ident == "DispatchError" {
            type_ids.insert(ty.id());
        }
    }
    // sp_runtime:: DispatchError

    // Collect extrisic type IDs.
    type_ids.insert(metadata.extrinsic.ty.id());
    for signed in &metadata.extrinsic.signed_extensions {
        type_ids.insert(signed.ty.id());
        type_ids.insert(signed.additional_signed.id());
    }
    // Collect runtime type ID.
    type_ids.insert(metadata.ty.id());

    // Extend the type IDs with their dependencies
    let registry = &metadata.types;
    let mut result = BTreeSet::new();
    let mut visited = HashSet::new();
    for id in type_ids.iter() {
        extend_type_id(&registry, *id, &mut result, &mut visited)?;
    }

    type_ids.extend(result.iter());

    println!("TypeIDs {:#?}", type_ids);

    let mut registry = registry.clone();
    println!("old registry len: {}", registry.types().len());
    let res = registry.retain(type_ids.clone()).unwrap();
    println!("new registry len: {}", registry.types().len());

    println!("Res is {:?}", res);

    // Modify the metadata
    let mut new_metadata = metadata.clone();
    new_metadata.types = registry;
    let mut pallet_metadata = pallet.clone();
    if let Some(storage) = &mut pallet_metadata.storage {
        for entry in storage.entries.iter_mut() {
            match &mut entry.ty {
                StorageEntryType::Plain(plain) => {
                    let new_id = res.get(&plain.id()).expect("Expected ID to exist");
                    *plain = (*new_id).into();
                }
                StorageEntryType::Map {
                    hashers,
                    key,
                    value,
                } => {
                    let new_id = res.get(&key.id()).expect("Expected ID to exist");
                    *key = (*new_id).into();

                    let new_id = res.get(&value.id()).expect("Expected ID to exist");
                    *value = (*new_id).into();
                }
            }
        }
    }

    if let Some(calls) = &mut pallet_metadata.calls {
        let new_id = res.get(&calls.ty.id()).expect("Expected ID to exist");
        calls.ty = (*new_id).into();
    }

    if let Some(event) = &mut pallet_metadata.event {
        let new_id = res.get(&event.ty.id()).expect("Expected ID to exist");
        event.ty = (*new_id).into();
    }

    for constant in pallet_metadata.constants.iter_mut() {
        let new_id = res.get(&constant.ty.id()).expect("Expected ID to exist");
        constant.ty = (*new_id).into();
    }

    if let Some(error) = &mut pallet_metadata.error {
        let new_id = res.get(&error.ty.id()).expect("Expected ID to exist");
        error.ty = (*new_id).into();
    }

    new_metadata.pallets = vec![pallet_metadata];

    let new_id = res
        .get(&new_metadata.extrinsic.ty.id())
        .expect("Expected ID to exist");
    new_metadata.extrinsic.ty = (*new_id).into();

    for ext in new_metadata.extrinsic.signed_extensions.iter_mut() {
        let new_id = res.get(&ext.ty.id()).expect("Expected ID to exist");
        ext.ty = (*new_id).into();

        let new_id = res
            .get(&ext.additional_signed.id())
            .expect("Expected ID to exist");
        ext.additional_signed = (*new_id).into();
    }

    println!(
        "Stripped {number:>3} vs full {full} for pallet [{pallet}]",
        number = type_ids.len(),
        full = metadata.types.types().len(),
        pallet = pallet_name.as_ref(),
    );

    Ok(new_metadata)
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
        io::Write,
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
        use codec::Encode;

        for pallet in metadata.pallets.iter() {
            let metadata = load_metadata();
            let metadata = keep_pallet(metadata, &pallet.name).unwrap();
            let meta: RuntimeMetadataPrefixed = metadata.into();
            let bytes: Vec<u8> = meta.encode();

            let mut file =
                fs::File::create(format!("../artifacts/{}.scale", pallet.name)).unwrap();
            file.write_all(bytes.as_slice()).unwrap();
        }
    }
}
