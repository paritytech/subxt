// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use heck::ToSnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use scale_typegen::typegen::ir::ToTokensWithSettings;
use scale_typegen::TypeGenerator;
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
/// - `type_gen` - [`scale_typegen::TypeGenerator`] that contains settings and all types from the runtime metadata.
/// - `pallet` - Pallet metadata from which the constants are generated.
/// - `crate_path` - The crate path under which the `subxt-core` crate is located, e.g. `::subxt::ext::subxt_core` when using subxt as a dependency.
pub fn generate_constants(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    crate_path: &syn::Path,
) -> Result<TokenStream2, CodegenError> {
    // Early return if the pallet has no constants.
    if pallet.constants().len() == 0 {
        return Ok(quote!());
    }

    let constant_fns = pallet
        .constants()
        .map(|constant| {
            let fn_name = format_ident!("{}", constant.name().to_snake_case());
            let pallet_name = pallet.name();
            let constant_name = constant.name();
            let Some(constant_hash) = pallet.constant_hash(constant_name) else {
                return Err(CodegenError::MissingConstantMetadata(
                    constant_name.into(),
                    pallet_name.into(),
                ));
            };

            let return_ty = type_gen
                .resolve_type_path(constant.ty())?
                .to_token_stream(type_gen.settings());
            let docs = constant.docs();
            let docs = type_gen
                .settings()
                .should_gen_docs
                .then_some(quote! { #( #[doc = #docs ] )* })
                .unwrap_or_default();

            Ok(quote! {
                #docs
                pub fn #fn_name(&self) -> #crate_path::constants::address::StaticAddress<#return_ty> {
                    #crate_path::constants::address::StaticAddress::new_static(
                        #pallet_name,
                        #constant_name,
                        [#(#constant_hash,)*]
                    )
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let types_mod_ident = type_gen.types_mod_ident();

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
