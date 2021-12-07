// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

    pub fn with_codec_encode_decode() -> Self {
        let mut derives = Punctuated::new();
        derives.push(parse_quote!(::subxt::codec::Encode));
        derives.push(parse_quote!(::subxt::codec::Decode));
        Self::new(derives)
    }

    /// Add `::subxt::codec::CompactAs` to the derives, if either `Encode` or `Decode` are already
    /// derived.
    pub fn push_codec_compact_as(&mut self) {
        if self.derives.iter().any(|derive: &syn::Path| {
            derive == &parse_quote!(::subxt::codec::Encode)
                || derive == &parse_quote!(::subxt::codec::Decode)
        }) {
            self.derives.push(parse_quote!(::subxt::codec::CompactAs));
        }
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
