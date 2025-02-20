// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use scale_typegen::TypeGenerator;
use subxt_metadata::PalletMetadata;

use super::CodegenError;
use scale_typegen::typegen::ir::ToTokensWithSettings;

/// Generate error type alias from the provided pallet metadata.
pub fn generate_error_type_alias(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
) -> Result<TokenStream2, CodegenError> {
    let Some(error_ty) = pallet.error_ty_id() else {
        return Ok(quote!());
    };

    let error_type = type_gen
        .resolve_type_path(error_ty)?
        .to_token_stream(type_gen.settings());
    let error_ty = type_gen.resolve_type(error_ty)?;
    let docs = &error_ty.docs;
    let docs = type_gen
        .settings()
        .should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();
    Ok(quote! {
        #docs
        pub type Error = #error_type;
    })
}
