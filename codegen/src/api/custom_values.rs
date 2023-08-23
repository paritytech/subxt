// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::collections::HashSet;

use crate::{types::TypeGenerator, CratePath};
use heck::ToSnakeCase as _;
use subxt_metadata::{CustomValueMetadata, Metadata};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

/// Generate the custom values mod, if there are any custom values in the metadata. Else returns None.
pub fn generate_custom_values<'a>(
    metadata: &'a Metadata,
    type_gen: &'a TypeGenerator,
    crate_path: &'a CratePath,
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
    crate_path: &CratePath,
    fn_names_taken: &mut HashSet<String>,
) -> Option<TokenStream2> {
    // names are transformed to snake case to make for good function identifiers.
    let name = custom_value.name();
    let fn_name = name.to_snake_case();
    // Skip elements where the fn name is already occupied. E.g. if you have custom values with names "Foo" and "foo" in the metadata.
    if fn_names_taken.contains(&fn_name) {
        return None;
    }
    let fn_name_ident = format_ident!("{fn_name}");
    fn_names_taken.insert(fn_name);

    let custom_value_hash = custom_value.hash();
    let return_ty = type_gen.resolve_type_path(custom_value.type_id());

    Some(quote!(
        pub fn #fn_name_ident() -> #crate_path::custom_values::StaticAddress<#return_ty> {
            #crate_path::custom_values::StaticAddress::new_static(#name, [#(#custom_value_hash,)*])
        }
    ))
}
