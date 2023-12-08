// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Generate a type safe Subxt interface for a Substrate runtime from its metadata.
//! This is used by the `#[subxt]` macro and `subxt codegen` CLI command, but can also
//! be used directly if preferable.

#![deny(unused_crate_dependencies, missing_docs)]

mod api;
pub mod error;
mod ir;

// These should probably be in a separate crate; they are used by the
// macro and CLI tool, so they only live here because this is a common
// crate that both depend on.
#[cfg(feature = "fetch-metadata")]
pub mod fetch_metadata;

#[cfg(feature = "web")]
use getrandom as _;

use api::RuntimeGenerator;
use proc_macro2::TokenStream as TokenStream2;
use scale_typegen::{
    typegen::settings::substitutes::absolute_path, DerivesRegistry, TypeGeneratorSettings,
    TypeSubstitutes, TypegenError,
};
use std::collections::HashMap;
use syn::parse_quote;

// Part of the public interface, so expose:
pub use error::CodegenError;
pub use subxt_metadata::Metadata;
pub use syn;

/// Generate a type safe interface to use with `subxt`.
/// The options exposed here are similar to those exposed via
/// the `#[subxt]` macro or via the `subxt codegen` CLI command.
/// Both use this under the hood.
///
/// # Example
///
/// Generating an interface using all of the defaults:
///
/// ```rust
/// use codec::Decode;
/// use subxt_codegen::{ Metadata, CodegenBuilder };
///
/// // Get hold of and decode some metadata:
/// let encoded = std::fs::read("../artifacts/polkadot_metadata_full.scale").unwrap();
/// let metadata = Metadata::decode(&mut &*encoded).unwrap();
///
/// // Generate a TokenStream representing the code for the interface.
/// // This can be converted to a string, displayed as-is or output from a macro.
/// let token_stream = CodegenBuilder::new().generate(metadata);
/// ````
pub struct CodegenBuilder {
    crate_path: syn::Path,
    use_default_derives: bool,
    use_default_substitutions: bool,
    generate_docs: bool,
    runtime_types_only: bool,
    item_mod: syn::ItemMod,
    extra_global_derives: Vec<syn::Path>,
    extra_global_attributes: Vec<syn::Attribute>,
    type_substitutes: HashMap<syn::Path, syn::Path>,
    derives_for_type: HashMap<syn::TypePath, Vec<syn::Path>>,
    attributes_for_type: HashMap<syn::TypePath, Vec<syn::Attribute>>,
    derives_for_type_recursive: HashMap<syn::TypePath, Vec<syn::Path>>,
    attributes_for_type_recursive: HashMap<syn::TypePath, Vec<syn::Attribute>>,
}

impl Default for CodegenBuilder {
    fn default() -> Self {
        CodegenBuilder {
            crate_path: syn::parse_quote!(::subxt),
            use_default_derives: true,
            use_default_substitutions: true,
            generate_docs: true,
            runtime_types_only: false,
            item_mod: syn::parse_quote!(
                pub mod api {}
            ),
            extra_global_derives: Vec::new(),
            extra_global_attributes: Vec::new(),
            type_substitutes: HashMap::new(),
            derives_for_type: HashMap::new(),
            attributes_for_type: HashMap::new(),
            derives_for_type_recursive: HashMap::new(),
            attributes_for_type_recursive: HashMap::new(),
        }
    }
}

impl CodegenBuilder {
    /// Construct a builder to configure and generate a type-safe interface for Subxt.
    pub fn new() -> Self {
        CodegenBuilder::default()
    }

    /// Disable the default derives that are applied to all types.
    ///
    /// # Warning
    ///
    /// This is not recommended, and is highly likely to break some part of the
    /// generated interface. Expect compile errors.
    pub fn disable_default_derives(&mut self) {
        self.use_default_derives = false;
    }

    /// Disable the default type substitutions that are applied to the generated
    /// code.
    ///
    /// # Warning
    ///
    /// This is not recommended, and is highly likely to break some part of the
    /// generated interface. Expect compile errors.
    pub fn disable_default_substitutes(&mut self) {
        self.use_default_substitutions = false;
    }

    /// Disable the output of doc comments associated with the generated types and
    /// methods. This can reduce the generated code size at the expense of losing
    /// documentation.
    pub fn no_docs(&mut self) {
        self.generate_docs = false;
    }

    /// Only generate the types, and don't generate the rest of the Subxt specific
    /// interface.
    pub fn runtime_types_only(&mut self) {
        self.runtime_types_only = true;
    }

    /// Set the additional derives that will be applied to all types. By default,
    /// a set of derives required for Subxt are automatically added for all types.
    ///
    /// # Warning
    ///
    /// Invalid derives, or derives that cannot be applied to _all_ of the generated
    /// types (taking into account that some types are substituted for hand written ones
    /// that we cannot add extra derives for) will lead to compile errors in the
    /// generated code.
    pub fn set_additional_global_derives(&mut self, derives: Vec<syn::Path>) {
        self.extra_global_derives = derives;
    }

    /// Set the additional attributes that will be applied to all types. By default,
    /// a set of attributes required for Subxt are automatically added for all types.
    ///
    /// # Warning
    ///
    /// Invalid attributes can very easily lead to compile errors in the generated code.
    pub fn set_additional_global_attributes(&mut self, attributes: Vec<syn::Attribute>) {
        self.extra_global_attributes = attributes;
    }

    /// Set additional derives for a specific type at the path given.
    ///
    /// # Warning
    ///
    /// For composite types, you may also need to set the same additional derives on all of
    /// the contained types as well to avoid compile errors in the generated code.
    pub fn add_derives_for_type(
        &mut self,
        ty: syn::TypePath,
        derives: impl IntoIterator<Item = syn::Path>,
        recursive: bool,
    ) {
        if recursive {
            self.derives_for_type_recursive
                .entry(ty)
                .or_default()
                .extend(derives);
        } else {
            self.derives_for_type.entry(ty).or_default().extend(derives);
        }
    }

    /// Set additional attributes for a specific type at the path given.
    ///
    /// # Warning
    ///
    /// For composite types, you may also need to consider contained types and whether they need
    /// similar attributes setting.
    pub fn add_attributes_for_type(
        &mut self,
        ty: syn::TypePath,
        attributes: impl IntoIterator<Item = syn::Attribute>,
        recursive: bool,
    ) {
        if recursive {
            self.attributes_for_type_recursive
                .entry(ty)
                .or_default()
                .extend(attributes);
        } else {
            self.attributes_for_type
                .entry(ty)
                .or_default()
                .extend(attributes);
        }
    }

    /// Substitute a type at the given path with some type at the second path. During codegen,
    /// we will avoid generating the type at the first path given, and instead point any references
    /// to that type to the second path given.
    ///
    /// The substituted type will need to implement the relevant traits to be compatible with the
    /// original, and it will need to SCALE encode and SCALE decode in a compatible way.
    pub fn set_type_substitute(&mut self, ty: syn::Path, with: syn::Path) {
        self.type_substitutes.insert(ty, with);
    }

    /// By default, all of the code is generated inside a module `pub mod api {}`. We decorate
    /// this module with a few attributes to reduce compile warnings and things. You can provide a
    /// target module here, allowing you to add additional attributes or inner code items (with the
    /// warning that duplicate identifiers will lead to compile errors).
    pub fn set_target_module(&mut self, item_mod: syn::ItemMod) {
        self.item_mod = item_mod;
    }

    /// Set the path to the `subxt` crate. By default, we expect it to be at `::subxt`.
    pub fn set_subxt_crate_path(&mut self, crate_path: syn::Path) {
        self.crate_path = crate_path;
    }

    /// Generate an interface, assuming that the default path to the `subxt` crate is `::subxt`.
    /// If the `subxt` crate is not available as a top level dependency, use `generate` and provide
    /// a valid path to the `subxt¦ crate.
    pub fn generate(self, metadata: Metadata) -> Result<TokenStream2, CodegenError> {
        let crate_path = self.crate_path;

        let mut derives_registry: DerivesRegistry = if self.use_default_derives {
            default_derives(&crate_path)
        } else {
            DerivesRegistry::new()
        };

        derives_registry.add_derives_for_all(self.extra_global_derives);
        derives_registry.add_attributes_for_all(self.extra_global_attributes);

        for (ty, derives) in self.derives_for_type {
            derives_registry.add_derives_for(ty, derives, false);
        }
        for (ty, derives) in self.derives_for_type_recursive {
            derives_registry.add_derives_for(ty, derives, true);
        }
        for (ty, attributes) in self.attributes_for_type {
            derives_registry.add_attributes_for(ty, attributes, false);
        }
        for (ty, attributes) in self.attributes_for_type_recursive {
            derives_registry.add_attributes_for(ty, attributes, true);
        }

        let mut type_substitutes: TypeSubstitutes = if self.use_default_substitutions {
            default_substitutes(&crate_path)
        } else {
            TypeSubstitutes::new()
        };

        for (from, with) in self.type_substitutes {
            let abs_path = absolute_path(with).map_err(TypegenError::from)?;
            type_substitutes
                .insert(from, abs_path)
                .map_err(TypegenError::from)?;
        }

        let item_mod = self.item_mod;
        let generator = RuntimeGenerator::new(metadata);
        let should_gen_docs = self.generate_docs;

        if self.runtime_types_only {
            generator.generate_runtime_types(
                item_mod,
                derives_registry,
                type_substitutes,
                crate_path,
                should_gen_docs,
            )
        } else {
            generator.generate_runtime(
                item_mod,
                derives_registry,
                type_substitutes,
                crate_path,
                should_gen_docs,
            )
        }
    }
}

/// The default [`scale_typegen::TypeGeneratorSettings`], subxt is using for generating code.
/// Useful for emulating subxt's code generation settings from e.g. subxt-explorer.
pub fn default_subxt_type_gen_settings() -> TypeGeneratorSettings {
    let crate_path: syn::Path = parse_quote!(::subxt);
    let derives = default_derives(&crate_path);
    let substitutes = default_substitutes(&crate_path);
    subxt_type_gen_settings(derives, substitutes, &crate_path, true)
}

fn subxt_type_gen_settings(
    derives: scale_typegen::DerivesRegistry,
    substitutes: scale_typegen::TypeSubstitutes,
    crate_path: &syn::Path,
    should_gen_docs: bool,
) -> TypeGeneratorSettings {
    TypeGeneratorSettings {
        types_mod_ident: parse_quote!(runtime_types),
        should_gen_docs,
        derives,
        substitutes,
        decoded_bits_type_path: Some(parse_quote!(#crate_path::utils::bits::DecodedBits)),
        compact_as_type_path: Some(parse_quote!(#crate_path::ext::codec::CompactAs)),
        compact_type_path: Some(parse_quote!(#crate_path::ext::codec::Compact)),
        insert_codec_attributes: true,
    }
}

fn default_derives(crate_path: &syn::Path) -> DerivesRegistry {
    let encode_crate_path = quote::quote! { #crate_path::ext::scale_encode }.to_string();
    let decode_crate_path = quote::quote! { #crate_path::ext::scale_decode }.to_string();

    let derives: [syn::Path; 5] = [
        parse_quote!(#crate_path::ext::scale_encode::EncodeAsType),
        parse_quote!(#crate_path::ext::scale_decode::DecodeAsType),
        parse_quote!(#crate_path::ext::codec::Encode),
        parse_quote!(#crate_path::ext::codec::Decode),
        parse_quote!(Debug),
    ];

    let attributes: [syn::Attribute; 3] = [
        parse_quote!(#[encode_as_type(crate_path = #encode_crate_path)]),
        parse_quote!(#[decode_as_type(crate_path = #decode_crate_path)]),
        parse_quote!(#[codec(crate = #crate_path::ext::codec)]),
    ];

    let mut derives_registry = DerivesRegistry::new();
    derives_registry.add_derives_for_all(derives);
    derives_registry.add_attributes_for_all(attributes);
    derives_registry
}

fn default_substitutes(crate_path: &syn::Path) -> TypeSubstitutes {
    let mut type_substitutes = TypeSubstitutes::new();

    let defaults: [(syn::Path, syn::Path); 11] = [
        (
            parse_quote!(bitvec::order::Lsb0),
            parse_quote!(#crate_path::utils::bits::Lsb0),
        ),
        (
            parse_quote!(bitvec::order::Msb0),
            parse_quote!(#crate_path::utils::bits::Msb0),
        ),
        (
            parse_quote!(sp_core::crypto::AccountId32),
            parse_quote!(#crate_path::utils::AccountId32),
        ),
        (
            parse_quote!(sp_runtime::multiaddress::MultiAddress),
            parse_quote!(#crate_path::utils::MultiAddress),
        ),
        (
            parse_quote!(primitive_types::H160),
            parse_quote!(#crate_path::utils::H160),
        ),
        (
            parse_quote!(primitive_types::H256),
            parse_quote!(#crate_path::utils::H256),
        ),
        (
            parse_quote!(primitive_types::H512),
            parse_quote!(#crate_path::utils::H512),
        ),
        (
            parse_quote!(frame_support::traits::misc::WrapperKeepOpaque),
            parse_quote!(#crate_path::utils::WrapperKeepOpaque),
        ),
        // BTreeMap and BTreeSet impose an `Ord` constraint on their key types. This
        // can cause an issue with generated code that doesn't impl `Ord` by default.
        // Decoding them to Vec by default (KeyedVec is just an alias for Vec with
        // suitable type params) avoids these issues.
        (
            parse_quote!(BTreeMap),
            parse_quote!(#crate_path::utils::KeyedVec),
        ),
        (parse_quote!(BTreeSet), parse_quote!(::std::vec::Vec)),
        // The `UncheckedExtrinsic(pub Vec<u8>)` is part of the runtime API calls.
        // The inner bytes represent the encoded extrinsic, however when deriving the
        // `EncodeAsType` the bytes would be re-encoded. This leads to the bytes
        // being altered by adding the length prefix in front of them.

        // Note: Not sure if this is appropriate or not. The most recent polkadot.rs file does not have these.
        (
            parse_quote!(sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic),
            parse_quote!(#crate_path::utils::UncheckedExtrinsic),
        ),
    ];

    let defaults = defaults.into_iter().map(|(from, to)| {
        (
            from,
            absolute_path(to).expect("default substitutes should be all absolute paths"),
        )
    });
    type_substitutes
        .extend(defaults)
        .expect("default substitutes should never error");
    type_substitutes
}
