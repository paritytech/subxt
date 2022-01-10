// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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

use crate::types::TypeGenerator;
use frame_metadata::{
    PalletCallMetadata,
    PalletMetadata,
};
use heck::SnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{
    format_ident,
    quote,
};
use scale_info::form::PortableForm;

pub fn generate_calls(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata<PortableForm>,
    call: &PalletCallMetadata<PortableForm>,
    types_mod_ident: &syn::Ident,
) -> TokenStream2 {
    let struct_defs =
        super::generate_structs_from_variants(type_gen, call.ty.id(), "Call");
    let (call_structs, call_fns): (Vec<_>, Vec<_>) = struct_defs
        .iter()
        .map(|struct_def| {
            let (call_fn_args, call_args): (Vec<_>, Vec<_>) = struct_def
                .named_fields()
                .unwrap_or_else(|| {
                    abort_call_site!(
                        "Call variant for type {} must have all named fields",
                        call.ty.id()
                    )
                })
                .iter()
                .map(|(name, ty)| (quote!( #name: #ty ), name))
                .unzip();

            let pallet_name = &pallet.name;
            let call_struct_name = &struct_def.name;
            let function_name = struct_def.name.to_string().to_snake_case();
            let fn_name = format_ident!("{}", function_name);

            let call_struct = quote! {
                #struct_def

                impl ::subxt::Call for #call_struct_name {
                    const PALLET: &'static str = #pallet_name;
                    const FUNCTION: &'static str = #function_name;
                }
            };
            let client_fn = quote! {
                pub fn #fn_name(
                    &self,
                    #( #call_fn_args, )*
                ) -> ::subxt::SubmittableExtrinsic<'a, T, E, A, #call_struct_name> {
                    let call = #call_struct_name { #( #call_args, )* };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            };
            (call_struct, client_fn)
        })
        .unzip();

    quote! {
        pub mod calls {
            use super::#types_mod_ident;
            #( #call_structs )*

            pub struct TransactionApi<'a, T: ::subxt::Config, E, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(E, A)>,
            }

            impl<'a, T, E, A> TransactionApi<'a, T, E, A>
            where
                T: ::subxt::Config,
                E: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }

                #( #call_fns )*
            }
        }
    }
}
