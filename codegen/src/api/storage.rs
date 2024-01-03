// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use heck::ToSnakeCase as _;
use heck::ToUpperCamelCase as _;
use proc_macro2::{Ident, TokenStream as TokenStream2, TokenStream};
use quote::{format_ident, quote};
use scale_info::TypeDef;
use scale_typegen::{typegen::type_path::TypePath, TypeGenerator};
use subxt_metadata::{
    PalletMetadata, StorageEntryMetadata, StorageEntryModifier, StorageEntryType,
};

use super::CodegenError;

/// Generate functions which create storage addresses from the provided pallet's metadata.
/// These addresses can be used to access and iterate over storage values.
///
/// # Arguments
///
/// - `metadata` - Runtime metadata from which the storages are generated.
/// - `type_gen` - The type generator containing all types defined by metadata.
/// - `pallet` - Pallet metadata from which the storages are generated.
/// - `types_mod_ident` - The ident of the base module that we can use to access the generated types from.
pub fn generate_storage(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    crate_path: &syn::Path,
) -> Result<TokenStream2, CodegenError> {
    let Some(storage) = pallet.storage() else {
        return Ok(quote!());
    };

    let (storage_fns, alias_modules): (Vec<_>, Vec<_>) = storage
        .entries()
        .iter()
        .map(|entry| generate_storage_entry_fns(type_gen, pallet, entry, crate_path))
        .collect::<Result<Vec<_>, CodegenError>>()?;

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

fn generate_storage_entry_fns(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    storage_entry: &StorageEntryMetadata,
    crate_path: &syn::Path,
) -> Result<TokenStream2, CodegenError> {
    let keys: Vec<(Ident, TypePath)> = match storage_entry.entry_type() {
        StorageEntryType::Plain(_) => vec![],
        StorageEntryType::Map { key_ty, .. } => {
            match &type_gen.resolve_type(*key_ty)?.type_def {
                // An N-map; return each of the keys separately.
                TypeDef::Tuple(tuple) => tuple
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let ident: Ident = format_ident!("_{}", syn::Index::from(i));
                        let ty_path = type_gen
                            .resolve_type_path(f.id)
                            .expect("resolving type should not fail");
                        (ident, ty_path)
                    })
                    .collect::<Vec<_>>(),
                // A map with a single key; return the single key.
                _ => {
                    let ident = format_ident!("_0");
                    let ty_path = type_gen.resolve_type_path(*key_ty)?;
                    vec![(ident, ty_path)]
                }
            }
        }
    };
    let pallet_name = pallet.name();
    let storage_name = storage_entry.name();
    let Some(storage_hash) = pallet.storage_hash(storage_name) else {
        return Err(CodegenError::MissingStorageMetadata(
            pallet_name.into(),
            storage_name.into(),
        ));
    };

    let snake_case_name = storage_entry.name().to_snake_case();
    let storage_entry_ty = storage_entry.entry_type().value_ty();
    let storage_entry_value_ty = type_gen.resolve_type_path(storage_entry_ty)?;
    let docs = storage_entry.docs();
    let docs = type_gen
        .settings()
        .should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();

    let is_defaultable_type = match storage_entry.modifier() {
        StorageEntryModifier::Default => quote!(#crate_path::storage::address::Yes),
        StorageEntryModifier::Optional => quote!(()),
    };

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
        let is_fetchable_type = is_fetchable.then_some(quote!(#crate_path::storage::address::Yes)).unwrap_or(quote!(()));
        let is_iterable_type = is_iterable.then_some(quote!(#crate_path::storage::address::Yes)).unwrap_or(quote!(()));
        let key_impls = keys_slice.iter().map(|(field_name, _, _)| quote!( #crate_path::storage::address::make_static_storage_map_key(#field_name.borrow()) ));
        let key_args = keys_slice.iter().map(|(field_name, _, path_to_alias )| {
            quote!( #field_name: impl ::std::borrow::Borrow<#path_to_alias> )
        });

        quote!(
            #docs
            pub fn #fn_name(
                &self,
                #(#key_args,)*
            ) -> #crate_path::storage::address::Address::<
                #crate_path::storage::address::StaticStorageMapKey,
                #alias_storage_path,
                #is_fetchable_type,
                #is_defaultable_type,
                #is_iterable_type
            > {
                #crate_path::storage::address::Address::new_static(
                    #pallet_name,
                    #storage_name,
                    vec![#(#key_impls,)*],
                    [#(#storage_hash,)*]
                )
            }
        )
    });

    let alias_types = keys.iter().map(|(_, alias_type, _)| alias_type);

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

fn primitive_type_alias(type_path: &TypePath) -> TokenStream {
    // Vec<T> is cast to [T]
    if let Some(ty) = type_path.vec_type_param() {
        return quote!([#ty]);
    }
    // String is cast to str
    if type_path.is_string() {
        return quote!(::core::primitive::str);
    }
    quote!(#type_path)
}

#[cfg(test)]
mod tests {
    use crate::RuntimeGenerator;
    use frame_metadata::v15;
    use heck::ToUpperCamelCase as _;
    use quote::{format_ident, quote};
    use scale_info::{meta_type, MetaType};

    use std::borrow::Cow;

    use subxt_metadata::Metadata;

    fn metadata_with_storage_entries(
        storage_entries: impl IntoIterator<Item = (&'static str, MetaType)>,
    ) -> Metadata {
        let storage_entries: Vec<v15::StorageEntryMetadata> = storage_entries
            .into_iter()
            .map(|(name, key)| v15::StorageEntryMetadata {
                name,
                modifier: v15::StorageEntryModifier::Optional,
                ty: v15::StorageEntryType::Map {
                    hashers: vec![],
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

    #[test]
    fn borrow_type_replacements() {
        let storage_entries = [
            ("vector", meta_type::<Vec<u8>>()),
            ("boxed", meta_type::<Box<u16>>()),
            ("string", meta_type::<String>()),
            ("static_string", meta_type::<&'static str>()),
            ("cow_string", meta_type::<Cow<'_, str>>()),
        ];

        let expected_borrowed_types = [
            quote!([::core::primitive::u8]),
            quote!(::core::primitive::u16),
            quote!(::core::primitive::str),
            quote!(::core::primitive::str),
            quote!(::core::primitive::str),
        ];

        let metadata = metadata_with_storage_entries(storage_entries);

        let item_mod = syn::parse_quote!(
            pub mod api {}
        );
        let generator = RuntimeGenerator::new(metadata);
        let generated = generator
            .generate_runtime(
                item_mod,
                Default::default(),
                Default::default(),
                syn::parse_str("::subxt_path").unwrap(),
                false,
            )
            .expect("should be able to generate runtime");
        let generated_str = generated.to_string();

        for ((name, _), expected_type) in storage_entries
            .into_iter()
            .zip(expected_borrowed_types.into_iter())
        {
            let name_ident = format_ident!("{}", name);
            let expected_storage_constructor = quote!(
                fn #name_ident(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::#name_ident::Param0>,
                )
            );
            assert!(generated_str.contains(&expected_storage_constructor.to_string()));

            let alias_name = format_ident!("{}", name.to_upper_camel_case());
            let expected_alias_module = quote!(
                pub mod #name_ident {
                    use super::runtime_types;

                    pub type #alias_name = ::core::primitive::bool;
                    pub type Param0 = #expected_type;
                }
            );

            assert!(generated_str.contains(&expected_alias_module.to_string()));
        }
    }
}
