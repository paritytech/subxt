// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use proc_macro_error::abort;
use syn::token;

#[derive(Debug, PartialEq, Eq)]
pub struct ItemMod {
    vis: syn::Visibility,
    mod_token: token::Mod,
    pub ident: syn::Ident,
    brace: token::Brace,
    items: Vec<syn::Item>,
}

impl From<syn::ItemMod> for ItemMod {
    fn from(module: syn::ItemMod) -> Self {
        let (brace, items) = match module.content {
            Some((brace, items)) => (brace, items),
            None => {
                abort!(module, "out-of-line subxt modules are not supported",)
            }
        };

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
    pub fn rust_items(&self) -> impl Iterator<Item = &syn::Item> {
        self.items.iter()
    }
}
