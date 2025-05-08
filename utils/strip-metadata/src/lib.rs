// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This utility crate provides a [`StripMetadata`] trait which exposes a [`StripMetadata::strip_metadata`] method
//! able to remove pallets and runtime APIs from the metadata in question.

use either::Either;
use frame_metadata::{v14, v15, v16};
use scale_info::PortableRegistry;
use std::collections::BTreeSet;

/// This trait is implemented for metadata versions to enable us to strip pallets and runtime APIs from them.
///
/// To implement the [`StripMetadata::strip_metadata`] method for a new metadata version, you'll probably:
/// - Remove any pallets and runtime APIs from the metadata based on the filter functions.
/// - Call `self.iter_type_ids_mut().collect()` to gather all of the type IDs to keep.
/// - This will require implementing [`IterateTypeIds`], which is the thing that iterates over all of the
///   type IDs still present in the metadata such that we know what we need to keep.
/// - Call `self.types.retain(..)` to filter any types not matching the type IDs above out of the registry.
/// - Iterate over the type IDs again, mapping those found in the metadata to the new IDs that calling
///   `self.types.retain(..)` handed back.
pub trait StripMetadata {
    /// Strip out any pallets and runtime APIs for which the provided filter functions return false.
    fn strip_metadata<PalletFilter, RuntimeApiFilter>(
        &mut self,
        keep_pallet: PalletFilter,
        keep_runtime_api: RuntimeApiFilter,
    ) where
        PalletFilter: Fn(&str) -> bool,
        RuntimeApiFilter: Fn(&str) -> bool;
}

impl StripMetadata for v14::RuntimeMetadataV14 {
    fn strip_metadata<PalletFilter, RuntimeApiFilter>(
        &mut self,
        keep_pallet: PalletFilter,
        _keep_runtime_api: RuntimeApiFilter,
    ) where
        PalletFilter: Fn(&str) -> bool,
        RuntimeApiFilter: Fn(&str) -> bool,
    {
        // Throw away pallets we don't care about:
        self.pallets.retain(|pallet| keep_pallet(&pallet.name));

        // Now, only retain types we care about in the registry:
        retain_types(self);
    }
}

impl StripMetadata for v15::RuntimeMetadataV15 {
    fn strip_metadata<PalletFilter, RuntimeApiFilter>(
        &mut self,
        keep_pallet: PalletFilter,
        keep_runtime_api: RuntimeApiFilter,
    ) where
        PalletFilter: Fn(&str) -> bool,
        RuntimeApiFilter: Fn(&str) -> bool,
    {
        // Throw away pallets and runtime APIs we don't care about:
        self.pallets.retain(|pallet| keep_pallet(&pallet.name));
        self.apis.retain(|api| keep_runtime_api(&api.name));

        // Now, only retain types we care about in the registry:
        retain_types(self);
    }
}

impl StripMetadata for v16::RuntimeMetadataV16 {
    fn strip_metadata<PalletFilter, RuntimeApiFilter>(
        &mut self,
        keep_pallet: PalletFilter,
        keep_runtime_api: RuntimeApiFilter,
    ) where
        PalletFilter: Fn(&str) -> bool,
        RuntimeApiFilter: Fn(&str) -> bool,
    {
        // Throw away pallets and runtime APIs we don't care about.
        // Keep the System pallet, because it has some associated types that we care about in Subxt.
        self.pallets
            .retain(|pallet| pallet.name == "System" || keep_pallet(&pallet.name));
        self.apis.retain(|api| keep_runtime_api(&api.name));

        // If the user asked to strip the System pallet, we'll strip most things from it but keep the
        // associated types, because Subxt makes use of them.
        if !keep_pallet("System") {
            if let Some(system_pallet) = self.pallets.iter_mut().find(|p| p.name == "System") {
                let index = system_pallet.index;
                let associated_types = core::mem::take(&mut system_pallet.associated_types);

                *system_pallet = v16::PalletMetadata {
                    name: "System".to_string(),
                    index,
                    associated_types,
                    // Everything else is empty:
                    storage: None,
                    calls: None,
                    event: None,
                    constants: vec![],
                    error: None,
                    view_functions: vec![],
                    docs: vec![],
                    deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
                };
            }
        }

        // Now, only retain types we care about in the registry:
        retain_types(self);
    }
}

fn retain_types<M: GetTypes + IterateTypeIds>(m: &mut M) {
    // We want to preserve this type even if it's not used anywhere:
    let dispatch_err_type_id = find_dispatch_error_type(m.get_types_mut());

    // Iterate over the type IDs and retain any that we still need:
    let keep_these_ids: BTreeSet<u32> = m
        .iter_type_ids_mut()
        .map(|id| *id)
        .chain(Some(dispatch_err_type_id))
        .collect();

    let new_ids = m.get_types_mut().retain(|id| keep_these_ids.contains(&id));

    // Map IDs found in the metadata to new ones as needed after the retaining:
    for id in m.iter_type_ids_mut() {
        if let Some(new_id) = new_ids.get(id) {
            *id = *new_id;
        };
    }
}

/// This trait is implemented for metadatas, and its purpose is to hand back iterators over
/// all of the type IDs (doesn't need to recurse into them) that are used in the metadata,
/// so that we know which ones we need to keep around in the type registry (and thus which
/// ones we can remove).
trait IterateTypeIds {
    /// This should iterate over all type IDs found in the metadata.
    fn iter_type_ids_mut(&mut self) -> impl Iterator<Item = &mut u32>;
}

impl IterateTypeIds for v14::RuntimeMetadataV14 {
    fn iter_type_ids_mut(&mut self) -> impl Iterator<Item = &mut u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter_mut().flat_map(|pallet| {
            let pallet_call_types = pallet
                .calls
                .as_mut()
                .into_iter()
                .map(|calls| &mut calls.ty.id);

            let pallet_storage_types = pallet
                .storage
                .as_mut()
                .into_iter()
                .flat_map(|s| &mut s.entries)
                .flat_map(|storage_entry| match &mut storage_entry.ty {
                    v14::StorageEntryType::Plain(ty) => Either::Left(core::iter::once(&mut ty.id)),
                    v14::StorageEntryType::Map { key, value, .. } => {
                        Either::Right([&mut key.id, &mut value.id].into_iter())
                    }
                });

            let pallet_constant_types = pallet
                .constants
                .iter_mut()
                .map(|constant| &mut constant.ty.id);

            let pallet_event_type = pallet
                .event
                .as_mut()
                .into_iter()
                .map(|events| &mut events.ty.id);

            let pallet_error_type = pallet
                .error
                .as_mut()
                .into_iter()
                .map(|error| &mut error.ty.id);

            pallet_call_types
                .chain(pallet_storage_types)
                .chain(pallet_constant_types)
                .chain(pallet_event_type)
                .chain(pallet_error_type)
        });

        // Transaction Extension types:
        let transaction_extension_types = self
            .extrinsic
            .signed_extensions
            .iter_mut()
            .flat_map(|ext| [&mut ext.ty.id, &mut ext.additional_signed.id].into_iter());

        // The extrinsic type:
        let extrinsic_type_id = &mut self.extrinsic.ty.id;

        // Return all IDs gathered:
        pallet_types
            .chain(Some(extrinsic_type_id))
            .chain(transaction_extension_types)
    }
}

impl IterateTypeIds for v15::RuntimeMetadataV15 {
    fn iter_type_ids_mut(&mut self) -> impl Iterator<Item = &mut u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter_mut().flat_map(|pallet| {
            let pallet_call_types = pallet
                .calls
                .as_mut()
                .into_iter()
                .map(|calls| &mut calls.ty.id);

            let pallet_storage_types = pallet
                .storage
                .as_mut()
                .into_iter()
                .flat_map(|s| &mut s.entries)
                .flat_map(|storage_entry| match &mut storage_entry.ty {
                    v14::StorageEntryType::Plain(ty) => Either::Left(core::iter::once(&mut ty.id)),
                    v14::StorageEntryType::Map { key, value, .. } => {
                        Either::Right([&mut key.id, &mut value.id].into_iter())
                    }
                });

            let pallet_constant_types = pallet
                .constants
                .iter_mut()
                .map(|constant| &mut constant.ty.id);

            let pallet_event_type = pallet
                .event
                .as_mut()
                .into_iter()
                .map(|events| &mut events.ty.id);

            let pallet_error_type = pallet
                .error
                .as_mut()
                .into_iter()
                .map(|error| &mut error.ty.id);

            pallet_call_types
                .chain(pallet_storage_types)
                .chain(pallet_constant_types)
                .chain(pallet_event_type)
                .chain(pallet_error_type)
        });

        // Runtime APIs:
        let runtime_api_types = self
            .apis
            .iter_mut()
            .flat_map(|api| &mut api.methods)
            .flat_map(|method| {
                let method_inputs = method.inputs.iter_mut().map(|input| &mut input.ty.id);
                let method_output = &mut method.output.id;
                method_inputs.chain(core::iter::once(method_output))
            });

        // The extrinsic type IDs:
        let extrinsic_type_ids = [
            &mut self.extrinsic.call_ty.id,
            &mut self.extrinsic.address_ty.id,
            &mut self.extrinsic.extra_ty.id,
            &mut self.extrinsic.signature_ty.id,
        ];

        // Outer enum type IDs:
        let outer_enum_type_ids = [
            &mut self.outer_enums.call_enum_ty.id,
            &mut self.outer_enums.event_enum_ty.id,
            &mut self.outer_enums.error_enum_ty.id,
        ];

        // Transaction Extension types:
        let transaction_extension_types = self
            .extrinsic
            .signed_extensions
            .iter_mut()
            .flat_map(|ext| [&mut ext.ty.id, &mut ext.additional_signed.id].into_iter());

        // Custom types:
        let custom_type_ids = self.custom.map.values_mut().map(|value| &mut value.ty.id);

        // Return all IDs gathered:
        pallet_types
            .chain(runtime_api_types)
            .chain(extrinsic_type_ids)
            .chain(outer_enum_type_ids)
            .chain(transaction_extension_types)
            .chain(custom_type_ids)
    }
}

impl IterateTypeIds for v16::RuntimeMetadataV16 {
    fn iter_type_ids_mut(&mut self) -> impl Iterator<Item = &mut u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter_mut().flat_map(|pallet| {
            let pallet_call_types = pallet
                .calls
                .as_mut()
                .into_iter()
                .map(|calls| &mut calls.ty.id);

            let pallet_storage_types = pallet
                .storage
                .as_mut()
                .into_iter()
                .flat_map(|s| &mut s.entries)
                .flat_map(|storage_entry| match &mut storage_entry.ty {
                    v16::StorageEntryType::Plain(ty) => Either::Left(core::iter::once(&mut ty.id)),
                    v16::StorageEntryType::Map { key, value, .. } => {
                        Either::Right([&mut key.id, &mut value.id].into_iter())
                    }
                });

            let pallet_constant_types = pallet
                .constants
                .iter_mut()
                .map(|constant| &mut constant.ty.id);

            let pallet_event_type = pallet
                .event
                .as_mut()
                .into_iter()
                .map(|events| &mut events.ty.id);

            let pallet_error_type = pallet
                .error
                .as_mut()
                .into_iter()
                .map(|error| &mut error.ty.id);

            let pallet_view_fns = pallet.view_functions.iter_mut().flat_map(|vf| {
                let inputs = vf.inputs.iter_mut().map(|input| &mut input.ty.id);
                let output = &mut vf.output.id;

                inputs.chain(core::iter::once(output))
            });

            let pallet_associated_types = pallet
                .associated_types
                .iter_mut()
                .map(|associated_type| &mut associated_type.ty.id);

            pallet_call_types
                .chain(pallet_storage_types)
                .chain(pallet_constant_types)
                .chain(pallet_event_type)
                .chain(pallet_error_type)
                .chain(pallet_view_fns)
                .chain(pallet_associated_types)
        });

        // Runtime APIs:
        let runtime_api_types = self
            .apis
            .iter_mut()
            .flat_map(|api| &mut api.methods)
            .flat_map(|method| {
                let method_inputs = method.inputs.iter_mut().map(|input| &mut input.ty.id);
                let method_output = &mut method.output.id;
                method_inputs.chain(core::iter::once(method_output))
            });

        // The extrinsic type IDs:
        let extrinsic_type_ids = [
            &mut self.extrinsic.address_ty.id,
            &mut self.extrinsic.signature_ty.id,
        ];

        // Outer enum type IDs:
        let outer_enum_type_ids = [
            &mut self.outer_enums.call_enum_ty.id,
            &mut self.outer_enums.event_enum_ty.id,
            &mut self.outer_enums.error_enum_ty.id,
        ];

        // Transaction Extension types:
        let transaction_extension_types = self
            .extrinsic
            .transaction_extensions
            .iter_mut()
            .flat_map(|ext| [&mut ext.ty.id, &mut ext.implicit.id].into_iter());

        // Custom types:
        let custom_type_ids = self.custom.map.values_mut().map(|value| &mut value.ty.id);

        // Return all IDs gathered:
        pallet_types
            .chain(runtime_api_types)
            .chain(extrinsic_type_ids)
            .chain(outer_enum_type_ids)
            .chain(transaction_extension_types)
            .chain(custom_type_ids)
    }
}

/// This trait defines how to get a type registry from the metadata
trait GetTypes {
    fn get_types_mut(&mut self) -> &mut PortableRegistry;
}

impl GetTypes for v14::RuntimeMetadataV14 {
    fn get_types_mut(&mut self) -> &mut PortableRegistry {
        &mut self.types
    }
}

impl GetTypes for v15::RuntimeMetadataV15 {
    fn get_types_mut(&mut self) -> &mut PortableRegistry {
        &mut self.types
    }
}

impl GetTypes for v16::RuntimeMetadataV16 {
    fn get_types_mut(&mut self) -> &mut PortableRegistry {
        &mut self.types
    }
}

/// Subxt needs this type so we always ensure to preserve it
/// even if it's not explicitly mentioned anywhere:
fn find_dispatch_error_type(types: &mut PortableRegistry) -> u32 {
    types
        .types
        .iter()
        .enumerate()
        .find(|(_idx, ty)| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
        .expect("Metadata must contain sp_runtime::DispatchError")
        .0 as u32
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::*;
    use codec::Compact;
    use scale_info::meta_type;

    /// Create dummy types that we can check the presense of with is_in_types.
    macro_rules! make_types {
        ($($name:ident)+) => {
            $(
                struct $name {}
                impl scale_info::TypeInfo for $name {
                    type Identity = $name;

                    fn type_info() -> scale_info::Type {
                        scale_info::Type {
                            path: scale_info::Path {
                                segments: vec!["dummy_type", stringify!($name)],
                            },
                            type_params: vec![],
                            type_def: scale_info::TypeDef::Composite(scale_info::TypeDefComposite { fields: vec![] }),
                            docs: vec![],
                        }
                    }
                }

                impl $name {
                    #[allow(dead_code)]
                    pub fn is_in_types(types: &scale_info::PortableRegistry) -> bool {
                        types.types.iter().any(|ty| ty.ty.path.segments == vec!["dummy_type", stringify!($name)])
                    }
                }
            )+
        }
    }

    /// Asserts that a set of the dummy types exist in a registry.
    macro_rules! assert_is_in_types {
        ($($name:ident)+ => $types:expr) => {{
            $(
                if !$name::is_in_types(&$types) {
                    panic!("{} was not found in {}", stringify!($name), stringify!($types));
                }
            )+
        }}
    }

    /// Asserts that a set of the dummy types do not exist in a registry.
    macro_rules! assert_not_in_types {
        ($($name:ident)+ => $types:expr) => {{
            $(
                if $name::is_in_types(&$types) {
                    panic!("{} was found in {}", stringify!($name), stringify!($types));
                }
            )+
        }}
    }

    #[allow(dead_code)]
    enum DummyDispatchError {
        A,
        B,
        C,
    }

    impl scale_info::TypeInfo for DummyDispatchError {
        type Identity = DummyDispatchError;

        fn type_info() -> scale_info::Type {
            scale_info::Type {
                path: scale_info::Path {
                    segments: vec!["sp_runtime", "DispatchError"],
                },
                type_params: vec![],
                type_def: scale_info::TypeDef::Variant(scale_info::TypeDefVariant {
                    variants: vec![],
                }),
                docs: vec![],
            }
        }
    }

    #[test]
    fn v14_stripping_works() {
        make_types!(A B C D E);

        let pallets = vec![
            v14::PalletMetadata {
                name: "First",
                index: 0,
                calls: None,
                storage: Some(v14::PalletStorageMetadata {
                    prefix: "___",
                    entries: vec![v14::StorageEntryMetadata {
                        name: "Hello",
                        modifier: v14::StorageEntryModifier::Optional,
                        ty: frame_metadata::v14::StorageEntryType::Plain(meta_type::<A>()),
                        default: vec![],
                        docs: vec![],
                    }],
                }),
                event: Some(v14::PalletEventMetadata {
                    ty: meta_type::<B>(),
                }),
                constants: vec![],
                error: None,
            },
            v14::PalletMetadata {
                name: "Second",
                index: 1,
                calls: Some(v15::PalletCallMetadata {
                    ty: meta_type::<C>(),
                }),
                storage: None,
                event: None,
                constants: vec![v14::PalletConstantMetadata {
                    name: "SomeConstant",
                    ty: meta_type::<D>(),
                    value: vec![],
                    docs: vec![],
                }],
                error: None,
            },
        ];

        let extrinsic = v14::ExtrinsicMetadata {
            version: 0,
            signed_extensions: vec![],
            ty: meta_type::<E>(),
        };

        let metadata =
            v14::RuntimeMetadataV14::new(pallets, extrinsic, meta_type::<DummyDispatchError>());

        assert_eq!(metadata.types.types.len(), 6);
        assert_is_in_types!(A B C D E => metadata.types);

        let only_first_pallet = {
            let mut md = metadata.clone();
            md.strip_metadata(|name| name == "First", |_| true);
            md
        };

        assert_eq!(only_first_pallet.types.types.len(), 4);
        assert_is_in_types!(A B E => only_first_pallet.types);
        assert_not_in_types!(C D => only_first_pallet.types);
        assert_eq!(only_first_pallet.pallets.len(), 1);
        assert_eq!(&only_first_pallet.pallets[0].name, "First");

        let only_second_pallet = {
            let mut md = metadata.clone();
            md.strip_metadata(|name| name == "Second", |_| true);
            md
        };

        assert_eq!(only_second_pallet.types.types.len(), 4);
        assert_is_in_types!(C D E => only_second_pallet.types);
        assert_not_in_types!(A B => only_second_pallet.types);
        assert_eq!(only_second_pallet.pallets.len(), 1);
        assert_eq!(&only_second_pallet.pallets[0].name, "Second");

        let no_pallets = {
            let mut md = metadata.clone();
            md.strip_metadata(|_| false, |_| true);
            md
        };

        assert_eq!(no_pallets.types.types.len(), 2);
        assert_is_in_types!(E => no_pallets.types);
        assert_not_in_types!(A B C D => no_pallets.types);
        assert_eq!(no_pallets.pallets.len(), 0);
    }

    #[test]
    fn v15_stripping_works() {
        make_types!(A B C D E F G H I J K L M N O P);

        let pallets = vec![
            v15::PalletMetadata {
                name: "First",
                index: 0,
                calls: None,
                storage: Some(v15::PalletStorageMetadata {
                    prefix: "___",
                    entries: vec![v15::StorageEntryMetadata {
                        name: "Hello",
                        modifier: v15::StorageEntryModifier::Optional,
                        ty: frame_metadata::v15::StorageEntryType::Plain(meta_type::<A>()),
                        default: vec![],
                        docs: vec![],
                    }],
                }),
                event: Some(v15::PalletEventMetadata {
                    ty: meta_type::<B>(),
                }),
                constants: vec![],
                error: None,
                docs: vec![],
            },
            v15::PalletMetadata {
                name: "Second",
                index: 1,
                calls: Some(v15::PalletCallMetadata {
                    ty: meta_type::<C>(),
                }),
                storage: None,
                event: None,
                constants: vec![v15::PalletConstantMetadata {
                    name: "SomeConstant",
                    ty: meta_type::<D>(),
                    value: vec![],
                    docs: vec![],
                }],
                error: None,
                docs: vec![],
            },
        ];

        let extrinsic = v15::ExtrinsicMetadata {
            version: 0,
            signed_extensions: vec![],
            call_ty: meta_type::<E>(),
            address_ty: meta_type::<F>(),
            signature_ty: meta_type::<G>(),
            extra_ty: meta_type::<H>(),
        };

        let runtime_apis = vec![
            v15::RuntimeApiMetadata {
                name: "SomeApi",
                docs: vec![],
                methods: vec![v15::RuntimeApiMethodMetadata {
                    name: "some_method",
                    inputs: vec![v15::RuntimeApiMethodParamMetadata {
                        name: "input1",
                        ty: meta_type::<I>(),
                    }],
                    output: meta_type::<J>(),
                    docs: vec![],
                }],
            },
            v15::RuntimeApiMetadata {
                name: "AnotherApi",
                docs: vec![],
                methods: vec![v15::RuntimeApiMethodMetadata {
                    name: "another_method",
                    inputs: vec![v15::RuntimeApiMethodParamMetadata {
                        name: "input1",
                        ty: meta_type::<K>(),
                    }],
                    output: meta_type::<L>(),
                    docs: vec![],
                }],
            },
        ];

        let outer_enums = v15::OuterEnums {
            call_enum_ty: meta_type::<M>(),
            error_enum_ty: meta_type::<N>(),
            event_enum_ty: meta_type::<O>(),
        };

        let custom_values = v15::CustomMetadata {
            map: BTreeMap::from_iter(vec![(
                "Item",
                v15::CustomValueMetadata {
                    ty: meta_type::<P>(),
                    value: vec![],
                },
            )]),
        };

        let metadata = v15::RuntimeMetadataV15::new(
            pallets,
            extrinsic,
            meta_type::<DummyDispatchError>(),
            runtime_apis,
            outer_enums,
            custom_values,
        );

        assert_is_in_types!(A B C D E F G H I J K L M N O P => metadata.types);

        let only_first_pallet = {
            let mut md = metadata.clone();
            md.strip_metadata(|name| name == "First", |_| true);
            md
        };

        assert_is_in_types!(A B E F G H I J K L M N O P => only_first_pallet.types);
        assert_not_in_types!(C D => only_first_pallet.types);
        assert_eq!(only_first_pallet.pallets.len(), 1);
        assert_eq!(&only_first_pallet.pallets[0].name, "First");

        let only_second_pallet = {
            let mut md = metadata.clone();
            md.strip_metadata(|name| name == "Second", |_| true);
            md
        };

        assert_is_in_types!(C D E F G H I J K L M N O P => only_second_pallet.types);
        assert_not_in_types!(A B => only_second_pallet.types);
        assert_eq!(only_second_pallet.pallets.len(), 1);
        assert_eq!(&only_second_pallet.pallets[0].name, "Second");

        let no_pallets = {
            let mut md = metadata.clone();
            md.strip_metadata(|_| false, |_| true);
            md
        };

        assert_is_in_types!(E F G H I J K L M N O P => no_pallets.types);
        assert_not_in_types!(A B C D => no_pallets.types);
        assert_eq!(no_pallets.pallets.len(), 0);

        let only_second_runtime_api = {
            let mut md = metadata.clone();
            md.strip_metadata(|_| true, |api| api == "AnotherApi");
            md
        };

        assert_is_in_types!(A B C D E F G H K L M N O P => only_second_runtime_api.types);
        assert_not_in_types!(I J => only_second_runtime_api.types);
        assert_eq!(only_second_runtime_api.pallets.len(), 2);
        assert_eq!(only_second_runtime_api.apis.len(), 1);
    }

    #[test]
    fn v16_stripping_works() {
        make_types!(A B C D E F G H I J K L M N O P);

        let pallets = vec![
            v16::PalletMetadata {
                name: "First",
                index: 0,
                calls: None,
                storage: Some(v16::PalletStorageMetadata {
                    prefix: "___",
                    entries: vec![v16::StorageEntryMetadata {
                        name: "Hello",
                        modifier: v16::StorageEntryModifier::Optional,
                        ty: frame_metadata::v16::StorageEntryType::Plain(meta_type::<A>()),
                        default: vec![],
                        docs: vec![],
                        deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
                    }],
                }),
                event: Some(v16::PalletEventMetadata {
                    ty: meta_type::<B>(),
                    deprecation_info: v16::EnumDeprecationInfo::nothing_deprecated(),
                }),
                constants: vec![],
                associated_types: vec![],
                view_functions: vec![],
                error: None,
                docs: vec![],
                deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
            },
            v16::PalletMetadata {
                name: "Second",
                index: 1,
                calls: Some(v16::PalletCallMetadata {
                    ty: meta_type::<C>(),
                    deprecation_info: v16::EnumDeprecationInfo::nothing_deprecated(),
                }),
                storage: None,
                event: None,
                constants: vec![v16::PalletConstantMetadata {
                    name: "SomeConstant",
                    ty: meta_type::<D>(),
                    value: vec![],
                    docs: vec![],
                    deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
                }],
                associated_types: vec![v16::PalletAssociatedTypeMetadata {
                    name: "Hasher",
                    ty: meta_type::<E>(),
                    docs: vec![],
                }],
                view_functions: vec![v16::PalletViewFunctionMetadata {
                    name: "some_view_function",
                    id: [0; 32],
                    inputs: vec![v16::FunctionParamMetadata {
                        name: "input1",
                        ty: meta_type::<F>(),
                    }],
                    output: meta_type::<G>(),
                    docs: vec![],
                    deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
                }],
                error: None,
                docs: vec![],
                deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
            },
        ];

        let extrinsic = v16::ExtrinsicMetadata {
            call_ty: meta_type::<N>(), // same as outer_enums.call_enum_ty
            versions: vec![0],
            transaction_extensions_by_version: BTreeMap::new(),
            transaction_extensions: vec![],
            address_ty: meta_type::<H>(),
            signature_ty: meta_type::<I>(),
        };

        let runtime_apis = vec![
            v16::RuntimeApiMetadata {
                name: "SomeApi",
                version: Compact(2),
                docs: vec![],
                deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
                methods: vec![v16::RuntimeApiMethodMetadata {
                    name: "some_method",
                    inputs: vec![v16::FunctionParamMetadata {
                        name: "input1",
                        ty: meta_type::<J>(),
                    }],
                    output: meta_type::<K>(),
                    docs: vec![],
                    deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
                }],
            },
            v16::RuntimeApiMetadata {
                name: "AnotherApi",
                version: Compact(1),
                docs: vec![],
                deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
                methods: vec![v16::RuntimeApiMethodMetadata {
                    name: "another_method",
                    inputs: vec![v16::FunctionParamMetadata {
                        name: "input1",
                        ty: meta_type::<L>(),
                    }],
                    output: meta_type::<M>(),
                    docs: vec![],
                    deprecation_info: v16::ItemDeprecationInfo::NotDeprecated,
                }],
            },
        ];

        let outer_enums = v16::OuterEnums {
            call_enum_ty: meta_type::<N>(),
            error_enum_ty: meta_type::<O>(),
            event_enum_ty: meta_type::<P>(),
        };

        let custom_values = v16::CustomMetadata {
            map: BTreeMap::from_iter(vec![(
                "Item",
                v16::CustomValueMetadata {
                    ty: meta_type::<DummyDispatchError>(),
                    value: vec![],
                },
            )]),
        };

        let metadata = v16::RuntimeMetadataV16::new(
            pallets,
            extrinsic,
            runtime_apis,
            outer_enums,
            custom_values,
        );

        assert_is_in_types!(A B C D E F G H I J K L M N O P => metadata.types);

        let only_first_pallet = {
            let mut md = metadata.clone();
            md.strip_metadata(|name| name == "First", |_| true);
            md
        };

        assert_is_in_types!(A B H I J K L M N O P => only_first_pallet.types);
        assert_not_in_types!(C D E F G => only_first_pallet.types);
        assert_eq!(only_first_pallet.pallets.len(), 1);
        assert_eq!(&only_first_pallet.pallets[0].name, "First");

        let only_second_pallet = {
            let mut md = metadata.clone();
            md.strip_metadata(|name| name == "Second", |_| true);
            md
        };

        assert_is_in_types!(C D E F G H I J K L M N O P => only_second_pallet.types);
        assert_not_in_types!(A B => only_second_pallet.types);
        assert_eq!(only_second_pallet.pallets.len(), 1);
        assert_eq!(&only_second_pallet.pallets[0].name, "Second");

        let no_pallets = {
            let mut md = metadata.clone();
            md.strip_metadata(|_| false, |_| true);
            md
        };

        assert_is_in_types!(H I J K L M N O P => no_pallets.types);
        assert_not_in_types!(A B C D E F G => no_pallets.types);
        assert_eq!(no_pallets.pallets.len(), 0);

        let only_second_runtime_api = {
            let mut md = metadata.clone();
            md.strip_metadata(|_| true, |api| api == "AnotherApi");
            md
        };

        assert_is_in_types!(A B C D E F G H I L M N O P => only_second_runtime_api.types);
        assert_not_in_types!(J K => only_second_runtime_api.types);
        assert_eq!(only_second_runtime_api.pallets.len(), 2);
        assert_eq!(only_second_runtime_api.apis.len(), 1);
    }
}
