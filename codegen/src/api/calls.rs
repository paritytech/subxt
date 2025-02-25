// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::CodegenError;
use heck::{ToSnakeCase as _, ToUpperCamelCase as _};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use scale_typegen::typegen::ir::ToTokensWithSettings;
use scale_typegen::{typegen::ir::type_ir::CompositeIRKind, TypeGenerator};
use subxt_metadata::PalletMetadata;

/// Generate calls from the provided pallet's metadata. Each call returns a `StaticPayload`
/// that can be passed to the subxt client to submit/sign/encode.
///
/// # Arguments
///
/// - `type_gen` - [`scale_typegen::TypeGenerator`] that contains settings and all types from the runtime metadata.
/// - `pallet` - Pallet metadata from which the calls are generated.
/// - `crate_path` - The crate path under which the `subxt-core` crate is located, e.g. `::subxt::ext::subxt_core` when using subxt as a dependency.
pub fn generate_calls(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    crate_path: &syn::Path,
) -> Result<TokenStream2, CodegenError> {
    // Early return if the pallet has no calls.
    let Some(call_ty) = pallet.call_ty_id() else {
        return Ok(quote!());
    };

    let variant_names_and_struct_defs = super::generate_structs_from_variants(
        type_gen,
        call_ty,
        |name| name.to_upper_camel_case().into(),
        "Call",
    )?;
    let (call_structs, call_fns): (Vec<_>, Vec<_>) = variant_names_and_struct_defs
        .into_iter()
        .map(|var| {
            let (call_fn_args, call_args): (Vec<_>, Vec<_>) = match &var.composite.kind {
                CompositeIRKind::Named(named_fields) => named_fields
                    .iter()
                    .map(|(name, field)| {
                        // Note: fn_arg_type this is relative the type path of the type alias when prefixed with `types::`, e.g. `set_max_code_size::New`
                        let fn_arg_type = field.type_path.to_token_stream(type_gen.settings());
                        let call_arg = if field.is_boxed {
                            quote! { #name: #crate_path::alloc::boxed::Box::new(#name) }
                        } else {
                            quote! { #name }
                        };
                        (quote!( #name: types::#fn_arg_type ), call_arg)
                    })
                    .unzip(),
                CompositeIRKind::NoFields => Default::default(),
                CompositeIRKind::Unnamed(_) => {
                    return Err(CodegenError::InvalidCallVariant(call_ty))
                }
            };

            let pallet_name = pallet.name();
            let call_name = &var.variant_name;
            let struct_name = &var.composite.name;
            let Some(call_hash) = pallet.call_hash(call_name) else {
                return Err(CodegenError::MissingCallMetadata(
                    pallet_name.into(),
                    call_name.to_string(),
                ));
            };
            let fn_name = format_ident!("{}", var.variant_name.to_snake_case());
            // Propagate the documentation just to `TransactionApi` methods, while
            // draining the documentation of inner call structures.
            let docs = &var.composite.docs;

            // this converts the composite into a full struct type. No Type Parameters needed here.
            let struct_def = type_gen
                .upcast_composite(&var.composite)
                .to_token_stream(type_gen.settings());
            let alias_mod = var.type_alias_mod;
            // The call structure's documentation was stripped above.
            let call_struct = quote! {
                #struct_def
                #alias_mod

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
                ) -> #crate_path::tx::payload::StaticPayload<types::#struct_name> {
                    #crate_path::tx::payload::StaticPayload::new_static(
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

    let call_type = type_gen
        .resolve_type_path(call_ty)?
        .to_token_stream(type_gen.settings());
    let call_ty = type_gen.resolve_type(call_ty)?;
    let docs = type_gen.docs_from_scale_info(&call_ty.docs);

    let types_mod_ident = type_gen.types_mod_ident();

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
