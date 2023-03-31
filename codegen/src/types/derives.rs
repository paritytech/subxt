// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::CratePath;
use syn::{parse_quote, Path};

use std::collections::{HashMap, HashSet};

/// A struct containing the derives that we'll be applying to types;
/// a combination of some common derives for all types, plus type
/// specific derives.
#[derive(Debug, Clone)]
pub struct DerivesRegistry {
    default_derives: Derives,
    specific_type_derives: HashMap<syn::TypePath, Derives>,
}

impl DerivesRegistry {
    /// Creates a new `DerivesRegistry` with the supplied `crate_path`.
    ///
    /// The `crate_path` denotes the `subxt` crate access path in the
    /// generated code.
    pub fn new(crate_path: &CratePath) -> Self {
        Self {
            default_derives: Derives::new(crate_path),
            specific_type_derives: Default::default(),
        }
    }

    /// Insert derives to be applied to all generated types.
    pub fn extend_for_all(&mut self, derives: impl IntoIterator<Item = syn::Path>) {
        self.default_derives.derives.extend(derives)
    }

    /// Insert derives to be applied to a specific generated type.
    pub fn extend_for_type(
        &mut self,
        ty: syn::TypePath,
        derives: impl IntoIterator<Item = syn::Path>,
        crate_path: &CratePath,
    ) {
        let type_derives = self
            .specific_type_derives
            .entry(ty)
            .or_insert_with(|| Derives::new(crate_path));
        type_derives.derives.extend(derives)
    }

    /// Returns the derives to be applied to all generated types.
    pub fn default_derives(&self) -> &Derives {
        &self.default_derives
    }

    /// Resolve the derives for a generated type. Includes:
    ///     - The default derives for all types e.g. `scale::Encode, scale::Decode`
    ///     - Any user-defined derives for all types via `generated_type_derives`
    ///     - Any user-defined derives for this specific type
    pub fn resolve(&self, ty: &syn::TypePath) -> Derives {
        let mut resolved_derives = self.default_derives.clone();
        if let Some(specific) = self.specific_type_derives.get(ty) {
            resolved_derives.extend_from(specific.clone());
        }
        resolved_derives
    }
}

/// A struct storing the set of derives and derive attributes that we'll apply
/// to generated types.
#[derive(Debug, Clone)]
pub struct Derives {
    derives: HashSet<syn::Path>,
    attributes: HashSet<syn::Attribute>,
}

impl FromIterator<syn::Path> for Derives {
    fn from_iter<T: IntoIterator<Item = Path>>(iter: T) -> Self {
        let derives = iter.into_iter().collect();
        Self {
            derives,
            attributes: HashSet::new(),
        }
    }
}

impl Derives {
    /// Creates a new instance of `Derives` with the `crate_path` prepended
    /// to the set of default derives that reside in `subxt`.
    pub fn new(crate_path: &CratePath) -> Self {
        let mut derives = HashSet::new();
        let mut attributes = HashSet::new();

        derives.insert(syn::parse_quote!(#crate_path::ext::scale_encode::EncodeAsType));
        let encode_crate_path = quote::quote! { #crate_path::ext::scale_encode }.to_string();
        attributes.insert(syn::parse_quote!(#[encode_as_type(crate_path = #encode_crate_path)]));
        derives.insert(syn::parse_quote!(#crate_path::ext::scale_decode::DecodeAsType));
        let decode_crate_path = quote::quote! { #crate_path::ext::scale_decode }.to_string();
        attributes.insert(syn::parse_quote!(#[decode_as_type(crate_path = #decode_crate_path)]));

        derives.insert(syn::parse_quote!(#crate_path::ext::codec::Encode));
        derives.insert(syn::parse_quote!(#crate_path::ext::codec::Decode));
        derives.insert(syn::parse_quote!(Debug));

        Self {
            derives,
            attributes,
        }
    }

    /// Extend this set of `Derives` from another.
    pub fn extend_from(&mut self, other: Derives) {
        self.derives.extend(other.derives.into_iter());
        self.attributes.extend(other.attributes.into_iter());
    }

    /// Add `#crate_path::ext::codec::CompactAs` to the derives.
    pub fn insert_codec_compact_as(&mut self, crate_path: &CratePath) {
        self.insert_derive(parse_quote!(#crate_path::ext::codec::CompactAs));
    }

    /// Extend the set of derives by providing an iterator of paths to derive macros.
    pub fn extend(&mut self, derives: impl Iterator<Item = syn::Path>) {
        for derive in derives {
            self.insert_derive(derive)
        }
    }

    /// Insert a single derive.
    pub fn insert_derive(&mut self, derive: syn::Path) {
        self.derives.insert(derive);
    }

    /// Insert a single attribute to be applied to types.
    pub fn insert_attribute(&mut self, attribute: syn::Attribute) {
        self.attributes.insert(attribute);
    }
}

impl quote::ToTokens for Derives {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.derives.is_empty() {
            let mut sorted = self.derives.iter().cloned().collect::<Vec<_>>();
            sorted.sort_by(|a, b| {
                quote::quote!(#a)
                    .to_string()
                    .cmp(&quote::quote!(#b).to_string())
            });

            tokens.extend(quote::quote! {
                #[derive(#( #sorted ),*)]
            })
        }
        if !self.attributes.is_empty() {
            let mut sorted = self.attributes.iter().cloned().collect::<Vec<_>>();
            sorted.sort_by(|a, b| {
                quote::quote!(#a)
                    .to_string()
                    .cmp(&quote::quote!(#b).to_string())
            });

            tokens.extend(quote::quote! {
                #( #sorted )*
            })
        }
    }
}
