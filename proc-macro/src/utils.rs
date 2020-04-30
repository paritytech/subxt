// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use proc_macro2::{
    Span,
    TokenStream,
};
use quote::quote;
use synstructure::{
    BindingInfo,
    Structure,
};

pub fn use_crate(name: &str) -> syn::Ident {
    let krate = proc_macro_crate::crate_name(name).unwrap();
    syn::Ident::new(&krate, Span::call_site())
}

pub fn bindings<'a>(s: &'a Structure) -> Vec<&'a BindingInfo<'a>> {
    let mut bindings = vec![];
    for variant in s.variants() {
        for binding in variant.bindings() {
            bindings.push(binding);
        }
    }
    bindings
}

pub fn module_name(generics: &syn::Generics) -> &syn::Path {
    generics
        .params
        .iter()
        .filter_map(|p| {
            if let syn::GenericParam::Type(p) = p {
                p.bounds
                    .iter()
                    .filter_map(|b| {
                        if let syn::TypeParamBound::Trait(t) = b {
                            Some(&t.path)
                        } else {
                            None
                        }
                    })
                    .next()
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

pub fn path_to_ident(path: &syn::Path) -> &syn::Ident {
    &path.segments.iter().last().unwrap().ident
}

pub fn type_params(generics: &syn::Generics) -> Vec<TokenStream> {
    generics
        .params
        .iter()
        .filter_map(|g| {
            match g {
                syn::GenericParam::Type(p) => {
                    let ident = &p.ident;
                    Some(quote!(#ident))
                }
                syn::GenericParam::Lifetime(p) => {
                    let lifetime = &p.lifetime;
                    Some(quote!(#lifetime))
                }
                syn::GenericParam::Const(_) => None,
            }
        })
        .collect()
}

#[cfg(test)]
pub(crate) fn assert_proc_macro(
    result: proc_macro2::TokenStream,
    expected: proc_macro2::TokenStream,
) {
    let result = result.to_string();
    let expected = expected.to_string();
    pretty_assertions::assert_eq!(result, expected);
}
