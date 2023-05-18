// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use frame_metadata::v15;
use super::TryFromError;
use std::collections::HashMap;
use scale_info::{form::PortableForm, PortableRegistry, TypeDef};
use crate::{
    ArcStr, Metadata, PalletMetadataInner, StorageMetadata, StorageEntryMetadata, StorageEntryModifier,
    StorageEntryType, StorageHasher, utils::ordered_map::OrderedMap, ConstantMetadata, ExtrinsicMetadata,
    SignedExtensionMetadata, RuntimeApiMetadataInner, RuntimeApiMethodMetadata, RuntimeApiMethodParamMetadata
};

// Converting from V15 metadata into our Subxt repr.
mod from_v15 {
    use super::*;

    impl TryFrom<v15::RuntimeMetadataV15> for Metadata {
        type Error = TryFromError;
        fn try_from(m: v15::RuntimeMetadataV15) -> Result<Self, TryFromError> {
            let mut pallets = OrderedMap::new();
            let mut pallets_by_index = HashMap::new();
            for (pos, p) in m.pallets.into_iter().enumerate() {
                let name: ArcStr = p.name.into();

                let storage = p.storage.map(|s| {
                    StorageMetadata {
                        prefix: s.prefix,
                        entries: s.entries.into_iter().map(|s| {
                            let name: ArcStr = s.name.clone().into();
                            (name.clone(), from_storage_entry_metadata(name, s))
                        }).collect()
                    }
                });
                let constants = p.constants.into_iter().map(|c| {
                    let name: ArcStr = c.name.clone().into();
                    (name.clone(), from_constant_metadata(name, c))
                });

                let call_variants_by_name = match &p.calls {
                    Some(ty) => variant_positions_by_name(ty.ty.id, &m.types)?,
                    None => HashMap::new()
                };
                let call_variants_by_index = match &p.calls {
                    Some(ty) => variant_positions_by_index(ty.ty.id, &m.types)?,
                    None => HashMap::new()
                };
                let event_variants_by_index = match &p.event {
                    Some(ty) => variant_positions_by_index(ty.ty.id, &m.types)?,
                    None => HashMap::new()
                };
                let error_variants_by_index = match &p.error {
                    Some(ty) => variant_positions_by_index(ty.ty.id, &m.types)?,
                    None => HashMap::new()
                };

                pallets_by_index.insert(p.index, pos);
                pallets.push_insert(name.clone(), PalletMetadataInner {
                    name: name,
                    index: p.index,
                    storage: storage,
                    call_ty: p.calls.map(|c| c.ty.id),
                    call_variants_by_index,
                    call_variants_by_name,
                    event_ty: p.event.map(|e| e.ty.id),
                    event_variants_by_index,
                    error_ty: p.error.map(|e| e.ty.id),
                    error_variants_by_index,
                    constants: constants.collect(),
                    docs: p.docs,
                    cached_call_hashes: Default::default(),
                    cached_constant_hashes: Default::default(),
                    cached_storage_hashes: Default::default(),
                });
            }

            let apis = m.apis.into_iter().map(|api| {
                let name: ArcStr = api.name.clone().into();
                (name.clone(), from_runtime_api_metadata(name, api))
            });

            let dispatch_error_ty = m
                .types
                .types
                .iter()
                .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
                .map(|ty| ty.id)
                .ok_or_else(|| TryFromError::DispatchErrorTypeNotFound)?;

            Ok(Metadata {
                types: m.types,
                pallets,
                pallets_by_index,
                extrinsic: from_extrinsic_metadata(m.extrinsic),
                runtime_ty: m.ty.id,
                dispatch_error_ty: dispatch_error_ty,
                apis: apis.collect()
            })
        }
    }

    fn from_signed_extension_metadata(value: v15::SignedExtensionMetadata<PortableForm>) -> SignedExtensionMetadata {
        SignedExtensionMetadata {
            identifier: value.identifier,
            extra_ty: value.ty.id,
            additional_ty: value.additional_signed.id,
        }
    }

    fn from_extrinsic_metadata(value: v15::ExtrinsicMetadata<PortableForm>) -> ExtrinsicMetadata {
        ExtrinsicMetadata {
            ty: value.ty.id,
            version: value.version,
            signed_extensions: value.signed_extensions.into_iter().map(from_signed_extension_metadata).collect()
        }
    }

    fn from_storage_hasher(value: v15::StorageHasher) -> StorageHasher {
        match value {
            v15::StorageHasher::Blake2_128 => StorageHasher::Blake2_128,
            v15::StorageHasher::Blake2_256 => StorageHasher::Blake2_256,
            v15::StorageHasher::Blake2_128Concat => StorageHasher::Blake2_128Concat,
            v15::StorageHasher::Twox128 => StorageHasher::Twox128,
            v15::StorageHasher::Twox256 => StorageHasher::Twox256,
            v15::StorageHasher::Twox64Concat => StorageHasher::Twox64Concat,
            v15::StorageHasher::Identity => StorageHasher::Identity,
        }
    }

    fn from_storage_entry_type(value: v15::StorageEntryType<PortableForm>) -> StorageEntryType {
        match value {
            v15::StorageEntryType::Plain(ty) => StorageEntryType::Plain(ty.id),
            v15::StorageEntryType::Map { hashers, key, value } => {
                StorageEntryType::Map {
                    hashers: hashers.into_iter().map(from_storage_hasher).collect(),
                    key_ty: key.id,
                    value_ty: value.id
                }
            },
        }
    }

    fn from_storage_entry_modifier(value: v15::StorageEntryModifier) -> StorageEntryModifier {
        match value {
            v15::StorageEntryModifier::Optional => StorageEntryModifier::Optional,
            v15::StorageEntryModifier::Default => StorageEntryModifier::Default,
        }
    }

    fn from_storage_entry_metadata(name: ArcStr, s: v15::StorageEntryMetadata<PortableForm>) -> StorageEntryMetadata {
        StorageEntryMetadata {
            name: name,
            modifier: from_storage_entry_modifier(s.modifier),
            entry_type: from_storage_entry_type(s.ty),
            default: s.default.into(),
            docs: s.docs,
        }
    }

    fn from_constant_metadata(name: ArcStr, s: v15::PalletConstantMetadata<PortableForm>) -> ConstantMetadata {
        ConstantMetadata {
            name: name,
            ty: s.ty.id,
            value: s.value,
            docs: s.docs,
        }
    }

    fn from_runtime_api_metadata(name: ArcStr, s: v15::RuntimeApiMetadata<PortableForm>) -> RuntimeApiMetadataInner {
        RuntimeApiMetadataInner {
            name,
            docs: s.docs,
            methods: s.methods.into_iter().map(|m| {
                let name: ArcStr = m.name.clone().into();
                (name.clone(), from_runtime_api_method_metadata(name, m))
            }).collect(),
            cached_runtime_hashes: Default::default()
        }
    }

    fn from_runtime_api_method_metadata(name: ArcStr, s: v15::RuntimeApiMethodMetadata<PortableForm>) -> RuntimeApiMethodMetadata {
        RuntimeApiMethodMetadata {
            name: name,
            inputs: s.inputs.into_iter().map(from_runtime_api_method_param_metadata).collect(),
            output_ty: s.output.id,
            docs: s.docs,
        }
    }

    fn from_runtime_api_method_param_metadata(s: v15::RuntimeApiMethodParamMetadata<PortableForm>) -> RuntimeApiMethodParamMetadata {
        RuntimeApiMethodParamMetadata {
            name: s.name,
            ty: s.ty.id
        }
    }

    fn variant_positions_by_index(id: u32, types: &PortableRegistry) -> Result<HashMap<u8, usize>, TryFromError> {
        let Some(ty) = types.resolve(id) else {
            return Err(TryFromError::TypeNotFound(id))
        };
        let TypeDef::Variant(v) = &ty.type_def else {
            return Err(TryFromError::VariantExpected(id))
        };

        let mut by_index = HashMap::new();
        for (pos, var) in v.variants.iter().enumerate() {
            by_index.insert(var.index, pos);
        }

        Ok(by_index)
    }

    fn variant_positions_by_name(id: u32, types: &PortableRegistry) -> Result<HashMap<String, usize>, TryFromError> {
        let Some(ty) = types.resolve(id) else {
            return Err(TryFromError::TypeNotFound(id))
        };
        let TypeDef::Variant(v) = &ty.type_def else {
            return Err(TryFromError::VariantExpected(id))
        };

        let mut by_name = HashMap::new();
        for (pos, var) in v.variants.iter().enumerate() {
            by_name.insert(var.name.clone(), pos);
        }

        Ok(by_name)
    }
}

// Converting from our metadata repr to V15 metadata.
mod into_v15 {
    use super::*;

    impl From<Metadata> for v15::RuntimeMetadataV15 {
        fn from(m: Metadata) -> Self {
            let pallets = m.pallets.into_values().into_iter().map(|p| {
                let storage = p.storage.map(|s| {
                    v15::PalletStorageMetadata {
                        prefix: s.prefix,
                        entries: s.entries.into_values().into_iter().map(from_storage_entry_metadata).collect()
                    }
                });

                v15::PalletMetadata {
                    name: (*p.name).to_owned(),
                    calls: p.call_ty.map(|id| v15::PalletCallMetadata { ty: id.into() }),
                    event: p.event_ty.map(|id| v15::PalletEventMetadata { ty: id.into() }),
                    error: p.error_ty.map(|id| v15::PalletErrorMetadata { ty: id.into() }),
                    storage,
                    constants: p.constants.into_values().into_iter().map(from_constant_metadata).collect(),
                    index: p.index,
                    docs: p.docs,
                }
            });

            v15::RuntimeMetadataV15 {
                types: m.types,
                pallets: pallets.collect(),
                ty: m.runtime_ty.into(),
                extrinsic: from_extrinsic_metadata(m.extrinsic),
                apis: m.apis.into_values().into_iter().map(from_runtime_api_metadata).collect(),
            }
        }
    }

    fn from_runtime_api_metadata(r: RuntimeApiMetadataInner) -> v15::RuntimeApiMetadata<PortableForm> {
        v15::RuntimeApiMetadata {
            name: (*r.name).to_owned(),
            methods: r.methods.into_values().into_iter().map(from_runtime_api_method_metadata).collect(),
            docs: r.docs,
        }
    }

    fn from_runtime_api_method_metadata(m: RuntimeApiMethodMetadata) -> v15::RuntimeApiMethodMetadata<PortableForm> {
        v15::RuntimeApiMethodMetadata {
            name: (*m.name).to_owned(),
            inputs: m.inputs.into_iter().map(from_runtime_api_method_param_metadata).collect(),
            output: m.output_ty.into(),
            docs: m.docs,
        }
    }

    fn from_runtime_api_method_param_metadata(p: RuntimeApiMethodParamMetadata) -> v15::RuntimeApiMethodParamMetadata<PortableForm> {
        v15::RuntimeApiMethodParamMetadata {
            name: p.name,
            ty: p.ty.into(),
        }
    }

    fn from_extrinsic_metadata(e: ExtrinsicMetadata) -> v15::ExtrinsicMetadata<PortableForm> {
        v15::ExtrinsicMetadata {
            ty: e.ty.into(),
            version: e.version,
            signed_extensions: e.signed_extensions.into_iter().map(from_signed_extension_metadata).collect(),
        }
    }

    fn from_signed_extension_metadata(s: SignedExtensionMetadata) -> v15::SignedExtensionMetadata<PortableForm> {
        v15::SignedExtensionMetadata {
            identifier: s.identifier,
            ty: s.extra_ty.into(),
            additional_signed: s.additional_ty.into(),
        }
    }

    fn from_constant_metadata(c: ConstantMetadata) -> v15::PalletConstantMetadata<PortableForm> {
        v15::PalletConstantMetadata {
            name: (*c.name).to_owned(),
            ty: c.ty.into(),
            value: c.value,
            docs: c.docs,
        }
    }

    fn from_storage_entry_metadata(s: StorageEntryMetadata) -> v15::StorageEntryMetadata<PortableForm> {
        v15::StorageEntryMetadata {
            docs: s.docs,
            default: s.default,
            name: (*s.name).to_owned(),
            ty: from_storage_entry_type(s.entry_type),
            modifier: from_storage_entry_modifier(s.modifier)
        }
    }

    fn from_storage_entry_modifier(s: StorageEntryModifier) -> v15::StorageEntryModifier {
        match s {
            StorageEntryModifier::Default => v15::StorageEntryModifier::Default,
            StorageEntryModifier::Optional => v15::StorageEntryModifier::Optional
        }
    }

    fn from_storage_entry_type(s: StorageEntryType) -> v15::StorageEntryType<PortableForm> {
        match s {
            StorageEntryType::Plain(ty) => v15::StorageEntryType::Plain(ty.into()),
            StorageEntryType::Map { hashers, key_ty, value_ty } => {
                v15::StorageEntryType::Map {
                    hashers: hashers.into_iter().map(from_storage_hasher).collect(),
                    key: key_ty.into(),
                    value: value_ty.into()
                }
            }
        }
    }

    fn from_storage_hasher(s: StorageHasher) -> v15::StorageHasher {
        match s {
            StorageHasher::Blake2_128 => v15::StorageHasher::Blake2_128,
            StorageHasher::Blake2_256 => v15::StorageHasher::Blake2_256,
            StorageHasher::Blake2_128Concat => v15::StorageHasher::Blake2_128Concat,
            StorageHasher::Twox128 => v15::StorageHasher::Twox128,
            StorageHasher::Twox256 => v15::StorageHasher::Twox256,
            StorageHasher::Twox64Concat => v15::StorageHasher::Twox64Concat,
            StorageHasher::Identity => v15::StorageHasher::Identity,
        }
    }
}