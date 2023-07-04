// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::collections::HashMap;

use super::TryFromError;
use crate::Metadata;
use frame_metadata::{v14, v15};

impl TryFrom<v14::RuntimeMetadataV14> for Metadata {
    type Error = TryFromError;
    fn try_from(value: v14::RuntimeMetadataV14) -> Result<Self, Self::Error> {
        // Convert to v15 and then convert that into Metadata.
        v14_to_v15(value).try_into()
    }
}

impl From<Metadata> for v14::RuntimeMetadataV14 {
    fn from(val: Metadata) -> Self {
        let v15 = val.into();
        v15_to_v14(v15)
    }
}

fn v15_to_v14(mut metadata: v15::RuntimeMetadataV15) -> v14::RuntimeMetadataV14 {
    let types = &mut metadata.types;

    // In subxt we care about the `Address`, `Call`, `Signature` and `Extra` types.
    let extrinsic_type = scale_info::Type {
        path: scale_info::Path {
            segments: vec![
                "primitives".to_string(),
                "runtime".to_string(),
                "generic".to_string(),
                "UncheckedExtrinsic".to_string(),
            ],
        },
        type_params: vec![
            scale_info::TypeParameter::<scale_info::form::PortableForm> {
                name: "Address".to_string(),
                ty: Some(metadata.extrinsic.address_ty),
            },
            scale_info::TypeParameter::<scale_info::form::PortableForm> {
                name: "Call".to_string(),
                ty: Some(metadata.extrinsic.call_ty),
            },
            scale_info::TypeParameter::<scale_info::form::PortableForm> {
                name: "Signature".to_string(),
                ty: Some(metadata.extrinsic.signature_ty),
            },
            scale_info::TypeParameter::<scale_info::form::PortableForm> {
                name: "Extra".to_string(),
                ty: Some(metadata.extrinsic.extra_ty),
            },
        ],
        type_def: scale_info::TypeDef::Composite(scale_info::TypeDefComposite { fields: vec![] }),
        docs: vec![],
    };
    let extrinsic_type_id = types.types.len() as u32;

    types.types.push(scale_info::PortableType {
        id: extrinsic_type_id,
        ty: extrinsic_type,
    });

    v14::RuntimeMetadataV14 {
        types: metadata.types,
        pallets: metadata
            .pallets
            .into_iter()
            .map(|pallet| frame_metadata::v14::PalletMetadata {
                name: pallet.name,
                storage: pallet
                    .storage
                    .map(|storage| frame_metadata::v14::PalletStorageMetadata {
                        prefix: storage.prefix,
                        entries: storage
                            .entries
                            .into_iter()
                            .map(|entry| {
                                let modifier = match entry.modifier {
                                    frame_metadata::v15::StorageEntryModifier::Optional => {
                                        frame_metadata::v14::StorageEntryModifier::Optional
                                    }
                                    frame_metadata::v15::StorageEntryModifier::Default => {
                                        frame_metadata::v14::StorageEntryModifier::Default
                                    }
                                };

                                let ty = match entry.ty {
                                    frame_metadata::v15::StorageEntryType::Plain(ty) => {
                                        frame_metadata::v14::StorageEntryType::Plain(ty)
                                    },
                                    frame_metadata::v15::StorageEntryType::Map {
                                        hashers,
                                        key,
                                        value,
                                    } => frame_metadata::v14::StorageEntryType::Map {
                                        hashers: hashers.into_iter().map(|hasher| match hasher {
                                            frame_metadata::v15::StorageHasher::Blake2_128 => frame_metadata::v14::StorageHasher::Blake2_128,
                                            frame_metadata::v15::StorageHasher::Blake2_256 => frame_metadata::v14::StorageHasher::Blake2_256,
                                            frame_metadata::v15::StorageHasher::Blake2_128Concat  => frame_metadata::v14::StorageHasher::Blake2_128Concat ,
                                            frame_metadata::v15::StorageHasher::Twox128 => frame_metadata::v14::StorageHasher::Twox128,
                                            frame_metadata::v15::StorageHasher::Twox256 => frame_metadata::v14::StorageHasher::Twox256,
                                            frame_metadata::v15::StorageHasher::Twox64Concat => frame_metadata::v14::StorageHasher::Twox64Concat,
                                            frame_metadata::v15::StorageHasher::Identity=> frame_metadata::v14::StorageHasher::Identity,
                                        }).collect(),
                                        key,
                                        value,
                                    },
                                };

                                frame_metadata::v14::StorageEntryMetadata {
                                    name: entry.name,
                                    modifier,
                                    ty,
                                    default: entry.default,
                                    docs: entry.docs,
                                }
                            })
                            .collect(),
                    }),
                calls: pallet.calls.map(|calls| frame_metadata::v14::PalletCallMetadata { ty: calls.ty } ),
                event: pallet.event.map(|event| frame_metadata::v14::PalletEventMetadata { ty: event.ty } ),
                constants: pallet.constants.into_iter().map(|constant| frame_metadata::v14::PalletConstantMetadata {
                    name: constant.name,
                    ty: constant.ty,
                    value: constant.value,
                    docs: constant.docs,
                } ).collect(),
                error: pallet.error.map(|error| frame_metadata::v14::PalletErrorMetadata { ty: error.ty } ),
                index: pallet.index,
            })
            .collect(),
        extrinsic: frame_metadata::v14::ExtrinsicMetadata {
            ty: extrinsic_type_id.into(),
            version: metadata.extrinsic.version,
            signed_extensions: metadata.extrinsic.signed_extensions.into_iter().map(|ext| {
                frame_metadata::v14::SignedExtensionMetadata {
                    identifier: ext.identifier,
                    ty: ext.ty,
                    additional_signed: ext.additional_signed,
                }
            }).collect()
        },
        ty: metadata.ty,
    }
}

fn v14_to_v15(mut metadata: v14::RuntimeMetadataV14) -> v15::RuntimeMetadataV15 {
    // Find the extrinsic types.
    let extrinsic_parts = ExtrinsicPartTypeIds::new(&metadata)
        .expect("Extrinsic types are always present on V14; qed");

    let outer_enums = generate_outer_enums(&mut metadata);

    v15::RuntimeMetadataV15 {
        types: metadata.types,
        pallets: metadata
            .pallets
            .into_iter()
            .map(|pallet| frame_metadata::v15::PalletMetadata {
                name: pallet.name,
                storage: pallet
                    .storage
                    .map(|storage| frame_metadata::v15::PalletStorageMetadata {
                        prefix: storage.prefix,
                        entries: storage
                            .entries
                            .into_iter()
                            .map(|entry| {
                                let modifier = match entry.modifier {
                                    frame_metadata::v14::StorageEntryModifier::Optional => {
                                        frame_metadata::v15::StorageEntryModifier::Optional
                                    }
                                    frame_metadata::v14::StorageEntryModifier::Default => {
                                        frame_metadata::v15::StorageEntryModifier::Default
                                    }
                                };

                                let ty = match entry.ty {
                                    frame_metadata::v14::StorageEntryType::Plain(ty) => {
                                        frame_metadata::v15::StorageEntryType::Plain(ty)
                                    },
                                    frame_metadata::v14::StorageEntryType::Map {
                                        hashers,
                                        key,
                                        value,
                                    } => frame_metadata::v15::StorageEntryType::Map {
                                        hashers: hashers.into_iter().map(|hasher| match hasher {
                                            frame_metadata::v14::StorageHasher::Blake2_128 => frame_metadata::v15::StorageHasher::Blake2_128,
                                            frame_metadata::v14::StorageHasher::Blake2_256 => frame_metadata::v15::StorageHasher::Blake2_256,
                                            frame_metadata::v14::StorageHasher::Blake2_128Concat  => frame_metadata::v15::StorageHasher::Blake2_128Concat ,
                                            frame_metadata::v14::StorageHasher::Twox128 => frame_metadata::v15::StorageHasher::Twox128,
                                            frame_metadata::v14::StorageHasher::Twox256 => frame_metadata::v15::StorageHasher::Twox256,
                                            frame_metadata::v14::StorageHasher::Twox64Concat => frame_metadata::v15::StorageHasher::Twox64Concat,
                                            frame_metadata::v14::StorageHasher::Identity=> frame_metadata::v15::StorageHasher::Identity,
                                        }).collect(),
                                        key,
                                        value,
                                    },
                                };

                                frame_metadata::v15::StorageEntryMetadata {
                                    name: entry.name,
                                    modifier,
                                    ty,
                                    default: entry.default,
                                    docs: entry.docs,
                                }
                            })
                            .collect(),
                    }),
                calls: pallet.calls.map(|calls| frame_metadata::v15::PalletCallMetadata { ty: calls.ty } ),
                event: pallet.event.map(|event| frame_metadata::v15::PalletEventMetadata { ty: event.ty } ),
                constants: pallet.constants.into_iter().map(|constant| frame_metadata::v15::PalletConstantMetadata {
                    name: constant.name,
                    ty: constant.ty,
                    value: constant.value,
                    docs: constant.docs,
                } ).collect(),
                error: pallet.error.map(|error| frame_metadata::v15::PalletErrorMetadata { ty: error.ty } ),
                index: pallet.index,
                docs: Default::default(),
            })
            .collect(),
        extrinsic: frame_metadata::v15::ExtrinsicMetadata {
            version: metadata.extrinsic.version,
            signed_extensions: metadata.extrinsic.signed_extensions.into_iter().map(|ext| {
                frame_metadata::v15::SignedExtensionMetadata {
                    identifier: ext.identifier,
                    ty: ext.ty,
                    additional_signed: ext.additional_signed,
                }
            }).collect(),
            address_ty: extrinsic_parts.address.into(),
            call_ty: extrinsic_parts.call.into(),
            signature_ty: extrinsic_parts.signature.into(),
            extra_ty: extrinsic_parts.extra.into(),
        },
        ty: metadata.ty,
        apis: Default::default(),
        outer_enums,
        custom: v15::CustomMetadata {
            map: Default::default(),
        },
    }
}

/// The type IDs extracted from the metadata that represent the
/// generic type parameters passed to the `UncheckedExtrinsic` from
/// the substrate-based chain.
struct ExtrinsicPartTypeIds {
    address: u32,
    call: u32,
    signature: u32,
    extra: u32,
}

impl ExtrinsicPartTypeIds {
    /// Extract the generic type parameters IDs from the extrinsic type.
    fn new(metadata: &v14::RuntimeMetadataV14) -> Result<Self, String> {
        const ADDRESS: &str = "Address";
        const CALL: &str = "Call";
        const SIGNATURE: &str = "Signature";
        const EXTRA: &str = "Extra";

        let extrinsic_id = metadata.extrinsic.ty.id;
        let Some(extrinsic_ty) = metadata.types.resolve(extrinsic_id) else {
            return Err("Missing extrinsic type".into())
        };

        let params: HashMap<_, _> = extrinsic_ty
            .type_params
            .iter()
            .map(|ty_param| {
                let Some(ty) = ty_param.ty else {
                    return Err("Missing type param type from extrinsic".to_string());
                };

                Ok((ty_param.name.as_str(), ty.id))
            })
            .collect::<Result<_, _>>()?;

        let Some(address) = params.get(ADDRESS) else {
            return Err("Missing address type from extrinsic".into());
        };
        let Some(call) = params.get(CALL) else {
            return Err("Missing call type from extrinsic".into());
        };
        let Some(signature) = params.get(SIGNATURE) else {
            return Err("Missing signature type from extrinsic".into());
        };
        let Some(extra) = params.get(EXTRA) else {
            return Err("Missing extra type from extrinsic".into());
        };

        Ok(ExtrinsicPartTypeIds {
            address: *address,
            call: *call,
            signature: *signature,
            extra: *extra,
        })
    }
}

fn generate_outer_enums(
    metadata: &mut v14::RuntimeMetadataV14,
) -> v15::OuterEnums<scale_info::form::PortableForm> {
    let call_enum = metadata
        .types
        .types
        .iter()
        .find(|ty| {
            let Some(ident) = ty.ty.path.ident() else { return false };
            ident == "RuntimeCall"
        })
        .expect("RuntimeCall exists in V14; qed");

    let event_enum = metadata
        .types
        .types
        .iter()
        .find(|ty| {
            let Some(ident) = ty.ty.path.ident() else { return false };
            ident == "RuntimeEvent"
        })
        .expect("RuntimeEvent exists in V14; qed");

    let call_ty = call_enum.id.into();
    let event_ty = event_enum.id.into();
    let error_ty_id = generate_runtime_error_type(metadata);

    v15::OuterEnums {
        call_enum_ty: call_ty,
        event_enum_ty: event_ty,
        error_enum_ty: error_ty_id.into(),
    }
}

/// Generate the `RuntimeError` type and add it to the metadata.
///
/// Returns the `RuntimeError` Id from the registry.
fn generate_runtime_error_type(metadata: &mut v14::RuntimeMetadataV14) -> u32 {
    let error_types: HashMap<_, _> = metadata
        .types
        .types
        .iter()
        .filter_map(|ty| {
            let segments = &ty.ty.path.segments;
            // Interested in segments that end with `pallet::Error`.
            let len = segments.len();
            if len < 2 {
                return None;
            }

            if segments[len - 2] != "pallet" || segments[len - 1] != "Error" {
                return None;
            }

            let pallet_name = segments[0].as_str();
            let pallet_name = if let Some(name) = pallet_name.strip_prefix("pallet_") {
                name.to_lowercase()
            } else if let Some(name) = pallet_name.strip_prefix("frame_") {
                name.to_lowercase()
            } else {
                pallet_name.to_lowercase()
            };

            Some((pallet_name, ty))
        })
        .collect();

    let variants: Vec<_> = metadata
        .pallets
        .iter()
        .filter_map(|pallet| {
            let Some(entry) = error_types.get(&pallet.name.to_lowercase()) else { return None };

            Some(scale_info::Variant {
                name: pallet.name.clone(),
                fields: vec![scale_info::Field {
                    name: None,
                    ty: entry.id.into(),
                    type_name: Some(format!("pallet_{}::Error<Runtime>", pallet.name)),
                    docs: vec![],
                }],
                index: pallet.index,
                docs: vec![],
            })
        })
        .collect();

    let error_type = scale_info::Type {
        path: scale_info::Path {
            segments: vec![
                "kitchensink_runtime".to_string(),
                "RuntimeError".to_string(),
            ],
        },
        type_params: vec![],
        type_def: scale_info::TypeDef::Variant(scale_info::TypeDefVariant { variants }),
        docs: vec![],
    };

    let error_type_id = metadata.types.types.len() as u32;

    metadata.types.types.push(scale_info::PortableType {
        id: error_type_id,
        ty: error_type,
    });

    error_type_id
}
