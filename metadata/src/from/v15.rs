// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::TryFromError;

use crate::utils::variant_index::VariantIndex;
use crate::{
    ConstantMetadata, ExtrinsicMetadata, Metadata, OuterEnumsMetadata,
    PalletMetadataInner, RuntimeApiMetadataInner, RuntimeApiMethodMetadataInner,
    StorageEntryMetadata, StorageMetadata,
    TransactionExtensionMetadataInner, utils::ordered_map::OrderedMap,
};
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use frame_metadata::v15;
use frame_decode::storage::StorageTypeInfo;
use frame_decode::runtime_apis::RuntimeApiTypeInfo;
use hashbrown::HashMap;
use scale_info::form::PortableForm;

impl TryFrom<v15::RuntimeMetadataV15> for Metadata {
    type Error = TryFromError;
    fn try_from(m: v15::RuntimeMetadataV15) -> Result<Self, TryFromError> {
        let mut pallets = OrderedMap::new();
        let mut pallets_by_index = HashMap::new();
        for (pos, p) in m.pallets.iter().enumerate() {
            let name = p.name.clone();

            let storage = match &p.storage {
                None => None,
                Some(s) => Some(StorageMetadata {
                    prefix: s.prefix.clone(),
                    entries: s
                        .entries
                        .iter()
                        .map(|s| {
                            let entry_name = s.name.clone();
                            let storage_info = m.storage_info(&name, &entry_name)
                                .map_err(|e| e.into_owned())?
                                .into_owned();
                            let storage_entry = StorageEntryMetadata {
                                name: entry_name.clone(),
                                info: storage_info,
                                docs: s.docs.clone().into(),
                            };

                            Ok::<_, TryFromError>((entry_name, storage_entry))
                        })
                        .collect::<Result<_, TryFromError>>()?,
                })
            };

            let constants = p.constants.iter().map(|c| {
                let name = c.name.clone();
                (name, from_constant_metadata(c.clone()))
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
                    call_ty: p.calls.as_ref().map(|c| c.ty.id),
                    call_variant_index,
                    event_ty: p.event.as_ref().map(|e| e.ty.id),
                    event_variant_index,
                    error_ty: p.error.as_ref().map(|e| e.ty.id),
                    error_variant_index,
                    constants: constants.collect(),
                    view_functions: Default::default(),
                    associated_types: Default::default(),
                    docs: p.docs.clone(),
                },
            );
        }

        let apis = m.apis.iter().map(|api| {
            let trait_name = api.name.clone();
            let methods  = api.methods.iter().map(|method| {
                let method_name = method.name.clone();
                let method_info = RuntimeApiMethodMetadataInner {
                    info: m.runtime_api_info(&trait_name, &method.name)
                        .map_err(|e| e.into_owned())?
                        .into_owned(),
                    name: method.name.clone(),
                    docs: method.docs.clone()
                };
                Ok((method_name, method_info))
            }).collect::<Result<_,TryFromError>>()?;

            let runtime_api_metadata = RuntimeApiMetadataInner {
                name: trait_name.clone(),
                methods,
                docs: api.docs.clone()
            };
            Ok((trait_name, runtime_api_metadata))
        }).collect::<Result<_,TryFromError>>()?;

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
            apis: apis,
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

fn from_constant_metadata(
    s: v15::PalletConstantMetadata<PortableForm>,
) -> ConstantMetadata {
    ConstantMetadata {
        name: s.name,
        ty: s.ty.id,
        value: s.value,
        docs: s.docs,
    }
}
