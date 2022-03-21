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

//! Wrapper to obtain unique deterministic hashed identifiers from portable type ids.
//!
//! # Note
//!
//! Used to determine API compatibility between generated interface and dynamic metadata.

use codec::Encode;
use frame_metadata::{
    ExtrinsicMetadata,
    RuntimeMetadataLastVersion,
    StorageEntryType,
};
use scale_info::{
    form::PortableForm,
    Field,
    PortableRegistry,
    TypeDef,
    Variant,
};
use std::collections::{
    HashMap,
    HashSet,
};

#[repr(u8)]
enum MetadataHashableIDs {
    Field,
    Variant,
    TypeDef,
    Type,
    Pallet,
    Extrinsic,
}

fn hash(bytes: &[u8]) -> [u8; 32] {
    sp_core::hashing::sha2_256(bytes)
}

fn get_field_hash(
    registry: &PortableRegistry,
    field: &Field<PortableForm>,
    visited_ids: &mut HashSet<u32>,
    cache: &mut MetadataHasherCache,
) -> [u8; 32] {
    let mut bytes = vec![MetadataHashableIDs::Field as u8];

    field.name().encode_to(&mut bytes);
    field.type_name().encode_to(&mut bytes);
    bytes.extend(get_type_hash(registry, field.ty().id(), visited_ids, cache));

    hash(&bytes)
}

fn get_variant_hash(
    registry: &PortableRegistry,
    var: &Variant<PortableForm>,
    visited_ids: &mut HashSet<u32>,
    cache: &mut MetadataHasherCache,
) -> [u8; 32] {
    let mut bytes = vec![MetadataHashableIDs::Variant as u8];

    var.name().encode_to(&mut bytes);
    for field in var.fields() {
        bytes.extend(get_field_hash(registry, field, visited_ids, cache));
    }

    hash(&bytes)
}

fn get_type_def_hash(
    registry: &PortableRegistry,
    ty_def: &TypeDef<PortableForm>,
    visited_ids: &mut HashSet<u32>,
    cache: &mut MetadataHasherCache,
) -> [u8; 32] {
    let mut bytes = vec![MetadataHashableIDs::TypeDef as u8];

    let data = match ty_def {
        TypeDef::Composite(composite) => {
            let mut bytes = Vec::new();
            for field in composite.fields() {
                bytes.extend(get_field_hash(registry, field, visited_ids, cache));
            }
            bytes
        }
        TypeDef::Variant(variant) => {
            let mut bytes = Vec::new();
            for var in variant.variants() {
                bytes.extend(get_variant_hash(registry, var, visited_ids, cache));
            }
            bytes
        }
        TypeDef::Sequence(sequence) => {
            let mut bytes = Vec::new();
            bytes.extend(get_type_hash(
                registry,
                sequence.type_param().id(),
                visited_ids,
                cache,
            ));
            bytes
        }
        TypeDef::Array(array) => {
            let mut bytes = Vec::new();
            array.len().encode_to(&mut bytes);
            bytes.extend(get_type_hash(
                registry,
                array.type_param().id(),
                visited_ids,
                cache,
            ));
            bytes
        }
        TypeDef::Tuple(tuple) => {
            let mut bytes = Vec::new();
            for field in tuple.fields() {
                bytes.extend(get_type_hash(registry, field.id(), visited_ids, cache));
            }
            bytes
        }
        TypeDef::Primitive(primitive) => {
            let mut bytes = Vec::new();
            primitive.encode_to(&mut bytes);
            bytes
        }
        TypeDef::Compact(compact) => {
            let mut bytes = Vec::new();
            bytes.extend(get_type_hash(
                registry,
                compact.type_param().id(),
                visited_ids,
                cache,
            ));
            bytes
        }
        TypeDef::BitSequence(bitseq) => {
            let mut bytes = Vec::new();
            bytes.extend(get_type_hash(
                registry,
                bitseq.bit_order_type().id(),
                visited_ids,
                cache,
            ));
            bytes.extend(get_type_hash(
                registry,
                bitseq.bit_store_type().id(),
                visited_ids,
                cache,
            ));
            bytes
        }
    };
    bytes.extend(data);
    hash(&bytes)
}

fn get_type_hash(
    registry: &PortableRegistry,
    id: u32,
    visited_ids: &mut HashSet<u32>,
    cache: &mut MetadataHasherCache,
) -> [u8; 32] {
    if let Some(cached) = cache.types.get(&id) {
        return *cached
    }

    let ty = registry.resolve(id).unwrap();

    let mut bytes = vec![MetadataHashableIDs::Type as u8];
    ty.path().segments().encode_to(&mut bytes);
    // Guard against recursive types
    if !visited_ids.insert(id) {
        return hash(&bytes)
    }

    let ty_def = ty.type_def();
    bytes.extend(get_type_def_hash(registry, ty_def, visited_ids, cache));

    let uid = hash(&bytes);
    cache.types.insert(id, uid);
    uid
}

fn get_extrinsic_hash(
    registry: &PortableRegistry,
    extrinsic: &ExtrinsicMetadata<PortableForm>,
    cache: &mut MetadataHasherCache,
) -> [u8; 32] {
    let mut visited_ids = HashSet::<u32>::new();
    let mut bytes = vec![MetadataHashableIDs::Extrinsic as u8];

    bytes.extend(get_type_hash(
        registry,
        extrinsic.ty.id(),
        &mut visited_ids,
        cache,
    ));
    bytes.push(extrinsic.version);
    for signed_extension in extrinsic.signed_extensions.iter() {
        signed_extension.identifier.encode_to(&mut bytes);
        bytes.extend(get_type_hash(
            registry,
            signed_extension.ty.id(),
            &mut visited_ids,
            cache,
        ));
        bytes.extend(get_type_hash(
            registry,
            signed_extension.additional_signed.id(),
            &mut visited_ids,
            cache,
        ));
    }

    hash(&bytes)
}

pub fn get_pallet_hash(
    registry: &PortableRegistry,
    pallet: &frame_metadata::PalletMetadata<PortableForm>,
    cache: &mut MetadataHasherCache,
) -> [u8; 32] {
    let mut bytes = vec![MetadataHashableIDs::Pallet as u8];
    let mut visited_ids = HashSet::<u32>::new();

    if let Some(ref calls) = pallet.calls {
        bytes.extend(get_type_hash(
            registry,
            calls.ty.id(),
            &mut visited_ids,
            cache,
        ));
    }
    if let Some(ref event) = pallet.event {
        bytes.extend(get_type_hash(
            registry,
            event.ty.id(),
            &mut visited_ids,
            cache,
        ));
    }
    for constant in pallet.constants.iter() {
        bytes.extend(constant.name.as_bytes());
        bytes.extend(&constant.value);
        bytes.extend(get_type_hash(
            registry,
            constant.ty.id(),
            &mut visited_ids,
            cache,
        ));
    }
    if let Some(ref error) = pallet.error {
        bytes.extend(get_type_hash(
            registry,
            error.ty.id(),
            &mut visited_ids,
            cache,
        ));
    }
    if let Some(ref storage) = pallet.storage {
        bytes.extend(storage.prefix.as_bytes());
        for entry in storage.entries.iter() {
            bytes.extend(entry.name.as_bytes());
            entry.modifier.encode_to(&mut bytes);
            match &entry.ty {
                StorageEntryType::Plain(ty) => {
                    bytes.extend(get_type_hash(
                        registry,
                        ty.id(),
                        &mut visited_ids,
                        cache,
                    ));
                }
                StorageEntryType::Map {
                    hashers,
                    key,
                    value,
                } => {
                    hashers.encode_to(&mut bytes);
                    bytes.extend(get_type_hash(
                        registry,
                        key.id(),
                        &mut visited_ids,
                        cache,
                    ));
                    bytes.extend(get_type_hash(
                        registry,
                        value.id(),
                        &mut visited_ids,
                        cache,
                    ));
                }
            }
            bytes.extend(&entry.default);
        }
    }

    hash(&bytes)
}

pub fn get_metadata_hash(metadata: &RuntimeMetadataLastVersion) -> [u8; 32] {
    let mut cache = MetadataHasherCache::new();

    // Collect all pairs of (pallet name, pallet hash).
    let mut pallets: Vec<(String, [u8; 32])> = metadata
        .pallets
        .iter()
        .map(|pallet| {
            let name = pallet.name.clone();
            let hash = get_pallet_hash(&metadata.types, pallet, &mut cache);
            (name, hash)
        })
        .collect();
    // Sort by pallet name to create a deterministic representation of the underlying metadata.
    pallets.sort_by_key(|key| key.1);

    // Note: pallet name is excluded from hashing.
    let mut bytes = Vec::with_capacity(pallets.len() * 32);
    for (_, hash) in pallets.iter() {
        bytes.extend(hash)
    }

    bytes.extend(get_extrinsic_hash(
        &metadata.types,
        &metadata.extrinsic,
        &mut cache,
    ));

    let mut visited_ids = HashSet::<u32>::new();
    bytes.extend(get_type_hash(
        &metadata.types,
        metadata.ty.id(),
        &mut visited_ids,
        &mut cache,
    ));

    hash(&bytes)
}

pub struct MetadataHasherCache {
    pub(crate) types: HashMap<u32, [u8; 32]>,
}

impl MetadataHasherCache {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }
}
