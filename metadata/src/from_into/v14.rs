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

    let mut path_segments = call_enum.ty.path.segments.clone();
    let last = path_segments
        .last_mut()
        .expect("Should have at least one segment checked above; qed");
    *last = "RuntimeError".to_string();

    let error_ty_id = generate_runtime_error_type(metadata, path_segments);

    v15::OuterEnums {
        call_enum_ty: call_ty,
        event_enum_ty: event_ty,
        error_enum_ty: error_ty_id.into(),
    }
}

/// Generate the `RuntimeError` type and add it to the metadata.
///
/// Returns the `RuntimeError` Id from the registry.
fn generate_runtime_error_type(
    metadata: &mut v14::RuntimeMetadataV14,
    path_segments: Vec<String>,
) -> u32 {
    let variants: Vec<_> = metadata
        .pallets
        .iter()
        .filter_map(|pallet| {
            let Some(pallet_error) = &pallet.error else { return None };
            let path = format!("{}Error", pallet.name);

            Some(scale_info::Variant {
                name: pallet.name.clone(),
                fields: vec![scale_info::Field {
                    name: None,
                    ty: pallet_error.ty.id.into(),
                    type_name: Some(path),
                    docs: vec![],
                }],
                index: pallet.index,
                docs: vec![],
            })
        })
        .collect();

    let error_type = scale_info::Type {
        path: scale_info::Path {
            segments: path_segments,
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

#[cfg(test)]
mod tests {
    use super::*;
    use codec::Decode;
    use frame_metadata::{v15::RuntimeMetadataV15, RuntimeMetadata, RuntimeMetadataPrefixed};
    use scale_info::TypeDef;
    use std::{fs, path::Path};

    fn load_v15_metadata() -> RuntimeMetadataV15 {
        let bytes = fs::read(Path::new("../artifacts/polkadot_metadata_full.scale"))
            .expect("Cannot read metadata blob");
        let meta: RuntimeMetadataPrefixed =
            Decode::decode(&mut &*bytes).expect("Cannot decode scale metadata");

        match meta.1 {
            RuntimeMetadata::V15(v15) => v15,
            _ => panic!("Unsupported metadata version {:?}", meta.1),
        }
    }

    #[test]
    fn test_extrinsic_id_generation() {
        let v15 = load_v15_metadata();
        let v14 = v15_to_v14(v15.clone());

        let ext_ty = v14.types.resolve(v14.extrinsic.ty.id).unwrap();
        let addr_id = ext_ty
            .type_params
            .iter()
            .find_map(|ty| {
                if ty.name == "Address" {
                    Some(ty.ty.unwrap().id)
                } else {
                    None
                }
            })
            .unwrap();
        let call_id = ext_ty
            .type_params
            .iter()
            .find_map(|ty| {
                if ty.name == "Call" {
                    Some(ty.ty.unwrap().id)
                } else {
                    None
                }
            })
            .unwrap();
        let extra_id = ext_ty
            .type_params
            .iter()
            .find_map(|ty| {
                if ty.name == "Extra" {
                    Some(ty.ty.unwrap().id)
                } else {
                    None
                }
            })
            .unwrap();
        let signature_id = ext_ty
            .type_params
            .iter()
            .find_map(|ty| {
                if ty.name == "Signature" {
                    Some(ty.ty.unwrap().id)
                } else {
                    None
                }
            })
            .unwrap();

        // Position in type registry shouldn't change.
        assert_eq!(v15.extrinsic.address_ty.id, addr_id);
        assert_eq!(v15.extrinsic.call_ty.id, call_id);
        assert_eq!(v15.extrinsic.extra_ty.id, extra_id);
        assert_eq!(v15.extrinsic.signature_ty.id, signature_id);

        let v15_addr = v15.types.resolve(v15.extrinsic.address_ty.id).unwrap();
        let v14_addr = v14.types.resolve(addr_id).unwrap();
        assert_eq!(v15_addr, v14_addr);

        let v15_call = v15.types.resolve(v15.extrinsic.call_ty.id).unwrap();
        let v14_call = v14.types.resolve(call_id).unwrap();
        assert_eq!(v15_call, v14_call);

        let v15_extra = v15.types.resolve(v15.extrinsic.extra_ty.id).unwrap();
        let v14_extra = v14.types.resolve(extra_id).unwrap();
        assert_eq!(v15_extra, v14_extra);

        let v15_sign = v15.types.resolve(v15.extrinsic.signature_ty.id).unwrap();
        let v14_sign = v14.types.resolve(signature_id).unwrap();
        assert_eq!(v15_sign, v14_sign);

        // Ensure we don't lose the information when converting back to v15.
        let converted_v15 = v14_to_v15(v14);

        let v15_addr = v15.types.resolve(v15.extrinsic.address_ty.id).unwrap();
        let converted_v15_addr = converted_v15
            .types
            .resolve(converted_v15.extrinsic.address_ty.id)
            .unwrap();
        assert_eq!(v15_addr, converted_v15_addr);

        let v15_call = v15.types.resolve(v15.extrinsic.call_ty.id).unwrap();
        let converted_v15_call = converted_v15
            .types
            .resolve(converted_v15.extrinsic.call_ty.id)
            .unwrap();
        assert_eq!(v15_call, converted_v15_call);

        let v15_extra = v15.types.resolve(v15.extrinsic.extra_ty.id).unwrap();
        let converted_v15_extra = converted_v15
            .types
            .resolve(converted_v15.extrinsic.extra_ty.id)
            .unwrap();
        assert_eq!(v15_extra, converted_v15_extra);

        let v15_sign = v15.types.resolve(v15.extrinsic.signature_ty.id).unwrap();
        let converted_v15_sign = converted_v15
            .types
            .resolve(converted_v15.extrinsic.signature_ty.id)
            .unwrap();
        assert_eq!(v15_sign, converted_v15_sign);
    }

    #[test]
    fn test_outer_enums_generation() {
        let v15 = load_v15_metadata();
        let v14 = v15_to_v14(v15.clone());

        // Convert back to v15 and expect to have the enum types properly generated.
        let converted_v15 = v14_to_v15(v14);

        // RuntimeCall and RuntimeEvent were already present in the metadata v14.
        let v15_call = v15.types.resolve(v15.outer_enums.call_enum_ty.id).unwrap();
        let converted_v15_call = converted_v15
            .types
            .resolve(converted_v15.outer_enums.call_enum_ty.id)
            .unwrap();
        assert_eq!(v15_call, converted_v15_call);

        let v15_event = v15.types.resolve(v15.outer_enums.event_enum_ty.id).unwrap();
        let converted_v15_event = converted_v15
            .types
            .resolve(converted_v15.outer_enums.event_enum_ty.id)
            .unwrap();
        assert_eq!(v15_event, converted_v15_event);

        let v15_error = v15.types.resolve(v15.outer_enums.error_enum_ty.id).unwrap();
        let converted_v15_error = converted_v15
            .types
            .resolve(converted_v15.outer_enums.error_enum_ty.id)
            .unwrap();

        // Ensure they match in terms of variants and fields ids.
        assert_eq!(v15_error.path, converted_v15_error.path);

        let TypeDef::Variant(v15_variant) = &v15_error.type_def else {
            panic!("V15 error must be a variant");
        };

        let TypeDef::Variant(converted_v15_variant) = &converted_v15_error.type_def else {
            panic!("Converted V15 error must be a variant");
        };

        assert_eq!(
            v15_variant.variants.len(),
            converted_v15_variant.variants.len()
        );

        for (v15_var, converted_v15_var) in v15_variant
            .variants
            .iter()
            .zip(converted_v15_variant.variants.iter())
        {
            // Variant name must match.
            assert_eq!(v15_var.name, converted_v15_var.name);
            assert_eq!(v15_var.fields.len(), converted_v15_var.fields.len());

            // Fields must have the same type.
            for (v15_field, converted_v15_field) in
                v15_var.fields.iter().zip(converted_v15_var.fields.iter())
            {
                assert_eq!(v15_field.ty.id, converted_v15_field.ty.id);

                let ty = v15.types.resolve(v15_field.ty.id).unwrap();
                let converted_ty = converted_v15
                    .types
                    .resolve(converted_v15_field.ty.id)
                    .unwrap();
                assert_eq!(ty, converted_ty);
            }
        }
    }
}
