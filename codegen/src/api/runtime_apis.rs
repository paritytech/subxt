// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::collections::HashSet;

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

        let mut unique_names = HashSet::new();
        let mut unique_aliases = HashSet::new();

        let inputs: Vec<_> = method.inputs().enumerate().map(|(idx, input)| {
            // These are method names, which can just be '_', but struct field names can't
            // just be an underscore, so fix any such names we find to work in structs.

            let mut name = input.name.trim_start_matches('_').to_string();
            if name.is_empty() {
                name = format!("_{}", idx);
            }
            while !unique_names.insert(name.clone()) {
                // Name is already used, append the index until it is unique.
                name = format!("{}_param{}", name, idx);
            }

            let mut alias = name.to_upper_camel_case();
            // Note: name is not empty.
            if alias.as_bytes()[0].is_ascii_digit() {
                alias = format!("Param{}", alias);
            }
            while !unique_aliases.insert(alias.clone()) {
                alias = format!("{}Param{}", alias, idx);
            }

            let (alias_name, name) = (format_ident!("{alias}"), format_ident!("{name}"));

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

                // Guard the `Output` name against collisions by placing it in a dedicated module.
                pub mod output {
                    use super::#types_mod_ident;
                    pub type Output = #output;
                }
            }
        );

        // From the method metadata generate a structure that holds
        // all parameter types. This structure is used with metadata
        // to encode parameters to the call via `encode_as_fields_to`.
        let derives = type_gen.default_derives();
        let struct_name = format_ident!("{}", method.name().to_upper_camel_case());
        let struct_input = quote!(
            #aliased_module

            #derives
            pub struct #struct_name {
                #( pub #struct_params, )*
            }
        );

        let Some(call_hash) = api.method_hash(method.name()) else {
            return Err(CodegenError::MissingRuntimeApiMetadata(
                trait_name_str.to_owned(),
                method_name_str.to_owned(),
            ))
        };

        let method = quote!(
            #docs
            pub fn #method_name(&self, #( #fn_params, )* ) -> #crate_path::runtime_api::Payload<types::#struct_name, types::#method_name::output::Output> {
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

        let structure = quote! {
            pub struct Test {
                pub foo: test::Foo,
                pub bar: test::Bar,
            }
        };
        let expected_alias = quote!(
            pub mod test {
                use super::runtime_types;
                pub type Foo = ::core::primitive::bool;
                pub type Bar = ::core::primitive::bool;
                pub mod output {
                    use super::runtime_types;
                    pub type Output = ::core::primitive::bool;
                }
            }
        );
        assert!(code.contains(&structure.to_string()));
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

        let structure = quote! {
            pub struct Test {
                pub a: test::A,
                pub a_param1: test::AParam1,
                pub a_param2: test::AParam2,
            }
        };
        let expected_alias = quote!(
            pub mod test {
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

        assert!(code.contains(&structure.to_string()));
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

        let structure = quote! {
            pub struct Test {
                pub _0: test::Param0,
                pub a: test::A,
                pub param_0: test::Param0Param2,
                pub _3: test::Param3,
                pub param_0_param_2: test::Param0Param2Param4,
            }
        };
        let expected_alias = quote!(
            pub mod test {
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

        assert!(code.contains(&structure.to_string()));
        assert!(code.contains(&expected_alias.to_string()));
    }
}
