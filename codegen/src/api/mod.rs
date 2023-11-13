// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Generate code for submitting extrinsics and query storage of a Substrate runtime.

mod calls;
mod constants;
mod custom_values;
mod errors;
mod events;
mod runtime_apis;
mod storage;

use scale_typegen::typegen::ir::type_ir::{CompositeIR, TypeIR, TypeIRKind};
use scale_typegen::typegen::type_params::TypeParameters;
use scale_typegen::{Derives, TypeGenerator, TypeGeneratorSettings, TypePathResolver};
use subxt_metadata::Metadata;

use crate::error::CodegenError;
use crate::{api::custom_values::generate_custom_values, ir};

use heck::ToSnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse_quote;

/// Create the API for interacting with a Substrate runtime.
pub struct RuntimeGenerator {
    metadata: Metadata,
}

impl RuntimeGenerator {
    /// Create a new runtime generator from the provided metadata.
    ///
    /// **Note:** If you have the metadata path, URL or bytes to hand, prefer to use
    /// `GenerateRuntimeApi` for generating the runtime API from that.
    ///
    /// # Panics
    ///
    /// Panics if the runtime metadata version is not supported.
    ///
    /// Supported versions: v14 and v15.
    pub fn new(mut metadata: Metadata) -> Self {
        metadata.ensure_unique_type_paths();
        RuntimeGenerator { metadata }
    }

    /// Generate the API for interacting with a Substrate runtime.
    ///
    /// # Arguments
    ///
    /// * `item_mod` - The module declaration for which the API is implemented.
    /// * `derives` - Provide custom derives for the generated types.
    /// * `type_substitutes` - Provide custom type substitutes.
    /// * `crate_path` - Path to the `subxt` crate.
    /// * `should_gen_docs` - True if the generated API contains the documentation from the metadata.
    pub fn generate_runtime_types(
        &self,
        item_mod: syn::ItemMod,
        derives: scale_typegen::DerivesRegistry,
        type_substitutes: scale_typegen::TypeSubstitutes,
        crate_path: syn::Path,
        should_gen_docs: bool,
    ) -> Result<TokenStream2, CodegenError> {
        let item_mod_attrs = item_mod.attrs.clone();
        let item_mod_ir = ir::ItemMod::try_from(item_mod)?;

        let settings =
            subxt_type_gen_settings(derives, type_substitutes, &crate_path, should_gen_docs);

        let type_gen = TypeGenerator::new(self.metadata.types(), settings)?;
        let types_mod = type_gen.generate_types_mod()?;
        let mod_ident = &item_mod_ir.ident;
        let rust_items = item_mod_ir.rust_items();

        Ok(quote! {
            #( #item_mod_attrs )*
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            #[allow(clippy::all)]
            #[allow(rustdoc::broken_intra_doc_links)]
            pub mod #mod_ident {
                // Preserve any Rust items that were previously defined in the adorned module
                #( #rust_items ) *

                // Make it easy to access the root items via `root_mod` at different levels
                // without reaching out of this module.
                #[allow(unused_imports)]
                mod root_mod {
                    pub use super::*;
                }

                #types_mod
            }
        })
    }

    /// Generate the API for interacting with a Substrate runtime.
    ///
    /// # Arguments
    ///
    /// * `item_mod` - The module declaration for which the API is implemented.
    /// * `derives` - Provide custom derives for the generated types.
    /// * `type_substitutes` - Provide custom type substitutes.
    /// * `crate_path` - Path to the `subxt` crate.
    /// * `should_gen_docs` - True if the generated API contains the documentation from the metadata.
    pub fn generate_runtime(
        &self,
        item_mod: syn::ItemMod,
        derives: scale_typegen::DerivesRegistry,
        type_substitutes: scale_typegen::TypeSubstitutes,
        crate_path: syn::Path,
        should_gen_docs: bool,
    ) -> Result<TokenStream2, CodegenError> {
        let item_mod_attrs = item_mod.attrs.clone();
        let item_mod_ir = ir::ItemMod::try_from(item_mod)?;

        let settings =
            subxt_type_gen_settings(derives, type_substitutes, &crate_path, should_gen_docs);

        let type_gen = TypeGenerator::new(self.metadata.types(), settings)?;
        let types_mod = type_gen.generate_types_mod()?;
        let types_mod_ident = type_gen.types_mod_ident();
        let pallets_with_mod_names = self
            .metadata
            .pallets()
            .map(|pallet| {
                (
                    pallet,
                    format_ident!("{}", pallet.name().to_string().to_snake_case()),
                )
            })
            .collect::<Vec<_>>();

        // Pallet names and their length are used to create PALLETS array.
        // The array is used to identify the pallets composing the metadata for
        // validation of just those pallets.
        let pallet_names: Vec<_> = self
            .metadata
            .pallets()
            .map(|pallet| pallet.name())
            .collect();
        let pallet_names_len = pallet_names.len();

        let runtime_api_names: Vec<_> = self
            .metadata
            .runtime_api_traits()
            .map(|api| api.name().to_string())
            .collect();
        let runtime_api_names_len = runtime_api_names.len();

        let metadata_hash = self.metadata.hasher().hash();

        let modules = pallets_with_mod_names
            .iter()
            .map(|(pallet, mod_name)| {
                let calls = calls::generate_calls(&type_gen, pallet, &crate_path)?;

                let event = events::generate_events(&type_gen, pallet, &crate_path)?;

                let storage_mod = storage::generate_storage(&type_gen, pallet, &crate_path)?;

                let constants_mod = constants::generate_constants(&type_gen, pallet, &crate_path)?;

                let errors = errors::generate_error_type_alias(&type_gen, pallet, should_gen_docs)?;

                Ok(quote! {
                    pub mod #mod_name {
                        use super::root_mod;
                        use super::#types_mod_ident;
                        #errors
                        #calls
                        #event
                        #storage_mod
                        #constants_mod
                    }
                })
            })
            .collect::<Result<Vec<_>, CodegenError>>()?;

        let mod_ident = &item_mod_ir.ident;
        let pallets_with_constants: Vec<_> = pallets_with_mod_names
            .iter()
            .filter_map(|(pallet, pallet_mod_name)| {
                pallet
                    .constants()
                    .next()
                    .is_some()
                    .then_some(pallet_mod_name)
            })
            .collect();

        let pallets_with_storage: Vec<_> = pallets_with_mod_names
            .iter()
            .filter_map(|(pallet, pallet_mod_name)| pallet.storage().map(|_| pallet_mod_name))
            .collect();

        let pallets_with_calls: Vec<_> = pallets_with_mod_names
            .iter()
            .filter_map(|(pallet, pallet_mod_name)| pallet.call_ty_id().map(|_| pallet_mod_name))
            .collect();

        let rust_items = item_mod_ir.rust_items();

        let apis_mod = runtime_apis::generate_runtime_apis(
            &self.metadata,
            &type_gen,
            types_mod_ident,
            &crate_path,
            should_gen_docs,
        )?;

        // Fetch the paths of the outer enums.
        // Substrate exposes those under `kitchensink_runtime`, while Polkadot under `polkadot_runtime`.

        let type_path_resolver = type_gen.type_path_resolver();
        let call_path =
            type_path_resolver.resolve_type_path(self.metadata.outer_enums().call_enum_ty())?;
        let event_path =
            type_path_resolver.resolve_type_path(self.metadata.outer_enums().event_enum_ty())?;
        let error_path =
            type_path_resolver.resolve_type_path(self.metadata.outer_enums().error_enum_ty())?;

        let custom_values = generate_custom_values(&self.metadata, &type_gen, &crate_path);

        Ok(quote! {
            #( #item_mod_attrs )*
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            #[allow(clippy::all)]
            #[allow(rustdoc::broken_intra_doc_links)]
            pub mod #mod_ident {
                // Preserve any Rust items that were previously defined in the adorned module.
                #( #rust_items ) *

                // Make it easy to access the root items via `root_mod` at different levels
                // without reaching out of this module.
                #[allow(unused_imports)]
                mod root_mod {
                    pub use super::*;
                }

                // Identify the pallets composing the static metadata by name.
                pub static PALLETS: [&str; #pallet_names_len] = [ #(#pallet_names,)* ];

                // Runtime APIs in the metadata by name.
                pub static RUNTIME_APIS: [&str; #runtime_api_names_len] = [ #(#runtime_api_names,)* ];

                /// The error type returned when there is a runtime issue.
                pub type DispatchError = #types_mod_ident::sp_runtime::DispatchError;

                /// The outer event enum.
                pub type Event = #event_path;

                /// The outer extrinsic enum.
                pub type Call = #call_path;

                /// The outer error enum representing the DispatchError's Module variant.
                pub type Error = #error_path;

                pub fn constants() -> ConstantsApi {
                    ConstantsApi
                }

                pub fn storage() -> StorageApi {
                    StorageApi
                }

                pub fn tx() -> TransactionApi {
                    TransactionApi
                }

                pub fn apis() -> runtime_apis::RuntimeApi {
                    runtime_apis::RuntimeApi
                }

                #apis_mod

                pub fn custom() -> CustomValuesApi {
                    CustomValuesApi
                }

                #custom_values

                pub struct ConstantsApi;
                impl ConstantsApi {
                    #(
                        pub fn #pallets_with_constants(&self) -> #pallets_with_constants::constants::ConstantsApi {
                            #pallets_with_constants::constants::ConstantsApi
                        }
                    )*
                }

                pub struct StorageApi;
                impl StorageApi {
                    #(
                        pub fn #pallets_with_storage(&self) -> #pallets_with_storage::storage::StorageApi {
                            #pallets_with_storage::storage::StorageApi
                        }
                    )*
                }

                pub struct TransactionApi;
                impl TransactionApi {
                    #(
                        pub fn #pallets_with_calls(&self) -> #pallets_with_calls::calls::TransactionApi {
                            #pallets_with_calls::calls::TransactionApi
                        }
                    )*
                }

                /// check whether the metadata provided is aligned with this statically generated code.
                pub fn is_codegen_valid_for(metadata: &#crate_path::Metadata) -> bool {
                    let runtime_metadata_hash = metadata
                        .hasher()
                        .only_these_pallets(&PALLETS)
                        .only_these_runtime_apis(&RUNTIME_APIS)
                        .hash();
                    runtime_metadata_hash == [ #(#metadata_hash,)* ]
                }

                #( #modules )*
                #types_mod
            }
        })
    }
}

fn subxt_type_gen_settings(
    derives: scale_typegen::DerivesRegistry,
    type_substitutes: scale_typegen::TypeSubstitutes,
    crate_path: &syn::Path,
    should_gen_docs: bool,
) -> TypeGeneratorSettings {
    let mut settings = TypeGeneratorSettings::default()
        .should_gen_docs(should_gen_docs)
        .decoded_bits_type_path(parse_quote!(#crate_path::utils::bits::DecodedBits));
    settings.derives = derives;
    settings.substitutes = type_substitutes;
    settings
}

/// Return a vector of tuples of variant names and corresponding struct definitions.
pub fn generate_structs_from_variants<F>(
    type_gen: &TypeGenerator,
    type_id: u32,
    variant_to_struct_name: F,
    error_message_type_name: &str,
    crate_path: &syn::Path,
) -> Result<Vec<(String, CompositeIR)>, CodegenError>
where
    F: Fn(&str) -> std::borrow::Cow<str>,
{
    let ty = type_gen.type_path_resolver().resolve_type(type_id)?;

    let scale_info::TypeDef::Variant(variant) = &ty.type_def else {
        return Err(CodegenError::InvalidType(error_message_type_name.into()));
    };

    variant
        .variants
        .iter()
        .map(|var| {
            let mut type_params = TypeParameters::from_scale_info(&[]);
            let composite_ir_kind =
                type_gen.create_composite_ir_kind(&var.fields, &mut type_params)?;
            // let fields = CompositeDefFields::from_scale_info_fields(
            //     struct_name.as_ref(),
            //     &var.fields,
            //     &[],
            //     type_gen,
            // )?;

            let struct_name = variant_to_struct_name(&var.name);
            let composite_ir = CompositeIR::new(
                syn::parse_str(&struct_name).expect("enum variant name is valid ident"),
                composite_ir_kind,
                type_gen.docs_from_scale_info(&var.docs),
            );

            // let docs = should_gen_docs.then_some(&*var.docs).unwrap_or_default();
            // let struct_def = CompositeDef::struct_def(
            //     &ty,
            //     struct_name.as_ref(),
            //     Default::default(),
            //     fields,
            //     Some(parse_quote!(pub)),
            //     type_gen,
            //     docs,
            //     crate_path,
            // )?;

            Ok((var.name.to_string(), composite_ir))
        })
        .collect()
}
