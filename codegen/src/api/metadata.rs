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
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::Mutex,
};

use lazy_static::lazy_static;

#[repr(u8)]
enum MetadataHashableIDs {
    Field,
    Variant,
    TypeDef,
    Type,
    Pallet,
}

fn hash(bytes: &[u8]) -> [u8; 32] {
    sp_core::hashing::sha2_256(bytes)
}

fn get_field_hash(
    registry: &PortableRegistry,
    field: &Field<PortableForm>,
    set: &mut HashSet<u32>,
) -> [u8; 32] {
    let mut bytes = vec![MetadataHashableIDs::Field as u8];

    if let Some(name) = field.name() {
        bytes.extend(name.as_bytes());
    }
    if let Some(ty_name) = field.type_name() {
        bytes.extend(ty_name.as_bytes());
    }
    bytes.extend(get_type_hash(registry, field.ty().id(), set));

    hash(&bytes)
}

fn get_variant_hash(
    registry: &PortableRegistry,
    var: &Variant<PortableForm>,
    set: &mut HashSet<u32>,
) -> [u8; 32] {
    let mut bytes = vec![MetadataHashableIDs::Variant as u8];

    bytes.extend(var.name().as_bytes());
    for field in var.fields() {
        bytes.extend(get_field_hash(registry, field, set));
    }

    hash(&bytes)
}

fn get_type_def_hash(
    registry: &PortableRegistry,
    ty_def: &TypeDef<PortableForm>,
    set: &mut HashSet<u32>,
) -> [u8; 32] {
    let mut bytes = vec![MetadataHashableIDs::TypeDef as u8];

    let data = match ty_def {
        TypeDef::Composite(composite) => {
            let mut bytes = Vec::new();
            for field in composite.fields() {
                bytes.extend(get_field_hash(registry, field, set));
            }
            bytes
        }
        TypeDef::Variant(variant) => {
            let mut bytes = Vec::new();
            for var in variant.variants() {
                bytes.extend(get_variant_hash(registry, var, set));
            }
            bytes
        }
        TypeDef::Sequence(sequence) => {
            let mut bytes = Vec::new();
            bytes.extend(get_type_hash(registry, sequence.type_param().id(), set));
            bytes
        }
        TypeDef::Array(array) => {
            let mut bytes = Vec::new();
            bytes.extend(array.len().to_be_bytes());
            bytes.extend(get_type_hash(registry, array.type_param().id(), set));
            bytes
        }
        TypeDef::Tuple(tuple) => {
            let mut bytes = Vec::new();
            for field in tuple.fields() {
                bytes.extend(get_type_hash(registry, field.id(), set));
            }
            bytes
        }
        TypeDef::Primitive(primitive) => {
            let mut bytes = Vec::new();
            bytes.extend(primitive.encode());
            bytes
        }
        TypeDef::Compact(compact) => {
            let mut bytes = Vec::new();
            bytes.extend(get_type_hash(registry, compact.type_param().id(), set));
            bytes
        }
        TypeDef::BitSequence(bitseq) => {
            let mut bytes = Vec::new();
            bytes.extend(get_type_hash(registry, bitseq.bit_order_type().id(), set));
            bytes.extend(get_type_hash(registry, bitseq.bit_store_type().id(), set));
            bytes
        }
    };
    bytes.extend(data);
    hash(&bytes)
}

fn get_type_hash(
    registry: &PortableRegistry,
    id: u32,
    set: &mut HashSet<u32>,
) -> [u8; 32] {
    lazy_static! {
        static ref CACHED_UID: Mutex<HashMap<u32, [u8; 32]>> = Mutex::new(HashMap::new());
    }

    if let Some(cached) = CACHED_UID.lock().unwrap().get(&id) {
        return *cached
    }

    let ty = registry.resolve(id).unwrap();

    let mut bytes = vec![MetadataHashableIDs::Type as u8];
    bytes.extend(ty.path().segments().concat().into_bytes());
    // Guard against recursive types
    if !set.insert(id) {
        return hash(&bytes)
    }

    let ty_def = ty.type_def();
    bytes.extend(get_type_def_hash(registry, ty_def, set));

    let uid = hash(&bytes);
    CACHED_UID.lock().unwrap().insert(id, uid);
    uid
}

pub fn get_pallet_hash(
    registry: &PortableRegistry,
    pallet: &frame_metadata::PalletMetadata<PortableForm>,
) -> [u8; 32] {
    let mut bytes = vec![MetadataHashableIDs::Pallet as u8];
    let mut set = HashSet::<u32>::new();

    if let Some(ref calls) = pallet.calls {
        bytes.extend(get_type_hash(registry, calls.ty.id(), &mut set));
    }
    if let Some(ref event) = pallet.event {
        bytes.extend(get_type_hash(registry, event.ty.id(), &mut set));
    }
    for constant in pallet.constants.iter() {
        bytes.extend(constant.name.as_bytes());
        bytes.extend(&constant.value);
        bytes.extend(get_type_hash(registry, constant.ty.id(), &mut set));
    }
    if let Some(ref error) = pallet.error {
        bytes.extend(get_type_hash(registry, error.ty.id(), &mut set));
    }
    if let Some(ref storage) = pallet.storage {
        bytes.extend(storage.prefix.as_bytes());
        for entry in storage.entries.iter() {
            bytes.extend(entry.name.as_bytes());
            bytes.extend(entry.modifier.encode());
            match &entry.ty {
                StorageEntryType::Plain(ty) => {
                    bytes.extend(get_type_hash(registry, ty.id(), &mut set));
                }
                StorageEntryType::Map {
                    hashers,
                    key,
                    value,
                } => {
                    bytes.extend(hashers.encode());
                    bytes.extend(get_type_hash(registry, key.id(), &mut set));
                    bytes.extend(get_type_hash(registry, value.id(), &mut set));
                }
            }
            bytes.extend(&entry.default);
        }
    }

    hash(&bytes)
}

pub fn get_metadata_hash(metadata: &RuntimeMetadataLastVersion) -> [u8; 32] {
    // Note: Order by pallets and use `get_pallet_uid`.
    let bytes = metadata.encode();
    hash(&bytes)
}
