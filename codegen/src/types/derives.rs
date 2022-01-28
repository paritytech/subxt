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
};

#[derive(Debug, Clone)]
pub struct GeneratedTypeDerives {
    derives: Punctuated<syn::Path, syn::Token![,]>,
}

impl GeneratedTypeDerives {
    pub fn new(derives: Punctuated<syn::Path, syn::Token!(,)>) -> Self {
        Self { derives }
    }

    /// Add `::subxt::codec::CompactAs` to the derives.
    pub fn push_codec_compact_as(&mut self) {
        self.derives.push(parse_quote!(::subxt::codec::CompactAs));
    }

    pub fn append(&mut self, derives: impl Iterator<Item = syn::Path>) {
        for derive in derives {
            self.derives.push(derive)
        }
    }

    pub fn push(&mut self, derive: syn::Path) {
        self.derives.push(derive);
    }
}

impl Default for GeneratedTypeDerives {
    fn default() -> Self {
        let mut derives = Punctuated::new();
        derives.push(syn::parse_quote!(::subxt::codec::Encode));
        derives.push(syn::parse_quote!(::subxt::codec::Decode));
        derives.push(syn::parse_quote!(Debug));
        Self::new(derives)
    }
}

impl quote::ToTokens for GeneratedTypeDerives {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.derives.is_empty() {
            let derives = &self.derives;
            tokens.extend(quote::quote! {
                #[derive(#derives)]
            })
        }
    }
}
