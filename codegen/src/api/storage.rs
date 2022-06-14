// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

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

/// Generate storage from the provided pallet's metadata.
///
/// The function creates a new module named `storage` under the pallet's module.
///
/// ```ignore
/// pub mod PalletName {
///     pub mod storage {
///     ...
///     }
/// }
/// ```
///
/// The function generates the storage as rust structs that implement the `subxt::StorageEntry`
/// trait to uniquely identify the storage's identity when creating the extrinsic.
///
/// ```ignore
/// pub struct StorageName {
///      pub storage_param: type,
/// }
/// impl ::subxt::StorageEntry for StorageName {
/// ...
/// }
/// ```
///
/// Storages are extracted from the API and wrapped into the generated `StorageApi` of
/// each module.
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

    let (storage_structs, storage_fns): (Vec<_>, Vec<_>) = storage
        .entries
        .iter()
        .map(|entry| generate_storage_entry_fns(metadata, type_gen, pallet, entry))
        .unzip();

    quote! {
        pub mod storage {
            use super::#types_mod_ident;

            #( #storage_structs )*

            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }

            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }

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
) -> (TokenStream2, TokenStream2) {
    let entry_struct_ident = format_ident!("{}", storage_entry.name);
    let (fields, entry_struct, constructor, key_impl, should_ref) = match storage_entry.ty
    {
        StorageEntryType::Plain(_) => {
            let entry_struct = quote!( pub struct #entry_struct_ident; );
            let constructor = quote!( #entry_struct_ident );
            let key_impl = quote!(::subxt::StorageEntryKey::Plain);
            (vec![], entry_struct, constructor, key_impl, false)
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
                    quote!( ::subxt::StorageHasher::#hasher )
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

                    let field_names = fields.iter().map(|(n, _)| n);
                    let field_types = fields.iter().map(|(_, t)| {
                        // If the field type is `::std::vec::Vec<T>` obtain the type parameter and
                        // surround with slice brackets. Otherwise, utilize the field_type as is.
                        match t.vec_type_param() {
                            Some(ty) => quote!([#ty]),
                            None => quote!(#t),
                        }
                    });

                    let entry_struct = quote! {
                        pub struct #entry_struct_ident <'a>( #( pub &'a #field_types ),* );

                    };
                    let constructor =
                        quote!( #entry_struct_ident( #( #field_names ),* ) );

                    let key_impl = if hashers.len() == fields.len() {
                        // If the number of hashers matches the number of fields, we're dealing with
                        // something shaped like a StorageNMap, and each field should be hashed separately
                        // according to the corresponding hasher.
                        let keys = hashers
                            .into_iter()
                            .enumerate()
                            .map(|(field_idx, hasher)| {
                                let index = syn::Index::from(field_idx);
                                quote!( ::subxt::StorageMapKey::new(&self.#index, #hasher) )
                            });
                        quote! {
                            ::subxt::StorageEntryKey::Map(
                                vec![ #( #keys ),* ]
                            )
                        }
                    } else if hashers.len() == 1 {
                        // If there is one hasher, then however many fields we have, we want to hash a
                        // tuple of them using the one hasher we're told about. This corresponds to a
                        // StorageMap.
                        let hasher = hashers.get(0).expect("checked for 1 hasher");
                        let items = (0..fields.len()).map(|field_idx| {
                            let index = syn::Index::from(field_idx);
                            quote!( &self.#index )
                        });
                        quote! {
                            ::subxt::StorageEntryKey::Map(
                                vec![ ::subxt::StorageMapKey::new(&(#( #items ),*), #hasher) ]
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

                    (fields, entry_struct, constructor, key_impl, true)
                }
                _ => {
                    let (lifetime_param, lifetime_ref) = (quote!(<'a>), quote!(&'a));

                    let ty_path = type_gen.resolve_type_path(key.id(), &[]);
                    let fields = vec![(format_ident!("_0"), ty_path.clone())];

                    // `ty_path` can be `std::vec::Vec<T>`. In such cases, the entry struct
                    // should contain a slice reference.
                    let ty_slice = match ty_path.vec_type_param() {
                        Some(ty) => quote!([#ty]),
                        None => quote!(#ty_path),
                    };
                    let entry_struct = quote! {
                        pub struct #entry_struct_ident #lifetime_param( pub #lifetime_ref #ty_slice );
                    };
                    let constructor = quote!( #entry_struct_ident(_0) );
                    let hasher = hashers.get(0).unwrap_or_else(|| {
                        abort_call_site!("No hasher found for single key")
                    });
                    let key_impl = quote! {
                        ::subxt::StorageEntryKey::Map(
                            vec![ ::subxt::StorageMapKey::new(&self.0, #hasher) ]
                        )
                    };
                    (fields, entry_struct, constructor, key_impl, true)
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
    let fn_name_iter = format_ident!("{}_iter", fn_name);
    let storage_entry_ty = match storage_entry.ty {
        StorageEntryType::Plain(ref ty) => ty,
        StorageEntryType::Map { ref value, .. } => value,
    };
    let storage_entry_value_ty = type_gen.resolve_type_path(storage_entry_ty.id(), &[]);
    let (return_ty, fetch) = match storage_entry.modifier {
        StorageEntryModifier::Default => {
            (quote!( #storage_entry_value_ty ), quote!(fetch_or_default))
        }
        StorageEntryModifier::Optional => {
            (
                quote!( ::core::option::Option<#storage_entry_value_ty> ),
                quote!(fetch),
            )
        }
    };

    let (lifetime_param, reference, anon_lifetime) = if should_ref {
        (quote!(<'a>), quote!(&), quote!(<'_>))
    } else {
        (quote!(), quote!(), quote!())
    };

    let storage_entry_impl = quote! (
        const PALLET: &'static str = #pallet_name;
        const STORAGE: &'static str = #storage_name;
        type Value = #storage_entry_value_ty;
        fn key(&self) -> ::subxt::StorageEntryKey {
            #key_impl
        }
    );

    let storage_entry_type = quote! {
        #entry_struct
        impl ::subxt::StorageEntry for #entry_struct_ident #anon_lifetime {
            #storage_entry_impl
        }
    };

    let docs = &storage_entry.docs;
    let docs_token = quote! { #( #[doc = #docs ] )* };
    let client_iter_fn = if matches!(storage_entry.ty, StorageEntryType::Map { .. }) {
        quote! (
            #docs_token
            pub async fn #fn_name_iter(
                &self,
                block_hash: ::core::option::Option<T::Hash>,
            ) -> ::core::result::Result<::subxt::KeyIter<'a, T, #entry_struct_ident #lifetime_param>, ::subxt::BasicError> {
                let runtime_storage_hash = {
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    metadata.storage_hash::<#entry_struct_ident>()?
                };
                if runtime_storage_hash == [#(#storage_hash,)*] {
                    self.client.storage().iter(block_hash).await
                } else {
                    Err(::subxt::MetadataError::IncompatibleMetadata.into())
                }
            }
        )
    } else {
        quote!()
    };

    let key_args = fields.iter().map(|(field_name, field_type)| {
        // The field type is translated from `std::vec::Vec<T>` to `[T]`, if the
        // interface should generate a reference. In such cases, the vector ultimately is
        // a slice.
        let field_ty = match field_type.vec_type_param() {
            Some(ty) if should_ref => quote!([#ty]),
            _ => quote!(#field_type),
        };
        quote!( #field_name: #reference #field_ty )
    });

    let client_fns = quote! {
        #docs_token
        pub async fn #fn_name(
            &self,
            #( #key_args, )*
            block_hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<#return_ty, ::subxt::BasicError> {
            let runtime_storage_hash = {
                let locked_metadata = self.client.metadata();
                let metadata = locked_metadata.read();
                metadata.storage_hash::<#entry_struct_ident>()?
            };
            if runtime_storage_hash == [#(#storage_hash,)*] {
                let entry = #constructor;
                self.client.storage().#fetch(&entry, block_hash).await
            } else {
                Err(::subxt::MetadataError::IncompatibleMetadata.into())
            }
        }

        #client_iter_fn
    };

    (storage_entry_type, client_fns)
}
