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
    v14::RuntimeMetadataV14,
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
    metadata: &RuntimeMetadataV14,
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata<PortableForm>,
    call: &PalletCallMetadata<PortableForm>,
    types_mod_ident: &syn::Ident,
) -> TokenStream2 {
    let mut struct_defs = super::generate_structs_from_variants(
        type_gen,
        call.ty.id(),
        |name| name.to_upper_camel_case().into(),
        "Call",
    );
    let (call_structs, call_fns): (Vec<_>, Vec<_>) = struct_defs
        .iter_mut()
        .map(|(variant_name, struct_def)| {
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
            let call_name = &variant_name;
            let struct_name = &struct_def.name;
            let call_hash = subxt_metadata::get_call_hash(metadata, pallet_name, call_name)
                .unwrap_or_else(|_| abort_call_site!("Metadata information for the call {}_{} could not be found", pallet_name, call_name));

            let fn_name = format_ident!("{}", variant_name.to_snake_case());
            // Propagate the documentation just to `TransactionApi` methods, while
            // draining the documentation of inner call structures.
            let docs = struct_def.docs.take();
            // The call structure's documentation was stripped above.
            let call_struct = quote! {
                #struct_def

                impl ::subxt::Call for #struct_name {
                    const PALLET: &'static str = #pallet_name;
                    const FUNCTION: &'static str = #call_name;
                }
            };
            let client_fn = quote! {
                #docs
                pub fn #fn_name(
                    &self,
                    #( #call_fn_args, )*
                ) -> Result<::subxt::SubmittableExtrinsic<'a, T, X, #struct_name, DispatchError, root_mod::Event>, ::subxt::BasicError> {
                    let runtime_call_hash = {
                        let locked_metadata = self.client.metadata();
                        let metadata = locked_metadata.read();
                        metadata.call_hash::<#struct_name>()?
                    };
                    if runtime_call_hash == [#(#call_hash,)*] {
                        let call = #struct_name { #( #call_args, )* };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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

            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }

            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }

                #( #call_fns )*
            }
        }
    }
}
