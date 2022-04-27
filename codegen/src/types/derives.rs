// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use syn::{
    parse_quote,
    punctuated::Punctuated,
    Path,
};

use std::collections::{
    HashMap,
    HashSet,
};

#[derive(Debug, Default, Clone)]
pub struct DerivesRegistry {
    default_derives: GeneratedTypeDerives,
    specific_type_derives: HashMap<syn::TypePath, GeneratedTypeDerives>,
}

impl DerivesRegistry {
    /// Insert derives to be applied to a specific generated type.
    pub fn insert_for_type(
        &mut self,
        ty: syn::TypePath,
        derives: impl Iterator<Item = syn::Path>,
    ) {
        let type_derives = self
            .specific_type_derives
            .entry(ty)
            .or_insert_with(GeneratedTypeDerives::default);
        type_derives.derives.extend(derives)
    }

    /// Returns a the derives to be applied to all generated types.
    pub fn default_derives(&self) -> &GeneratedTypeDerives {
        &self.default_derives
    }

    /// Resolve the derives for a generated type. Includes:
    ///     - The default derives for all types e.g. `scale::Encode, scale::Decode`
    ///     - Any user-defined derives for all types via `generated_type_derives`
    ///     - Any user-defined derives for this specific type
    pub fn resolve(&self, ty: &syn::TypePath) -> GeneratedTypeDerives {
        let mut defaults = self.default_derives.derives.clone();
        if let Some(specific) = self.specific_type_derives.get(ty) {
            defaults.extend(specific.derives.iter().cloned());
        }
        GeneratedTypeDerives { derives: defaults }
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedTypeDerives {
    derives: HashSet<syn::Path>,
}

impl FromIterator<syn::Path> for GeneratedTypeDerives {
    fn from_iter<T: IntoIterator<Item = Path>>(iter: T) -> Self {
        let derives = iter.into_iter().collect();
        Self { derives }
    }
}

impl GeneratedTypeDerives {
    /// Add `::subxt::codec::CompactAs` to the derives.
    pub fn push_codec_compact_as(&mut self) {
        self.insert(parse_quote!(::subxt::codec::CompactAs));
    }

    pub fn append(&mut self, derives: impl Iterator<Item = syn::Path>) {
        for derive in derives {
            self.insert(derive)
        }
    }

    pub fn insert(&mut self, derive: syn::Path) {
        self.derives.insert(derive);
    }
}

impl Default for GeneratedTypeDerives {
    fn default() -> Self {
        let mut derives = HashSet::new();
        derives.insert(syn::parse_quote!(::subxt::codec::Encode));
        derives.insert(syn::parse_quote!(::subxt::codec::Decode));
        derives.insert(syn::parse_quote!(Debug));
        Self { derives }
    }
}

impl quote::ToTokens for GeneratedTypeDerives {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.derives.is_empty() {
            let derives: Punctuated<syn::Path, syn::Token![,]> =
                self.derives.iter().cloned().collect();
            tokens.extend(quote::quote! {
                #[derive(#derives)]
            })
        }
    }
}
