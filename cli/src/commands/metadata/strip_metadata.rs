// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::collections::BTreeSet;

use frame_metadata::{ v14, v15, v16 };

/// This trait is implemented for metadata versions to enable us to strip pallets and runtime APIs from them.
/// 
/// To implement the [`StripMetadata::strip_metadata`] method for a new metadata version, you'll probably:
/// - Remove any pallets and runtime APIs from the metadata based on the filter functions.
/// - Call `self.iterate_all_type_ids().collect()` to gather all of the type IDs to keep.
/// - This will require implementing [`IterateTypeIds`], which is the thing that iterates over all of the
///   type IDs still present in the metadata such that we know what we need to keep.
/// - Call `self.types.retain(..)` to filter any types not matching the IDs above out of the registry.
pub trait StripMetadata {
    /// Strip out any pallets and runtime APIs for which the provided filter functions return false.
    fn strip_metadata<PalletFilter, RuntimeApiFilter>(
        &mut self,
        keep_pallet: PalletFilter,
        keep_runtime_api: RuntimeApiFilter,
    )
    where
        PalletFilter: Fn(&str) -> bool,
        RuntimeApiFilter: Fn(&str) -> bool;
}

impl StripMetadata for v14::RuntimeMetadataV14 {
    fn strip_metadata<PalletFilter, RuntimeApiFilter>(
        &mut self,
        keep_pallet: PalletFilter,
        _keep_runtime_api: RuntimeApiFilter,
    )
    where
        PalletFilter: Fn(&str) -> bool,
        RuntimeApiFilter: Fn(&str) -> bool 
    {
        // Throw away pallets we don't care about:
        self.pallets.retain(|pallet| {
            keep_pallet(&pallet.name)
        });

        // Iterate over the type IDs and retain any that we still need:
        let keep_these_ids: BTreeSet<u32> = self.iterate_all_type_ids().collect();
        self.types.retain(|id | keep_these_ids.contains(&id));
    }
}

impl StripMetadata for v15::RuntimeMetadataV15 {
    fn strip_metadata<PalletFilter, RuntimeApiFilter>(
        &mut self,
        keep_pallet: PalletFilter,
        keep_runtime_api: RuntimeApiFilter,
    )
    where
        PalletFilter: Fn(&str) -> bool,
        RuntimeApiFilter: Fn(&str) -> bool 
    {
        // Throw away pallets and runtime APIs we don't care about:
        self.pallets.retain(|pallet| {
            keep_pallet(&pallet.name)
        });
        self.apis.retain(|api| {
            keep_runtime_api(&api.name)
        });

        // Iterate over the type IDs and retain any that we still need:
        let keep_these_ids: BTreeSet<u32> = self.iterate_all_type_ids().collect();
        self.types.retain(|id | keep_these_ids.contains(&id));
    }
}

impl StripMetadata for v16::RuntimeMetadataV16 {
    fn strip_metadata<PalletFilter, RuntimeApiFilter>(
        &mut self,
        keep_pallet: PalletFilter,
        keep_runtime_api: RuntimeApiFilter,
    )
    where
        PalletFilter: Fn(&str) -> bool,
        RuntimeApiFilter: Fn(&str) -> bool 
    {
        // Throw away pallets and runtime APIs we don't care about:
        self.pallets.retain(|pallet| {
            keep_pallet(&pallet.name)
        });
        self.apis.retain(|api| {
            keep_runtime_api(&api.name)
        });

        // Iterate over the type IDs and retain any that we still need:
        let keep_these_ids: BTreeSet<u32> = self.iterate_all_type_ids().collect();
        self.types.retain(|id | keep_these_ids.contains(&id));
    }
}

/// This trait is implemented for metadatas, and its purpose is to hand back iterators over
/// all of the type IDs (doesn't need to recurse into them) that are used in the metadata,
/// so that we know which ones we need to keep around in the type registry (and thus which 
/// ones we can remove).
trait IterateTypeIds {
    /// This should iterate over all type IDs found in the metadata.
    fn iterate_all_type_ids(&self) -> impl Iterator<Item=u32>;
}

impl IterateTypeIds for v14::RuntimeMetadataV14 {
    fn iterate_all_type_ids(&self) -> impl Iterator<Item=u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter().flat_map(|pallet| {
            let pallet_call_types = pallet.calls.as_ref().into_iter().map(|calls| {
                calls.ty.id
            });

            let pallet_storage_types = pallet.storage.as_ref().into_iter().flat_map(|s| {
                &s.entries
            }).flat_map(|storage_entry| {
                match &storage_entry.ty {
                    v14::StorageEntryType::Plain(ty) => {
                        Either::A(std::iter::once(ty.id))
                    }
                    v14::StorageEntryType::Map { key, value, ..} => {
                        Either::B([key.id, value.id].into_iter())
                    }
                }
            });

            let pallet_constant_types = pallet.constants.iter().map(|constant| {
                constant.ty.id
            });
    
            let pallet_event_type = pallet.event.as_ref().into_iter().map(|events| {
                events.ty.id
            });
    
            let pallet_error_type = pallet.error.as_ref().into_iter().map(|error| {
                error.ty.id
            });

            pallet_call_types
                .chain(pallet_storage_types)
                .chain(pallet_constant_types)
                .chain(pallet_event_type)
                .chain(pallet_error_type)
        });

        // Transaction Extension types:
        let transaction_extension_types = self.extrinsic.signed_extensions.iter().flat_map(|ext| {
            [ext.ty.id, ext.additional_signed.id].into_iter()
        });

        // The extrinsic type:
        let extrinsic_type_id = self.extrinsic.ty.id;

        // Subxt needs this type so we always ensure to preserve it 
        // even if it's not explicitly mentioned anywhere:
        let dispatch_error_ty = std::iter::once_with(|| {
            self
                .types
                .types
                .iter()
                .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
                .expect("Metadata must contain sp_runtime::DispatchError")
                .id
        });

        // Return all IDs gathered:
        pallet_types
            .chain(Some(extrinsic_type_id))
            .chain(transaction_extension_types)
            .chain(dispatch_error_ty)
    }
}

impl IterateTypeIds for v15::RuntimeMetadataV15 {
    fn iterate_all_type_ids(&self) -> impl Iterator<Item=u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter().flat_map(|pallet| {
            let pallet_call_types = pallet.calls.as_ref().into_iter().map(|calls| {
                calls.ty.id
            });
            
            let pallet_storage_types = pallet.storage.as_ref().into_iter().flat_map(|s| {
                &s.entries
            }).flat_map(|storage_entry| {
                match &storage_entry.ty {
                    v15::StorageEntryType::Plain(ty) => {
                        Either::A(std::iter::once(ty.id))
                    }
                    v15::StorageEntryType::Map { key, value, ..} => {
                        Either::B([key.id, value.id].into_iter())
                    }
                }
            });
            
            let pallet_constant_types = pallet.constants.iter().map(|constant| {
                constant.ty.id
            });
    
            let pallet_event_type = pallet.event.as_ref().into_iter().map(|events| {
                events.ty.id
            });
    
            let pallet_error_type = pallet.error.as_ref().into_iter().map(|error| {
                error.ty.id
            });

            pallet_call_types
                .chain(pallet_storage_types)
                .chain(pallet_constant_types)
                .chain(pallet_event_type)
                .chain(pallet_error_type)
        });

        // Runtime APIs:
        let runtime_api_types = self.apis.iter().flat_map(|api| {
            &api.methods
        }).flat_map(|method| {
            let method_inputs = method.inputs.iter().map(|input| input.ty.id);
            let method_output = method.output.id;
            method_inputs.chain(std::iter::once(method_output))
        });

        // The extrinsic type IDs:
        let extrinsic_type_ids = [
            self.extrinsic.call_ty.id,
            self.extrinsic.address_ty.id,
            self.extrinsic.extra_ty.id,
            self.extrinsic.signature_ty.id
        ];

        // Transaction Extension types:
        let transaction_extension_types = self.extrinsic.signed_extensions.iter().flat_map(|ext| {
            [ext.ty.id, ext.additional_signed.id].into_iter()
        });

        // Custom types:
        let custom_type_ids = self.custom.map.values().map(|value| {
            value.ty.id
        });

        // Subxt needs this type so we always ensure to preserve it 
        // even if it's not explicitly mentioned anywhere:
        let dispatch_error_ty = std::iter::once_with(|| {
            self
                .types
                .types
                .iter()
                .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
                .expect("Metadata must contain sp_runtime::DispatchError")
                .id
        });

        // Return all IDs gathered:
        pallet_types
            .chain(runtime_api_types)
            .chain(extrinsic_type_ids)
            .chain(transaction_extension_types)
            .chain(custom_type_ids)
            .chain(dispatch_error_ty)
    }
}

impl IterateTypeIds for v16::RuntimeMetadataV16 {
    fn iterate_all_type_ids(&self) -> impl Iterator<Item=u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter().flat_map(|pallet| {
            let pallet_call_types = pallet.calls.as_ref().into_iter().map(|calls| {
                calls.ty.id
            });
            
            let pallet_storage_types = pallet.storage.as_ref().into_iter().flat_map(|s| {
                &s.entries
            }).flat_map(|storage_entry| {
                match &storage_entry.ty {
                    v16::StorageEntryType::Plain(ty) => {
                        Either::A(std::iter::once(ty.id))
                    }
                    v16::StorageEntryType::Map { key, value, ..} => {
                        Either::B([key.id, value.id].into_iter())
                    }
                }
            });
            
            let pallet_constant_types = pallet.constants.iter().map(|constant| {
                constant.ty.id
            });
    
            let pallet_event_type = pallet.event.as_ref().into_iter().map(|events| {
                events.ty.id
            });
    
            let pallet_error_type = pallet.error.as_ref().into_iter().map(|error| {
                error.ty.id
            });

            let pallet_view_fns = pallet.view_functions.iter().flat_map(|vf| {
                let inputs = vf.inputs.iter().map(|input| input.ty.id);
                let output = vf.output.id;

                inputs.chain(std::iter::once(output))
            });

            let pallet_associated_types = pallet.associated_types.iter().map(|associated_type| {
                associated_type.ty.id
            });

            pallet_call_types
                .chain(pallet_storage_types)
                .chain(pallet_constant_types)
                .chain(pallet_event_type)
                .chain(pallet_error_type)
                .chain(pallet_view_fns)
                .chain(pallet_associated_types)
        });

        // Runtime APIs:
        let runtime_api_types = self.apis.iter().flat_map(|api| {
            &api.methods
        }).flat_map(|method| {
            let method_inputs = method.inputs.iter().map(|input| input.ty.id);
            let method_output = method.output.id;
            method_inputs.chain(std::iter::once(method_output))
        });

        // The extrinsic type IDs:
        let extrinsic_type_ids = [
            self.extrinsic.address_ty.id,
            self.extrinsic.signature_ty.id
        ];
        
        // Transaction Extension types:
        let transaction_extension_types = self.extrinsic.transaction_extensions.iter().flat_map(|ext| {
            [ext.ty.id, ext.implicit.id].into_iter()
        });

        // Custom types:
        let custom_type_ids = self.custom.map.values().map(|value| {
            value.ty.id
        });

        // Subxt needs this type so we always ensure to preserve it 
        // even if it's not explicitly mentioned anywhere:
        let dispatch_error_ty = std::iter::once_with(|| {
            self
                .types
                .types
                .iter()
                .find(|ty| ty.ty.path.segments == ["sp_runtime", "DispatchError"])
                .expect("Metadata must contain sp_runtime::DispatchError")
                .id
        });

        // Return all IDs gathered:
        pallet_types
            .chain(runtime_api_types)
            .chain(extrinsic_type_ids)
            .chain(transaction_extension_types)
            .chain(custom_type_ids)
            .chain(dispatch_error_ty)
    }
}

/// Create Either enums which can be iterated over.
macro_rules! either{
    ($name:ident: $first_tok:ident $($tok:ident)*) => {
        enum $name<$first_tok, $($tok),+> {
            $first_tok($first_tok),
            $( $tok($tok), )*
        }

        impl <$first_tok, $($tok),+> Iterator for $name<$first_tok, $($tok),+> 
        where
            $first_tok: Iterator,
            $($tok: Iterator<Item = $first_tok::Item>,)*
        {
            type Item = $first_tok::Item;
            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    $name::$first_tok(item) => item.next(),
                    $($name::$tok(item) => item.next(),)*
                }
            }
        }
    }
}

either!(Either: A B);
