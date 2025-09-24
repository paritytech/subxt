// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use heck::ToUpperCamelCase as _;

use crate::CodegenError;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use scale_typegen::TypeGenerator;
use scale_typegen::typegen::ir::ToTokensWithSettings;
use std::collections::HashSet;
use subxt_metadata::{PalletMetadata, ViewFunctionMetadata};

fn generate_pallet_view_function(
    view_function: ViewFunctionMetadata<'_>,
    type_gen: &TypeGenerator,
    crate_path: &syn::Path,
) -> Result<(TokenStream2, TokenStream2), CodegenError> {
    let types_mod_ident = type_gen.types_mod_ident();

    let view_function_name_str = view_function.name();
    let view_function_name_ident = format_ident!("{}", view_function_name_str);

    let query_id = view_function.query_id();
    let validation_hash = view_function.hash();

    let docs = view_function.docs();
    let docs: TokenStream2 = type_gen
        .settings()
        .should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();

    struct Input {
        name: syn::Ident,
        type_alias: syn::Ident,
        type_path: TokenStream2,
    }

    let view_function_inputs: Vec<Input> = {
        let mut unique_names = HashSet::new();
        let mut unique_aliases = HashSet::new();

        view_function
            .inputs()
            .enumerate()
            .map(|(idx, input)| {
                // These are method names, which can just be '_', but struct field names can't
                // just be an underscore, so fix any such names we find to work in structs.
                let mut name = input.name.trim_start_matches('_').to_string();
                if name.is_empty() {
                    name = format!("_{idx}");
                }
                while !unique_names.insert(name.clone()) {
                    name = format!("{name}_param{idx}");
                }

                // The alias type name is based on the name, above.
                let mut alias = name.to_upper_camel_case();
                // Note: name is not empty.
                if alias.as_bytes()[0].is_ascii_digit() {
                    alias = format!("Param{alias}");
                }
                while !unique_aliases.insert(alias.clone()) {
                    alias = format!("{alias}Param{idx}");
                }

                // Path to the actual type we'll have generated for this input.
                let type_path = type_gen
                    .resolve_type_path(input.id)
                    .expect("view function input type is in metadata; qed")
                    .to_token_stream(type_gen.settings());

                Input {
                    name: format_ident!("{name}"),
                    type_alias: format_ident!("{alias}"),
                    type_path,
                }
            })
            .collect()
    };

    let input_struct_params = view_function_inputs
        .iter()
        .map(|i| {
            let arg = &i.name;
            let ty = &i.type_alias;
            quote!(pub #arg: #ty)
        })
        .collect::<Vec<_>>();

    let input_args = view_function_inputs
        .iter()
        .map(|i| {
            let arg = &i.name;
            let ty = &i.type_alias;
            quote!(#arg: #view_function_name_ident::#ty)
        })
        .collect::<Vec<_>>();

    let input_type_aliases = view_function_inputs.iter().map(|i| {
        let ty = &i.type_alias;
        let path = &i.type_path;
        quote!(pub type #ty = #path;)
    });

    let input_param_names = view_function_inputs.iter().map(|i| &i.name);

    let output_type_path = type_gen
        .resolve_type_path(view_function.output_ty())?
        .to_token_stream(type_gen.settings());

    let input_struct_derives = type_gen.settings().derives.default_derives();

    // Define the input and output type bits.
    let view_function_def = quote!(
        pub mod #view_function_name_ident {
            use super::root_mod;
            use super::#types_mod_ident;

            #input_struct_derives
            pub struct Input {
                #(#input_struct_params,)*
            }

            #(#input_type_aliases)*

            pub mod output {
                use super::#types_mod_ident;
                pub type Output = #output_type_path;
            }
        }
    );

    // Define the getter method that will live on the `ViewFunctionApi` type.
    let view_function_getter = quote!(
        #docs
        pub fn #view_function_name_ident(
            &self,
            #(#input_args),*
        ) -> #crate_path::view_functions::payload::StaticPayload<
            #view_function_name_ident::Input,
            #view_function_name_ident::output::Output
        > {
            #crate_path::view_functions::payload::StaticPayload::new_static(
                [#(#query_id,)*],
                #view_function_name_ident::Input {
                    #(#input_param_names,)*
                },
                [#(#validation_hash,)*],
            )
        }
    );

    Ok((view_function_def, view_function_getter))
}

pub fn generate_pallet_view_functions(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    crate_path: &syn::Path,
) -> Result<TokenStream2, CodegenError> {
    if !pallet.has_view_functions() {
        // If there are no view functions in this pallet, we
        // don't generate anything.
        return Ok(quote! {});
    }

    let view_functions: Vec<_> = pallet
        .view_functions()
        .map(|vf| generate_pallet_view_function(vf, type_gen, crate_path))
        .collect::<Result<_, _>>()?;

    let view_functions_defs = view_functions.iter().map(|(apis, _)| apis);
    let view_functions_getters = view_functions.iter().map(|(_, getters)| getters);

    let types_mod_ident = type_gen.types_mod_ident();

    Ok(quote! {
        pub mod view_functions {
            use super::root_mod;
            use super::#types_mod_ident;

            pub struct ViewFunctionsApi;

            impl ViewFunctionsApi {
                #( #view_functions_getters )*
            }

            #( #view_functions_defs )*
        }
    })
}
