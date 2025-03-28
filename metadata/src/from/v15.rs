// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::TryFromError;

use crate::utils::variant_index::VariantIndex;
use crate::{
    utils::ordered_map::OrderedMap, ArcStr, ConstantMetadata, ExtrinsicMetadata, Metadata,
    MethodParamMetadata, OuterEnumsMetadata, PalletMetadataInner, RuntimeApiMetadataInner,
    RuntimeApiMethodMetadataInner, StorageEntryMetadata, StorageEntryModifier, StorageEntryType,
    StorageHasher, StorageMetadata, TransactionExtensionMetadataInner,
};
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use frame_metadata::v15;
use hashbrown::HashMap;
use scale_info::form::PortableForm;

impl TryFrom<v15::RuntimeMetadataV15> for Metadata {
    type Error = TryFromError;
    fn try_from(m: v15::RuntimeMetadataV15) -> Result<Self, TryFromError> {
        let mut pallets = OrderedMap::new();
        let mut pallets_by_index = HashMap::new();
        for (pos, p) in m.pallets.into_iter().enumerate() {
            let name: ArcStr = p.name.into();

            let storage = p.storage.map(|s| StorageMetadata {
                prefix: s.prefix,
                entries: s
                    .entries
                    .into_iter()
                    .map(|s| {
                        let name: ArcStr = s.name.clone().into();
                        (name.clone(), from_storage_entry_metadata(name, s))
                    })
                    .collect(),
            });
            let constants = p.constants.into_iter().map(|c| {
                let name: ArcStr = c.name.clone().into();
                (name.clone(), from_constant_metadata(name, c))
            });

            let call_variant_index =
                VariantIndex::build(p.calls.as_ref().map(|c| c.ty.id), &m.types);
            let error_variant_index =
                VariantIndex::build(p.error.as_ref().map(|e| e.ty.id), &m.types);
            let event_variant_index =
                VariantIndex::build(p.event.as_ref().map(|e| e.ty.id), &m.types);

            pallets_by_index.insert(p.index, pos);
            pallets.push_insert(
                name.clone(),
                PalletMetadataInner {
                    name,
                    index: p.index,
                    storage,
                    call_ty: p.calls.map(|c| c.ty.id),
                    call_variant_index,
                    event_ty: p.event.map(|e| e.ty.id),
                    event_variant_index,
                    error_ty: p.error.map(|e| e.ty.id),
                    error_variant_index,
                    constants: constants.collect(),
                    view_functions: vec![],
                    associated_types: Default::default(),
                    docs: p.docs,
                },
            );
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
            .map(|ty| ty.id);

        Ok(Metadata {
            types: m.types,
            pallets,
            pallets_by_index,
            extrinsic: from_extrinsic_metadata(m.extrinsic),
            dispatch_error_ty,
            apis: apis.collect(),
            outer_enums: OuterEnumsMetadata {
                call_enum_ty: m.outer_enums.call_enum_ty.id,
                event_enum_ty: m.outer_enums.event_enum_ty.id,
                error_enum_ty: m.outer_enums.error_enum_ty.id,
            },
            custom: m.custom,
        })
    }
}

fn from_signed_extension_metadata(
    value: v15::SignedExtensionMetadata<PortableForm>,
) -> TransactionExtensionMetadataInner {
    TransactionExtensionMetadataInner {
        identifier: value.identifier,
        extra_ty: value.ty.id,
        additional_ty: value.additional_signed.id,
    }
}

fn from_extrinsic_metadata(value: v15::ExtrinsicMetadata<PortableForm>) -> ExtrinsicMetadata {
    let transaction_extensions: Vec<_> = value
        .signed_extensions
        .into_iter()
        .map(from_signed_extension_metadata)
        .collect();

    let transaction_extension_indexes = (0..transaction_extensions.len() as u32).collect();

    ExtrinsicMetadata {
        supported_versions: vec![value.version],
        transaction_extensions,
        address_ty: value.address_ty.id,
        signature_ty: value.signature_ty.id,
        transaction_extensions_by_version: BTreeMap::from_iter([(
            0,
            transaction_extension_indexes,
        )]),
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
        v15::StorageEntryType::Map {
            hashers,
            key,
            value,
        } => StorageEntryType::Map {
            hashers: hashers.into_iter().map(from_storage_hasher).collect(),
            key_ty: key.id,
            value_ty: value.id,
        },
    }
}

fn from_storage_entry_modifier(value: v15::StorageEntryModifier) -> StorageEntryModifier {
    match value {
        v15::StorageEntryModifier::Optional => StorageEntryModifier::Optional,
        v15::StorageEntryModifier::Default => StorageEntryModifier::Default,
    }
}

fn from_storage_entry_metadata(
    name: ArcStr,
    s: v15::StorageEntryMetadata<PortableForm>,
) -> StorageEntryMetadata {
    StorageEntryMetadata {
        name,
        modifier: from_storage_entry_modifier(s.modifier),
        entry_type: from_storage_entry_type(s.ty),
        default: s.default,
        docs: s.docs,
    }
}

fn from_constant_metadata(
    name: ArcStr,
    s: v15::PalletConstantMetadata<PortableForm>,
) -> ConstantMetadata {
    ConstantMetadata {
        name,
        ty: s.ty.id,
        value: s.value,
        docs: s.docs,
    }
}

fn from_runtime_api_metadata(
    name: ArcStr,
    s: v15::RuntimeApiMetadata<PortableForm>,
) -> RuntimeApiMetadataInner {
    RuntimeApiMetadataInner {
        name,
        docs: s.docs,
        methods: s
            .methods
            .into_iter()
            .map(|m| {
                let name: ArcStr = m.name.clone().into();
                (name.clone(), from_runtime_api_method_metadata(name, m))
            })
            .collect(),
    }
}

fn from_runtime_api_method_metadata(
    name: ArcStr,
    s: v15::RuntimeApiMethodMetadata<PortableForm>,
) -> RuntimeApiMethodMetadataInner {
    RuntimeApiMethodMetadataInner {
        name,
        inputs: s
            .inputs
            .into_iter()
            .map(from_runtime_api_method_param_metadata)
            .collect(),
        output_ty: s.output.id,
        docs: s.docs,
    }
}

fn from_runtime_api_method_param_metadata(
    s: v15::RuntimeApiMethodParamMetadata<PortableForm>,
) -> MethodParamMetadata {
    MethodParamMetadata {
        name: s.name,
        ty: s.ty.id,
    }
}
