// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

mod calls;
mod events;
mod storage;

use super::GeneratedTypeDerives;
use crate::{
    ir,
    struct_def::StructDef,
    types::TypeGenerator,
};
use codec::Decode;
use frame_metadata::{
    v14::RuntimeMetadataV14,
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
};
use heck::SnakeCase as _;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{
    format_ident,
    quote,
};
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

            quote! {
                pub mod #mod_name {
                    use super::#types_mod_ident;
                    #calls
                    #event
                    #storage_mod
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

        quote! {
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            pub mod #mod_ident {
                #outer_event
                #( #modules )*
                #types_mod

                /// Default configuration of common types for a target Substrate runtime.
                #[derive(Clone, Debug, Default, Eq, PartialEq)]
                pub struct DefaultConfig;

                impl ::subxt::Config for DefaultConfig {
                    type Index = u32;
                    type BlockNumber = u32;
                    type Hash = ::subxt::sp_core::H256;
                    type Hashing = ::subxt::sp_runtime::traits::BlakeTwo256;
                    type AccountId = ::subxt::sp_runtime::AccountId32;
                    type Address = ::subxt::sp_runtime::MultiAddress<Self::AccountId, u32>;
                    type Header = ::subxt::sp_runtime::generic::Header<
                        Self::BlockNumber, ::subxt::sp_runtime::traits::BlakeTwo256
                    >;
                    type Signature = ::subxt::sp_runtime::MultiSignature;
                    type Extrinsic = ::subxt::sp_runtime::OpaqueExtrinsic;
                }

                impl ::subxt::ExtrinsicExtraData<DefaultConfig> for DefaultConfig {
                    type AccountData = AccountData;
                    type Extra = ::subxt::DefaultExtra<DefaultConfig>;
                }

                pub type AccountData = self::system::storage::Account;

                impl ::subxt::AccountData<DefaultConfig> for AccountData {
                    fn nonce(result: &<Self as ::subxt::StorageEntry>::Value) -> <DefaultConfig as ::subxt::Config>::Index {
                        result.nonce
                    }
                    fn storage_entry(account_id: <DefaultConfig as ::subxt::Config>::AccountId) -> Self {
                        Self(account_id)
                    }
                }

                pub struct RuntimeApi<T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                    pub client: ::subxt::Client<T>,
                }

                impl<T> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T>
                where
                    T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
                {
                    fn from(client: ::subxt::Client<T>) -> Self {
                        Self { client }
                    }
                }

                impl<'a, T> RuntimeApi<T>
                where
                    T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
                {
                    pub fn storage(&'a self) -> StorageApi<'a, T> {
                        StorageApi { client: &self.client }
                    }

                    pub fn tx(&'a self) -> TransactionApi<'a, T> {
                        TransactionApi { client: &self.client }
                    }
                }

                pub struct StorageApi<'a, T>
                where
                    T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
                {
                    client: &'a ::subxt::Client<T>,
                }

                impl<'a, T> StorageApi<'a, T>
                where
                    T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
                {
                    #(
                        pub fn #pallets_with_storage(&self) -> #pallets_with_storage::storage::StorageApi<'a, T> {
                            #pallets_with_storage::storage::StorageApi::new(self.client)
                        }
                    )*
                }

                pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                    client: &'a ::subxt::Client<T>,
                }

                impl<'a, T> TransactionApi<'a, T>
                where
                    T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
                {
                    #(
                        pub fn #pallets_with_calls(&self) -> #pallets_with_calls::calls::TransactionApi<'a, T> {
                            #pallets_with_calls::calls::TransactionApi::new(self.client)
                        }
                    )*
                }
            }
        }
    }
}

pub fn generate_structs_from_variants(
    type_gen: &TypeGenerator,
    type_id: u32,
    error_message_type_name: &str,
) -> Vec<StructDef> {
    let ty = type_gen.resolve_type(type_id);
    if let scale_info::TypeDef::Variant(variant) = ty.type_def() {
        variant
            .variants()
            .iter()
            .map(|var| {
                StructDef::new(
                    var.name(),
                    var.fields(),
                    Some(syn::parse_quote!(pub)),
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
