// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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
use proc_macro_error::abort;
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

fn parse_returns_attr(attr: &syn::Attribute) -> Option<(syn::Type, syn::Type, bool)> {
    let attrs: StoreAttrs = syn::parse2(attr.tokens.clone())
        .map_err(|err| abort!("{}", err))
        .unwrap();
    attrs.attrs.into_iter().next().map(|attr| {
        let StoreAttr::Returns(attr) = attr;
        let ty = attr.value;
        if let Some(inner) = utils::parse_option(&ty) {
            (ty, inner, false)
        } else {
            (ty.clone(), ty, true)
        }
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
    let store_iter = format_ident!("{}_iter", store_name.to_snake_case());
    let store_trait = format_ident!("{}StoreExt", store_name);
    let bindings = utils::bindings(&s);
    let fields = utils::fields(&bindings);
    let marker = utils::marker_field(&fields).unwrap_or_else(|| format_ident!("_"));
    let filtered_fields = utils::filter_fields(&fields, &marker);
    let args = utils::fields_to_args(&filtered_fields);
    let build_struct = utils::build_struct(ident, &fields);
    let (ret, store_ret, uses_default) = bindings
        .iter()
        .filter_map(|bi| bi.ast().attrs.iter().filter_map(parse_returns_attr).next())
        .next()
        .unwrap_or_else(|| {
            abort!(ident, "#[store(returns = ..)] needs to be specified.")
        });
    let fetch = if uses_default {
        quote!(fetch_or_default)
    } else {
        quote!(fetch)
    };
    let store_ty = format_ident!(
        "{}",
        match filtered_fields.len() {
            0 => "plain",
            1 => "map",
            2 => "double_map",
            _ => {
                abort!(
                    ident,
                    "Expected 0-2 fields but found {}",
                    filtered_fields.len()
                );
            }
        }
    );
    let keys = filtered_fields
        .iter()
        .map(|(field, _)| quote!(&self.#field));
    let key_iter = quote!(#subxt::KeyIter<T, #ident<#(#params),*>>);

    quote! {
        impl#generics #subxt::Store<T> for #ident<#(#params),*> {
            const MODULE: &'static str = MODULE;
            const FIELD: &'static str = #store_name;
            type Returns = #store_ret;

            fn prefix(
                metadata: &#subxt::Metadata,
            ) -> Result<#subxt::sp_core::storage::StorageKey, #subxt::MetadataError> {
                Ok(metadata
                    .module(Self::MODULE)?
                    .storage(Self::FIELD)?
                    .prefix())
            }

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
        #[async_trait::async_trait]
        pub trait #store_trait<T: #subxt::Runtime + #module> {
            /// Retrieve the store element.
            async fn #store<'a>(
                &'a self,
                #args
                hash: Option<T::Hash>,
            ) -> Result<#ret, #subxt::Error>;

            /// Iterate over the store element.
            async fn #store_iter<'a>(
                &'a self,
                hash: Option<T::Hash>,
            ) -> Result<#key_iter, #subxt::Error>;
        }

        #[async_trait::async_trait]
        impl<T: #subxt::Runtime + #module> #store_trait<T> for #subxt::Client<T> {
            async fn #store<'a>(
                &'a self,
                #args
                hash: Option<T::Hash>,
            ) -> Result<#ret, #subxt::Error> {
                let #marker = core::marker::PhantomData::<T>;
                self.#fetch(&#build_struct, hash).await
            }

            async fn #store_iter<'a>(
                &'a self,
                hash: Option<T::Hash>,
            ) -> Result<#key_iter, #subxt::Error> {
                self.iter(hash).await
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

                fn prefix(
                    metadata: &substrate_subxt::Metadata,
                ) -> Result<substrate_subxt::sp_core::storage::StorageKey, substrate_subxt::MetadataError> {
                    Ok(metadata
                        .module(Self::MODULE)?
                        .storage(Self::FIELD)?
                        .prefix())
                }

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
            #[async_trait::async_trait]
            pub trait AccountStoreExt<T: substrate_subxt::Runtime + Balances> {
                /// Retrieve the store element.
                async fn account<'a>(
                    &'a self,
                    account_id: &'a <T as System>::AccountId,
                    hash: Option<T::Hash>,
                ) -> Result<AccountData<T::Balance>, substrate_subxt::Error>;
                /// Iterate over the store element.
                async fn account_iter<'a>(
                    &'a self,
                    hash: Option<T::Hash>,
                ) -> Result<substrate_subxt::KeyIter<T, AccountStore<'a, T>>, substrate_subxt::Error>;
            }

            #[async_trait::async_trait]
            impl<T: substrate_subxt::Runtime + Balances> AccountStoreExt<T> for substrate_subxt::Client<T> {
                async fn account<'a>(
                    &'a self,
                    account_id: &'a <T as System>::AccountId,
                    hash: Option<T::Hash>,
                ) -> Result<AccountData<T::Balance>, substrate_subxt::Error>
                {
                    let _ = core::marker::PhantomData::<T>;
                    self.fetch_or_default(&AccountStore { account_id, }, hash).await
                }

                async fn account_iter<'a>(
                    &'a self,
                    hash: Option<T::Hash>,
                ) -> Result<substrate_subxt::KeyIter<T, AccountStore<'a, T>>, substrate_subxt::Error> {
                    self.iter(hash).await
                }
            }
        };
        let derive_input = syn::parse2(input).unwrap();
        let s = Structure::new(&derive_input);
        let result = store(s);
        utils::assert_proc_macro(result, expected);
    }
}
