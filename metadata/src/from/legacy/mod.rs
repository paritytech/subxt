mod portable_registry_builder;

use crate::Metadata;
use crate::utils::ordered_map::OrderedMap;
use crate::utils::variant_index::VariantIndex;
use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use frame_decode::constants::ConstantTypeInfo;
use frame_decode::extrinsics::ExtrinsicTypeInfo;
use frame_decode::runtime_apis::RuntimeApiTypeInfo;
use frame_decode::storage::StorageTypeInfo;
use frame_metadata::v15;
use portable_registry_builder::PortableRegistryBuilder;
use scale_info_legacy::TypeRegistrySet;
use scale_info_legacy::type_registry::RuntimeApiName;

macro_rules! from_historic {
    ($vis:vis fn $fn_name:ident($metadata:path $(, builtin_index: $builtin_index:ident)? )) => {
        $vis fn $fn_name(metadata: &$metadata, mut types: TypeRegistrySet<'_>) -> Result<Metadata, Error> {
            // Extend the types with important information from the metadata:
            {
                let builtin_types = frame_decode::helpers::type_registry_from_metadata(metadata)
                    .map_err(Error::CannotEnhanceTypesFromMetadata)?;
                types.prepend(builtin_types);
            }

            // This will be used to construct our `PortableRegistry` from old-style types.
            let mut portable_registry_builder = PortableRegistryBuilder::new(&types);

            // We use this type in a few places to denote that we don't know how to decode it.
            let unknown_type_id = portable_registry_builder.add_type_str("special::Unknown", None)
                .map_err(|e| Error::add_type("constructing 'Unknown' type", e))?;

            // Pallet metadata
            let mut call_index = 0u8;
            let mut error_index = 0u8;
            let mut event_index = 0u8;

            let new_pallets = as_decoded(&metadata.modules).iter().map(|pallet| {
                // In older metadatas, calls and event enums can have different indexes
                // in a given pallet. Pallets without calls or events don't increment
                // the respective index for them.
                //
                // We assume since errors are non optional, that the pallet index _always_
                // increments for errors (no `None`s to skip).
                let (call_index, event_index, error_index) = {
                    let out = (call_index, event_index, error_index);
                    if pallet.calls.is_some() {
                        call_index += 1;
                    }
                    if pallet.event.is_some() {
                        event_index += 1;
                    }
                    error_index += 1;

                    out
                };

                // For v12 and v13 metadata, there is a builtin index for everything in a pallet.
                // We enable this logic for those metadatas to get the correct index.
                $(
                    let $builtin_index = true;
                    let (call_index, event_index, error_index) = if $builtin_index {
                        (pallet.index, pallet.index, pallet.index)
                    } else {
                        (call_index, event_index, error_index)
                    };
                )?

                let pallet_name = as_decoded(&pallet.name).to_string();

                // Storage entries:
                let storage = pallet.storage.as_ref().map(|s| {
                    let storage = as_decoded(s);
                    let prefix = as_decoded(&storage.prefix);
                    let entries = metadata.storage_entries_in_pallet(&pallet_name).map(|entry_name| {
                        let info = metadata
                            .storage_info(&pallet_name, &entry_name)
                            .map_err(|e| Error::StorageInfoError(e.into_owned()))?;
                        let entry_name = entry_name.into_owned();

                        let info = info.map_ids(|old_id| {
                            portable_registry_builder.add_type(old_id)
                        }).map_err(|e| {
                            let ctx = format!("adding type used in storage entry {pallet_name}.{entry_name}");
                            Error::add_type(ctx, e)
                        })?;

                        let entry = crate::StorageEntryMetadata {
                            name: entry_name.clone(),
                            info: info.into_owned(),
                            // We don't expose docs via our storage info yet.
                            docs: Vec::new(),
                        };

                        Ok((entry_name, entry))
                    }).collect::<Result<OrderedMap<_, _>, _>>()?;

                    Ok(crate::StorageMetadata {
                        prefix: prefix.clone(),
                        entries,
                    })
                }).transpose()?;

                // Pallet error type is just a builtin type:
                let error_ty = portable_registry_builder.add_type_str(&format!("builtin::module::error::{pallet_name}"), None)
                    .map_err(|e| {
                        let ctx = format!("converting the error enum for pallet {pallet_name}");
                        Error::add_type(ctx, e)
                    })?;

                // Pallet calls also just a builtin type:
                let call_ty = pallet.calls.as_ref().map(|_| {
                    portable_registry_builder.add_type_str(&format!("builtin::module::call::{pallet_name}"), None)
                        .map_err(|e| {
                            let ctx = format!("converting the call enum for pallet {pallet_name}");
                            Error::add_type(ctx, e)
                        })
                }).transpose()?;

                // Pallet events also just a builtin type:
                let event_ty = pallet.event.as_ref().map(|_| {
                    portable_registry_builder.add_type_str(&format!("builtin::module::event::{pallet_name}"), None)
                        .map_err(|e| {
                            let ctx = format!("converting the event enum for pallet {pallet_name}");
                            Error::add_type(ctx, e)
                        })
                }).transpose()?;

                let call_variant_index =
                    VariantIndex::build(call_ty, portable_registry_builder.types());
                let error_variant_index =
                    VariantIndex::build(Some(error_ty), portable_registry_builder.types());
                let event_variant_index =
                    VariantIndex::build(event_ty, portable_registry_builder.types());

                let constants = metadata.constants_in_pallet(&pallet_name).map(|name| {
                    let name = name.into_owned();
                    let info = metadata.constant_info(&pallet_name, &name)
                        .map_err(|e| Error::ConstantInfoError(e.into_owned()))?;
                    let new_type_id = portable_registry_builder.add_type(info.type_id)
                        .map_err(|e| {
                            let ctx = format!("converting the constant {name} for pallet {pallet_name}");
                            Error::add_type(ctx, e)
                        })?;

                    let constant = crate::ConstantMetadata {
                        name: name.clone(),
                        ty: new_type_id,
                        value: info.bytes.to_vec(),
                        // We don't expose docs via our constant info yet.
                        docs: Vec::new(),
                    };

                    Ok((name, constant))
                }).collect::<Result<_,Error>>()?;

                let pallet_metadata = crate::PalletMetadataInner {
                    name: pallet_name.clone(),
                    call_index,
                    event_index,
                    error_index,
                    storage,
                    error_ty: Some(error_ty),
                    call_ty,
                    event_ty,
                    call_variant_index,
                    error_variant_index,
                    event_variant_index,
                    constants,
                    view_functions: Default::default(),
                    associated_types: Default::default(),
                    // Pallets did not have docs prior to V15.
                    docs: Default::default(),
                };

                Ok((pallet_name, pallet_metadata))
            }).collect::<Result<OrderedMap<_,_>,Error>>()?;

            // Extrinsic metadata
            let new_extrinsic = {
                let signature_info = metadata
                    .extrinsic_signature_info()
                    .map_err(|e| Error::ExtrinsicInfoError(e.into_owned()))?;

                let address_ty_id = portable_registry_builder.add_type(signature_info.address_id)
                    .map_err(|_| Error::CannotFindAddressType)?;

                let signature_ty_id = portable_registry_builder.add_type(signature_info.signature_id)
                    .map_err(|_| Error::CannotFindCallType)?;

                let transaction_extensions = metadata
                    .extrinsic_extension_info(None)
                    .map_err(|e| Error::ExtrinsicInfoError(e.into_owned()))?
                    .extension_ids
                    .into_iter()
                    .map(|ext| {
                        let ext_name = ext.name.into_owned();
                        let ext_type = portable_registry_builder.add_type(ext.id)
                            .map_err(|e| {
                                let ctx = format!("converting the signed extension {ext_name}");
                                Error::add_type(ctx, e)
                            })?;

                        Ok(crate::TransactionExtensionMetadataInner {
                            identifier: ext_name,
                            extra_ty: ext_type,
                            // This only started existing in V14+ metadata, but in any case,
                            // we don't need to know how to decode the signed payload for
                            // historic blocks (hopefully), so set to unknown.
                            additional_ty: unknown_type_id.into()
                        })
                    })
                    .collect::<Result<Vec<_>,Error>>()?;

                let transaction_extensions_by_version = BTreeMap::from_iter([(
                    0,
                    (0..transaction_extensions.len() as u32).collect()
                )]);

                crate::ExtrinsicMetadata {
                    address_ty: address_ty_id.into(),
                    signature_ty: signature_ty_id.into(),
                    supported_versions: Vec::from_iter([4]),
                    transaction_extensions,
                    transaction_extensions_by_version,
                }
            };

            // Outer enum types
            let outer_enums = crate::OuterEnumsMetadata {
                call_enum_ty: portable_registry_builder.add_type_str("builtin::Call", None)
                    .map_err(|e| {
                        let ctx = format!("constructing the 'builtin::Call' type to put in the OuterEnums metadata");
                        Error::add_type(ctx, e)
                    })?,
                event_enum_ty: portable_registry_builder.add_type_str("builtin::Event", None)
                    .map_err(|e| {
                        let ctx = format!("constructing the 'builtin::Event' type to put in the OuterEnums metadata");
                        Error::add_type(ctx, e)
                    })?,
                error_enum_ty: portable_registry_builder.add_type_str("builtin::Error", None)
                    .map_err(|e| {
                        let ctx = format!("constructing the 'builtin::Error' type to put in the OuterEnums metadata");
                        Error::add_type(ctx, e)
                    })?,
            };

            // These are all the same in V13, but be explicit anyway for clarity.
            let pallets_by_call_index = new_pallets
                .values()
                .iter()
                .enumerate()
                .map(|(idx,p)| (p.call_index, idx))
                .collect();
            let pallets_by_error_index = new_pallets
                .values()
                .iter()
                .enumerate()
                .map(|(idx,p)| (p.error_index, idx))
                .collect();
            let pallets_by_event_index = new_pallets
                .values()
                .iter()
                .enumerate()
                .map(|(idx,p)| (p.event_index, idx))
                .collect();

            // This is optional in the sense that Subxt will return an error if it needs to decode this type,
            // and I think for historic metadata we wouldn't end up down that path anyway. Historic metadata
            // tends to call it just "DispatchError" but search more specific paths first.
            let dispatch_error_ty = portable_registry_builder
                .try_add_type_str("hardcoded::DispatchError", None)
                .or_else(|| portable_registry_builder.try_add_type_str("sp_runtime::DispatchError", None))
                .or_else(|| portable_registry_builder.try_add_type_str("DispatchError", None))
                .transpose()
                .map_err(|e| Error::add_type("constructing DispatchError", e))?;

            // Runtime API definitions live with type definitions.
            let apis = type_registry_to_runtime_apis(&types, &mut portable_registry_builder)?;

            Ok(crate::Metadata {
                types: portable_registry_builder.finish(),
                pallets: new_pallets,
                pallets_by_call_index,
                pallets_by_error_index,
                pallets_by_event_index,
                extrinsic: new_extrinsic,
                outer_enums,
                dispatch_error_ty,
                apis,
                // Nothing custom existed in V13
                custom: v15::CustomMetadata { map: Default::default() },
            })
    }}
}

from_historic!(pub fn from_v13(frame_metadata::v13::RuntimeMetadataV13, builtin_index: yes));
from_historic!(pub fn from_v12(frame_metadata::v12::RuntimeMetadataV12, builtin_index: yes));
from_historic!(pub fn from_v11(frame_metadata::v11::RuntimeMetadataV11));
from_historic!(pub fn from_v10(frame_metadata::v10::RuntimeMetadataV10));
from_historic!(pub fn from_v9(frame_metadata::v9::RuntimeMetadataV9));
from_historic!(pub fn from_v8(frame_metadata::v8::RuntimeMetadataV8));

fn as_decoded<A, B>(item: &frame_metadata::decode_different::DecodeDifferent<A, B>) -> &B {
    match item {
        frame_metadata::decode_different::DecodeDifferent::Encode(_a) => {
            panic!("Expecting decoded data")
        }
        frame_metadata::decode_different::DecodeDifferent::Decoded(b) => b,
    }
}

// Obtain Runtime API information from some type registry.
pub fn type_registry_to_runtime_apis(
    types: &TypeRegistrySet<'_>,
    portable_registry_builder: &mut PortableRegistryBuilder,
) -> Result<OrderedMap<String, crate::RuntimeApiMetadataInner>, Error> {
    let mut apis = OrderedMap::new();
    let mut trait_name = "";
    let mut trait_methods = OrderedMap::new();

    for api in types.runtime_apis() {
        match api {
            RuntimeApiName::Trait(name) => {
                if !trait_methods.is_empty() {
                    apis.push_insert(
                        trait_name.into(),
                        crate::RuntimeApiMetadataInner {
                            name: trait_name.into(),
                            methods: trait_methods,
                            docs: Vec::new(),
                        },
                    );
                }
                trait_methods = OrderedMap::new();
                trait_name = name;
            }
            RuntimeApiName::Method(name) => {
                let info = types
                    .runtime_api_info(trait_name, name)
                    .map_err(|e| Error::RuntimeApiInfoError(e.into_owned()))?;

                let info = info.map_ids(|id| {
                    portable_registry_builder.add_type(id).map_err(|e| {
                        let c = format!("converting type for runtime API {trait_name}.{name}");
                        Error::add_type(c, e)
                    })
                })?;

                trait_methods.push_insert(
                    name.to_owned(),
                    crate::RuntimeApiMethodMetadataInner {
                        name: name.into(),
                        info,
                        docs: Vec::new(),
                    },
                );
            }
        }
    }

    Ok(apis)
}

/// An error encountered converting some legacy metadata to our internal format.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Cannot add a type.
    #[error("Cannot add type ({context}): {error}")]
    AddTypeError {
        context: String,
        error: portable_registry_builder::PortableRegistryAddTypeError,
    },
    #[error("Cannot enhance the types with information from metadata: {0}")]
    CannotEnhanceTypesFromMetadata(scale_info_legacy::lookup_name::ParseError),
    #[error("Cannot find 'hardcoded::ExtrinsicAddress' type in legacy types")]
    CannotFindAddressType,
    #[error("Cannot find 'hardcoded::ExtrinsicSignature' type in legacy types")]
    CannotFindSignatureType,
    #[error(
        "Cannot find 'builtin::Call' type in legacy types (this should have been automatically added)"
    )]
    CannotFindCallType,
    #[error("Cannot obtain the storage information we need to convert storage entries")]
    StorageInfoError(frame_decode::storage::StorageInfoError<'static>),
    #[error("Cannot obtain the extrinsic information we need to convert transaction extensions")]
    ExtrinsicInfoError(frame_decode::extrinsics::ExtrinsicInfoError<'static>),
    #[error("Cannot obtain the Runtime API information we need")]
    RuntimeApiInfoError(frame_decode::runtime_apis::RuntimeApiInfoError<'static>),
    #[error("Cannot obtain the Constant information we need")]
    ConstantInfoError(frame_decode::constants::ConstantInfoError<'static>),
}

impl Error {
    /// A shorthand for the [`Error::AddTypeError`] variant.
    fn add_type(
        context: impl Into<String>,
        error: impl Into<portable_registry_builder::PortableRegistryAddTypeError>,
    ) -> Self {
        Error::AddTypeError {
            context: context.into(),
            error: error.into(),
        }
    }
}
