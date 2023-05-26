// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::CodegenError;
use crate::{
    types::{CompositeDefFields, TypeGenerator},
    CratePath,
};
use heck::{ToSnakeCase as _, ToUpperCamelCase as _};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use subxt_metadata::PalletMetadata;

/// Generate calls from the provided pallet's metadata. Each call returns a `StaticTxPayload`
/// that can be passed to the subxt client to submit/sign/encode.
///
/// # Arguments
///
/// - `metadata` - Runtime metadata from which the calls are generated.
/// - `type_gen` - The type generator containing all types defined by metadata.
/// - `pallet` - Pallet metadata from which the calls are generated.
/// - `types_mod_ident` - The ident of the base module that we can use to access the generated types from.
pub fn generate_calls(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    types_mod_ident: &syn::Ident,
    crate_path: &CratePath,
    should_gen_docs: bool,
) -> Result<TokenStream2, CodegenError> {
    // Early return if the pallet has no calls.
    let Some(call_ty) = pallet.call_ty_id() else {
        return Ok(quote!());
    };

    let mut struct_defs = super::generate_structs_from_variants(
        type_gen,
        call_ty,
        |name| name.to_upper_camel_case().into(),
        "Call",
        crate_path,
        should_gen_docs,
    )?;
    let (call_structs, call_fns): (Vec<_>, Vec<_>) = struct_defs
        .iter_mut()
        .map(|(variant_name, struct_def)| {
            let (call_fn_args, call_args): (Vec<_>, Vec<_>) = match struct_def.fields {
                CompositeDefFields::Named(ref named_fields) => named_fields
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
                    .unzip(),
                CompositeDefFields::NoFields => Default::default(),
                CompositeDefFields::Unnamed(_) => {
                    return Err(CodegenError::InvalidCallVariant(call_ty))
                }
            };

            let pallet_name = pallet.name();
            let call_name = &variant_name;
            let struct_name = &struct_def.name;
            let Some(call_hash) = pallet.call_hash(call_name) else {
                return Err(CodegenError::MissingCallMetadata(
                    pallet_name.into(),
                    call_name.to_string(),
                ))
            };
            let fn_name = format_ident!("{}", variant_name.to_snake_case());
            // Propagate the documentation just to `TransactionApi` methods, while
            // draining the documentation of inner call structures.
            let docs = should_gen_docs.then_some(struct_def.docs.take()).flatten();

            // The call structure's documentation was stripped above.
            let call_struct = quote! {
                #struct_def

                impl #crate_path::blocks::StaticExtrinsic for #struct_name {
                    const PALLET: &'static str = #pallet_name;
                    const CALL: &'static str = #call_name;
                }
            };

            let client_fn = quote! {
                #docs
                pub fn #fn_name(
                    &self,
                    #( #call_fn_args, )*
                ) -> #crate_path::tx::Payload<types::#struct_name> {
                    #crate_path::tx::Payload::new_static(
                        #pallet_name,
                        #call_name,
                        types::#struct_name { #( #call_args, )* },
                        [#(#call_hash,)*]
                    )
                }
            };

            Ok((call_struct, client_fn))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .unzip();

    let call_type = type_gen.resolve_type_path(call_ty);
    let call_ty = type_gen.resolve_type(call_ty);
    let docs = &call_ty.docs;
    let docs = should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();

    Ok(quote! {
        #docs
        pub type Call = #call_type;
        pub mod calls {
            use super::root_mod;
            use super::#types_mod_ident;

            type DispatchError = #types_mod_ident::sp_runtime::DispatchError;

            pub mod types {
                use super::#types_mod_ident;

                #( #call_structs )*
            }

            pub struct TransactionApi;

            impl TransactionApi {
                #( #call_fns )*
            }
        }
    })
}
