// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Generate code for submitting extrinsics and query storage of a Substrate runtime.

mod calls;
mod constants;
mod events;
mod storage;

use subxt_metadata::get_metadata_per_pallet_hash;

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
        FetchMetadataError,
        Uri,
    },
    CratePath,
};
use codec::Decode;
use frame_metadata::{
    v14::RuntimeMetadataV14,
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
};
use heck::ToSnakeCase as _;
use proc_macro2::{
    Span,
    TokenStream as TokenStream2,
};
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

/// Error returned when the Codegen cannot generate the runtime API.
#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    /// The given metadata type could not be found.
    #[error("Could not find type with ID {0} in the type registry; please raise a support issue.")]
    TypeNotFound(u32),
    /// Cannot fetch the metadata bytes.
    #[error("Failed to fetch metadata, make sure that you're pointing at a node which is providing V14 metadata: {0}")]
    Fetch(#[from] FetchMetadataError),
    /// Failed IO for the metadata file.
    #[error("Failed IO for {0}, make sure that you are providing the correct file path for metadata V14: {1}")]
    Io(String, std::io::Error),
    /// Cannot decode the metadata bytes.
    #[error("Could not decode metadata, only V14 metadata is supported: {0}")]
    Decode(#[from] codec::Error),
    /// Out of line modules are not supported.
    #[error("Out-of-line subxt modules are not supported, make sure you are providing a body to your module: pub mod polkadot {{ ... }}")]
    InvalidModule(Span),
    /// Expected named or unnamed fields.
    #[error("Fields should either be all named or all unnamed, make sure you are providing a valid metadata V14: {0}")]
    InvalidFields(String),
    /// Substitute types must have a valid path.
    #[error("Substitute types must have a valid path")]
    EmptySubstitutePath(Span),
    /// Invalid type path.
    #[error("Invalid type path {0}: {1}")]
    InvalidTypePath(String, syn::Error),
    /// Metadata for constant could not be found.
    #[error("Metadata for constant entry {0}_{1} could not be found. Make sure you are providing a valid metadata V14")]
    MissingConstantMetadata(String, String),
    /// Metadata for storage could not be found.
    #[error("Metadata for storage entry {0}_{1} could not be found. Make sure you are providing a valid metadata V14")]
    MissingStorageMetadata(String, String),
    /// Metadata for call could not be found.
    #[error("Metadata for call entry {0}_{1} could not be found. Make sure you are providing a valid metadata V14")]
    MissingCallMetadata(String, String),
    /// Call variant must have all named fields.
    #[error("Call variant for type {0} must have all named fields. Make sure you are providing a valid metadata V14")]
    InvalidCallVariant(u32),
    /// Type should be an variant/enum.
    #[error("{0} type should be an variant/enum type. Make sure you are providing a valid metadata V14")]
    InvalidType(String),
}

impl CodegenError {
    /// Render the error as an invocation of syn::compile_error!.
    pub fn into_compile_error(self) -> TokenStream2 {
        let msg = self.to_string();
        let span = match self {
            Self::InvalidModule(span) => span,
            Self::EmptySubstitutePath(span) => span,
            Self::InvalidTypePath(_, err) => err.span(),
            _ => proc_macro2::Span::call_site(),
        };
        syn::Error::new(span, msg).into_compile_error()
    }
}

/// Generates the API for interacting with a Substrate runtime.
///
/// # Arguments
///
/// * `item_mod` - The module declaration for which the API is implemented.
/// * `path` - The path to the scale encoded metadata of the runtime node.
/// * `derives` - Provide custom derives for the generated types.
/// * `type_substitutes` - Provide custom type substitutes.
/// * `crate_path` - Path to the `subxt` crate.
/// * `should_gen_docs` - True if the generated API contains the documentation from the metadata.
/// * `runtime_types_only` - Whether to limit code generation to only runtime types.
///
/// **Note:** This is a wrapper over [RuntimeGenerator] for static metadata use-cases.
pub fn generate_runtime_api_from_path<P>(
    item_mod: syn::ItemMod,
    path: P,
    derives: DerivesRegistry,
    type_substitutes: TypeSubstitutes,
    crate_path: CratePath,
    should_gen_docs: bool,
    runtime_types_only: bool,
) -> Result<TokenStream2, CodegenError>
where
    P: AsRef<path::Path>,
{
    let to_err = |err| CodegenError::Io(path.as_ref().to_string_lossy().into(), err);

    let mut file = fs::File::open(&path).map_err(to_err)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(to_err)?;

    generate_runtime_api_from_bytes(
        item_mod,
        &bytes,
        derives,
        type_substitutes,
        crate_path,
        should_gen_docs,
        runtime_types_only,
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
/// * `should_gen_docs` - True if the generated API contains the documentation from the metadata.
/// * `runtime_types_only` - Whether to limit code generation to only runtime types.
///
/// **Note:** This is a wrapper over [RuntimeGenerator] for static metadata use-cases.
pub fn generate_runtime_api_from_url(
    item_mod: syn::ItemMod,
    url: &Uri,
    derives: DerivesRegistry,
    type_substitutes: TypeSubstitutes,
    crate_path: CratePath,
    should_gen_docs: bool,
    runtime_types_only: bool,
) -> Result<TokenStream2, CodegenError> {
    let bytes = fetch_metadata_bytes_blocking(url)?;

    generate_runtime_api_from_bytes(
        item_mod,
        &bytes,
        derives,
        type_substitutes,
        crate_path,
        should_gen_docs,
        runtime_types_only,
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
/// * `should_gen_docs` - True if the generated API contains the documentation from the metadata.
/// * `runtime_types_only` - Whether to limit code generation to only runtime types.
///
/// **Note:** This is a wrapper over [RuntimeGenerator] for static metadata use-cases.
pub fn generate_runtime_api_from_bytes(
    item_mod: syn::ItemMod,
    bytes: &[u8],
    derives: DerivesRegistry,
    type_substitutes: TypeSubstitutes,
    crate_path: CratePath,
    should_gen_docs: bool,
    runtime_types_only: bool,
) -> Result<TokenStream2, CodegenError> {
    let metadata = frame_metadata::RuntimeMetadataPrefixed::decode(&mut &bytes[..])?;

    let generator = RuntimeGenerator::new(metadata);
    if runtime_types_only {
        generator.generate_runtime_types(
            item_mod,
            derives,
            type_substitutes,
            crate_path,
            should_gen_docs,
        )
    } else {
        generator.generate_runtime(
            item_mod,
            derives,
            type_substitutes,
            crate_path,
            should_gen_docs,
        )
    }
}

/// Create the API for interacting with a Substrate runtime.
pub struct RuntimeGenerator {
    metadata: RuntimeMetadataV14,
}

impl RuntimeGenerator {
    /// Create a new runtime generator from the provided metadata.
    ///
    /// **Note:** If you have the metadata path, URL or bytes to hand, prefer to use
    /// one of the `generate_runtime_api_from_*` functions for generating the runtime API
    /// from that.
    pub fn new(metadata: RuntimeMetadataPrefixed) -> Self {
        match metadata.1 {
            RuntimeMetadata::V14(v14) => Self { metadata: v14 },
            _ => panic!("Unsupported metadata version {:?}", metadata.1),
        }
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
        derives: DerivesRegistry,
        type_substitutes: TypeSubstitutes,
        crate_path: CratePath,
        should_gen_docs: bool,
    ) -> Result<TokenStream2, CodegenError> {
        let item_mod_attrs = item_mod.attrs.clone();

        let item_mod_ir = ir::ItemMod::try_from(item_mod)?;
        let mod_ident = &item_mod_ir.ident;
        let rust_items = item_mod_ir.rust_items();

        let type_gen = TypeGenerator::new(
            &self.metadata.types,
            "runtime_types",
            type_substitutes,
            derives,
            crate_path,
            should_gen_docs,
        );
        let types_mod = type_gen.generate_types_mod()?;

        Ok(quote! {
            #( #item_mod_attrs )*
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            #[allow(clippy::all)]
            pub mod #mod_ident {
                // Preserve any Rust items that were previously defined in the adorned module
                #( #rust_items ) *

                // Make it easy to access the root via `root_mod` at different levels:
                use super::#mod_ident as root_mod;
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
        derives: DerivesRegistry,
        type_substitutes: TypeSubstitutes,
        crate_path: CratePath,
        should_gen_docs: bool,
    ) -> Result<TokenStream2, CodegenError> {
        let item_mod_attrs = item_mod.attrs.clone();
        let item_mod_ir = ir::ItemMod::try_from(item_mod)?;

        let type_gen = TypeGenerator::new(
            &self.metadata.types,
            "runtime_types",
            type_substitutes,
            derives,
            crate_path.clone(),
            should_gen_docs,
        );
        let types_mod = type_gen.generate_types_mod()?;
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

        // Get the path to the `Runtime` struct. We assume that the same path contains
        // RuntimeCall and RuntimeEvent.
        let runtime_type_id = self.metadata.ty.id();
        let runtime_path_segments = self
            .metadata
            .types
            .resolve(runtime_type_id)
            .ok_or(CodegenError::TypeNotFound(runtime_type_id))?
            .path()
            .namespace()
            .iter()
            .map(|part| syn::PathSegment::from(format_ident!("{}", part)));
        let runtime_path_suffix = syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::from_iter(runtime_path_segments),
        };
        let runtime_path = if runtime_path_suffix.segments.is_empty() {
            quote!(#types_mod_ident)
        } else {
            quote!(#types_mod_ident::#runtime_path_suffix)
        };

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

        let modules = pallets_with_mod_names
            .iter()
            .map(|(pallet, mod_name)| {
                let calls = calls::generate_calls(
                    &self.metadata,
                    &type_gen,
                    pallet,
                    types_mod_ident,
                    &crate_path,
                    should_gen_docs,
                )?;

                let event = events::generate_events(
                    &type_gen,
                    pallet,
                    types_mod_ident,
                    &crate_path,
                    should_gen_docs,
                )?;

                let storage_mod = storage::generate_storage(
                    &self.metadata,
                    &type_gen,
                    pallet,
                    types_mod_ident,
                    &crate_path,
                    should_gen_docs,
                )?;

                let constants_mod = constants::generate_constants(
                    &self.metadata,
                    &type_gen,
                    pallet,
                    types_mod_ident,
                    &crate_path,
                    should_gen_docs,
                )?;

                Ok(quote! {
                    pub mod #mod_name {
                        use super::root_mod;
                        use super::#types_mod_ident;
                        #calls
                        #event
                        #storage_mod
                        #constants_mod
                    }
                })
            })
            .collect::<Result<Vec<_>, CodegenError>>()?;

        let root_event_if_arms = self.metadata.pallets.iter().filter_map(|p| {
            let variant_name_str = &p.name;
            let variant_name = format_ident!("{}", variant_name_str);
            let mod_name = format_ident!("{}", variant_name_str.to_string().to_snake_case());
            p.event.as_ref().map(|_| {
                // An 'if' arm for the RootEvent impl to match this variant name:
                quote! {
                    if pallet_name == #variant_name_str {
                        return Ok(Event::#variant_name(#mod_name::Event::decode_with_metadata(
                            &mut &*pallet_bytes,
                            pallet_ty,
                            metadata
                        )?));
                    }
                }
            })
        });

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

        Ok(quote! {
            #( #item_mod_attrs )*
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            #[allow(clippy::all)]
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

                /// The statically generated runtime call type.
                pub type Call = #runtime_path::RuntimeCall;

                /// The error type returned when there is a runtime issue.
                pub type DispatchError = #types_mod_ident::sp_runtime::DispatchError;

                // Make the runtime event type easily accessible, and impl RootEvent to help decode into it.
                pub type Event = #runtime_path::RuntimeEvent;

                impl #crate_path::events::RootEvent for Event {
                    fn root_event(pallet_bytes: &[u8], pallet_name: &str, pallet_ty: u32, metadata: &#crate_path::Metadata) -> Result<Self, #crate_path::Error> {
                        use #crate_path::metadata::DecodeWithMetadata;
                        #( #root_event_if_arms )*
                        Err(#crate_path::ext::scale_decode::Error::custom(format!("Pallet name '{}' not found in root Event enum", pallet_name)).into())
                    }
                }

                pub fn constants() -> ConstantsApi {
                    ConstantsApi
                }

                pub fn storage() -> StorageApi {
                    StorageApi
                }

                pub fn tx() -> TransactionApi {
                    TransactionApi
                }

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
                pub fn validate_codegen<T: #crate_path::Config, C: #crate_path::client::OfflineClientT<T>>(client: &C) -> Result<(), #crate_path::error::MetadataError> {
                    let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
                    if runtime_metadata_hash != [ #(#metadata_hash,)* ] {
                        Err(#crate_path::error::MetadataError::IncompatibleMetadata)
                    } else {
                        Ok(())
                    }
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
    crate_path: &CratePath,
    should_gen_docs: bool,
) -> Result<Vec<(String, CompositeDef)>, CodegenError>
where
    F: Fn(&str) -> std::borrow::Cow<str>,
{
    let ty = type_gen.resolve_type(type_id);

    let scale_info::TypeDef::Variant(variant) = ty.type_def() else {
        return Err(CodegenError::InvalidType(error_message_type_name.into()))
    };

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
            )?;

            let docs = should_gen_docs.then_some(var.docs()).unwrap_or_default();
            let struct_def = CompositeDef::struct_def(
                &ty,
                struct_name.as_ref(),
                Default::default(),
                fields,
                Some(parse_quote!(pub)),
                type_gen,
                docs,
                crate_path,
            )?;

            Ok((var.name.to_string(), struct_def))
        })
        .collect()
}
