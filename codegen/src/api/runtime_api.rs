// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    types::TypeGenerator,
    CratePath,
};
use frame_metadata::v15::{
    RuntimeMetadataV15,
    RuntimeApiMetadata,
};

use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
};
use scale_info::form::PortableForm;

/// Generates the accessor functions for the given trait.
fn generate_trait_api(
    trait_: &RuntimeApiMetadata<PortableForm>,
    type_gen: &TypeGenerator,
    types_mod_ident: &syn::Ident,
    crate_path: &CratePath,
) -> TokenStream2 {
    let trait_name = &trait_.name;
    let docs = &trait_.docs;

    const VEC_ACC: &'static str = "result";
    let vec_acc = format_ident!("{}", VEC_ACC);

    let methods: Vec<_> = trait_.methods.iter().map(|method| {
        let method_name = format_ident!("{}", &method.name);
        let func_name = format!("{}_{}", trait_name, method_name);
        let docs = &method.docs;

        let inputs: Vec<_> = method.inputs.iter().map(|input| {
            let name = format_ident!("{}", &input.name);
            let ty = type_gen.resolve_type_path(input.ty.id());

            let param = quote!(#name: #ty);
            let encoded = quote!(#name.encode_to(&mut #vec_acc));
            (param, encoded)
        }).collect();

        let params = inputs.iter().map(|(param, _)| param);
        let encoded = inputs.iter().map(|(_, encoded)| encoded);

        let method_target = format_ident!("{}_target", &method.name);
        let output = type_gen.resolve_type_path(method.output.id());

        quote!(
            #( #[doc = #docs ] )*
            pub fn #method_name( #( #params, )* ) -> #crate_path::runtime_api::RuntimeAPIPayload<#output> {
                let mut #vec_acc = Vec::new();
                #( #encoded; )*

                #crate_path::runtime_api::RuntimeAPIPayload::new(
                    #func_name,
                    #vec_acc,
                    [0; 32],
                )
            }

            pub type #method_target = #output;
        )
    }).collect();

    let trait_name = format_ident!("{}", &trait_.name);

    quote!(
        #( #[doc = #docs ] )*
        pub mod #trait_name {
            use super::root_mod;
            use super::#types_mod_ident;

            use #crate_path::ext::codec::Encode;

            #( #methods )*
        }
    )
}

/// Generate the runtime API.
pub fn generate_runtime_api(
    metadata: &RuntimeMetadataV15,
    type_gen: &TypeGenerator,
    types_mod_ident: &syn::Ident,
    crate_path: &CratePath,
) -> TokenStream2 {
    let runtime = &metadata.runtime;

    if runtime.is_empty() {
        return quote!()
    }

    let runtime_mods = runtime
        .iter()
        .map(|rt| generate_trait_api(rt, type_gen, &types_mod_ident, crate_path));

    quote! {
        pub mod runtime_api {
            use super::root_mod;
            use super::#types_mod_ident;

            #( #runtime_mods )*
        }
    }
}
