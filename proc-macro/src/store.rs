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
use heck::{
    CamelCase,
    SnakeCase,
};
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use syn::parse::{
    Parse,
    ParseStream,
};
use synstructure::Structure;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(returns);
}

#[derive(Debug)]
enum StoreAttr {
    Returns(utils::Attr<kw::returns, syn::Type>),
}

impl Parse for StoreAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self::Returns(input.parse()?))
    }
}

type StoreAttrs = utils::Attrs<StoreAttr>;

fn parse_returns_attr(attr: &syn::Attribute) -> Option<syn::Type> {
    let attrs: StoreAttrs = syn::parse2(attr.tokens.clone()).unwrap();
    attrs.attrs.into_iter().next().map(|attr| {
        let StoreAttr::Returns(attr) = attr;
        attr.value
    })
}

pub fn store(s: Structure) -> TokenStream {
    let subxt = utils::use_crate("substrate-subxt");
    let ident = &s.ast().ident;
    let generics = &s.ast().generics;
    let params = utils::type_params(generics);
    let module = utils::module_name(generics);
    let store_name = utils::ident_to_name(ident, "Store").to_camel_case();
    let store = format_ident!("{}", store_name.to_snake_case());
    let store_trait = format_ident!("{}StoreExt", store_name);
    let bindings = utils::bindings(&s);
    let fields = utils::fields(&bindings);
    let marker = utils::marker_field(&fields).unwrap_or_else(|| format_ident!("_"));
    let filtered_fields = utils::filter_fields(&fields, &marker);
    let args = utils::fields_to_args(&filtered_fields);
    let build_struct = utils::build_struct(ident, &fields);
    let ret = bindings
        .iter()
        .filter_map(|bi| bi.ast().attrs.iter().filter_map(parse_returns_attr).next())
        .next()
        .expect("#[store(returns = ..)] needs to be specified.");
    let store_ty = format_ident!(
        "{}",
        match filtered_fields.len() {
            0 => "plain",
            1 => "map",
            2 => "double_map",
            _ => panic!("invalid number of arguments"),
        }
    );
    let keys = filtered_fields
        .iter()
        .map(|(field, _)| quote!(&self.#field));

    quote! {
        impl#generics #subxt::Store<T> for #ident<#(#params),*> {
            const MODULE: &'static str = MODULE;
            const FIELD: &'static str = #store_name;
            type Returns = #ret;
            fn key(
                &self,
                metadata: &#subxt::Metadata,
            ) -> Result<#subxt::sp_core::storage::StorageKey, #subxt::MetadataError> {
                Ok(metadata
                    .module(Self::MODULE)?
                    .storage(Self::FIELD)?
                    .#store_ty()?
                    .key(#(#keys,)*))
            }
        }

        /// Store extension trait.
        pub trait #store_trait<T: #module> {
            /// Retrive the store element.
            fn #store<'a>(
                &'a self,
                #args
            ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<#ret, #subxt::Error>> + Send + 'a>>;
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
            ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<#ret, #subxt::Error>> + Send + 'a>> {
                let #marker = core::marker::PhantomData::<T>;
                Box::pin(self.fetch(#build_struct, None))
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
                ) -> Result<substrate_subxt::sp_core::storage::StorageKey, substrate_subxt::MetadataError> {
                    Ok(metadata
                        .module(Self::MODULE)?
                        .storage(Self::FIELD)?
                        .map()?
                        .key(&self.account_id,))
                }
            }

            /// Store extension trait.
            pub trait AccountStoreExt<T: Balances> {
                /// Retrive the store element.
                fn account<'a>(
                    &'a self,
                    account_id: &'a <T as System>::AccountId,
                ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<AccountData<T::Balance>, substrate_subxt::Error>> + Send + 'a>>;
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
                ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<AccountData<T::Balance>, substrate_subxt::Error>> + Send + 'a>>
                {
                    let _ = core::marker::PhantomData::<T>;
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
