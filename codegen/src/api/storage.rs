// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::types::TypeGenerator;
use frame_metadata::{
    v14::RuntimeMetadataV14,
    PalletMetadata,
    StorageEntryMetadata,
    StorageEntryModifier,
    StorageEntryType,
    StorageHasher,
};
use heck::ToSnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{
    format_ident,
    quote,
};
use scale_info::{
    form::PortableForm,
    TypeDef,
};

/// Generate functions which create storage addresses from the provided pallet's metadata.
/// These addresses can be used to access and iterate over storage values.
///
/// # Arguments
///
/// - `metadata` - Runtime metadata from which the storages are generated.
/// - `type_gen` - The type generator containing all types defined by metadata.
/// - `pallet` - Pallet metadata from which the storages are generated.
/// - `types_mod_ident` - The ident of the base module that we can use to access the generated types from.
pub fn generate_storage(
    metadata: &RuntimeMetadataV14,
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata<PortableForm>,
    types_mod_ident: &syn::Ident,
) -> TokenStream2 {
    let storage = if let Some(ref storage) = pallet.storage {
        storage
    } else {
        return quote!()
    };

    let storage_fns: Vec<_> = storage
        .entries
        .iter()
        .map(|entry| generate_storage_entry_fns(metadata, type_gen, pallet, entry))
        .collect();

    quote! {
        pub mod storage {
            use super::#types_mod_ident;

            pub struct StorageApi;

            impl StorageApi {
                #( #storage_fns )*
            }
        }
    }
}

fn generate_storage_entry_fns(
    metadata: &RuntimeMetadataV14,
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata<PortableForm>,
    storage_entry: &StorageEntryMetadata<PortableForm>,
) -> TokenStream2 {
    let (fields, key_impl) = match storage_entry.ty {
        StorageEntryType::Plain(_) => {
            (vec![], quote!(::subxt::storage::StorageEntryKey::Plain))
        }
        StorageEntryType::Map {
            ref key,
            ref hashers,
            ..
        } => {
            let key_ty = type_gen.resolve_type(key.id());
            let hashers = hashers
                .iter()
                .map(|hasher| {
                    let hasher = match hasher {
                        StorageHasher::Blake2_128 => "Blake2_128",
                        StorageHasher::Blake2_256 => "Blake2_256",
                        StorageHasher::Blake2_128Concat => "Blake2_128Concat",
                        StorageHasher::Twox128 => "Twox128",
                        StorageHasher::Twox256 => "Twox256",
                        StorageHasher::Twox64Concat => "Twox64Concat",
                        StorageHasher::Identity => "Identity",
                    };
                    let hasher = format_ident!("{}", hasher);
                    quote!( ::subxt::storage::StorageHasher::#hasher )
                })
                .collect::<Vec<_>>();
            match key_ty.type_def() {
                TypeDef::Tuple(tuple) => {
                    let fields = tuple
                        .fields()
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            let field_name = format_ident!("_{}", syn::Index::from(i));
                            let field_type = type_gen.resolve_type_path(f.id(), &[]);
                            (field_name, field_type)
                        })
                        .collect::<Vec<_>>();

                    let key_impl = if hashers.len() == fields.len() {
                        // If the number of hashers matches the number of fields, we're dealing with
                        // something shaped like a StorageNMap, and each field should be hashed separately
                        // according to the corresponding hasher.
                        let keys = hashers
                            .into_iter()
                            .zip(&fields)
                            .map(|(hasher, (field_name, _))| {
                                quote!( ::subxt::storage::StorageMapKey::new(#field_name, #hasher) )
                            });
                        quote! {
                            ::subxt::storage::StorageEntryKey::Map(
                                vec![ #( #keys ),* ]
                            )
                        }
                    } else if hashers.len() == 1 {
                        // If there is one hasher, then however many fields we have, we want to hash a
                        // tuple of them using the one hasher we're told about. This corresponds to a
                        // StorageMap.
                        let hasher = hashers.get(0).expect("checked for 1 hasher");
                        let items = fields.iter().map(|(field_name, _)| {
                            quote!( #field_name )
                        });
                        quote! {
                            ::subxt::storage::StorageEntryKey::Map(
                                vec![ ::subxt::storage::StorageMapKey::new(&(#( #items ),*), #hasher) ]
                            )
                        }
                    } else {
                        // If we hit this condition, we don't know how to handle the number of hashes vs fields
                        // that we've been handed, so abort.
                        abort_call_site!(
                            "Number of hashers ({}) does not equal 1 for StorageMap, or match number of fields ({}) for StorageNMap",
                            hashers.len(),
                            fields.len()
                        )
                    };

                    (fields, key_impl)
                }
                _ => {
                    let ty_path = type_gen.resolve_type_path(key.id(), &[]);
                    let fields = vec![(format_ident!("_0"), ty_path.clone())];
                    let hasher = hashers.get(0).unwrap_or_else(|| {
                        abort_call_site!("No hasher found for single key")
                    });
                    let key_impl = quote! {
                        ::subxt::storage::StorageEntryKey::Map(
                            vec![ ::subxt::storage::StorageMapKey::new(_0, #hasher) ]
                        )
                    };
                    (fields, key_impl)
                }
            }
        }
    };

    let pallet_name = &pallet.name;
    let storage_name = &storage_entry.name;
    let storage_hash =
        subxt_metadata::get_storage_hash(metadata, pallet_name, storage_name)
            .unwrap_or_else(|_| {
                abort_call_site!(
                    "Metadata information for the storage entry {}_{} could not be found",
                    pallet_name,
                    storage_name
                )
            });

    let fn_name = format_ident!("{}", storage_entry.name.to_snake_case());
    let storage_entry_ty = match storage_entry.ty {
        StorageEntryType::Plain(ref ty) => ty,
        StorageEntryType::Map { ref value, .. } => value,
    };
    let storage_entry_value_ty = type_gen.resolve_type_path(storage_entry_ty.id(), &[]);
    let return_ty = match storage_entry.modifier {
        StorageEntryModifier::Default => {
            quote!( #storage_entry_value_ty )
        }
        StorageEntryModifier::Optional => {
            quote!( ::core::option::Option<#storage_entry_value_ty> )
        }
    };

    let docs = &storage_entry.docs;
    let docs_token = quote! { #( #[doc = #docs ] )* };

    let key_args = fields.iter().map(|(field_name, field_type)| {
        // The field type is translated from `std::vec::Vec<T>` to `[T]`, if the
        // interface should generate a reference. In such cases, the vector ultimately is
        // a slice.
        let field_ty = match field_type.vec_type_param() {
            Some(ty) => quote!(&[#ty]),
            _ => quote!(#field_type),
        };
        quote!( #field_name: & #field_ty )
    });

    // If the item is a map, we want a way to access the root entry to do things like iterate over it,
    // so expose a function to create this entry, too:
    let root_entry_fn = if matches!(storage_entry.ty, StorageEntryType::Map { .. }) {
        let fn_name_root = format_ident!("{}_root", fn_name);
        quote! (
            #docs_token
            pub fn #fn_name_root(
                &self,
            ) -> ::subxt::storage::StorageAddress::<'static, #return_ty> {
                ::subxt::storage::StorageAddress::new_with_validation(
                    #pallet_name,
                    #storage_name,
                    ::subxt::storage::StorageEntryKey::Plain,
                    [#(#storage_hash,)*]
                )
            }
        )
    } else {
        quote!()
    };

    quote! {
        // Access a specific value from a storage entry
        #docs_token
        pub fn #fn_name(
            &self,
            #( #key_args, )*
        ) -> ::subxt::storage::StorageAddress::<'static, #return_ty> {
            ::subxt::storage::StorageAddress::new_with_validation(
                #pallet_name,
                #storage_name,
                #key_impl,
                [#(#storage_hash,)*]
            )
        }

        #root_entry_fn
    }
}
