// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::collections::BTreeSet;

use frame_metadata::{ v14, v15, v16 };
use scale_info::PortableRegistry;
use subxt_metadata::utilities::OuterEnums;

/// This trait is implemented for metadata versions to enable us to strip pallets and runtime APIs from them.
/// 
/// To implement the [`RetainMetadata::retain_metadata`] method for a new metadata version, you'll probably:
/// - Remove any pallets and runtime APIs from the metadata based on the filter functions.
/// - Gather the outer enum IDs and then call [`retain_relevant_types_in_registry`] to strip the
///   type registry down given the pallets and runtime APIs left in the metadata.
/// - This will require implementing [`IterateTypeIds`], which is the thing that determines which
///   IDs in the metadata to pay attention to or ignore when it comes to stripping things out.
pub trait RetainMetadata {
    /// Strip out any pallets and runtime APIs for which the provided filter functions return false.
    fn retain_metadata<PalletFilter, RuntimeApiFilter>(
        &mut self,
        keep_pallet: PalletFilter,
        keep_runtime_api: RuntimeApiFilter,
    )
    where
        PalletFilter: Fn(&str) -> bool,
        RuntimeApiFilter: Fn(&str) -> bool;
}

impl RetainMetadata for v14::RuntimeMetadataV14 {
    fn retain_metadata<PalletFilter, RuntimeApiFilter>(
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

        // Throw away any type IDs we no longer need as a result:
        let outer_enum_ids = OuterEnums::find_in(&self.types);
        retain_relevant_types_in_registry(self, keep_pallet, outer_enum_ids);
    }
}

impl RetainMetadata for v15::RuntimeMetadataV15 {
    fn retain_metadata<PalletFilter, RuntimeApiFilter>(
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

        // Throw away any type IDs we no longer need as a result:
        let outer_enum_ids = OuterEnums {
            call_ty: Some(self.outer_enums.call_enum_ty.id),
            event_ty: Some(self.outer_enums.event_enum_ty.id),
            error_ty: Some(self.outer_enums.error_enum_ty.id),
        };
        retain_relevant_types_in_registry(self, keep_pallet, outer_enum_ids);
    }
}

impl RetainMetadata for v16::RuntimeMetadataV16 {
    fn retain_metadata<PalletFilter, RuntimeApiFilter>(
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

        // Throw away any type IDs we no longer need as a result:
        let outer_enum_ids = OuterEnums {
            call_ty: Some(self.outer_enums.call_enum_ty.id),
            event_ty: Some(self.outer_enums.event_enum_ty.id),
            error_ty: Some(self.outer_enums.error_enum_ty.id),
        };
        retain_relevant_types_in_registry(self, keep_pallet, outer_enum_ids);
    }
}

/// This function:
/// 
/// - Iterates deeply over all IDs that matter when considering whether we can strip variants from
///   RuntimeCall/Event/Error outer enum types, to see whether said outer enums are found.
/// - Strips unwanted pallets from any outer enums which we didn't find above.
/// - Strips types from the `PortableRegistry` that are no longer needed.
/// 
/// This function primarily relies on [`IterateTypeIds`], which is implemented for metadata versions
/// and hands back iterators over type IDs that we care about.
fn retain_relevant_types_in_registry<M, PalletFilter>(metadata: &mut M, keep_pallet: PalletFilter, mut outer_enum_ids: OuterEnums) 
where
    M: IterateTypeIds + GetPortableRegistry,
    PalletFilter: Fn(&str) -> bool
{
    // Deeply iterate over all type IDs we need to consider w.r.t whether we
    //can strip any of the outer enums.
    let all_ids_to_keep = recurse_type_ids(
        metadata.get_portable_registry(), 
        metadata.iterate_type_ids_to_consider_for_stripping()
    );

    // If we can't strip an outer enum type because it appears in the IDs,
    // set it to `None`.
    for id in all_ids_to_keep {
        if outer_enum_ids.call_ty.is_some_and(|i| i == id) {
            outer_enum_ids.call_ty = None;
        }
        if outer_enum_ids.event_ty.is_some_and(|i| i == id) {
            outer_enum_ids.event_ty = None;
        }
        if outer_enum_ids.error_ty.is_some_and(|i| i == id) {
            outer_enum_ids.error_ty = None;
        }
    }

    // Strip whichever variants we can get away with:
    for id in outer_enum_ids.iter() {
        strip_variants_in_enum_type(
            &mut metadata.get_portable_registry_mut(), 
            &keep_pallet, 
            id
        )
    }

    // Iterate over the type IDs again (no need to deeply recurse this time), and retain
    // any of these 9and their children) in the type registry:
    let keep_these_ids: BTreeSet<u32> = metadata.iterate_all_type_ids().collect();
    metadata.get_portable_registry_mut().retain(|id | keep_these_ids.contains(&id));
}

/// This trait is implemented for metadatas, and its purpose is to hand back iterators over
/// all of the type IDs (doesn't need to recurse into them) that are used in the metadata,
/// so that we know which ones we need to keep around in the type registry (and thus which 
/// ones we can remove).
/// 
/// There are two methods that need implementing:
/// - [`IterateTypeIds::iterate_type_ids_to_consider_for_stripping`] should return all type IDs
///   that, if they contain outer enum types, would mean that we cannot strip varaints out of
///   said outer enum types.
/// - [`IterateTypeIds::iterate_type_ids_ignored_for_stripping`] should return all of the type
///   IDs found in the metadata but _not_ returned from the above call.
/// 
/// Which types do we need to consider when it comes ot knowing whether to strip our outer
/// enum variants (ie remove the variants representing pallets that we are not interested in)?
/// Basically, any type that the user could obtain through Subxt which may contains an outer enum, 
/// and which Subxt needs to decode as a result of this. (If we strip things from the outer enums, then
/// they won't necessarily be able to decode everything we get given back any more!). Examples:
/// - Runtime API output types (but not input types: we can still encode them ok regardless)
/// - Storage keys and values (both are decoded)
/// 
/// One exception here is that Subxt can decode extrinsics in blocks, but if metadata is stripped
/// then some extrinsics won't be decodable any more, and that's fine. This may also happen if the 
/// metadata being used is behind the latest runtime version, and in any case, users should not
/// strip metadata if they want to decode extrinsics freom _any_ pallet.
/// 
/// When implementing this, if you're ever not sure whether a type ID should be considered for
/// stripping or not, always assume that it is to play it safe. If _everything_ is in 
/// [`IterateTypeIds::iterate_type_ids_to_consider_for_stripping`], then things will still work
/// (we'll just end up stripping fewer things from the metadata).
trait IterateTypeIds {
    /// This should return an iterator over all type IDs which, if seen, should prevent us
    /// from stripping variants from RuntimeCall/Error/Event enums ("outer enums").
    fn iterate_type_ids_to_consider_for_stripping(&self) -> impl Iterator<Item=u32>;

    /// This should iterate over any type IDs missed by the above, such that with this _and_ 
    /// the above we are iterating over all type IDs that we need to keep.
    fn iterate_type_ids_ignored_for_stripping(&self) -> impl Iterator<Item=u32>;

    /// A helper which chains the above.
    fn iterate_all_type_ids(&self) -> impl Iterator<Item=u32> {
        self.iterate_type_ids_to_consider_for_stripping().chain(self.iterate_type_ids_ignored_for_stripping())
    }
}

impl IterateTypeIds for v14::RuntimeMetadataV14 {
    fn iterate_type_ids_to_consider_for_stripping(&self) -> impl Iterator<Item=u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter().flat_map(|pallet| {
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

            pallet_storage_types
                .chain(pallet_constant_types)
                .chain(pallet_event_type)
                .chain(pallet_error_type)
        });

        // Transaction Extension types:
        let transaction_extension_types = self.extrinsic.signed_extensions.iter().flat_map(|ext| {
            [ext.ty.id, ext.additional_signed.id].into_iter()
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
            .chain(transaction_extension_types)
            .chain(dispatch_error_ty)
    }

    fn iterate_type_ids_ignored_for_stripping(&self) -> impl Iterator<Item=u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter().flat_map(|pallet| {
            // We don't need to _decode_ any outer enums used in calls.
            pallet.calls.as_ref().into_iter().map(|calls| {
                calls.ty.id
            })
        });

        // The extrinsic type (stripping Call type won't adversely affect this):
        let extrinsic_type_id = self.extrinsic.ty.id;

        pallet_types.chain(Some(extrinsic_type_id))
    }
}

impl IterateTypeIds for v15::RuntimeMetadataV15 {
    fn iterate_type_ids_to_consider_for_stripping(&self) -> impl Iterator<Item=u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter().flat_map(|pallet| {
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

            pallet_storage_types
                .chain(pallet_constant_types)
                .chain(pallet_event_type)
                .chain(pallet_error_type)
        });

        // Runtime API outputs (inputs don't matter for stripping):
        let runtime_api_types = self.apis.iter().flat_map(|api| {
            &api.methods
        }).map(|method| {
            method.output.id
        });

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
            .chain(transaction_extension_types)
            .chain(custom_type_ids)
            .chain(dispatch_error_ty)
    }

    fn iterate_type_ids_ignored_for_stripping(&self) -> impl Iterator<Item=u32> {
        // Pallet calls:
        let pallet_types = self.pallets.iter().flat_map(|pallet| {
            // We don't need to _decode_ any outer enums used in calls.
            pallet.calls.as_ref().into_iter().map(|calls| {
                calls.ty.id
            })
        });

        // Runtime API inputs:
        let runtime_api_types = self.apis.iter().flat_map(|api| {
            &api.methods
        }).flat_map(|method| {
            // We don't need to decode runtime API inputs, only the outputs.
            method.inputs.iter().map(|input| input.ty.id)
        });

        // The extrinsic type IDs:
        let extrinsic_type_ids = [
            self.extrinsic.call_ty.id,
            self.extrinsic.address_ty.id,
            self.extrinsic.extra_ty.id,
            self.extrinsic.signature_ty.id
        ];

        pallet_types.chain(runtime_api_types).chain(extrinsic_type_ids)
    }
}

impl IterateTypeIds for v16::RuntimeMetadataV16 {
    fn iterate_type_ids_to_consider_for_stripping(&self) -> impl Iterator<Item=u32> {
        // Gather pallet types:
        let pallet_types = self.pallets.iter().flat_map(|pallet| {
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

            let pallet_view_fn_outputs = pallet.view_functions.iter().map(|vf| {
                vf.output.id
            });

            pallet_storage_types
                .chain(pallet_constant_types)
                .chain(pallet_event_type)
                .chain(pallet_error_type)
                .chain(pallet_view_fn_outputs)
        });

        // Runtime API outputs (inputs don't matter for stripping):
        let runtime_api_outputs = self.apis.iter().flat_map(|api| {
            &api.methods
        }).map(|method| {
            method.output.id
        });

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
            .chain(runtime_api_outputs)
            .chain(transaction_extension_types)
            .chain(custom_type_ids)
            .chain(dispatch_error_ty)
    }

    fn iterate_type_ids_ignored_for_stripping(&self) -> impl Iterator<Item=u32> {
        // Pallet calls:
        let pallet_types = self.pallets.iter().flat_map(|pallet| {
            // We don't need to _decode_ any outer enums used in calls.
            let pallet_call_types = pallet.calls.as_ref().into_iter().map(|calls| {
                calls.ty.id
            });

            // Likewise with pallet view function input types.
            let pallet_view_fn_inputs = pallet.view_functions.iter().flat_map(|vf| {
                vf.inputs.iter().map(|input| input.ty.id)
            });

            pallet_call_types.chain(pallet_view_fn_inputs)
        });

        // Runtime API inputs:
        let runtime_api_inputs = self.apis.iter().flat_map(|api| {
            &api.methods
        }).flat_map(|method| {
            method.inputs.iter().map(|input| input.ty.id)
        });

        // The extrinsic type IDs:
        let extrinsic_type_ids = [
            self.extrinsic.address_ty.id,
            self.extrinsic.signature_ty.id
        ];

        pallet_types.chain(runtime_api_inputs).chain(extrinsic_type_ids)
    }
}

/// This is implemented for any metadata version that a [`PortableRegistry`] can be obtained from.
trait GetPortableRegistry {
    fn get_portable_registry(&self) -> &PortableRegistry;
    fn get_portable_registry_mut(&mut self) -> &mut PortableRegistry;
}

impl GetPortableRegistry for v14::RuntimeMetadataV14 {
    fn get_portable_registry(&self) -> &PortableRegistry {
        &self.types
    }
    fn get_portable_registry_mut(&mut self) -> &mut PortableRegistry {
        &mut self.types
    }
}

impl GetPortableRegistry for v15::RuntimeMetadataV15 {
    fn get_portable_registry(&self) -> &PortableRegistry {
        &self.types
    }
    fn get_portable_registry_mut(&mut self) -> &mut PortableRegistry {
        &mut self.types
    }
}

impl GetPortableRegistry for v16::RuntimeMetadataV16 {
    fn get_portable_registry(&self) -> &PortableRegistry {
        &self.types
    }
    fn get_portable_registry_mut(&mut self) -> &mut PortableRegistry {
        &mut self.types
    }
}

/// This deeply recurses over any IDs given, handing back any child IDs seen in addition to the original IDs.
fn recurse_type_ids<'a, I: Iterator<Item = u32> + 'a>(types: &'a PortableRegistry, ids: I) -> impl Iterator<Item = u32> + 'a {
    ids.flat_map(|id| {
        use scale_info::TypeDef;
        let ty = &types.resolve(id).unwrap();

        let params = ty.type_params.iter().filter_map(|p| p.ty.map(|t| t.id));
        let inner_ids = match &ty.type_def {
            TypeDef::Composite(c) => {
                Either6::A(c.fields.iter().map(|field| field.ty.id))
            },
            TypeDef::Variant(v) => {
                Either6::B(v.variants.iter().flat_map(|v| &v.fields).map(|field| field.ty.id))
            },
            TypeDef::Sequence(s) => {
                Either6::C(std::iter::once(s.type_param.id))
            },
            TypeDef::Array(a) => {
                Either6::C(std::iter::once(a.type_param.id))
            },
            TypeDef::Compact(c) => {
                Either6::C(std::iter::once(c.type_param.id))
            },
            TypeDef::Tuple(t) => {
                Either6::D(t.fields.iter().map(|field| field.id))
            },
            TypeDef::Primitive(_) => {
                Either6::E(std::iter::empty())
            },
            TypeDef::BitSequence(b) => {
                Either6::F([b.bit_order_type.id, b.bit_store_type.id].into_iter())
            },
        };

        let iter: Box<dyn Iterator<Item = u32>> = Box::new(recurse_type_ids(types, inner_ids));
        std::iter::once(id).chain(params).chain(iter)
    })
}

/// This strips variants from the given variant type (panicking if not a known variant type).
fn strip_variants_in_enum_type<F>(types: &mut PortableRegistry, mut pallets_filter: F, id: u32)
where
    F: FnMut(&str) -> bool,
{
    let ty = {
        types
            .types
            .get_mut(id as usize)
            .expect("Metadata should contain enum type in registry")
    };

    let scale_info::TypeDef::Variant(variant) = &mut ty.ty.type_def else {
        panic!("Metadata type is expected to be a variant type");
    };

    variant.variants.retain(|v| pallets_filter(&v.name));
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
either!(Either6: A B C D E F);
