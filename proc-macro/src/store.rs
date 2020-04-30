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

use crate::utils;
use heck::SnakeCase;
use proc_macro2::{
    TokenStream,
    TokenTree,
};
use quote::{
    format_ident,
    quote,
};
use syn::{
    parse::{
        Parse,
        ParseStream,
    },
    Token,
};
use synstructure::Structure;

struct Returns {
    returns: syn::Ident,
    _eq: Token![=],
    ty: syn::Type,
}

impl Parse for Returns {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Returns {
            returns: input.parse()?,
            _eq: input.parse()?,
            ty: input.parse()?,
        })
    }
}

fn parse_returns_attr(attr: &syn::Attribute) -> Option<syn::Type> {
    if let TokenTree::Group(group) = attr.tokens.clone().into_iter().next().unwrap() {
        if let Ok(Returns { returns, ty, .. }) = syn::parse2(group.stream()) {
            if returns.to_string() == "returns" {
                return Some(ty)
            }
        }
    }
    None
}

pub fn store(s: Structure) -> TokenStream {
    let subxt = utils::use_crate("substrate-subxt");
    let sp_core = utils::use_crate("sp-core");
    let ident = &s.ast().ident;
    let generics = &s.ast().generics;
    let params = utils::type_params(generics);
    let module = utils::module_name(generics);
    let store_name = ident.to_string().trim_end_matches("Store").to_string();
    let store = format_ident!("{}", store_name.to_snake_case());
    let store_trait = format_ident!("{}StoreExt", store_name);
    let bindings = utils::bindings(&s);
    let fields = bindings
        .iter()
        .enumerate()
        .map(|(i, bi)| {
            (
                bi.ast()
                    .ident
                    .clone()
                    .unwrap_or_else(|| format_ident!("key{}", i)),
                bi.ast().ty.clone(),
            )
        })
        .collect::<Vec<_>>();
    let ret = bindings
        .iter()
        .filter_map(|bi| bi.ast().attrs.iter().filter_map(parse_returns_attr).next())
        .next()
        .expect("#[store(returns = ..)] needs to be specified.");
    let store_ty = format_ident!(
        "{}",
        match fields.len() {
            0 => "plain",
            1 => "map",
            2 => "double_map",
            _ => panic!("invalid number of arguments"),
        }
    );
    let args = fields.iter().map(|(field, ty)| quote!(#field: #ty,));
    let args = quote!(#(#args)*);
    let keys = fields.iter().map(|(field, _)| quote!(&self.#field,));
    let keys = quote!(#(#keys)*);
    let fields = fields.iter().map(|(field, _)| quote!(#field,));
    let fields = quote!(#(#fields)*);

    quote! {
        impl#generics #subxt::Store<T> for #ident<#(#params),*> {
            const MODULE: &'static str = MODULE;
            const FIELD: &'static str = #store_name;
            type Returns = #ret;
            fn key(
                &self,
                metadata: &#subxt::Metadata,
            ) -> Result<#sp_core::storage::StorageKey, #subxt::MetadataError> {
                Ok(metadata
                    .module(Self::MODULE)?
                    .storage(Self::FIELD)?
                    .#store_ty()?
                    .key(#keys))
            }
        }

        pub trait #store_trait<T: #module> {
            fn #store<'a>(
                &'a self,
                #args
            ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<Option<#ret>, #subxt::Error>> + Send + 'a>>;
        }

        impl<T, S, E> #store_trait<T> for #subxt::Client<T, S, E>
        where
            T: #module + Send + Sync,
            S: 'static,
            E: Send + Sync + 'static,
        {
            fn #store<'a>(
                &'a self,
                #args
            ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<Option<#ret>, #subxt::Error>> + Send + 'a>> {
                Box::pin(self.fetch(#ident { #fields }, None))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_store() {
        let input = quote! {
            #[derive(Encode, Store)]
            pub struct AccountStore<'a, T: Balances> {
                #[store(returns = AccountData<T::Balance>)]
                account_id: &'a <T as System>::AccountId,
            }
        };
        let expected = quote! {
            impl<'a, T: Balances> substrate_subxt::Store<T> for AccountStore<'a, T> {
                const MODULE: &'static str = MODULE;
                const FIELD: &'static str = "Account";
                type Returns = AccountData<T::Balance>;
                fn key(
                    &self,
                    metadata: &substrate_subxt::Metadata,
                ) -> Result<sp_core::storage::StorageKey, substrate_subxt::MetadataError> {
                    Ok(metadata
                        .module(Self::MODULE)?
                        .storage(Self::FIELD)?
                        .map()?
                        .key(&self.account_id,))
                }
            }

            pub trait AccountStoreExt<T: Balances> {
                fn account<'a>(
                    &'a self,
                    account_id: &'a <T as System>::AccountId,
                ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<Option<AccountData<T::Balance> >, substrate_subxt::Error>> + Send + 'a>>;
            }

            impl<T, S, E> AccountStoreExt<T> for substrate_subxt::Client<T, S, E>
            where
                T: Balances + Send + Sync,
                S: 'static,
                E: Send + Sync + 'static,
            {
                fn account<'a>(
                    &'a self,
                    account_id: &'a <T as System>::AccountId,
                ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<Option<AccountData<T::Balance> >, substrate_subxt::Error>> + Send + 'a>>
                {
                    Box::pin(self.fetch(AccountStore { account_id, }, None))
                }
            }
        };
        let derive_input = syn::parse2(input).unwrap();
        let s = Structure::new(&derive_input);
        let result = store(s);
        utils::assert_proc_macro(result, expected);
    }
}
