// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Generate code for submitting extrinsics and query storage of a Substrate runtime.
//!
//! ## Note
//!
//! By default the codegen will search for the `System` pallet's `Account` storage item, which is
//! the conventional location where an account's index (aka nonce) is stored.
//!
//! If this `System::Account` storage item is discovered, then it is assumed that:
//!
//!   1. The type of the storage item is a `struct` (aka a composite type)
//!   2. There exists a field called `nonce` which contains the account index.
//!
//! These assumptions are based on the fact that the `frame_system::AccountInfo` type is the default
//! configured type, and that the vast majority of chain configurations will use this.
//!
//! If either of these conditions are not satisfied, the codegen will fail.

mod calls;
mod constants;
mod errors;
mod events;
mod storage;

use super::GeneratedTypeDerives;
use crate::{
    ir,
    types::{
        CompositeDef,
        CompositeDefFields,
        TypeGenerator,
    },
};
use codec::Decode;
use frame_metadata::{
    v14::RuntimeMetadataV14,
    PalletMetadata,
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
    StorageEntryType,
};
use heck::SnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{
    format_ident,
    quote,
};
use scale_info::form::PortableForm;
use std::{
    collections::HashMap,
    fs,
    io::Read,
    path,
    string::ToString,
};
use syn::{
    parse_quote,
    punctuated::Punctuated,
};

pub fn generate_runtime_api<P>(
    item_mod: syn::ItemMod,
    path: P,
    generated_type_derives: Option<Punctuated<syn::Path, syn::Token![,]>>,
) -> TokenStream2
where
    P: AsRef<path::Path>,
{
    let mut file = fs::File::open(&path).unwrap_or_else(|e| {
        abort_call_site!("Failed to open {}: {}", path.as_ref().to_string_lossy(), e)
    });

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .unwrap_or_else(|e| abort_call_site!("Failed to read metadata file: {}", e));

    let metadata = frame_metadata::RuntimeMetadataPrefixed::decode(&mut &bytes[..])
        .unwrap_or_else(|e| abort_call_site!("Failed to decode metadata: {}", e));

    let mut derives = GeneratedTypeDerives::default();
    if let Some(user_derives) = generated_type_derives {
        derives.append(user_derives.iter().cloned())
    }

    let generator = RuntimeGenerator::new(metadata);
    generator.generate_runtime(item_mod, derives)
}

pub struct RuntimeGenerator {
    metadata: RuntimeMetadataV14,
}

impl RuntimeGenerator {
    pub fn new(metadata: RuntimeMetadataPrefixed) -> Self {
        match metadata.1 {
            RuntimeMetadata::V14(v14) => Self { metadata: v14 },
            _ => panic!("Unsupported metadata version {:?}", metadata.1),
        }
    }

    pub fn generate_runtime(
        &self,
        item_mod: syn::ItemMod,
        derives: GeneratedTypeDerives,
    ) -> TokenStream2 {
        let item_mod_ir = ir::ItemMod::from(item_mod);

        // some hardcoded default type substitutes, can be overridden by user
        let mut type_substitutes = [
            (
                "bitvec::order::Lsb0",
                parse_quote!(::subxt::bitvec::order::Lsb0),
            ),
            (
                "bitvec::order::Msb0",
                parse_quote!(::subxt::bitvec::order::Msb0),
            ),
            (
                "sp_core::crypto::AccountId32",
                parse_quote!(::subxt::sp_core::crypto::AccountId32),
            ),
            (
                "primitive_types::H256",
                parse_quote!(::subxt::sp_core::H256),
            ),
            (
                "sp_runtime::multiaddress::MultiAddress",
                parse_quote!(::subxt::sp_runtime::MultiAddress),
            ),
            (
                "frame_support::traits::misc::WrapperKeepOpaque",
                parse_quote!(::subxt::WrapperKeepOpaque),
            ),
        ]
        .iter()
        .map(|(path, substitute): &(&str, syn::TypePath)| {
            (path.to_string(), substitute.clone())
        })
        .collect::<HashMap<_, _>>();

        for (path, substitute) in item_mod_ir.type_substitutes().iter() {
            type_substitutes.insert(path.to_string(), substitute.clone());
        }

        let type_gen = TypeGenerator::new(
            &self.metadata.types,
            "runtime_types",
            type_substitutes,
            derives.clone(),
        );
        let types_mod = type_gen.generate_types_mod();
        let types_mod_ident = types_mod.ident();
        let pallets_with_mod_names = self
            .metadata
            .pallets
            .iter()
            .map(|pallet| {
                (
                    pallet,
                    format_ident!("{}", pallet.name.to_string().to_snake_case()),
                )
            })
            .collect::<Vec<_>>();

        let modules = pallets_with_mod_names.iter().map(|(pallet, mod_name)| {
            let calls = if let Some(ref calls) = pallet.calls {
                calls::generate_calls(&type_gen, pallet, calls, types_mod_ident)
            } else {
                quote!()
            };

            let event = if let Some(ref event) = pallet.event {
                events::generate_events(&type_gen, pallet, event, types_mod_ident)
            } else {
                quote!()
            };

            let storage_mod = if let Some(ref storage) = pallet.storage {
                storage::generate_storage(&type_gen, pallet, storage, types_mod_ident)
            } else {
                quote!()
            };

            let constants_mod = if !pallet.constants.is_empty() {
                constants::generate_constants(
                    &type_gen,
                    &pallet.constants,
                    types_mod_ident,
                )
            } else {
                quote!()
            };

            quote! {
                pub mod #mod_name {
                    use super::#types_mod_ident;
                    #calls
                    #event
                    #storage_mod
                    #constants_mod
                }
            }
        });

        let outer_event_variants = self.metadata.pallets.iter().filter_map(|p| {
            let variant_name = format_ident!("{}", p.name);
            let mod_name = format_ident!("{}", p.name.to_string().to_snake_case());
            let index = proc_macro2::Literal::u8_unsuffixed(p.index);

            p.event.as_ref().map(|_| {
                quote! {
                    #[codec(index = #index)]
                    #variant_name(#mod_name::Event),
                }
            })
        });

        let outer_event = quote! {
            #derives
            pub enum Event {
                #( #outer_event_variants )*
            }
        };

        let mod_ident = item_mod_ir.ident;
        let pallets_with_constants =
            pallets_with_mod_names
                .iter()
                .filter_map(|(pallet, pallet_mod_name)| {
                    (!pallet.constants.is_empty()).then(|| pallet_mod_name)
                });
        let pallets_with_storage =
            pallets_with_mod_names
                .iter()
                .filter_map(|(pallet, pallet_mod_name)| {
                    pallet.storage.as_ref().map(|_| pallet_mod_name)
                });
        let pallets_with_calls =
            pallets_with_mod_names
                .iter()
                .filter_map(|(pallet, pallet_mod_name)| {
                    pallet.calls.as_ref().map(|_| pallet_mod_name)
                });

        let error_details = errors::generate_error_details(&self.metadata);
        let error_type = error_details.type_def;
        let error_fn = error_details.dispatch_error_impl_fn;

        let default_account_data_ident = format_ident!("DefaultAccountData");
        let default_account_data_impl = generate_default_account_data_impl(
            &pallets_with_mod_names,
            &default_account_data_ident,
            &type_gen,
        );
        let type_parameter_default_impl = default_account_data_impl
            .as_ref()
            .map(|_| quote!( = #default_account_data_ident ));

        quote! {
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            pub mod #mod_ident {
                #outer_event
                #( #modules )*
                #types_mod

                /// The default error type returned when there is a runtime issue.
                pub type DispatchError = self::runtime_types::sp_runtime::DispatchError;

                // Statically generate error information so that we don't need runtime metadata for it.
                #error_type
                impl DispatchError {
                    #error_fn
                }

                #default_account_data_impl

                pub struct RuntimeApi<T: ::subxt::Config, X, A #type_parameter_default_impl> {
                    pub client: ::subxt::Client<T>,
                    marker: ::core::marker::PhantomData<(X, A)>,
                }

                impl<T, X, A> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X, A>
                where
                    T: ::subxt::Config,
                    X: ::subxt::SignedExtra<T>,
                    A: ::subxt::AccountData,
                {
                    fn from(client: ::subxt::Client<T>) -> Self {
                        Self { client, marker: ::core::marker::PhantomData }
                    }
                }

                impl<'a, T, X, A> RuntimeApi<T, X, A>
                where
                    T: ::subxt::Config,
                    X: ::subxt::SignedExtra<T>,
                    A: ::subxt::AccountData,
                {
                    pub fn constants(&'a self) -> ConstantsApi {
                        ConstantsApi
                    }

                    pub fn storage(&'a self) -> StorageApi<'a, T> {
                        StorageApi { client: &self.client }
                    }

                    pub fn tx(&'a self) -> TransactionApi<'a, T, X, A> {
                        TransactionApi { client: &self.client, marker: ::core::marker::PhantomData }
                    }
                }

                pub struct ConstantsApi;

                impl ConstantsApi
                {
                    #(
                        pub fn #pallets_with_constants(&self) -> #pallets_with_constants::constants::ConstantsApi {
                            #pallets_with_constants::constants::ConstantsApi
                        }
                    )*
                }

                pub struct StorageApi<'a, T: ::subxt::Config> {
                    client: &'a ::subxt::Client<T>,
                }

                impl<'a, T> StorageApi<'a, T>
                where
                    T: ::subxt::Config,
                {
                    #(
                        pub fn #pallets_with_storage(&self) -> #pallets_with_storage::storage::StorageApi<'a, T> {
                            #pallets_with_storage::storage::StorageApi::new(self.client)
                        }
                    )*
                }

                pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                    client: &'a ::subxt::Client<T>,
                    marker: ::core::marker::PhantomData<(X, A)>,
                }

                impl<'a, T, X, A> TransactionApi<'a, T, X, A>
                where
                    T: ::subxt::Config,
                    X: ::subxt::SignedExtra<T>,
                    A: ::subxt::AccountData,
                {
                    #(
                        pub fn #pallets_with_calls(&self) -> #pallets_with_calls::calls::TransactionApi<'a, T, X, A> {
                            #pallets_with_calls::calls::TransactionApi::new(self.client)
                        }
                    )*
                }
            }
        }
    }
}

/// Most chains require a valid account nonce as part of the extrinsic, so the default behaviour of
/// the client is to fetch the nonce for the current account.
///
/// The account index (aka nonce) is commonly stored in the `System` pallet's `Account` storage item.
/// This function attempts to find that storage item, and if it is present will implement the
/// `subxt::AccountData` trait for it. This allows the client to construct the appropriate
/// storage key from the account id, and then retrieve the `nonce` from the resulting storage item.
fn generate_default_account_data_impl(
    pallets_with_mod_names: &[(&PalletMetadata<PortableForm>, syn::Ident)],
    default_impl_name: &syn::Ident,
    type_gen: &TypeGenerator,
) -> Option<TokenStream2> {
    let storage = pallets_with_mod_names
        .iter()
        .find(|(pallet, _)| pallet.name == "System")
        .and_then(|(pallet, _)| pallet.storage.as_ref())?;
    let storage_entry = storage
        .entries
        .iter()
        .find(|entry| entry.name == "Account")?;

    // resolve the concrete types for `AccountId` (to build the key) and `Index` to extract the
    // account index (nonce) value from the result.
    let (account_id_ty, account_nonce_ty) =
        if let StorageEntryType::Map { key, value, .. } = &storage_entry.ty {
            let account_id_ty = type_gen.resolve_type_path(key.id(), &[]);
            let account_data_ty = type_gen.resolve_type(value.id());
            let nonce_field = if let scale_info::TypeDef::Composite(composite) =
                account_data_ty.type_def()
            {
                composite
                    .fields()
                    .iter()
                    .find(|f| f.name() == Some(&"nonce".to_string()))?
            } else {
                abort_call_site!("Expected a `nonce` field in the account info struct")
            };
            let account_nonce_ty = type_gen.resolve_type_path(nonce_field.ty().id(), &[]);
            (account_id_ty, account_nonce_ty)
        } else {
            abort_call_site!("System::Account should be a `StorageEntryType::Map`")
        };

    // this path to the storage entry depends on storage codegen.
    let storage_entry_path = quote!(self::system::storage::Account);

    Some(quote! {
        /// The default storage entry from which to fetch an account nonce, required for
        /// constructing a transaction.
        pub enum #default_impl_name {}

        impl ::subxt::AccountData for #default_impl_name {
            type StorageEntry = #storage_entry_path;
            type AccountId = #account_id_ty;
            type Index = #account_nonce_ty;

            fn nonce(result: &<Self::StorageEntry as ::subxt::StorageEntry>::Value) -> Self::Index {
                result.nonce
            }
            fn storage_entry(account_id: Self::AccountId) -> Self::StorageEntry {
                #storage_entry_path(account_id)
            }
        }
    })
}

pub fn generate_structs_from_variants<'a, F>(
    type_gen: &'a TypeGenerator,
    type_id: u32,
    variant_to_struct_name: F,
    error_message_type_name: &str,
) -> Vec<CompositeDef>
where
    F: Fn(&str) -> std::borrow::Cow<str>,
{
    let ty = type_gen.resolve_type(type_id);
    if let scale_info::TypeDef::Variant(variant) = ty.type_def() {
        variant
            .variants()
            .iter()
            .map(|var| {
                let struct_name = variant_to_struct_name(var.name());
                let fields = CompositeDefFields::from_scale_info_fields(
                    struct_name.as_ref(),
                    var.fields(),
                    &[],
                    type_gen,
                );
                CompositeDef::struct_def(
                    var.name(),
                    Default::default(),
                    fields,
                    Some(parse_quote!(pub)),
                    type_gen,
                )
            })
            .collect()
    } else {
        abort_call_site!(
            "{} type should be an variant/enum type",
            error_message_type_name
        )
    }
}
