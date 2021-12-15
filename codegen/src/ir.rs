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

use proc_macro_error::abort;
use std::collections::HashMap;
use syn::{spanned::Spanned as _, token};

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

    pub fn
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Rust(syn::Item),
    Subxt(SubxtItem),
}

impl From<syn::Item> for Item {
    fn from(item: syn::Item) -> Self {
        fn config_struct(attrs: &[syn::Attribute]) -> Option<bool> {
            let subxt_attrs = attrs::from_attrs(attrs);
            if attrs::check_for_duplicates(&subxt_attrs, |attr| matches!(attr, attrs::Subxt::Config)) {
                abort!(
                    attrs[0].span(),
                    "Duplicate `substitute_type` attributes"
                )
            }
            if let Some(attr) = subxt_attrs.get(0) {
                if let attrs::Subxt::Config = attr {
                    return true
                }
            }
            false
        }

        match &item {
            syn::Item::Use(ref use_) => {
                let substitute_attrs = attrs::from_attrs(&use_.attrs);
                if attrs::check_for_duplicates(&substitute_attrs, |attr| matches!(attr, attrs::Subxt::SubstituteType(_))) {
                    abort!(
                        use_.attrs[0].span(),
                        "Duplicate `substitute_type` attributes"
                    )
                }

                if let Some(attr) = substitute_attrs.get(0) {
                    if let attrs::Subxt::SubstituteType(path) = attr {
                        let use_path = &use_.tree;
                        let substitute_with: syn::TypePath = syn::parse_quote!( #use_path );
                        let type_substitute = SubxtItem::TypeSubstitute {
                            generated_type_path: path.clone(),
                            substitute_with,
                        };
                        return Self::Subxt(type_substitute)
                    }
                }
            }
            syn::Item::Struct(struct_) => {
                if config_struct(&struct_.attrs) {
                    if !struct_.fields.is_empty() {
                        abort!(
                            enum_.span(),
                            "Config type must be a struct with no fields"
                        )
                    }
                    let config = SubxtItemConfig { config_struct: struct_.clone(), generate_default_impls: }
                    return Self::Subxt(SubxtItem::Config(struct_.clone()))
                }
            }
            syn::Item::Enum(enum_) => {
                if config_struct(&enum_.attrs) {
                    abort!(
                        enum_.span(),
                        "Config type must be a struct with no fields"
                    )
                }
            }
            _ => ()
        }
        Self::Rust(item)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubxtItem {
    TypeSubstitute {
        generated_type_path: String,
        substitute_with: syn::TypePath,
    },
    Config(syn::ItemStruct),
}

mod attrs {
    use darling::FromMeta;
    use super::*;

    /// Parse the `#[subxt(..)]` attributes.
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Vec<Subxt> {
        attrs
            .iter()
            .map(Subxt::from)
            .collect()
    }

    /// Returns true if any duplicates matching the predicate found.
    pub fn check_for_duplicates<P: Fn(&Subxt) -> bool>(attrs: &[Subxt], predicate: P) -> bool {
        let matched = false;
        for attr in attrs {
            if matched && predicate(attr) {
                return true;
            }
        }
        return false
    }

    #[derive(Debug, FromMeta)]
    #[darling(rename_all = "snake_case")]
    pub enum Subxt {
        SubstituteType(String),
        Config,
    }

    impl From<&syn::Attribute> for Subxt {
        fn from(attr: &syn::Attribute) -> Self {
            let meta = attr.parse_meta().unwrap_or_else(|e| {
                abort!(attr.span(), "Error parsing attribute: {}", e)
            });
            <attrs::Subxt as darling::FromMeta>::from_meta(&meta).unwrap_or_else(
                |e| abort!(attr.span(), "Error parsing attribute meta: {}", e),
            )
        }
    }
}
