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

use scale_typegen::typegen::ir::type_ir::{CompositeFieldIR, CompositeIR, CompositeIRKind};
use scale_typegen::typegen::type_params::TypeParameters;
use scale_typegen::typegen::type_path::TypePath;
use scale_typegen::TypeGenerator;
use subxt_metadata::Metadata;
use syn::{parse_quote, Ident};

use crate::error::CodegenError;
use crate::subxt_type_gen_settings;
use crate::{api::custom_values::generate_custom_values, ir};

use heck::{ToSnakeCase as _, ToUpperCamelCase};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

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
        scale_typegen::utils::ensure_unique_type_paths(metadata.types_mut());
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

        let type_gen = TypeGenerator::new(self.metadata.types(), &settings);
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

        let type_gen = TypeGenerator::new(self.metadata.types(), &settings);
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

                let errors = errors::generate_error_type_alias(&type_gen, pallet)?;

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
        )?;

        // Fetch the paths of the outer enums.
        // Substrate exposes those under `kitchensink_runtime`, while Polkadot under `polkadot_runtime`.

        let call_path = type_gen.resolve_type_path(self.metadata.outer_enums().call_enum_ty())?;
        let event_path = type_gen.resolve_type_path(self.metadata.outer_enums().event_enum_ty())?;
        let error_path = type_gen.resolve_type_path(self.metadata.outer_enums().error_enum_ty())?;

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

/// Return a vector of tuples of variant names and corresponding struct definitions.
pub fn generate_structs_from_variants<F>(
    type_gen: &TypeGenerator,
    type_id: u32,
    variant_to_struct_name: F,
    error_message_type_name: &str,
) -> Result<Vec<StructFromVariant>, CodegenError>
where
    F: Fn(&str) -> std::borrow::Cow<str>,
{
    let ty = type_gen.resolve_type(type_id)?;

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
            let struct_name = variant_to_struct_name(&var.name);
            let mut composite = CompositeIR::new(
                syn::parse_str(&struct_name).expect("enum variant is a valid ident; qed"),
                composite_ir_kind,
                type_gen.docs_from_scale_info(&var.docs),
            );

            let type_alias_mod = generate_type_alias_mod(&mut composite, type_gen);
            Ok(StructFromVariant {
                variant_name: var.name.to_string(),
                composite,
                type_alias_mod,
            })
        })
        .collect()
}

pub struct StructFromVariant {
    variant_name: String,
    composite: CompositeIR,
    type_alias_mod: TokenStream2,
}

/// Modifies the composite, by replacing its types with references to the generated type alias module.
/// Returns the TokenStream of the type alias module.
///
/// E.g a struct like this:
/// ```ignore
/// pub struct SetMaxCodeSize {
///     pub new: ::core::primitive::u32,
/// }
/// ```
/// will be made into this:
/// ```ignore
/// pub struct SetMaxCodeSize {
///     pub new: set_max_code_size::New,
/// }
/// ```
/// And the type alias module will look like this:
/// ```ignore
/// pub mod set_max_code_size {
///     use super::runtime_types;
///     pub type New = ::core::primitive::u32;
/// }
/// ```
pub fn generate_type_alias_mod(
    composite: &mut CompositeIR,
    type_gen: &TypeGenerator,
) -> TokenStream2 {
    let mut aliases: Vec<TokenStream2> = vec![];
    let alias_mod_name: Ident = syn::parse_str(&composite.name.to_string().to_snake_case())
        .expect("composite name in snake_case should be a valid identifier");

    let mut modify_field_to_be_type_alias = |field: &mut CompositeFieldIR, alias_name: Ident| {
        let type_path = &field.type_path;
        aliases.push(quote!(pub type #alias_name = #type_path;));

        let type_alias_path: syn::Path = parse_quote!(#alias_mod_name::#alias_name);
        field.type_path = TypePath::from_syn_path(type_alias_path);
    };

    match &mut composite.kind {
        CompositeIRKind::NoFields => {
            return quote!(); // no types mod generated for unit structs.
        }
        CompositeIRKind::Named(named) => {
            for (name, field) in named.iter_mut() {
                let alias_name = format_ident!("{}", name.to_string().to_upper_camel_case());
                modify_field_to_be_type_alias(field, alias_name);
            }
        }
        CompositeIRKind::Unnamed(unnamed) => {
            for (i, field) in unnamed.iter_mut().enumerate() {
                let alias_name = format_ident!("Field{}", i);
                modify_field_to_be_type_alias(field, alias_name);
            }
        }
    };

    let types_mod_ident = type_gen.types_mod_ident();
    quote!(pub mod #alias_mod_name {
        use super::#types_mod_ident;
        #( #aliases )*
    })
}
