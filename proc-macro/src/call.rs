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
use quote::{
    format_ident,
    quote,
};
use synstructure::Structure;

pub fn call(s: Structure) -> TokenStream {
    let subxt = utils::use_crate("substrate-subxt");
    let ident = &s.ast().ident;
    let generics = &s.ast().generics;
    let params = utils::type_params(generics);
    let module = utils::module_name(generics);
    let call_name = utils::ident_to_name(ident, "Call").to_snake_case();
    let bindings = utils::bindings(&s);
    let fields = utils::fields(&bindings);
    let marker = utils::marker_field(&fields).unwrap_or_else(|| format_ident!("_"));
    let filtered_fields = utils::filter_fields(&fields, &marker);
    let args = utils::fields_to_args(&filtered_fields);
    let build_struct = utils::build_struct(ident, &fields);
    let call_trait = format_ident!("{}CallExt", call_name.to_camel_case());
    let call = format_ident!("{}", call_name);
    let call_and_watch = format_ident!("{}_and_watch", call_name);

    quote! {
        impl#generics #subxt::Call<T> for #ident<#(#params),*> {
            const MODULE: &'static str = MODULE;
            const FUNCTION: &'static str = #call_name;
        }

        /// Call extension trait.
        #[async_trait::async_trait]
        pub trait #call_trait<T: #subxt::Runtime + #module> {
            /// Create and submit an extrinsic.
            async fn #call<'a>(
                &'a self,
                signer: &'a (dyn #subxt::Signer<T> + Send + Sync),
                #args
            ) -> Result<T::Hash, #subxt::Error>;

            /// Create, submit and watch an extrinsic.
            async fn #call_and_watch<'a>(
                &'a self,
                signer: &'a (dyn #subxt::Signer<T> + Send + Sync),
                #args
            ) -> Result<#subxt::ExtrinsicSuccess<T>, #subxt::Error>;
        }

        #[async_trait::async_trait]
        impl<T: #subxt::Runtime + #module> #call_trait<T> for #subxt::Client<T>
        where
            <<T::Extra as #subxt::SignedExtra<T>>::Extra as #subxt::SignedExtension>::AdditionalSigned: Send + Sync,
        {
            async fn #call<'a>(
                &'a self,
                signer: &'a (dyn #subxt::Signer<T> + Send + Sync),
                #args
            ) -> Result<T::Hash, #subxt::Error> {
                let #marker = core::marker::PhantomData::<T>;
                self.submit(#build_struct, signer).await
            }

            async fn #call_and_watch<'a>(
                &'a self,
                signer: &'a (dyn #subxt::Signer<T> + Send + Sync),
                #args
            ) -> Result<#subxt::ExtrinsicSuccess<T>, #subxt::Error> {
                let #marker = core::marker::PhantomData::<T>;
                self.watch(#build_struct, signer).await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_call() {
        let input = quote! {
            #[derive(Call, Encode)]
            pub struct TransferCall<'a, T: Balances> {
                pub to: &'a <T as System>::Address,
                #[codec(compact)]
                pub amount: T::Balance,
            }
        };
        let expected = quote! {
            impl<'a, T: Balances> substrate_subxt::Call<T> for TransferCall<'a, T> {
                const MODULE: &'static str = MODULE;
                const FUNCTION: &'static str = "transfer";
            }

            /// Call extension trait.
            #[async_trait::async_trait]
            pub trait TransferCallExt<T: substrate_subxt::Runtime + Balances> {
                /// Create and submit an extrinsic.
                async fn transfer<'a>(
                    &'a self,
                    signer: &'a (dyn substrate_subxt::Signer<T> + Send + Sync),
                    to: &'a <T as System>::Address,
                    amount: T::Balance,
                ) -> Result<T::Hash, substrate_subxt::Error>;

                /// Create, submit and watch an extrinsic.
                async fn transfer_and_watch<'a>(
                    &'a self,
                    signer: &'a (dyn substrate_subxt::Signer<T> + Send + Sync),
                    to: &'a <T as System>::Address,
                    amount: T::Balance,
                ) -> Result<substrate_subxt::ExtrinsicSuccess<T>, substrate_subxt::Error>;
            }

            #[async_trait::async_trait]
            impl<T: substrate_subxt::Runtime + Balances> TransferCallExt<T> for substrate_subxt::Client<T>
            where
                <<T::Extra as substrate_subxt::SignedExtra<T>>::Extra as substrate_subxt::SignedExtension>::AdditionalSigned: Send + Sync,
            {
                async fn transfer<'a>(
                    &'a self,
                    signer: &'a (dyn substrate_subxt::Signer<T> + Send + Sync),
                    to: &'a <T as System>::Address,
                    amount: T::Balance,
                ) -> Result<T::Hash, substrate_subxt::Error> {
                    let _ = core::marker::PhantomData::<T>;
                    self.submit(TransferCall { to, amount, }, signer).await
                }

                async fn transfer_and_watch<'a>(
                    &'a self,
                    signer: &'a (dyn substrate_subxt::Signer<T> + Send + Sync),
                    to: &'a <T as System>::Address,
                    amount: T::Balance,
                ) -> Result<substrate_subxt::ExtrinsicSuccess<T>, substrate_subxt::Error> {
                    let _ = core::marker::PhantomData::<T>;
                    self.watch(TransferCall { to, amount, }, signer).await
                }
            }
        };
        let derive_input = syn::parse2(input).unwrap();
        let s = Structure::new(&derive_input);
        let result = call(s);
        utils::assert_proc_macro(result, expected);
    }
}
