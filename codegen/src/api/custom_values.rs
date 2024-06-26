// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use heck::ToSnakeCase as _;
use scale_info::{TypeDef, TypeDefComposite};
use scale_typegen::typegen::ir::ToTokensWithSettings;
use scale_typegen::TypeGenerator;
use std::{any::Any, collections::HashSet};
use subxt_metadata::{CustomValueMetadata, Metadata};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

/// Generate the custom values mod, if there are any custom values in the metadata. Else returns None.
pub fn generate_custom_values(
    metadata: &Metadata,
    type_gen: &TypeGenerator,
    crate_path: &syn::Path,
) -> TokenStream2 {
    let mut fn_names_taken = HashSet::new();
    let custom = metadata.custom();
    let custom_types = custom.iter().map(|custom| {
        let name_str = custom.name();

        let name = format_ident!("{}", name_str);

        let Ok(ty_path) = type_gen.resolve_type_path(custom.type_id()) else {
            return quote! {};
        };
        let ty = ty_path.to_token_stream(type_gen.settings());

        let mut maybe_impl = None;
        let mut extra = None;
        if name_str == "Hashing" {
            // Extract hasher name
            let ty_path_str = ty.to_string();
            if ty_path_str.contains("BlakeTwo256") {
                maybe_impl = Some(quote! {
                    impl #crate_path::config::Hasher for #ty {
                        type Output = #crate_path::utils::H256;

                        fn hash(s: &[u8]) -> Self::Output {
                            let mut bytes = Vec::new();
                            #crate_path::storage::utils::hash_bytes(s, #crate_path::storage::utils::StorageHasher::Blake2_256, &mut bytes);
                            let arr: [u8; 32] = bytes.try_into().expect("Invalid hashing output provided");
                            arr.into()
                        }
                    }
                });
            }
        }

        if name_str == "Header" {
            // Extract header number from the provided type.
            let Ok(ty_res) = type_gen.resolve_type(custom.type_id()) else {
                return quote! {};
            };

            let TypeDef::Composite(composite) = &ty_res.type_def else {
                return quote! {};
            };

            // Sanity check for the number type.
            let number_ty = composite.fields.iter().find_map(
                |field| if let Some(n) = &field.name {
                    if n == "number" {
                        Some(field.ty.id)
                    } else {
                        None
                    }
                } else {
                    None
                }
            );

            if let Some(num) = number_ty {

                let Ok(ty_path) = type_gen.resolve_type_path(num) else {
                    return quote! {};
                };

                if !ty_path.is_compact() {
                    let ty = ty_path.to_token_stream(type_gen.settings());

                    extra = Some(quote! {
                        pub type HeaderNumber = #ty; 
                    });
                } else {
                    // Ty is compact.
                    let Ok(ty_res) = type_gen.resolve_type(num) else {
                        return quote! {};
                    };

                    let TypeDef::Compact(compact) = &ty_res.type_def else {
                        return quote! {};
                    };
                    let compact_ty_id = compact.type_param.id;

                    let Ok(ty_path) = type_gen.resolve_type_path(compact_ty_id) else {
                        return quote! {};
                    };
                    let ty = ty_path.to_token_stream(type_gen.settings());

                    extra = Some(quote! {
                        pub type HeaderNumber = #ty; 
                    });
                }


                maybe_impl = Some(quote! {
                    impl #crate_path::config::Header for #ty {
                        type Number = HeaderNumber;
                        type Hasher = Hashing;
    
                        // If we got to this point, the `number` field exists on the header
                        // structure.
                        fn number(&self) -> Self::Number {
                            self.number
                        }
                    }
                });
            }
        }

        quote! {
            pub type #name = #ty;

            #maybe_impl
            #extra
        }
    });

    let custom_values_fns = custom.iter().filter_map(|custom_value| {
        generate_custom_value_fn(custom_value, type_gen, crate_path, &mut fn_names_taken)
    });

    quote! {
        pub struct CustomValuesApi;

        impl CustomValuesApi {
            #(#custom_values_fns)*
        }

        pub mod custom_types {
            pub use super::*;

            #(#custom_types)*
        }
    }
}

/// Generates runtime functions for the given API metadata.
/// Returns None, if the name would not make for a valid identifier.
fn generate_custom_value_fn(
    custom_value: CustomValueMetadata,
    type_gen: &TypeGenerator,
    crate_path: &syn::Path,
    fn_names_taken: &mut HashSet<String>,
) -> Option<TokenStream2> {
    // names are transformed to snake case to make for good function identifiers.
    let name = custom_value.name();
    let fn_name = name.to_snake_case();
    if fn_names_taken.contains(&fn_name) {
        return None;
    }
    // if the fn_name would be an invalid ident, return None:
    let fn_name_ident = syn::parse_str::<syn::Ident>(&fn_name).ok()?;
    fn_names_taken.insert(fn_name);

    let custom_value_hash = custom_value.hash();

    // for custom values it is important to check if the type id is actually in the metadata:
    let type_is_valid = custom_value
        .types()
        .resolve(custom_value.type_id())
        .is_some();
    let (return_ty, decodable) = if type_is_valid {
        let return_ty = type_gen
            .resolve_type_path(custom_value.type_id())
            .expect("type is in metadata; qed")
            .to_token_stream(type_gen.settings());
        let decodable = quote!(#crate_path::utils::Yes);
        (return_ty, decodable)
    } else {
        // if type registry does not contain the type, we can just return the Encoded scale bytes.
        (quote!(()), quote!(()))
    };

    Some(quote!(
        pub fn #fn_name_ident(&self) -> #crate_path::custom_values::address::StaticAddress<#return_ty, #decodable> {
            #crate_path::custom_values::address::StaticAddress::new_static(#name, [#(#custom_value_hash,)*])
        }
    ))
}
