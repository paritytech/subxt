// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use heck::ToSnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use scale_typegen::TypeGenerator;
use subxt_metadata::{PalletMetadata, StorageEntryMetadata};

use super::CodegenError;

use scale_typegen::typegen::ir::ToTokensWithSettings;

/// Generate functions which create storage addresses from the provided pallet's metadata.
/// These addresses can be used to access and iterate over storage values.
///
/// # Arguments
///
/// - `type_gen` - [`scale_typegen::TypeGenerator`] that contains settings and all types from the runtime metadata.
/// - `pallet` - Pallet metadata from which the storage items are generated.
/// - `crate_path` - The crate path under which the `subxt-core` crate is located, e.g. `::subxt::ext::subxt_core` when using subxt as a dependency.
pub fn generate_storage(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    crate_path: &syn::Path,
) -> Result<TokenStream2, CodegenError> {
    let Some(storage) = pallet.storage() else {
        // If there are no storage entries in this pallet, we
        // don't generate anything.
        return Ok(quote!());
    };

    let storage_entries = storage
        .entries()
        .iter()
        .map(|entry| generate_storage_entry_fns(type_gen, pallet, entry, crate_path))
        .collect::<Result<Vec<_>, CodegenError>>()?;

    let storage_entry_types = storage_entries.iter().map(|(types, _)| types);
    let storage_entry_methods = storage_entries.iter().map(|(_, method)| method);

    let types_mod_ident = type_gen.types_mod_ident();

    Ok(quote! {
        pub mod storage {
            use super::root_mod;
            use super::#types_mod_ident;

            pub struct StorageApi;

            impl StorageApi {
                #( #storage_entry_methods )*
            }

            #( #storage_entry_types )*
        }
    })
}

/// Returns storage entry functions and alias modules.
fn generate_storage_entry_fns(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    storage_entry: &StorageEntryMetadata,
    crate_path: &syn::Path,
) -> Result<(TokenStream2, TokenStream2), CodegenError> {
    let types_mod_ident = type_gen.types_mod_ident();

    let pallet_name = pallet.name();
    let storage_entry_name_str = storage_entry.name();
    let storage_entry_snake_case_name = storage_entry_name_str.to_snake_case();
    let storage_entry_snake_case_ident = format_ident!("{storage_entry_snake_case_name}");
    let Some(validation_hash) = pallet.storage_hash(storage_entry_name_str) else {
        return Err(CodegenError::MissingStorageMetadata(
            pallet_name.into(),
            storage_entry_name_str.into(),
        ));
    };

    let docs = storage_entry.docs();
    let docs: TokenStream2 = type_gen
        .settings()
        .should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();

    struct Input {
        type_alias: syn::Ident,
        type_path: TokenStream2,
    }

    let storage_key_types: Vec<Input> = storage_entry
        .keys()
        .enumerate()
        .map(|(idx, key)| {
            // Storage key aliases are just indexes; no names to use.
            let type_alias = format_ident!("Param{}", idx);

            // Path to the actual type we'll have generated for this input.
            let type_path = type_gen
                .resolve_type_path(key.key_id)
                .expect("view function input type is in metadata; qed")
                .to_token_stream(type_gen.settings());

            Input {
                type_alias,
                type_path,
            }
        })
        .collect();

    let storage_key_tuple_types = storage_key_types
        .iter()
        .map(|i| {
            let ty = &i.type_alias;
            quote!(#storage_entry_snake_case_ident::#ty)
        })
        .collect::<Vec<_>>();

    let storage_key_type_aliases = storage_key_types
        .iter()
        .map(|i| {
            let ty = &i.type_alias;
            let path = &i.type_path;
            quote!(pub type #ty = #path;)
        })
        .collect::<Vec<_>>();

    let storage_value_type_path = type_gen
        .resolve_type_path(storage_entry.value_ty())?
        .to_token_stream(type_gen.settings());

    let is_plain = if storage_entry.keys().len() == 0 {
        quote!(#crate_path::utils::Yes)
    } else {
        quote!(#crate_path::utils::Maybe)
    };

    let storage_entry_types = quote!(
        pub mod #storage_entry_snake_case_ident {
            use super::root_mod;
            use super::#types_mod_ident;

            #(#storage_key_type_aliases)*

            pub mod output {
                use super::#types_mod_ident;
                pub type Output = #storage_value_type_path;
            }
        }
    );

    let storage_entry_method = quote!(
        #docs
        pub fn #storage_entry_snake_case_ident(&self) -> #crate_path::storage::address::StaticAddress<
            (#(#storage_key_tuple_types,)*),
            #storage_entry_snake_case_ident::output::Output,
            #is_plain
        > {
            #crate_path::storage::address::StaticAddress::new_static(
                #pallet_name,
                #storage_entry_name_str,
                [#(#validation_hash,)*],
            )
        }
    );

    Ok((storage_entry_types, storage_entry_method))
}

#[cfg(test)]
mod tests {
    use frame_metadata::v15;
    use scale_info::{MetaType, meta_type};
    use subxt_metadata::Metadata;

    // TODO: Think about adding tests for storage codegen which can use this sort of function.
    #[allow(dead_code)]
    fn metadata_with_storage_entries(
        storage_entries: impl IntoIterator<Item = (&'static str, MetaType)>,
    ) -> Metadata {
        let storage_entries: Vec<v15::StorageEntryMetadata> = storage_entries
            .into_iter()
            .map(|(name, key)| v15::StorageEntryMetadata {
                name,
                modifier: v15::StorageEntryModifier::Optional,
                ty: v15::StorageEntryType::Map {
                    hashers: vec![v15::StorageHasher::Blake2_128Concat],
                    key,
                    value: meta_type::<bool>(),
                },
                default: vec![],
                docs: vec![],
            })
            .collect();

        let pallet_1 = v15::PalletMetadata {
            name: "Pallet1",
            storage: Some(v15::PalletStorageMetadata {
                prefix: Default::default(),
                entries: storage_entries,
            }),
            calls: None,
            event: None,
            constants: vec![],
            error: None,
            index: 0,
            docs: vec![],
        };

        let extrinsic_metadata = v15::ExtrinsicMetadata {
            version: 0,
            signed_extensions: vec![],
            address_ty: meta_type::<()>(),
            call_ty: meta_type::<()>(),
            signature_ty: meta_type::<()>(),
            extra_ty: meta_type::<()>(),
        };

        let metadata: Metadata = v15::RuntimeMetadataV15::new(
            vec![pallet_1],
            extrinsic_metadata,
            meta_type::<()>(),
            vec![],
            v15::OuterEnums {
                call_enum_ty: meta_type::<()>(),
                event_enum_ty: meta_type::<()>(),
                error_enum_ty: meta_type::<()>(),
            },
            v15::CustomMetadata {
                map: Default::default(),
            },
        )
        .try_into()
        .expect("can build valid metadata");
        metadata
    }
}
