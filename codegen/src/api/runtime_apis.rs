// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{types::TypeGenerator, CodegenError};
use heck::ToSnakeCase as _;
use heck::ToUpperCamelCase as _;
use subxt_metadata::{Metadata, RuntimeApiMetadata};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

/// Generates runtime functions for the given API metadata.
fn generate_runtime_api(
    api: RuntimeApiMetadata,
    type_gen: &TypeGenerator,
    types_mod_ident: &syn::Ident,
    crate_path: &syn::Path,
    should_gen_docs: bool,
) -> Result<(TokenStream2, TokenStream2), CodegenError> {
    // Trait name must remain as is (upper case) to identity the runtime call.
    let trait_name_str = api.name();
    // The snake case for the trait name.
    let trait_name_snake = format_ident!("{}", api.name().to_snake_case());
    let docs = api.docs();
    let docs: TokenStream2 = should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();

    let structs_and_methods: Vec<_> = api.methods().map(|method| {
        let method_name = format_ident!("{}", method.name());
        let method_name_str = method.name();

        let docs = method.docs();
        let docs: TokenStream2 = should_gen_docs
            .then_some(quote! { #( #[doc = #docs ] )* })
            .unwrap_or_default();

        let inputs: Vec<_> = method.inputs().enumerate().map(|(idx, input)| {
            // These are method names, which can just be '_', but struct field names can't
            // just be an underscore, so fix any such names we find to work in structs.
            let (alias_name, name) = if input.name == "_" {
                (format_ident!("Param{}", idx), format_ident!("_{}", idx))
            } else {
                (format_ident!("{}", input.name.to_upper_camel_case()), format_ident!("{}", &input.name))
            };

            // Generate alias for runtime type.
            let ty = type_gen.resolve_type_path(input.ty);
            let aliased_param = quote!( pub type #alias_name = #ty; );

            // Structures are placed on the same level as the alias module.
            let struct_ty_path = quote!( #method_name::#alias_name );
            let struct_param = quote!(#name: #struct_ty_path);

            // Function parameters must be indented by `types`.
            let fn_param = quote!(#name: types::#struct_ty_path);
            (fn_param, struct_param, name, aliased_param)
        }).collect();

        let fn_params = inputs.iter().map(|(fn_param, _, _, _)| fn_param);
        let struct_params = inputs.iter().map(|(_, struct_param, _, _)| struct_param);
        let param_names = inputs.iter().map(|(_, _, name, _,)| name);
        let type_aliases = inputs.iter().map(|(_, _, _, aliased_param)| aliased_param);

        let output = type_gen.resolve_type_path(method.output_ty());
        let aliased_module = quote!(
            pub mod #method_name {
                use super::#types_mod_ident;

                #( #type_aliases )*
                pub type Output = #output;
            }
        );

        // From the method metadata generate a structure that holds
        // all parameter types. This structure is used with metadata
        // to encode parameters to the call via `encode_as_fields_to`.
        let derives = type_gen.default_derives();
        let struct_name = format_ident!("{}", method.name().to_upper_camel_case());
        let struct_input = quote!(
            #derives
            pub struct #struct_name {
                #( pub #struct_params, )*
            }

            #aliased_module
        );

        let Some(call_hash) = api.method_hash(method.name()) else {
            return Err(CodegenError::MissingRuntimeApiMetadata(
                trait_name_str.to_owned(),
                method_name_str.to_owned(),
            ))
        };

        let method = quote!(
            #docs
            pub fn #method_name(&self, #( #fn_params, )* ) -> #crate_path::runtime_api::Payload<types::#struct_name, types::#method_name::Output> {
                #crate_path::runtime_api::Payload::new_static(
                    #trait_name_str,
                    #method_name_str,
                    types::#struct_name { #( #param_names, )* },
                    [#(#call_hash,)*],
                )
            }
        );

        Ok((struct_input, method))
    }).collect::<Result<_, _>>()?;

    let trait_name = format_ident!("{}", trait_name_str);

    let structs = structs_and_methods.iter().map(|(struct_, _)| struct_);
    let methods = structs_and_methods.iter().map(|(_, method)| method);

    let runtime_api = quote!(
        pub mod #trait_name_snake {
            use super::root_mod;
            use super::#types_mod_ident;

            #docs
            pub struct #trait_name;

            impl #trait_name {
                #( #methods )*
            }

            pub mod types {
                use super::#types_mod_ident;

                #( #structs )*
            }
        }
    );

    // A getter for the `RuntimeApi` to get the trait structure.
    let trait_getter = quote!(
        pub fn #trait_name_snake(&self) -> #trait_name_snake::#trait_name {
            #trait_name_snake::#trait_name
        }
    );

    Ok((runtime_api, trait_getter))
}

/// Generate the runtime APIs.
pub fn generate_runtime_apis(
    metadata: &Metadata,
    type_gen: &TypeGenerator,
    types_mod_ident: &syn::Ident,
    crate_path: &syn::Path,
    should_gen_docs: bool,
) -> Result<TokenStream2, CodegenError> {
    let runtime_fns: Vec<_> = metadata
        .runtime_api_traits()
        .map(|api| {
            generate_runtime_api(api, type_gen, types_mod_ident, crate_path, should_gen_docs)
        })
        .collect::<Result<_, _>>()?;

    let runtime_apis_def = runtime_fns.iter().map(|(apis, _)| apis);
    let runtime_apis_getters = runtime_fns.iter().map(|(_, getters)| getters);

    Ok(quote! {
        pub mod runtime_apis {
            use super::root_mod;
            use super::#types_mod_ident;

            use #crate_path::ext::codec::Encode;

            pub struct RuntimeApi;

            impl RuntimeApi {
                #( #runtime_apis_getters )*
            }

            #( #runtime_apis_def )*
        }
    })
}
