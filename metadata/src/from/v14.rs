// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::TryFromError;

use crate::utils::variant_index::VariantIndex;
use crate::{
    ConstantMetadata, CustomMetadataInner, ExtrinsicMetadata, Metadata, OuterEnumsMetadata,
    PalletMetadataInner, StorageEntryMetadata, StorageMetadata, TransactionExtensionMetadataInner,
    utils::ordered_map::OrderedMap,
};
use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};
use frame_decode::storage::StorageTypeInfo;
use frame_metadata::v14;
use hashbrown::HashMap;
use scale_info::form::PortableForm;

impl TryFrom<v14::RuntimeMetadataV14> for Metadata {
    type Error = TryFromError;
    fn try_from(mut m: v14::RuntimeMetadataV14) -> Result<Self, TryFromError> {
        let outer_enums = generate_outer_enums(&mut m)?;
        let missing_extrinsic_type_ids = MissingExtrinsicTypeIds::generate_from(&m)?;

        let mut pallets = OrderedMap::new();
        let mut pallets_by_index = HashMap::new();
        for (pos, p) in m.pallets.iter().enumerate() {
            let name: String = p.name.clone();

            let storage = match &p.storage {
                None => None,
                Some(s) => Some(StorageMetadata {
                    prefix: s.prefix.clone(),
                    entries: s
                        .entries
                        .iter()
                        .map(|s| {
                            let entry_name: String = s.name.clone();
                            let storage_info = m
                                .storage_info(&name, &entry_name)
                                .map_err(|e| e.into_owned())?
                                .into_owned();
                            let storage_entry = StorageEntryMetadata {
                                name: entry_name.clone(),
                                info: storage_info,
                                docs: s.docs.clone(),
                            };

                            Ok::<_, TryFromError>((entry_name, storage_entry))
                        })
                        .collect::<Result<_, TryFromError>>()?,
                }),
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
                    name: name.clone(),
                    call_index: p.index,
                    event_index: p.index,
                    error_index: p.index,
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
                    docs: vec![],
                },
            );
        }

        let dispatch_error_ty = m
            .types
            .types
            .iter()
            .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
            .map(|ty| ty.id);

        Ok(Metadata {
            types: m.types,
            pallets,
            pallets_by_call_index: pallets_by_index.clone(),
            pallets_by_error_index: pallets_by_index.clone(),
            pallets_by_event_index: pallets_by_index,
            extrinsic: from_extrinsic_metadata(m.extrinsic, missing_extrinsic_type_ids),
            dispatch_error_ty,
            outer_enums: OuterEnumsMetadata {
                call_enum_ty: outer_enums.call_enum_ty.id,
                event_enum_ty: outer_enums.event_enum_ty.id,
                error_enum_ty: outer_enums.error_enum_ty.id,
            },
            apis: Default::default(),
            custom: CustomMetadataInner {
                map: Default::default(),
            },
        })
    }
}

fn from_signed_extension_metadata(
    value: v14::SignedExtensionMetadata<PortableForm>,
) -> TransactionExtensionMetadataInner {
    TransactionExtensionMetadataInner {
        identifier: value.identifier,
        extra_ty: value.ty.id,
        additional_ty: value.additional_signed.id,
    }
}

fn from_extrinsic_metadata(
    value: v14::ExtrinsicMetadata<PortableForm>,
    missing_ids: MissingExtrinsicTypeIds,
) -> ExtrinsicMetadata {
    let transaction_extensions: Vec<_> = value
        .signed_extensions
        .into_iter()
        .map(from_signed_extension_metadata)
        .collect();

    let transaction_extension_indexes = (0..transaction_extensions.len() as u32).collect();

    ExtrinsicMetadata {
        supported_versions: vec![value.version],
        transaction_extensions,
        address_ty: missing_ids.address,
        signature_ty: missing_ids.signature,
        transaction_extensions_by_version: BTreeMap::from_iter([(
            0,
            transaction_extension_indexes,
        )]),
    }
}

fn from_constant_metadata(s: v14::PalletConstantMetadata<PortableForm>) -> ConstantMetadata {
    ConstantMetadata {
        name: s.name,
        ty: s.ty.id,
        value: s.value,
        docs: s.docs,
    }
}

fn generate_outer_enums(
    metadata: &mut v14::RuntimeMetadataV14,
) -> Result<frame_metadata::v15::OuterEnums<scale_info::form::PortableForm>, TryFromError> {
    let outer_enums = OuterEnums::find_in(&metadata.types);

    let Some(call_enum_id) = outer_enums.call_ty else {
        return Err(TryFromError::TypeNameNotFound("RuntimeCall".into()));
    };
    let Some(event_type_id) = outer_enums.event_ty else {
        return Err(TryFromError::TypeNameNotFound("RuntimeEvent".into()));
    };
    let error_type_id = if let Some(id) = outer_enums.error_ty {
        id
    } else {
        let call_enum = &metadata.types.types[call_enum_id as usize];
        let mut error_path = call_enum.ty.path.segments.clone();

        let Some(last) = error_path.last_mut() else {
            return Err(TryFromError::InvalidTypePath("RuntimeCall".into()));
        };
        "RuntimeError".clone_into(last);
        generate_outer_error_enum_type(metadata, error_path)
    };

    Ok(frame_metadata::v15::OuterEnums {
        call_enum_ty: call_enum_id.into(),
        event_enum_ty: event_type_id.into(),
        error_enum_ty: error_type_id.into(),
    })
}

/// Generates an outer `RuntimeError` enum type and adds it to the metadata.
///
/// Returns the id of the generated type from the registry.
fn generate_outer_error_enum_type(
    metadata: &mut v14::RuntimeMetadataV14,
    path_segments: Vec<String>,
) -> u32 {
    let variants: Vec<_> = metadata
        .pallets
        .iter()
        .filter_map(|pallet| {
            let error = pallet.error.as_ref()?;
            let path = format!("{}Error", pallet.name);
            let ty = error.ty.id.into();

            Some(scale_info::Variant {
                name: pallet.name.clone(),
                fields: vec![scale_info::Field {
                    name: None,
                    ty,
                    type_name: Some(path),
                    docs: vec![],
                }],
                index: pallet.index,
                docs: vec![],
            })
        })
        .collect();

    let enum_type = scale_info::Type {
        path: scale_info::Path {
            segments: path_segments,
        },
        type_params: vec![],
        type_def: scale_info::TypeDef::Variant(scale_info::TypeDefVariant { variants }),
        docs: vec![],
    };

    let enum_type_id = metadata.types.types.len() as u32;

    metadata.types.types.push(scale_info::PortableType {
        id: enum_type_id,
        ty: enum_type,
    });

    enum_type_id
}

/// The type IDs extracted from the metadata that represent the
/// generic type parameters passed to the `UncheckedExtrinsic` from
/// the substrate-based chain.
#[derive(Clone, Copy)]
struct MissingExtrinsicTypeIds {
    address: u32,
    signature: u32,
}

impl MissingExtrinsicTypeIds {
    fn generate_from(
        metadata: &v14::RuntimeMetadataV14,
    ) -> Result<MissingExtrinsicTypeIds, TryFromError> {
        const ADDRESS: &str = "Address";
        const SIGNATURE: &str = "Signature";

        let extrinsic_id = metadata.extrinsic.ty.id;
        let Some(extrinsic_ty) = metadata.types.resolve(extrinsic_id) else {
            return Err(TryFromError::TypeNotFound(extrinsic_id));
        };

        let find_param = |name: &'static str| -> Option<u32> {
            extrinsic_ty
                .type_params
                .iter()
                .find(|param| param.name.as_str() == name)
                .and_then(|param| param.ty.as_ref())
                .map(|ty| ty.id)
        };

        let Some(address) = find_param(ADDRESS) else {
            return Err(TryFromError::TypeNameNotFound(ADDRESS.into()));
        };
        let Some(signature) = find_param(SIGNATURE) else {
            return Err(TryFromError::TypeNameNotFound(SIGNATURE.into()));
        };

        Ok(MissingExtrinsicTypeIds { address, signature })
    }
}

/// Outer enum IDs, which are required in Subxt but are not present in V14 metadata.
pub struct OuterEnums {
    /// The RuntimeCall type ID.
    pub call_ty: Option<u32>,
    /// The RuntimeEvent type ID.
    pub event_ty: Option<u32>,
    /// The RuntimeError type ID.
    pub error_ty: Option<u32>,
}

impl OuterEnums {
    pub fn find_in(types: &scale_info::PortableRegistry) -> OuterEnums {
        // Find a type by its ident (last segment of path)
        let find_type_by_ident = |name: &str| {
            types.types.iter().find_map(|ty| {
                let ident = ty.ty.path.ident()?;

                if ident != name {
                    return None;
                }

                let scale_info::TypeDef::Variant(_) = &ty.ty.type_def else {
                    return None;
                };

                Some(ty.id)
            })
        };

        // Find a type by checking if path ends with specific segments (e.g., ["polkadot_runtime", "Call"])
        // This handles older runtimes where the outer enums were named just "Call"/"Event"
        // in the runtime module rather than "RuntimeCall"/"RuntimeEvent"
        let find_runtime_outer_enum = |short_name: &str| {
            types.types.iter().find_map(|ty| {
                let segments = &ty.ty.path.segments;

                // Check if this is a variant type
                let scale_info::TypeDef::Variant(_) = &ty.ty.type_def else {
                    return None;
                };

                // Check if path ends with ["*_runtime", short_name] (e.g., ["polkadot_runtime", "Call"])
                if segments.len() >= 2 {
                    let last = segments.last()?;
                    let second_last = segments.get(segments.len() - 2)?;

                    if last == short_name && second_last.ends_with("_runtime") {
                        return Some(ty.id);
                    }
                }

                None
            })
        };

        // First try the modern names (RuntimeCall, RuntimeEvent, RuntimeError)
        // If not found, fall back to legacy names (Call, Event in *_runtime module)
        let call_ty = find_type_by_ident("RuntimeCall").or_else(|| find_runtime_outer_enum("Call"));
        let event_ty =
            find_type_by_ident("RuntimeEvent").or_else(|| find_runtime_outer_enum("Event"));
        let error_ty = find_type_by_ident("RuntimeError");

        OuterEnums {
            call_ty,
            event_ty,
            error_ty,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Metadata;

    // In early V14 metadatas, RuntimeCall, RuntimeEvent and RuntimeError didn't exist, and
    // instead we have types like polkadot_runtime::Call, kusama_runtime::Call etc.
    // See https://github.com/paritytech/subxt/pull/2154.
    #[test]
    fn can_decode_early_polkadot_rc_v14_metadata() {
        let md_bytes = std::fs::read("../artifacts/polkadot_metadata_v14_9110.scale").unwrap();
        let _md = Metadata::decode_from(&md_bytes).unwrap();
    }
}
