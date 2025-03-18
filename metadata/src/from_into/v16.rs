// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::TryFromError;

use crate::utils::variant_index::VariantIndex;
use crate::{
    utils::ordered_map::OrderedMap, ArcStr, ConstantMetadata, ExtrinsicMetadata, Metadata,
    OuterEnumsMetadata, PalletMetadataInner, RuntimeApiMetadataInner, RuntimeApiMethodMetadataInner,
    MethodParamMetadata, TransactionExtensionMetadataInner, StorageEntryMetadata,
    StorageEntryModifier, StorageEntryType, StorageHasher, StorageMetadata, PalletViewFunctionMetadataInner,
};
use frame_metadata::{v15, v16};
use hashbrown::HashMap;
use scale_info::form::PortableForm;

impl TryFrom<v16::RuntimeMetadataV16> for Metadata {
    type Error = TryFromError;
    fn try_from(m: v16::RuntimeMetadataV16) -> Result<Self, TryFromError> {
        let types = m.types;

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
            let view_functions = p.view_functions
                .into_iter()
                .map(from_view_function_metadata);

            let call_variant_index =
                VariantIndex::build(p.calls.as_ref().map(|c| c.ty.id), &types);
            let error_variant_index =
                VariantIndex::build(p.error.as_ref().map(|e| e.ty.id), &types);
            let event_variant_index =
                VariantIndex::build(p.event.as_ref().map(|e| e.ty.id), &types);

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
                    view_functions: view_functions.collect(),
                    docs: p.docs,
                },
            );
        }

        let apis = m.apis.into_iter().map(|api| {
            let name: ArcStr = api.name.clone().into();
            (name.clone(), from_runtime_api_metadata(name, api))
        });

        let custom_map = m.custom.map.into_iter().map(|(key, val)| {
            let custom_val = v15::CustomValueMetadata {
                ty: val.ty,
                value: val.value
            };
            (key, custom_val)
        }).collect();

        let dispatch_error_ty = types
            .types
            .iter()
            .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
            .map(|ty| ty.id);

        Ok(Metadata {
            types,
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
            custom: v15::CustomMetadata { map: custom_map },
        })
    }
}

fn from_transaction_extension_metadata(
    value: v16::TransactionExtensionMetadata<PortableForm>,
) -> TransactionExtensionMetadataInner {
    TransactionExtensionMetadataInner {
        identifier: value.identifier,
        extra_ty: value.ty.id,
        additional_ty: value.implicit.id,
    }
}

fn from_extrinsic_metadata(value: v16::ExtrinsicMetadata<PortableForm>) -> ExtrinsicMetadata {
    ExtrinsicMetadata {
        supported_versions: value.versions,
        transaction_extensions_by_version: value.transaction_extensions_by_version,
        transaction_extensions: value
            .transaction_extensions
            .into_iter()
            .map(from_transaction_extension_metadata)
            .collect(),
        address_ty: value.address_ty.id,
        signature_ty: value.signature_ty.id,
    }
}

fn from_storage_hasher(value: v16::StorageHasher) -> StorageHasher {
    match value {
        v16::StorageHasher::Blake2_128 => StorageHasher::Blake2_128,
        v16::StorageHasher::Blake2_256 => StorageHasher::Blake2_256,
        v16::StorageHasher::Blake2_128Concat => StorageHasher::Blake2_128Concat,
        v16::StorageHasher::Twox128 => StorageHasher::Twox128,
        v16::StorageHasher::Twox256 => StorageHasher::Twox256,
        v16::StorageHasher::Twox64Concat => StorageHasher::Twox64Concat,
        v16::StorageHasher::Identity => StorageHasher::Identity,
    }
}

fn from_storage_entry_type(value: v16::StorageEntryType<PortableForm>) -> StorageEntryType {
    match value {
        v16::StorageEntryType::Plain(ty) => StorageEntryType::Plain(ty.id),
        v16::StorageEntryType::Map {
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

fn from_storage_entry_modifier(value: v16::StorageEntryModifier) -> StorageEntryModifier {
    match value {
        v16::StorageEntryModifier::Optional => StorageEntryModifier::Optional,
        v16::StorageEntryModifier::Default => StorageEntryModifier::Default,
    }
}

fn from_storage_entry_metadata(
    name: ArcStr,
    s: v16::StorageEntryMetadata<PortableForm>,
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
    s: v16::PalletConstantMetadata<PortableForm>,
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
    s: v16::RuntimeApiMetadata<PortableForm>,
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
    s: v16::RuntimeApiMethodMetadata<PortableForm>,
) -> RuntimeApiMethodMetadataInner {
    RuntimeApiMethodMetadataInner {
        name,
        inputs: s
            .inputs
            .into_iter()
            .map(|param| MethodParamMetadata {
                name: param.name,
                ty: param.ty.id,
            })
            .collect(),
        output_ty: s.output.id,
        docs: s.docs,
    }
}

fn from_view_function_metadata(
    s: v16::PalletViewFunctionMetadata<PortableForm>
) -> PalletViewFunctionMetadataInner {
    PalletViewFunctionMetadataInner {
        name: s.name,
        query_id: s.id,
        inputs: s
            .inputs
            .into_iter()
            .map(|param| MethodParamMetadata {
                name: param.name,
                ty: param.ty.id,
            })
        .collect(),
        output_ty: s.output.id,
        docs: s.docs,
    }
}
