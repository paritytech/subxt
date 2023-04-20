// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{types::TypeGenerator, CodegenError, CratePath};
use frame_metadata::v15::{RuntimeApiMetadata, RuntimeMetadataV15};
use heck::ToSnakeCase as _;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use scale_info::form::PortableForm;

/// Generates runtime functions for the given API metadata.
fn generate_runtime_api(
    metadata: &RuntimeMetadataV15,
    api: &RuntimeApiMetadata<PortableForm>,
    type_gen: &TypeGenerator,
    crate_path: &CratePath,
) -> Result<(TokenStream2, TokenStream2), CodegenError> {
    // Trait name must remain as is (upper case) to identity the runtime call.
    let trait_name = &api.name;
    // The snake case for the trait name.
    let trait_name_snake = format_ident!("{}", api.name.to_snake_case());
    let docs = &api.docs;

    // Parameters are scale encoded into the `result` vector.
    const ENCODED_PARAMS: &str = "result";
    let encoded_params = format_ident!("{}", ENCODED_PARAMS);

    let methods: Vec<_> = api.methods.iter().map(|method| {
        let method_name = format_ident!("{}", method.name);

        // Runtime function name is `TraitName_MethodName`.
        let runtime_fn_name = format!("{}_{}", trait_name, method_name);
        let docs = &method.docs;

        let inputs: Vec<_> = method.inputs.iter().map(|input| {
            let name = format_ident!("{}", &input.name);
            let ty = type_gen.resolve_type_path(input.ty.id);

            let param = quote!(#name: #ty);
            let encoded = quote!(#name.encode_to(&mut #encoded_params));
            (param, encoded)
        }).collect();

        // The `let [mut] result = Vec::new()` accumulator needs to be
        // mutable only if we have inputs to add to it.
        let encoded_mut = if inputs.is_empty() {
            quote!()
        } else {
            quote!(mut)
        };

        let params = inputs.iter().map(|(param, _)| param);
        let encoded = inputs.iter().map(|(_, encoded)| encoded);

        let output = type_gen.resolve_type_path(method.output.id);

        let Ok(call_hash) =
            subxt_metadata::get_runtime_api_hash(metadata, trait_name, &method.name) else {
                return Err(CodegenError::MissingRuntimeApiMetadata(
                    trait_name.into(),
                    method.name.clone(),
                ))
            };

        Ok(quote!(
            #( #[doc = #docs ] )*
            pub fn #method_name(&self, #( #params, )* ) -> #crate_path::runtime_api::StaticRuntimeApiPayload<#output> {
                let #encoded_mut #encoded_params = Vec::new();
                #( #encoded; )*

                #crate_path::runtime_api::StaticRuntimeApiPayload::new_static(
                    #runtime_fn_name,
                    #encoded_params,
                    [#(#call_hash,)*],
                )
            }
        ))
    }).collect::<Result<_, _>>()?;

    let trait_name = format_ident!("{}", trait_name);
    // The structure that identifies the `TraitName` of the runtime call.
    let runtime_api = quote!(
        #( #[doc = #docs ] )*
        pub struct #trait_name;

        impl #trait_name {
            #( #methods )*
        }
    );

    // A getter for the `RuntimeApi` to get the trait structure.
    let trait_getter = quote!(
        pub fn #trait_name_snake(&self) -> #trait_name {
            #trait_name
        }
    );

    Ok((runtime_api, trait_getter))
}

/// Generate the runtime APIs.
pub fn generate_runtime_apis(
    metadata: &RuntimeMetadataV15,
    type_gen: &TypeGenerator,
    types_mod_ident: &syn::Ident,
    crate_path: &CratePath,
) -> Result<TokenStream2, CodegenError> {
    let apis = &metadata.apis;

    let runtime_fns: Vec<_> = apis
        .iter()
        .map(|api| generate_runtime_api(metadata, api, type_gen, crate_path))
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
