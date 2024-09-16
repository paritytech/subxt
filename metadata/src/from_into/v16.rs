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
use alloc::borrow::ToOwned;
use alloc::vec;
use frame_metadata::v16;
use hashbrown::HashMap;
use scale_info::form::PortableForm;

// Converting from V16 metadata into our Subxt repr.
mod from_v16 {
    use frame_metadata::v15;

    use crate::AssociatedTypeMetadata;

    use super::*;

    impl TryFrom<v16::RuntimeMetadataV16> for Metadata {
        type Error = TryFromError;
        fn try_from(m: v16::RuntimeMetadataV16) -> Result<Self, TryFromError> {
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
                        associated_types: p
                            .associated_types
                            .into_iter()
                            .map(from_associated_type_metadata)
                            .collect(),
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
                runtime_ty: 0,
                dispatch_error_ty,
                apis: apis.collect(),
                outer_enums: OuterEnumsMetadata {
                    call_enum_ty: m.outer_enums.call_enum_ty.id,
                    event_enum_ty: m.outer_enums.event_enum_ty.id,
                    error_enum_ty: m.outer_enums.error_enum_ty.id,
                },
                custom: v15::CustomMetadata {
                    map: Default::default(),
                },
            })
        }
    }

    fn from_signed_extension_metadata(
        value: v16::TransactionExtensionMetadata<PortableForm>,
    ) -> SignedExtensionMetadata {
        SignedExtensionMetadata {
            identifier: value.identifier,
            extra_ty: value.ty.id,
            additional_ty: value.additional_signed.id,
        }
    }

    fn from_extrinsic_metadata(value: v16::ExtrinsicMetadata<PortableForm>) -> ExtrinsicMetadata {
        ExtrinsicMetadata {
            version: value.versions[0],
            signed_extensions: value
                .transaction_extensions
                .into_iter()
                .map(from_signed_extension_metadata)
                .collect(),
            address_ty: value.address_ty.id,
            call_ty: value.call_ty.id,
            signature_ty: value.signature_ty.id,
            extra_ty: value.extra_ty.id,
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

    fn from_associated_type_metadata(
        value: v16::PalletAssociatedTypeMetadata<PortableForm>,
    ) -> AssociatedTypeMetadata {
        AssociatedTypeMetadata {
            name: value.name,
            ty: value.ty.id,
            docs: value.docs,
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
        s: v16::RuntimeApiMethodParamMetadata<PortableForm>,
    ) -> RuntimeApiMethodParamMetadata {
        RuntimeApiMethodParamMetadata {
            name: s.name,
            ty: s.ty.id,
        }
    }
}

// Converting from our metadata repr to v16 metadata.
mod into_v16 {
    use crate::AssociatedTypeMetadata;

    use super::*;

    impl From<Metadata> for v16::RuntimeMetadataV16 {
        fn from(m: Metadata) -> Self {
            let pallets = m.pallets.into_values().into_iter().map(|p| {
                let storage = p.storage.map(|s| v16::PalletStorageMetadata {
                    prefix: s.prefix,
                    entries: s
                        .entries
                        .into_values()
                        .into_iter()
                        .map(from_storage_entry_metadata)
                        .collect(),
                });

                v16::PalletMetadata {
                    name: (*p.name).to_owned(),
                    calls: p.call_ty.map(|id| v16::PalletCallMetadata {
                        ty: id.into(),
                        deprecation_info: v16::DeprecationInfo::NotDeprecated,
                    }),
                    event: p.event_ty.map(|id| v16::PalletEventMetadata {
                        ty: id.into(),
                        deprecation_info: v16::DeprecationInfo::NotDeprecated,
                    }),
                    error: p.error_ty.map(|id| v16::PalletErrorMetadata {
                        ty: id.into(),
                        deprecation_info: v16::DeprecationInfo::NotDeprecated,
                    }),
                    storage,
                    constants: p
                        .constants
                        .into_values()
                        .into_iter()
                        .map(from_constant_metadata)
                        .collect(),
                    index: p.index,
                    docs: p.docs,
                    associated_types: p
                        .associated_types
                        .into_iter()
                        .map(from_associated_type_metadata)
                        .collect(),
                    deprecation_info: v16::DeprecationStatus::NotDeprecated,
                }
            });

            v16::RuntimeMetadataV16 {
                types: m.types,
                pallets: pallets.collect(),
                extrinsic: from_extrinsic_metadata(m.extrinsic),
                apis: m
                    .apis
                    .into_values()
                    .into_iter()
                    .map(from_runtime_api_metadata)
                    .collect(),
                outer_enums: v16::OuterEnums {
                    call_enum_ty: m.outer_enums.call_enum_ty.into(),
                    event_enum_ty: m.outer_enums.event_enum_ty.into(),
                    error_enum_ty: m.outer_enums.error_enum_ty.into(),
                },
                custom: v16::CustomMetadata {
                    map: Default::default(),
                },
            }
        }
    }

    fn from_associated_type_metadata(
        a: AssociatedTypeMetadata,
    ) -> v16::PalletAssociatedTypeMetadata<PortableForm> {
        v16::PalletAssociatedTypeMetadata {
            name: a.name,
            ty: a.ty.into(),
            docs: a.docs,
        }
    }

    fn from_runtime_api_metadata(
        r: RuntimeApiMetadataInner,
    ) -> v16::RuntimeApiMetadata<PortableForm> {
        v16::RuntimeApiMetadata {
            name: (*r.name).to_owned(),
            methods: r
                .methods
                .into_values()
                .into_iter()
                .map(from_runtime_api_method_metadata)
                .collect(),
            docs: r.docs,
            deprecation_info: v16::DeprecationStatus::NotDeprecated,
        }
    }

    fn from_runtime_api_method_metadata(
        m: RuntimeApiMethodMetadata,
    ) -> v16::RuntimeApiMethodMetadata<PortableForm> {
        v16::RuntimeApiMethodMetadata {
            name: (*m.name).to_owned(),
            inputs: m
                .inputs
                .into_iter()
                .map(from_runtime_api_method_param_metadata)
                .collect(),
            output: m.output_ty.into(),
            docs: m.docs,
            deprecation_info: v16::DeprecationStatus::NotDeprecated,
        }
    }

    fn from_runtime_api_method_param_metadata(
        p: RuntimeApiMethodParamMetadata,
    ) -> v16::RuntimeApiMethodParamMetadata<PortableForm> {
        v16::RuntimeApiMethodParamMetadata {
            name: p.name,
            ty: p.ty.into(),
        }
    }

    fn from_extrinsic_metadata(e: ExtrinsicMetadata) -> v16::ExtrinsicMetadata<PortableForm> {
        v16::ExtrinsicMetadata {
            versions: vec![e.version],
            transaction_extensions: e
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
    ) -> v16::TransactionExtensionMetadata<PortableForm> {
        v16::TransactionExtensionMetadata {
            identifier: s.identifier,
            ty: s.extra_ty.into(),
            additional_signed: s.additional_ty.into(),
        }
    }

    fn from_constant_metadata(c: ConstantMetadata) -> v16::PalletConstantMetadata<PortableForm> {
        v16::PalletConstantMetadata {
            name: (*c.name).to_owned(),
            ty: c.ty.into(),
            value: c.value,
            docs: c.docs,
            deprecation_info: v16::DeprecationStatus::NotDeprecated,
        }
    }

    fn from_storage_entry_metadata(
        s: StorageEntryMetadata,
    ) -> v16::StorageEntryMetadata<PortableForm> {
        v16::StorageEntryMetadata {
            docs: s.docs,
            default: s.default,
            name: (*s.name).to_owned(),
            ty: from_storage_entry_type(s.entry_type),
            modifier: from_storage_entry_modifier(s.modifier),
            deprecation_info: v16::DeprecationStatus::NotDeprecated,
        }
    }

    fn from_storage_entry_modifier(s: StorageEntryModifier) -> v16::StorageEntryModifier {
        match s {
            StorageEntryModifier::Default => v16::StorageEntryModifier::Default,
            StorageEntryModifier::Optional => v16::StorageEntryModifier::Optional,
        }
    }

    fn from_storage_entry_type(s: StorageEntryType) -> v16::StorageEntryType<PortableForm> {
        match s {
            StorageEntryType::Plain(ty) => v16::StorageEntryType::Plain(ty.into()),
            StorageEntryType::Map {
                hashers,
                key_ty,
                value_ty,
            } => v16::StorageEntryType::Map {
                hashers: hashers.into_iter().map(from_storage_hasher).collect(),
                key: key_ty.into(),
                value: value_ty.into(),
            },
        }
    }

    fn from_storage_hasher(s: StorageHasher) -> v16::StorageHasher {
        match s {
            StorageHasher::Blake2_128 => v16::StorageHasher::Blake2_128,
            StorageHasher::Blake2_256 => v16::StorageHasher::Blake2_256,
            StorageHasher::Blake2_128Concat => v16::StorageHasher::Blake2_128Concat,
            StorageHasher::Twox128 => v16::StorageHasher::Twox128,
            StorageHasher::Twox256 => v16::StorageHasher::Twox256,
            StorageHasher::Twox64Concat => v16::StorageHasher::Twox64Concat,
            StorageHasher::Identity => v16::StorageHasher::Identity,
        }
    }
}
