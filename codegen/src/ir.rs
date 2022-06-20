// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use proc_macro_error::abort;
use std::collections::HashMap;
use syn::{
    spanned::Spanned as _,
    token,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ItemMod {
    // attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    mod_token: token::Mod,
    pub ident: syn::Ident,
    brace: token::Brace,
    items: Vec<Item>,
}

impl From<syn::ItemMod> for ItemMod {
    fn from(module: syn::ItemMod) -> Self {
        let (brace, items) = match module.content {
            Some((brace, items)) => (brace, items),
            None => {
                abort!(module, "out-of-line subxt modules are not supported",)
            }
        };
        let items = items
            .into_iter()
            .map(<Item as From<syn::Item>>::from)
            .collect::<Vec<_>>();
        Self {
            vis: module.vis,
            mod_token: module.mod_token,
            ident: module.ident,
            brace,
            items,
        }
    }
}

impl ItemMod {
    pub fn type_substitutes(&self) -> HashMap<String, syn::TypePath> {
        self.items
            .iter()
            .filter_map(|item| {
                if let Item::Subxt(SubxtItem::TypeSubstitute {
                    generated_type_path,
                    substitute_with: substitute_type,
                }) = item
                {
                    Some((generated_type_path.clone(), substitute_type.clone()))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Rust(syn::Item),
    Subxt(SubxtItem),
}

impl From<syn::Item> for Item {
    fn from(item: syn::Item) -> Self {
        if let syn::Item::Use(ref use_) = item {
            let substitute_attrs = use_
                .attrs
                .iter()
                .map(|attr| {
                    let meta = attr.parse_meta().unwrap_or_else(|e| {
                        abort!(attr.span(), "Error parsing attribute: {}", e)
                    });
                    <attrs::Subxt as darling::FromMeta>::from_meta(&meta).unwrap_or_else(
                        |e| abort!(attr.span(), "Error parsing attribute meta: {}", e),
                    )
                })
                .collect::<Vec<_>>();
            if substitute_attrs.len() > 1 {
                abort!(
                    use_.attrs[0].span(),
                    "Duplicate `substitute_type` attributes"
                )
            }
            if let Some(attr) = substitute_attrs.get(0) {
                let use_path = &use_.tree;
                let substitute_with: syn::TypePath = syn::parse_quote!( #use_path );
                let type_substitute = SubxtItem::TypeSubstitute {
                    generated_type_path: attr.substitute_type(),
                    substitute_with,
                };
                Self::Subxt(type_substitute)
            } else {
                Self::Rust(item)
            }
        } else {
            Self::Rust(item)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubxtItem {
    TypeSubstitute {
        generated_type_path: String,
        substitute_with: syn::TypePath,
    },
}

mod attrs {
    use darling::FromMeta;

    #[derive(Debug, FromMeta)]
    #[darling(rename_all = "snake_case")]
    pub enum Subxt {
        SubstituteType(String),
    }

    impl Subxt {
        pub fn substitute_type(&self) -> String {
            match self {
                Self::SubstituteType(path) => path.clone(),
            }
        }
    }
}
