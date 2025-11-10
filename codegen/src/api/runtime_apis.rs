// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::collections::HashSet;

use heck::ToSnakeCase as _;
use heck::ToUpperCamelCase as _;

use scale_typegen::TypeGenerator;
use scale_typegen::typegen::ir::ToTokensWithSettings;
use subxt_metadata::{Metadata, RuntimeApiMetadata};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use crate::CodegenError;

/// Generate the runtime APIs.
pub fn generate_runtime_apis(
    metadata: &Metadata,
    type_gen: &TypeGenerator,
    types_mod_ident: &syn::Ident,
    crate_path: &syn::Path,
) -> Result<TokenStream2, CodegenError> {
    let runtime_fns: Vec<_> = metadata
        .runtime_api_traits()
        .map(|api| generate_runtime_api(api, type_gen, crate_path))
        .collect::<Result<_, _>>()?;

    let trait_defs = runtime_fns.iter().map(|(apis, _)| apis);
    let trait_getters = runtime_fns.iter().map(|(_, getters)| getters);

    Ok(quote! {
        pub mod runtime_apis {
            use super::root_mod;
            use super::#types_mod_ident;

            use #crate_path::ext::codec::Encode;

            pub struct RuntimeApi;

            impl RuntimeApi {
                #( #trait_getters )*
            }

            #( #trait_defs )*
        }
    })
}

/// Generates runtime functions for the given API metadata.
fn generate_runtime_api(
    api: RuntimeApiMetadata,
    type_gen: &TypeGenerator,
    crate_path: &syn::Path,
) -> Result<(TokenStream2, TokenStream2), CodegenError> {
    let types_mod_ident = type_gen.types_mod_ident();
    // Trait name must remain as is (upper case) to identify the runtime call.
    let trait_name_str = api.name();
    // The snake case for the trait name.
    let trait_name_snake = format_ident!("{}", api.name().to_snake_case());

    let docs = api.docs();
    let docs: TokenStream2 = type_gen
        .settings()
        .should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();

    let types_and_methods = api
        .methods()
        .map(|method| {
            let method_name = format_ident!("{}", method.name());
            let method_name_str = method.name();
            let validation_hash = method.hash();

            let docs = method.docs();
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

            let runtime_api_inputs: Vec<Input> = {
                let mut unique_names = HashSet::new();
                let mut unique_aliases = HashSet::new();

                method
                    .inputs()
                    .enumerate()
                    .map(|(idx, input)| {
                        // The method argument name is either the input name or the
                        // index (eg _1, _2 etc) if one isn't provided.
                        // if we get unlucky we'll end up with param_param1 etc.
                        let mut name = input.name.trim_start_matches('_').to_string();
                        if name.is_empty() {
                            name = format!("_{idx}");
                        }
                        while !unique_names.insert(name.clone()) {
                            name = format!("{name}_param{idx}");
                        }

                        // The alias is either InputName if provided, or Param1, Param2 etc if not.
                        // If we get unlucky we may even end up with ParamParam1 etc.
                        let mut alias = name.trim_start_matches('_').to_upper_camel_case();
                        // Note: name is not empty.
                        if alias.as_bytes()[0].is_ascii_digit() {
                            alias = format!("Param{alias}");
                        }
                        while !unique_aliases.insert(alias.clone()) {
                            alias = format!("{alias}Param{idx}");
                        }

                        // Generate alias for runtime type.
                        let type_path = type_gen
                            .resolve_type_path(input.id)
                            .expect("runtime api input type is in metadata; qed")
                            .to_token_stream(type_gen.settings());

                        Input {
                            name: format_ident!("{name}"),
                            type_alias: format_ident!("{alias}"),
                            type_path,
                        }
                    })
                    .collect()
            };

            let input_tuple_types = runtime_api_inputs
                .iter()
                .map(|i| {
                    let ty = &i.type_alias;
                    quote!(#method_name::#ty)
                })
                .collect::<Vec<_>>();

            let input_args = runtime_api_inputs
                .iter()
                .map(|i| {
                    let arg = &i.name;
                    let ty = &i.type_alias;
                    quote!(#arg: #method_name::#ty)
                })
                .collect::<Vec<_>>();

            let input_param_names = runtime_api_inputs.iter().map(|i| &i.name);

            let input_type_aliases = runtime_api_inputs.iter().map(|i| {
                let ty = &i.type_alias;
                let path = &i.type_path;
                quote!(pub type #ty = #path;)
            });

            let output_type_path = type_gen
                .resolve_type_path(method.output_ty())?
                .to_token_stream(type_gen.settings());

            // Define the input and output type bits for the method.
            let runtime_api_types = quote! {
                pub mod #method_name {
                    use super::root_mod;
                    use super::#types_mod_ident;

                    #(#input_type_aliases)*

                    pub mod output {
                        use super::#types_mod_ident;
                        pub type Output = #output_type_path;
                    }
                }
            };

            // Define the getter method that will live on the `ViewFunctionApi` type.
            let runtime_api_method = quote!(
                #docs
                pub fn #method_name(
                    &self,
                    #(#input_args),*
                ) -> #crate_path::runtime_api::payload::StaticPayload<
                    (#(#input_tuple_types,)*),
                    #method_name::output::Output
                > {
                    #crate_path::runtime_api::payload::StaticPayload::new_static(
                        #trait_name_str,
                        #method_name_str,
                        (#(#input_param_names,)*),
                        [#(#validation_hash,)*],
                    )
                }
            );

            Ok((runtime_api_types, runtime_api_method))
        })
        .collect::<Result<Vec<_>, CodegenError>>()?;

    let trait_name = format_ident!("{}", trait_name_str);
    let types = types_and_methods.iter().map(|(types, _)| types);
    let methods = types_and_methods.iter().map(|(_, methods)| methods);

    // The runtime API definition and types.
    let trait_defs = quote!(
        pub mod #trait_name_snake {
            use super::root_mod;
            use super::#types_mod_ident;

            #docs
            pub struct #trait_name;

            impl #trait_name {
                #( #methods )*
            }

            #( #types )*
        }
    );

    // A getter for the `RuntimeApi` to get the trait structure.
    let trait_getter = quote!(
        pub fn #trait_name_snake(&self) -> #trait_name_snake::#trait_name {
            #trait_name_snake::#trait_name
        }
    );

    Ok((trait_defs, trait_getter))
}

#[cfg(test)]
mod tests {
    use crate::RuntimeGenerator;
    use frame_metadata::v15::{
        self, RuntimeApiMetadata, RuntimeApiMethodMetadata, RuntimeApiMethodParamMetadata,
    };
    use quote::quote;
    use scale_info::meta_type;
    use subxt_metadata::Metadata;

    fn metadata_with_runtime_apis(runtime_apis: Vec<RuntimeApiMetadata>) -> Metadata {
        let extrinsic_metadata = v15::ExtrinsicMetadata {
            version: 0,
            signed_extensions: vec![],
            address_ty: meta_type::<()>(),
            call_ty: meta_type::<()>(),
            signature_ty: meta_type::<()>(),
            extra_ty: meta_type::<()>(),
        };

        let metadata: Metadata = v15::RuntimeMetadataV15::new(
            vec![],
            extrinsic_metadata,
            meta_type::<()>(),
            runtime_apis,
            v15::OuterEnums {
                call_enum_ty: meta_type::<()>(),
                event_enum_ty: meta_type::<()>(),
                error_enum_ty: meta_type::<()>(),
            },
            v15::CustomMetadata {
                map: Default::default(),
            },
        )
        .try_into()
        .expect("can build valid metadata");
        metadata
    }

    fn generate_code(runtime_apis: Vec<RuntimeApiMetadata>) -> String {
        let metadata = metadata_with_runtime_apis(runtime_apis);
        let item_mod = syn::parse_quote!(
            pub mod api {}
        );
        let generator = RuntimeGenerator::new(metadata);
        let generated = generator
            .generate_runtime(
                item_mod,
                Default::default(),
                Default::default(),
                syn::parse_str("::subxt_path").unwrap(),
                false,
            )
            .expect("should be able to generate runtime");
        generated.to_string()
    }

    #[test]
    fn unique_param_names() {
        let runtime_apis = vec![RuntimeApiMetadata {
            name: "Test",
            methods: vec![RuntimeApiMethodMetadata {
                name: "test",
                inputs: vec![
                    RuntimeApiMethodParamMetadata {
                        name: "foo",
                        ty: meta_type::<bool>(),
                    },
                    RuntimeApiMethodParamMetadata {
                        name: "bar",
                        ty: meta_type::<bool>(),
                    },
                ],
                output: meta_type::<bool>(),
                docs: vec![],
            }],

            docs: vec![],
        }];

        let code = generate_code(runtime_apis);

        let expected_alias = quote!(
            pub mod test {
                use super::root_mod;
                use super::runtime_types;
                pub type Foo = ::core::primitive::bool;
                pub type Bar = ::core::primitive::bool;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::bool;
                }
            }
        );

        assert!(code.contains(&expected_alias.to_string()));
    }

    #[test]
    fn duplicate_param_names() {
        let runtime_apis = vec![RuntimeApiMetadata {
            name: "Test",
            methods: vec![RuntimeApiMethodMetadata {
                name: "test",
                inputs: vec![
                    RuntimeApiMethodParamMetadata {
                        name: "_a",
                        ty: meta_type::<bool>(),
                    },
                    RuntimeApiMethodParamMetadata {
                        name: "a",
                        ty: meta_type::<bool>(),
                    },
                    RuntimeApiMethodParamMetadata {
                        name: "__a",
                        ty: meta_type::<bool>(),
                    },
                ],
                output: meta_type::<bool>(),
                docs: vec![],
            }],

            docs: vec![],
        }];

        let code = generate_code(runtime_apis);

        let expected_alias = quote!(
            pub mod test {
                use super::root_mod;
                use super::runtime_types;
                pub type A = ::core::primitive::bool;
                pub type AParam1 = ::core::primitive::bool;
                pub type AParam2 = ::core::primitive::bool;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::bool;
                }
            }
        );

        assert!(code.contains(&expected_alias.to_string()));
    }

    #[test]
    fn duplicate_param_and_alias_names() {
        let runtime_apis = vec![RuntimeApiMetadata {
            name: "Test",
            methods: vec![RuntimeApiMethodMetadata {
                name: "test",
                inputs: vec![
                    RuntimeApiMethodParamMetadata {
                        name: "_",
                        ty: meta_type::<bool>(),
                    },
                    RuntimeApiMethodParamMetadata {
                        name: "_a",
                        ty: meta_type::<bool>(),
                    },
                    RuntimeApiMethodParamMetadata {
                        name: "_param_0",
                        ty: meta_type::<bool>(),
                    },
                    RuntimeApiMethodParamMetadata {
                        name: "__",
                        ty: meta_type::<bool>(),
                    },
                    RuntimeApiMethodParamMetadata {
                        name: "___param_0_param_2",
                        ty: meta_type::<bool>(),
                    },
                ],
                output: meta_type::<bool>(),
                docs: vec![],
            }],

            docs: vec![],
        }];

        let code = generate_code(runtime_apis);

        let expected_alias = quote!(
            pub mod test {
                use super::root_mod;
                use super::runtime_types;
                pub type Param0 = ::core::primitive::bool;
                pub type A = ::core::primitive::bool;
                pub type Param0Param2 = ::core::primitive::bool;
                pub type Param3 = ::core::primitive::bool;
                pub type Param0Param2Param4 = ::core::primitive::bool;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::bool;
                }
            }
        );

        assert!(code.contains(&expected_alias.to_string()));
    }
}
