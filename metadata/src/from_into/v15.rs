// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::TryFromError;
use crate::utils::variant_index::VariantIndex;
use crate::{
    utils::ordered_map::OrderedMap, ArcStr, ConstantMetadata, ExtrinsicMetadata, Metadata,
    OuterEnumsMetadata, PalletMetadataInner, RuntimeApiMetadataInner, RuntimeApiMethodMetadata,
    RuntimeApiMethodParamMetadata, SignedExtensionMetadata, StorageEntryMetadata,
    StorageEntryModifier, StorageEntryType, StorageHasher, StorageMetadata,
};
use frame_metadata::v15;
use scale_info::form::PortableForm;
use std::collections::HashMap;

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
                runtime_ty: m.ty.id,
                dispatch_error_ty,
                apis: apis.collect(),
                outer_enums: OuterEnumsMetadata {
                    call_enum_ty: m.outer_enums.call_enum_ty.id,
                    event_enum_ty: m.outer_enums.event_enum_ty.id,
                    error_enum_ty: m.outer_enums.error_enum_ty.id,
                },
            })
        }
    }

    fn from_signed_extension_metadata(
        value: v15::SignedExtensionMetadata<PortableForm>,
    ) -> SignedExtensionMetadata {
        SignedExtensionMetadata {
            identifier: value.identifier,
            extra_ty: value.ty.id,
            additional_ty: value.additional_signed.id,
        }
    }

    fn from_extrinsic_metadata(value: v15::ExtrinsicMetadata<PortableForm>) -> ExtrinsicMetadata {
        ExtrinsicMetadata {
            version: value.version,
            signed_extensions: value
                .signed_extensions
                .into_iter()
                .map(from_signed_extension_metadata)
                .collect(),
            address_ty: value.address_ty.id,
            call_ty: value.call_ty.id,
            signature_ty: value.signature_ty.id,
            extra_ty: value.extra_ty.id,
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
    ) -> RuntimeApiMethodMetadata {
        RuntimeApiMethodMetadata {
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
    ) -> RuntimeApiMethodParamMetadata {
        RuntimeApiMethodParamMetadata {
            name: s.name,
            ty: s.ty.id,
        }
    }
}

// Converting from our metadata repr to V15 metadata.
mod into_v15 {
    use super::*;

    impl From<Metadata> for v15::RuntimeMetadataV15 {
        fn from(m: Metadata) -> Self {
            let pallets = m.pallets.into_values().into_iter().map(|p| {
                let storage = p.storage.map(|s| v15::PalletStorageMetadata {
                    prefix: s.prefix,
                    entries: s
                        .entries
                        .into_values()
                        .into_iter()
                        .map(from_storage_entry_metadata)
                        .collect(),
                });

                v15::PalletMetadata {
                    name: (*p.name).to_owned(),
                    calls: p
                        .call_ty
                        .map(|id| v15::PalletCallMetadata { ty: id.into() }),
                    event: p
                        .event_ty
                        .map(|id| v15::PalletEventMetadata { ty: id.into() }),
                    error: p
                        .error_ty
                        .map(|id| v15::PalletErrorMetadata { ty: id.into() }),
                    storage,
                    constants: p
                        .constants
                        .into_values()
                        .into_iter()
                        .map(from_constant_metadata)
                        .collect(),
                    index: p.index,
                    docs: p.docs,
                }
            });

            v15::RuntimeMetadataV15 {
                types: m.types,
                pallets: pallets.collect(),
                ty: m.runtime_ty.into(),
                extrinsic: from_extrinsic_metadata(m.extrinsic),
                apis: m
                    .apis
                    .into_values()
                    .into_iter()
                    .map(from_runtime_api_metadata)
                    .collect(),
                outer_enums: v15::OuterEnums {
                    call_enum_ty: m.outer_enums.call_enum_ty.into(),
                    event_enum_ty: m.outer_enums.event_enum_ty.into(),
                    error_enum_ty: m.outer_enums.error_enum_ty.into(),
                },
                custom: v15::CustomMetadata {
                    map: Default::default(),
                },
            }
        }
    }

    fn from_runtime_api_metadata(
        r: RuntimeApiMetadataInner,
    ) -> v15::RuntimeApiMetadata<PortableForm> {
        v15::RuntimeApiMetadata {
            name: (*r.name).to_owned(),
            methods: r
                .methods
                .into_values()
                .into_iter()
                .map(from_runtime_api_method_metadata)
                .collect(),
            docs: r.docs,
        }
    }

    fn from_runtime_api_method_metadata(
        m: RuntimeApiMethodMetadata,
    ) -> v15::RuntimeApiMethodMetadata<PortableForm> {
        v15::RuntimeApiMethodMetadata {
            name: (*m.name).to_owned(),
            inputs: m
                .inputs
                .into_iter()
                .map(from_runtime_api_method_param_metadata)
                .collect(),
            output: m.output_ty.into(),
            docs: m.docs,
        }
    }

    fn from_runtime_api_method_param_metadata(
        p: RuntimeApiMethodParamMetadata,
    ) -> v15::RuntimeApiMethodParamMetadata<PortableForm> {
        v15::RuntimeApiMethodParamMetadata {
            name: p.name,
            ty: p.ty.into(),
        }
    }

    fn from_extrinsic_metadata(e: ExtrinsicMetadata) -> v15::ExtrinsicMetadata<PortableForm> {
        v15::ExtrinsicMetadata {
            version: e.version,
            signed_extensions: e
                .signed_extensions
                .into_iter()
                .map(from_signed_extension_metadata)
                .collect(),
            address_ty: e.address_ty.into(),
            call_ty: e.call_ty.into(),
            signature_ty: e.signature_ty.into(),
            extra_ty: e.extra_ty.into(),
        }
    }

    fn from_signed_extension_metadata(
        s: SignedExtensionMetadata,
    ) -> v15::SignedExtensionMetadata<PortableForm> {
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

    fn from_storage_entry_metadata(
        s: StorageEntryMetadata,
    ) -> v15::StorageEntryMetadata<PortableForm> {
        v15::StorageEntryMetadata {
            docs: s.docs,
            default: s.default,
            name: (*s.name).to_owned(),
            ty: from_storage_entry_type(s.entry_type),
            modifier: from_storage_entry_modifier(s.modifier),
        }
    }

    fn from_storage_entry_modifier(s: StorageEntryModifier) -> v15::StorageEntryModifier {
        match s {
            StorageEntryModifier::Default => v15::StorageEntryModifier::Default,
            StorageEntryModifier::Optional => v15::StorageEntryModifier::Optional,
        }
    }

    fn from_storage_entry_type(s: StorageEntryType) -> v15::StorageEntryType<PortableForm> {
        match s {
            StorageEntryType::Plain(ty) => v15::StorageEntryType::Plain(ty.into()),
            StorageEntryType::Map {
                hashers,
                key_ty,
                value_ty,
            } => v15::StorageEntryType::Map {
                hashers: hashers.into_iter().map(from_storage_hasher).collect(),
                key: key_ty.into(),
                value: value_ty.into(),
            },
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
