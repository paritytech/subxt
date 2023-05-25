// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{types::TypeGenerator, CratePath};
use heck::ToSnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use subxt_metadata::PalletMetadata;

use super::CodegenError;

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
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    types_mod_ident: &syn::Ident,
    crate_path: &CratePath,
    should_gen_docs: bool,
) -> Result<TokenStream2, CodegenError> {
    // Early return if the pallet has no constants.
    if pallet.constants().len() == 0 {
        return Ok(quote!());
    }

    let constant_fns = pallet.constants().map(|constant| {
        let fn_name = format_ident!("{}", constant.name().to_snake_case());
        let pallet_name = pallet.name();
        let constant_name = constant.name();
        let Some(constant_hash) = pallet.constant_hash(constant_name) else {
            return Err(CodegenError::MissingConstantMetadata(constant_name.into(), pallet_name.into()));
        };

        let return_ty = type_gen.resolve_type_path(constant.ty());
        let docs = constant.docs();
        let docs = should_gen_docs
            .then_some(quote! { #( #[doc = #docs ] )* })
            .unwrap_or_default();

        Ok(quote! {
            #docs
            pub fn #fn_name(&self) -> #crate_path::constants::Address<#return_ty> {
                #crate_path::constants::Address::new_static(
                    #pallet_name,
                    #constant_name,
                    [#(#constant_hash,)*]
                )
            }
        })
    }).collect::<Result<Vec<_>, _>>()?;

    Ok(quote! {
        pub mod constants {
            use super::#types_mod_ident;

            pub struct ConstantsApi;

            impl ConstantsApi {
                #(#constant_fns)*
            }
        }
    })
}
