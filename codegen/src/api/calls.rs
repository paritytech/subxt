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

use crate::types::{
    CompositeDefFields,
    TypeGenerator,
};
use frame_metadata::{
    PalletCallMetadata,
    PalletMetadata,
};
use heck::{
    ToSnakeCase as _,
    ToUpperCamelCase as _,
};
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
    let struct_defs = super::generate_structs_from_variants(
        type_gen,
        call.ty.id(),
        |name| name.to_upper_camel_case().into(),
        "Call",
    );
    let (call_structs, call_fns): (Vec<_>, Vec<_>) = struct_defs
        .iter()
        .map(|struct_def| {
            let (call_fn_args, call_args): (Vec<_>, Vec<_>) =
                match struct_def.fields {
                    CompositeDefFields::Named(ref named_fields) => {
                        named_fields
                            .iter()
                            .map(|(name, field)| {
                                let fn_arg_type = &field.type_path;
                                let call_arg = if field.is_boxed() {
                                    quote! { #name: ::std::boxed::Box::new(#name) }
                                } else {
                                    quote! { #name }
                                };
                                (quote!( #name: #fn_arg_type ), call_arg)
                            })
                            .unzip()
                    }
                    CompositeDefFields::NoFields => Default::default(),
                    CompositeDefFields::Unnamed(_) =>
                        abort_call_site!(
                            "Call variant for type {} must have all named fields",
                            call.ty.id()
                        )
                };

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
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, #call_struct_name, DispatchError, root_mod::Event> {
                    let call = #call_struct_name { #( #call_args, )* };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            };
            (call_struct, client_fn)
        })
        .unzip();

    quote! {
        pub mod calls {
            use super::root_mod;
            use super::#types_mod_ident;

            type DispatchError = #types_mod_ident::sp_runtime::DispatchError;

            #( #call_structs )*

            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }

            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }

                #( #call_fns )*
            }
        }
    }
}
