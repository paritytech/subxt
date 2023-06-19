// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{types::TypeGenerator, CratePath};
use heck::ToSnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use scale_info::TypeDef;
use subxt_metadata::{
    PalletMetadata, StorageEntryMetadata, StorageEntryModifier, StorageEntryType,
};

use super::CodegenError;

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
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    types_mod_ident: &syn::Ident,
    crate_path: &CratePath,
    should_gen_docs: bool,
) -> Result<TokenStream2, CodegenError> {
    let Some(storage) = pallet.storage() else {
        return Ok(quote!());
    };

    let storage_fns = storage
        .entries()
        .iter()
        .map(|entry| {
            generate_storage_entry_fns(type_gen, pallet, entry, crate_path, should_gen_docs)
        })
        .collect::<Result<Vec<_>, CodegenError>>()?;

    Ok(quote! {
        pub mod storage {
            use super::#types_mod_ident;

            pub struct StorageApi;

            impl StorageApi {
                #( #storage_fns )*
            }
        }
    })
}

fn generate_storage_entry_fns(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    storage_entry: &StorageEntryMetadata,
    crate_path: &CratePath,
    should_gen_docs: bool,
) -> Result<TokenStream2, CodegenError> {
    let (fields, key_impl) = match storage_entry.entry_type() {
        StorageEntryType::Plain(_) => (vec![], quote!(vec![])),
        StorageEntryType::Map { key_ty, .. } => {
            match &type_gen.resolve_type(*key_ty).type_def {
                // An N-map; return each of the keys separately.
                TypeDef::Tuple(tuple) => {
                    let fields = tuple
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            let field_name = format_ident!("_{}", syn::Index::from(i));
                            let field_type = type_gen.resolve_type_path(f.id);
                            (field_name, field_type)
                        })
                        .collect::<Vec<_>>();

                    let keys = fields
                        .iter()
                        .map(|(field_name, _)| {
                            quote!( #crate_path::storage::address::make_static_storage_map_key(#field_name.borrow()) )
                        });
                    let key_impl = quote! {
                        vec![ #( #keys ),* ]
                    };

                    (fields, key_impl)
                }
                // A map with a single key; return the single key.
                _ => {
                    let ty_path = type_gen.resolve_type_path(*key_ty);
                    let fields = vec![(format_ident!("_0"), ty_path)];
                    let key_impl = quote! {
                        vec![ #crate_path::storage::address::make_static_storage_map_key(_0.borrow()) ]
                    };
                    (fields, key_impl)
                }
            }
        }
    };

    let pallet_name = pallet.name();
    let storage_name = storage_entry.name();
    let Some(storage_hash) = pallet.storage_hash(storage_name) else {
        return Err(CodegenError::MissingStorageMetadata(pallet_name.into(), storage_name.into()));
    };

    let fn_name = format_ident!("{}", storage_entry.name().to_snake_case());
    let storage_entry_ty = match storage_entry.entry_type() {
        StorageEntryType::Plain(ty) => *ty,
        StorageEntryType::Map { value_ty, .. } => *value_ty,
    };
    let storage_entry_value_ty = type_gen.resolve_type_path(storage_entry_ty);

    let docs = storage_entry.docs();
    let docs = should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();

    let key_args = fields.iter().map(|(field_name, field_type)| {
        // The field type is translated from `std::vec::Vec<T>` to `[T]`. We apply
        // Borrow to all types, so this just makes it a little more ergonomic.
        //
        // TODO [jsdw]: Support mappings like `String -> str` too for better borrow
        // ergonomics.
        let field_ty = match field_type.vec_type_param() {
            Some(ty) => quote!([#ty]),
            _ => quote!(#field_type),
        };
        quote!( #field_name: impl ::std::borrow::Borrow<#field_ty> )
    });

    let is_map_type = matches!(storage_entry.entry_type(), StorageEntryType::Map { .. });

    // Is the entry iterable?
    let is_iterable_type = if is_map_type {
        quote!(#crate_path::storage::address::Yes)
    } else {
        quote!(())
    };

    let has_default_value = match storage_entry.modifier() {
        StorageEntryModifier::Default => true,
        StorageEntryModifier::Optional => false,
    };

    // Does the entry have a default value?
    let is_defaultable_type = if has_default_value {
        quote!(#crate_path::storage::address::Yes)
    } else {
        quote!(())
    };

    // If the item is a map, we want a way to access the root entry to do things like iterate over it,
    // so expose a function to create this entry, too:
    let root_entry_fn = if is_map_type {
        let fn_name_root = format_ident!("{}_root", fn_name);
        quote!(
            #docs
            pub fn #fn_name_root(
                &self,
            ) -> #crate_path::storage::address::Address::<
                #crate_path::storage::address::StaticStorageMapKey,
                #storage_entry_value_ty,
                (),
                #is_defaultable_type,
                #is_iterable_type
            > {
                #crate_path::storage::address::Address::new_static(
                    #pallet_name,
                    #storage_name,
                    Vec::new(),
                    [#(#storage_hash,)*]
                )
            }
        )
    } else {
        quote!()
    };

    Ok(quote! {
        // Access a specific value from a storage entry
        #docs
        pub fn #fn_name(
            &self,
            #( #key_args, )*
        ) -> #crate_path::storage::address::Address::<
            #crate_path::storage::address::StaticStorageMapKey,
            #storage_entry_value_ty,
            #crate_path::storage::address::Yes,
            #is_defaultable_type,
            #is_iterable_type
        > {
            #crate_path::storage::address::Address::new_static(
                #pallet_name,
                #storage_name,
                #key_impl,
                [#(#storage_hash,)*]
            )
        }

        #root_entry_fn
    })
}
