// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::types::TypeGenerator;
use frame_metadata::{
    v14::RuntimeMetadataV14,
    PalletMetadata,
};
use heck::ToSnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{
    format_ident,
    quote,
};
use scale_info::form::PortableForm;

/// Generate constants from the provided pallet's metadata.
///
/// The function creates a new module named `constants` under the pallet's module.
/// ```ignore
/// pub mod PalletName {
///     pub mod constants {
///     ...
///     }
/// }
/// ```
///
/// The constants are exposed via the `ConstantsApi` wrapper.
///
/// Although the constants are defined in the provided static metadata, the API
/// ensures that the constants are returned from the runtime metadata of the node.
/// This ensures that if the node's constants change value, we'll always see the latest values.
///
/// # Arguments
///
/// - `metadata` - Runtime metadata from which the calls are generated.
/// - `type_gen` - The type generator containing all types defined by metadata
/// - `pallet` - Pallet metadata from which the calls are generated.
/// - `types_mod_ident` - The ident of the base module that we can use to access the generated types from.
pub fn generate_constants(
    metadata: &RuntimeMetadataV14,
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata<PortableForm>,
    types_mod_ident: &syn::Ident,
) -> TokenStream2 {
    // Early return if the pallet has no constants.
    if pallet.constants.is_empty() {
        return quote!()
    }
    let constants = &pallet.constants;

    let constant_fns = constants.iter().map(|constant| {
        let fn_name = format_ident!("{}", constant.name.to_snake_case());
        let pallet_name = &pallet.name;
        let constant_name = &constant.name;
        let constant_hash = subxt_metadata::get_constant_hash(metadata, pallet_name, constant_name)
            .unwrap_or_else(|_| abort_call_site!("Metadata information for the constant {}_{} could not be found", pallet_name, constant_name));

        let return_ty = type_gen.resolve_type_path(constant.ty.id(), &[]);
        let docs = &constant.docs;

        quote! {
            #( #[doc = #docs ] )*
            pub fn #fn_name(&self) -> ::core::result::Result<#return_ty, ::subxt::BasicError> {
                let locked_metadata = self.client.metadata();
                let metadata = locked_metadata.read();
                if metadata.constant_hash(#pallet_name, #constant_name)? == [#(#constant_hash,)*] {
                    let pallet = metadata.pallet(#pallet_name)?;
                    let constant = pallet.constant(#constant_name)?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                } else {
                    Err(::subxt::MetadataError::IncompatibleMetadata.into())
                }
            }
        }
    });

    quote! {
        pub mod constants {
            use super::#types_mod_ident;

            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }

            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }

                #(#constant_fns)*
            }
        }
    }
}
