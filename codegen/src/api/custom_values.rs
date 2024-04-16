// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use heck::ToSnakeCase as _;
use scale_typegen::typegen::ir::ToTokensWithSettings;
use scale_typegen::TypeGenerator;
use std::collections::HashSet;
use subxt_metadata::{CustomValueMetadata, Metadata};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Generate the custom values mod, if there are any custom values in the metadata. Else returns None.
pub fn generate_custom_values(
    metadata: &Metadata,
    type_gen: &TypeGenerator,
    crate_path: &syn::Path,
) -> TokenStream2 {
    let mut fn_names_taken = HashSet::new();
    let custom = metadata.custom();
    let custom_values_fns = custom.iter().filter_map(|custom_value| {
        generate_custom_value_fn(custom_value, type_gen, crate_path, &mut fn_names_taken)
    });

    quote! {
        pub struct CustomValuesApi;

        impl CustomValuesApi {
            #(#custom_values_fns)*
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
