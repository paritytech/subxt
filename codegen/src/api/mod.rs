// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Generate code for submitting extrinsics and query storage of a Substrate runtime.

mod calls;
mod constants;
mod events;
mod runtime_api;
mod storage;

use scale_info::form::PortableForm;
use subxt_metadata::get_metadata_per_pallet_hash;

use self::runtime_api::generate_runtime_api;

use super::DerivesRegistry;
use crate::{
    ir,
    types::{
        CompositeDef,
        CompositeDefFields,
        TypeGenerator,
        TypeSubstitutes,
    },
    utils::{
        fetch_metadata_bytes_blocking,
        Uri,
    },
    CratePath,
};
use codec::Decode;
use frame_metadata::{
    v15::{
        RuntimeMetadataV15,
        TraitMetadata,
    },
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
};
use heck::ToSnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{
    format_ident,
    quote,
};
use std::{
    fs,
    io::Read,
    path,
    string::ToString,
};
use syn::parse_quote;

/// Generates the API for interacting with a Substrate runtime.
///
/// # Arguments
///
/// * `item_mod` - The module declaration for which the API is implemented.
/// * `path` - The path to the scale encoded metadata of the runtime node.
/// * `derives` - Provide custom derives for the generated types.
/// * `type_substitutes` - Provide custom type substitutes.
/// * `crate_path` - Path to the `subxt` crate.
///
/// **Note:** This is a wrapper over [RuntimeGenerator] for static metadata use-cases.
pub fn generate_runtime_api_from_path<P>(
    item_mod: syn::ItemMod,
    path: P,
    derives: DerivesRegistry,
    type_substitutes: TypeSubstitutes,
    crate_path: CratePath,
) -> TokenStream2
where
    P: AsRef<path::Path>,
{
    let mut file = fs::File::open(&path).unwrap_or_else(|e| {
        abort_call_site!("Failed to open {}: {}", path.as_ref().to_string_lossy(), e)
    });

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .unwrap_or_else(|e| abort_call_site!("Failed to read metadata file: {}", e));

    generate_runtime_api_from_bytes(
        item_mod,
        &bytes,
        derives,
        type_substitutes,
        crate_path,
    )
}

/// Generates the API for interacting with a substrate runtime, using metadata
/// that can be downloaded from a node at the provided URL. This function blocks
/// while retrieving the metadata.
///
/// # Arguments
///
/// * `item_mod` - The module declaration for which the API is implemented.
/// * `url` - HTTP/WS URL to the substrate node you'd like to pull metadata from.
/// * `derives` - Provide custom derives for the generated types.
/// * `type_substitutes` - Provide custom type substitutes.
/// * `crate_path` - Path to the `subxt` crate.
///
/// **Note:** This is a wrapper over [RuntimeGenerator] for static metadata use-cases.
pub fn generate_runtime_api_from_url(
    item_mod: syn::ItemMod,
    url: &Uri,
    derives: DerivesRegistry,
    type_substitutes: TypeSubstitutes,
    crate_path: CratePath,
) -> TokenStream2 {
    let bytes = fetch_metadata_bytes_blocking(url)
        .unwrap_or_else(|e| abort_call_site!("Failed to obtain metadata: {}", e));

    generate_runtime_api_from_bytes(
        item_mod,
        &bytes,
        derives,
        type_substitutes,
        crate_path,
    )
}

/// Generates the API for interacting with a substrate runtime, using metadata bytes.
///
/// # Arguments
///
/// * `item_mod` - The module declaration for which the API is implemented.
/// * `bytes` - The raw metadata bytes.
/// * `derives` - Provide custom derives for the generated types.
/// * `type_substitutes` - Provide custom type substitutes.
/// * `crate_path` - Path to the `subxt` crate.
///
/// **Note:** This is a wrapper over [RuntimeGenerator] for static metadata use-cases.
pub fn generate_runtime_api_from_bytes(
    item_mod: syn::ItemMod,
    bytes: &[u8],
    derives: DerivesRegistry,
    type_substitutes: TypeSubstitutes,
    crate_path: CratePath,
) -> TokenStream2 {
    let decoded: Option<frame_metadata::OpaqueMetadata> = Decode::decode(&mut &*bytes)
        .unwrap_or_else(|e| abort_call_site!("Failed to decode opaque metadata: {}", e));
    let decoded = decoded.unwrap();
    let bytes = &decoded.0;
    let metadata: RuntimeMetadataPrefixed = Decode::decode(&mut &bytes[..])
        .unwrap_or_else(|e| abort_call_site!("Failed to decode metadata: {}", e));

    let generator = RuntimeGenerator::new(metadata);
    generator.generate_runtime(item_mod, derives, type_substitutes, crate_path)
}

/// Create the API for interacting with a Substrate runtime.
pub struct RuntimeGenerator {
    metadata: RuntimeMetadataV15,
}

fn generate_runtime_call_api(
    type_gen: &TypeGenerator,
    runtime: &Vec<TraitMetadata<PortableForm>>,
) -> TokenStream2 {
    let mut result = quote!();

    for trait_ in runtime {
        for method in &trait_.methods {
            let trait_name = trait_.name.clone();
            let method_name = method.name.clone();

            let fn_name = format_ident!("{}_{}", trait_.name, method.name);

            let mut params = Vec::new();
            for input in &method.inputs {
                let name = &input.name;
                let ty = input.ty;
                let ty = type_gen.resolve_type_path(ty.id());
                let param = quote!( #name: #ty );
                params.push(param);
            }

            let output = method.output;
            let output = type_gen.resolve_type_path(output.id());

            let fn_ = quote!(
                pub fn #fn_name( #( #params, )* ) -> #output {
                    // call into RPC.
                }
            );

            result.extend(fn_);
        }
    }

    result
}

impl RuntimeGenerator {
    /// Create a new runtime generator from the provided metadata.
    ///
    /// **Note:** If you have the metadata path, URL or bytes to hand, prefer to use
    /// one of the `generate_runtime_api_from_*` functions for generating the runtime API
    /// from that.
    pub fn new(metadata: RuntimeMetadataPrefixed) -> Self {
        match metadata.1 {
            RuntimeMetadata::V15(v15) => Self { metadata: v15 },
            _ => panic!("Unsupported metadata version {:?}", metadata.1),
        }
    }

    /// Generate the API for interacting with a Substrate runtime.
    ///
    /// # Arguments
    ///
    /// * `item_mod` - The module declaration for which the API is implemented.
    /// * `derives` - Provide custom derives for the generated types.
    pub fn generate_runtime(
        &self,
        item_mod: syn::ItemMod,
        derives: DerivesRegistry,
        type_substitutes: TypeSubstitutes,
        crate_path: CratePath,
    ) -> TokenStream2 {
        let item_mod_attrs = item_mod.attrs.clone();
        let item_mod_ir = ir::ItemMod::from(item_mod);
        let default_derives = derives.default_derives();

        let type_gen = TypeGenerator::new(
            &self.metadata.types,
            "runtime_types",
            type_substitutes,
            derives.clone(),
            crate_path.clone(),
        );
        let runtime_api_fns =
            generate_runtime_call_api(&type_gen, &self.metadata.runtime);

        let types_mod = type_gen.generate_types_mod();
        let types_mod_ident = types_mod.ident();
        let pallets_with_mod_names = self
            .metadata
            .pallets
            .iter()
            .map(|pallet| {
                (
                    pallet,
                    format_ident!("{}", pallet.name.to_string().to_snake_case()),
                )
            })
            .collect::<Vec<_>>();

        // Generate the runtime API.
        let runtime_api =
            generate_runtime_api(&self.metadata, &type_gen, types_mod_ident, &crate_path);

        // Pallet names and their length are used to create PALLETS array.
        // The array is used to identify the pallets composing the metadata for
        // validation of just those pallets.
        let pallet_names: Vec<_> = self
            .metadata
            .pallets
            .iter()
            .map(|pallet| &pallet.name)
            .collect();
        let pallet_names_len = pallet_names.len();

        let metadata_hash = get_metadata_per_pallet_hash(&self.metadata, &pallet_names);

        let modules = pallets_with_mod_names.iter().map(|(pallet, mod_name)| {
            let calls = calls::generate_calls(
                &self.metadata,
                &type_gen,
                pallet,
                types_mod_ident,
                &crate_path,
            );

            let event =
                events::generate_events(&type_gen, pallet, types_mod_ident, &crate_path);

            let storage_mod = storage::generate_storage(
                &self.metadata,
                &type_gen,
                pallet,
                types_mod_ident,
                &crate_path,
            );

            let constants_mod = constants::generate_constants(
                &self.metadata,
                &type_gen,
                pallet,
                types_mod_ident,
                &crate_path,
            );

            quote! {
                pub mod #mod_name {
                    use super::root_mod;
                    use super::#types_mod_ident;
                    #calls
                    #event
                    #storage_mod
                    #constants_mod
                }
            }
        });

        let outer_event_variants = self.metadata.pallets.iter().filter_map(|p| {
            let variant_name = format_ident!("{}", p.name);
            let mod_name = format_ident!("{}", p.name.to_string().to_snake_case());
            let index = proc_macro2::Literal::u8_unsuffixed(p.index);

            p.event.as_ref().map(|_| {
                quote! {
                    #[codec(index = #index)]
                    #variant_name(#mod_name::Event),
                }
            })
        });

        let outer_event = quote! {
            #default_derives
            pub enum Event {
                #( #outer_event_variants )*
            }
        };

        let mod_ident = &item_mod_ir.ident;
        let pallets_with_constants: Vec<_> = pallets_with_mod_names
            .iter()
            .filter_map(|(pallet, pallet_mod_name)| {
                (!pallet.constants.is_empty()).then_some(pallet_mod_name)
            })
            .collect();

        let pallets_with_storage: Vec<_> = pallets_with_mod_names
            .iter()
            .filter_map(|(pallet, pallet_mod_name)| {
                pallet.storage.as_ref().map(|_| pallet_mod_name)
            })
            .collect();

        let pallets_with_calls: Vec<_> = pallets_with_mod_names
            .iter()
            .filter_map(|(pallet, pallet_mod_name)| {
                pallet.calls.as_ref().map(|_| pallet_mod_name)
            })
            .collect();

        let rust_items = item_mod_ir.rust_items();

        quote! {
            #( #item_mod_attrs )*
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            #[allow(clippy::all)]
            pub mod #mod_ident {
                // Preserve any Rust items that were previously defined in the adorned module
                #( #rust_items ) *

                // Make it easy to access the root via `root_mod` at different levels:
                use super::#mod_ident as root_mod;
                // Identify the pallets composing the static metadata by name.
                pub static PALLETS: [&str; #pallet_names_len] = [ #(#pallet_names,)* ];

                #outer_event
                #( #modules )*
                #types_mod

                /// The default error type returned when there is a runtime issue,
                /// exposed here for ease of use.
                pub type DispatchError = #types_mod_ident::sp_runtime::DispatchError;

                pub fn constants() -> ConstantsApi {
                    ConstantsApi
                }

                pub fn storage() -> StorageApi {
                    StorageApi
                }

                pub fn tx() -> TransactionApi {
                    TransactionApi
                }

                // pub mod runtime_api {
                //     #runtime_api_fns
                // }

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

                /// check whether the Client you are using is aligned with the statically generated codegen.
                pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(client: &C) -> Result<(), ::subxt::error::MetadataError> {
                    let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
                    if runtime_metadata_hash != [ #(#metadata_hash,)* ] {
                        Err(::subxt::error::MetadataError::IncompatibleMetadata)
                    } else {
                        Ok(())
                    }
                }

                /// Runtime API.
                #runtime_api
            }
        }
    }
}

/// Return a vector of tuples of variant names and corresponding struct definitions.
pub fn generate_structs_from_variants<'a, F>(
    type_gen: &'a TypeGenerator,
    type_id: u32,
    variant_to_struct_name: F,
    error_message_type_name: &str,
    crate_path: &CratePath,
) -> Vec<(String, CompositeDef)>
where
    F: Fn(&str) -> std::borrow::Cow<str>,
{
    let ty = type_gen.resolve_type(type_id);
    if let scale_info::TypeDef::Variant(variant) = ty.type_def() {
        variant
            .variants()
            .iter()
            .map(|var| {
                let struct_name = variant_to_struct_name(var.name());
                let fields = CompositeDefFields::from_scale_info_fields(
                    struct_name.as_ref(),
                    var.fields(),
                    &[],
                    type_gen,
                );
                let struct_def = CompositeDef::struct_def(
                    &ty,
                    struct_name.as_ref(),
                    Default::default(),
                    fields,
                    Some(parse_quote!(pub)),
                    type_gen,
                    var.docs(),
                    crate_path,
                );
                (var.name().to_string(), struct_def)
            })
            .collect()
    } else {
        abort_call_site!(
            "{} type should be an variant/enum type",
            error_message_type_name
        )
    }
}
