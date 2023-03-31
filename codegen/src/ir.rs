// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::CodegenError;
use syn::token;

#[derive(Debug, PartialEq, Eq)]
pub struct ItemMod {
    vis: syn::Visibility,
    mod_token: token::Mod,
    pub ident: syn::Ident,
    brace: token::Brace,
    items: Vec<syn::Item>,
}

impl TryFrom<syn::ItemMod> for ItemMod {
    type Error = CodegenError;

    fn try_from(module: syn::ItemMod) -> Result<Self, Self::Error> {
        let (brace, items) = match module.content {
            Some((brace, items)) => (brace, items),
            None => return Err(CodegenError::InvalidModule(module.ident.span())),
        };

        Ok(Self {
            vis: module.vis,
            mod_token: module.mod_token,
            ident: module.ident,
            brace,
            items,
        })
    }
}

impl ItemMod {
    pub fn rust_items(&self) -> impl Iterator<Item = &syn::Item> {
        self.items.iter()
    }
}
