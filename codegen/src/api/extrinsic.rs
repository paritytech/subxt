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

use crate::types::TypeGenerator;
use frame_metadata::ExtrinsicMetadata;
use heck::SnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{
    format_ident,
    quote,
};
use scale_info::form::PortableForm;

pub fn generate_extensions(
    type_gen: &TypeGenerator,
    extrinsic_metadata: &ExtrinsicMetadata<PortableForm>,
    types_mod_ident: &syn::Ident,
) -> TokenStream2 {

    let extensions = extrinsic_metadata
        .signed_extensions
        .iter()
        .map(|ext| {
            let id = &ext.identifier;
            let type_ident = format_ident!("{}", id);
            let extra_ty = type_gen.resolve_type_path(ext.ty.id(), &[]);
            let additional_signed_ty = type_gen.resolve_type_path(ext.additional_signed.id(), &[]);
            quote! {
                pub struct #type_ident {
                    pub extra: #extra_ty,
                    pub additional_signed: #additional_signed_ty,
                }

                // impl #type_ident {
                //     pub fn new(extra: #, additional_signed) -> Self {
                //
                //     }
                // }

                // todo: implement constructor for extra and additional signed data,
                // todo: possibly for use via a builder

                impl ::subxt::sp_runtime::traits::SignedExtension for #type_ident {
                    const IDENTIFIER: &'static str = #id;
                    type AccountId = ();
                    type Call = ();
                    type AdditionalSigned = #additional_signed_ty;
                    type Pre = ();
                    fn additional_signed(
                        &self,
                    ) -> Result<Self::AdditionalSigned, TransactionValidityError> {
                        Ok(self.additional_signed)
                    }
                }
            }
        });

    quote! {
        pub mod extensions {
            use super::#types_mod_ident;


        }
    }
}