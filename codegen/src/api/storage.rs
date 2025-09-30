// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use heck::{ToSnakeCase as _, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream as TokenStream2, TokenStream};
use quote::{format_ident, quote};
use scale_typegen::TypeGenerator;
use subxt_metadata::{
    PalletMetadata, StorageEntryMetadata, StorageHasher,
};

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
        return Ok(quote!());
    };

    let (storage_fns, alias_modules): (Vec<TokenStream2>, Vec<TokenStream2>) = storage
        .entries()
        .iter()
        .map(|entry| generate_storage_entry_fns(type_gen, pallet, entry, crate_path))
        .collect::<Result<Vec<_>, CodegenError>>()?
        .into_iter()
        .unzip();
    let types_mod_ident = type_gen.types_mod_ident();

    Ok(quote! {
        pub mod storage {
            use super::#types_mod_ident;

            pub mod types {
                use super::#types_mod_ident;

                #( #alias_modules )*
            }

            pub struct StorageApi;

            impl StorageApi {
                #( #storage_fns )*
            }
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
    let snake_case_name = storage_entry.name().to_snake_case();
    let storage_entry_ty = storage_entry.value_ty();
    let storage_entry_value_ty = type_gen
        .resolve_type_path(storage_entry_ty)
        .expect("storage type is in metadata; qed")
        .to_token_stream(type_gen.settings());

    let alias_name = format_ident!("{}", storage_entry.name().to_upper_camel_case());
    let alias_module_name = format_ident!("{snake_case_name}");
    let alias_storage_path = quote!( types::#alias_module_name::#alias_name );

    struct MapEntryKey {
        arg_name: Ident,
        alias_type_def: TokenStream,
        alias_type_path: TokenStream,
        hasher: StorageHasher,
    }

    let map_entry_key = |idx, id, hasher| -> MapEntryKey {
        let arg_name: Ident = format_ident!("_{}", idx);
        let ty_path = type_gen
            .resolve_type_path(id)
            .expect("type is in metadata; qed");

        let alias_name = format_ident!("Param{}", idx);
        let alias_type = ty_path.to_token_stream(type_gen.settings());

        let alias_type_def = quote!( pub type #alias_name = #alias_type; );
        let alias_type_path = quote!( types::#alias_module_name::#alias_name );

        MapEntryKey {
            arg_name,
            alias_type_def,
            alias_type_path,
            hasher,
        }
    };

    let keys: Vec<MapEntryKey> = storage_entry
        .keys()
        .enumerate()
        .map(|(idx, key)| map_entry_key(idx, key.key_id, key.hasher))
        .collect();

    let pallet_name = pallet.name();
    let entry_name = storage_entry.name();
    let Some(storage_hash) = pallet.storage_hash(entry_name) else {
        return Err(CodegenError::MissingStorageMetadata(
            pallet_name.into(),
            entry_name.into(),
        ));
    };

    let docs = storage_entry.docs();
    let docs = type_gen
        .settings()
        .should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();

    let is_defaultable_type = if storage_entry.default_value().is_some() {
        quote!(#crate_path::utils::Yes)
    } else {
        quote!(())
    };

    // Note: putting `#crate_path::storage::address::StaticStorageKey` into this variable is necessary
    // to get the line width below a certain limit. If not done, rustfmt will refuse to format the following big expression.
    // for more information see [this post](https://users.rust-lang.org/t/rustfmt-silently-fails-to-work/75485/4).
    let static_storage_key: TokenStream = quote!(#crate_path::storage::address::StaticStorageKey);
    let all_fns = (0..=keys.len()).map(|n_keys| {
        let keys_slice = &keys[..n_keys];
        let (fn_name, is_fetchable, is_iterable) = if n_keys == keys.len() {
            let fn_name = format_ident!("{snake_case_name}");
            (fn_name, true, false)
        } else {
            let fn_name = if n_keys == 0 {
                format_ident!("{snake_case_name}_iter")
            } else {
                format_ident!("{snake_case_name}_iter{}", n_keys)
            };
            (fn_name, false, true)
        };
        let is_fetchable_type = is_fetchable
            .then_some(quote!(#crate_path::utils::Yes))
            .unwrap_or(quote!(()));
        let is_iterable_type = is_iterable
            .then_some(quote!(#crate_path::utils::Yes))
            .unwrap_or(quote!(()));

        let (keys, keys_type) = match keys_slice.len() {
            0 => (quote!(()), quote!(())),
            1 => {
                let key = &keys_slice[0];
                if key.hasher.ends_with_key() {
                    let arg = &key.arg_name;
                    let keys = quote!(#static_storage_key::new(#arg));
                    let path = &key.alias_type_path;
                    let path = quote!(#static_storage_key<#path>);
                    (keys, path)
                } else {
                    (quote!(()), quote!(()))
                }
            }
            _ => {
                let keys_iter = keys_slice.iter().map(
                    |MapEntryKey {
                         arg_name, hasher, ..
                     }| {
                        if hasher.ends_with_key() {
                            quote!( #static_storage_key::new(#arg_name) )
                        } else {
                            quote!(())
                        }
                    },
                );
                let keys = quote!( (#(#keys_iter,)*) );
                let paths_iter = keys_slice.iter().map(
                    |MapEntryKey {
                         alias_type_path,
                         hasher,
                         ..
                     }| {
                        if hasher.ends_with_key() {
                            quote!( #static_storage_key<#alias_type_path> )
                        } else {
                            quote!(())
                        }
                    },
                );
                let paths = quote!( (#(#paths_iter,)*) );
                (keys, paths)
            }
        };

        let key_args = keys_slice.iter().map(
            |MapEntryKey {
                 arg_name,
                 alias_type_path,
                 ..
             }| quote!( #arg_name: #alias_type_path ),
        );

        quote!(
            #docs
            pub fn #fn_name(
                &self,
                #(#key_args,)*
            ) -> #crate_path::storage::address::StaticAddress::<
                #keys_type,
                #alias_storage_path,
                #is_fetchable_type,
                #is_defaultable_type,
                #is_iterable_type
            > {
                #crate_path::storage::address::StaticAddress::new_static(
                    #pallet_name,
                    #entry_name,
                    #keys,
                    [#(#storage_hash,)*]
                )
            }
        )
    });

    let alias_types = keys
        .iter()
        .map(|MapEntryKey { alias_type_def, .. }| alias_type_def);

    let types_mod_ident = type_gen.types_mod_ident();
    // Generate type alias for the return type only, since
    // the keys of the storage entry are not explicitly named.
    let alias_module = quote! {
        pub mod #alias_module_name {
            use super::#types_mod_ident;

            pub type #alias_name = #storage_entry_value_ty;

            #( #alias_types )*
        }
    };

    Ok((
        quote! {
            #( #all_fns )*
        },
        alias_module,
    ))
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
