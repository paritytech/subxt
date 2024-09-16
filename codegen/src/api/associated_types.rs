// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::any::Any;

use super::CodegenError;
use heck::{ToSnakeCase as _, ToUpperCamelCase as _};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use scale_typegen::typegen::ir::ToTokensWithSettings;
use scale_typegen::{typegen::ir::type_ir::CompositeIRKind, TypeGenerator};
use subxt_metadata::{AssociatedTypeMetadata, PalletMetadata};

/// The name of the system pallet.
const PALLET_SYSTEM: &str = "System";
/// The name of the system pallet block type.
const PALLET_SYSTEM_BLOCK: &str = "Block";

fn handle_block_type(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    ty: &AssociatedTypeMetadata,
    crate_path: &syn::Path,
) -> Option<TokenStream2> {
    // Only handle the system pallet block type.
    if pallet.name() != PALLET_SYSTEM || ty.name() != PALLET_SYSTEM_BLOCK {
        return None;
    }

    // println!("System pallet, block type: {:?}", ty);

    let resolved_ty = type_gen.resolve_type(ty.type_id()).ok()?;
    // First generic param is the header of the chain.
    let header = resolved_ty.type_params.get(0)?;

    // Second generic param is the unchecked extrinsics.
    let extrinsics = resolved_ty.type_params.get(1)?;
    let extrinsics_ty = type_gen.resolve_type(extrinsics.ty?.id).ok()?;
    // Which contains the Address Type as first generic parameter.
    let account_id = extrinsics_ty.type_params.get(0)?;
    let resolved_account_id = type_gen.resolve_type_path(account_id.ty?.id).ok()?;
    let resolved_account_id = resolved_account_id.to_token_stream(type_gen.settings());

    let ty_path = type_gen.resolve_type_path(ty.type_id()).ok()?;
    let ty = ty_path.to_token_stream(type_gen.settings());

    Some(quote! {
        pub type Address = #resolved_account_id;
        // TODO: add the header type here.
        // pub type Header = <#crate_path::system::Block as #crate_path::Block>::Header;
    })
}

/// Generate associated types.
pub fn generate_associated_types(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    crate_path: &syn::Path,
) -> Result<TokenStream2, CodegenError> {
    let associated_types = pallet.associated_types();

    let collected = associated_types.iter().map(|ty| {
        let name = format_ident!("{}", ty.name());
        let docs = type_gen.docs_from_scale_info(&ty.docs());

        let Ok(ty_path) = type_gen.resolve_type_path(ty.type_id()) else {
            // We don't have the information in the type generator to handle this type.
            return quote! {};
        };

        let maybe_block_ty = handle_block_type(type_gen, pallet, ty, crate_path);
        let name_str = ty.name();
        let ty = ty_path.to_token_stream(type_gen.settings());

        let mut maybe_impl = None;
        if name_str == "Hashing" {
            // Extract hasher name
            let ty_path_str = ty.to_string();
            if ty_path_str.contains("BlakeTwo256") {
                maybe_impl = Some(quote! {
                    impl #crate_path::config::Hasher for #ty {
                        type Output = #crate_path::utils::H256;

                        fn hash(s: &[u8]) -> Self::Output {
                            let mut bytes = Vec::new();
                            #crate_path::storage::utils::hash_bytes(s, #crate_path::storage::utils::StorageHasher::Blake2_256, &mut bytes);
                            let arr: [u8; 32] = bytes.try_into().expect("Invalid hashing output provided");
                            arr.into()
                        }
                    }
                });
            }
        }



        quote! {
            #docs
            pub type #name = #ty;

            // Types extracted from the generic parameters of the system pallet block type.
            #maybe_block_ty

            // Implementation for the hasher type.
            #maybe_impl
        }
    });

    Ok(quote! {
        #( #collected )*
    })
}
