// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

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
            pub fn #fn_name(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<#return_ty>> {
                ::subxt::constants::StaticConstantAddress::new(
                    #pallet_name,
                    #constant_name,
                    [#(#constant_hash,)*]
                )
            }
        }
    });

    quote! {
        pub mod constants {
            use super::#types_mod_ident;

            pub struct ConstantsApi;

            impl ConstantsApi {
                #(#constant_fns)*
            }
        }
    }
}
