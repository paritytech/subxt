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

mod calls;
mod constants;
mod errors;
mod events;
mod storage;

use subxt_metadata::get_metadata_per_pallet_hash;

use super::DerivesRegistry;
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
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
};
use heck::ToSnakeCase as _;
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
use syn::parse_quote;

/// Generates the API for interacting with a Substrate runtime.
///
/// # Arguments
///
/// * `item_mod` - The module declaration for which the API is implemented.
/// * `path` - The path to the scale encoded metadata of the runtime node.
/// * `derives` - Provide custom derives for the generated types.
///
/// **Note:** This is a wrapper over [RuntimeGenerator] for static metadata use-cases.
pub fn generate_runtime_api<P>(
    item_mod: syn::ItemMod,
    path: P,
    derives: DerivesRegistry,
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

    let generator = RuntimeGenerator::new(metadata);
    generator.generate_runtime(item_mod, derives)
}

/// Create the API for interacting with a Substrate runtime.
pub struct RuntimeGenerator {
    metadata: RuntimeMetadataV14,
}

impl RuntimeGenerator {
    /// Create a new runtime generator from the provided metadata.
    ///
    /// **Note:** If you have a path to the metadata, prefer to use [generate_runtime_api]
    /// for generating the runtime API.
    pub fn new(metadata: RuntimeMetadataPrefixed) -> Self {
        match metadata.1 {
            RuntimeMetadata::V14(v14) => Self { metadata: v14 },
            _ => panic!("Unsupported metadata version {:?}", metadata.1),
        }
    }

    /// Generate the API for interacting with a Substrate runtime.
    ///
    /// # Arguments
    ///
    /// * `item_mod` - The module declaration for which the API is implemented.
    /// * `derives` - Provide custom derives for the generated types.
    pub fn generate_runtime(
        &self,
        item_mod: syn::ItemMod,
        derives: DerivesRegistry,
    ) -> TokenStream2 {
        let item_mod_ir = ir::ItemMod::from(item_mod);
        let default_derives = derives.default_derives();

        // Some hardcoded default type substitutes, can be overridden by user
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
            // BTreeMap and BTreeSet impose an `Ord` constraint on their key types. This
            // can cause an issue with generated code that doesn't impl `Ord` by default.
            // Decoding them to Vec by default (KeyedVec is just an alias for Vec with
            // suitable type params) avoids these issues.
            ("BTreeMap", parse_quote!(::subxt::KeyedVec)),
            ("BTreeSet", parse_quote!(::std::vec::Vec)),
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

        // Pallet names and their length are used to create PALLETS array.
        // The array is used to identify the pallets composing the metadata for
        // validation of just those pallets.
        let pallet_names: Vec<_> = self
            .metadata
            .pallets
            .iter()
            .map(|pallet| &pallet.name)
            .collect();
        let pallet_names_len = pallet_names.len();

        let metadata_hash = get_metadata_per_pallet_hash(&self.metadata, &pallet_names);

        let modules = pallets_with_mod_names.iter().map(|(pallet, mod_name)| {
            let calls =
                calls::generate_calls(&self.metadata, &type_gen, pallet, types_mod_ident);

            let event = events::generate_events(&type_gen, pallet, types_mod_ident);

            let storage_mod = storage::generate_storage(
                &self.metadata,
                &type_gen,
                pallet,
                types_mod_ident,
            );

            let constants_mod = constants::generate_constants(
                &self.metadata,
                &type_gen,
                pallet,
                types_mod_ident,
            );

            quote! {
                pub mod #mod_name {
                    use super::root_mod;
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
            #default_derives
            pub enum Event {
                #( #outer_event_variants )*
            }
        };

        let mod_ident = item_mod_ir.ident;
        let pallets_with_constants: Vec<_> = pallets_with_mod_names
            .iter()
            .filter_map(|(pallet, pallet_mod_name)| {
                (!pallet.constants.is_empty()).then(|| pallet_mod_name)
            })
            .collect();

        let pallets_with_storage: Vec<_> = pallets_with_mod_names
            .iter()
            .filter_map(|(pallet, pallet_mod_name)| {
                pallet.storage.as_ref().map(|_| pallet_mod_name)
            })
            .collect();

        let pallets_with_calls: Vec<_> = pallets_with_mod_names
            .iter()
            .filter_map(|(pallet, pallet_mod_name)| {
                pallet.calls.as_ref().map(|_| pallet_mod_name)
            })
            .collect();

        let has_module_error_impl =
            errors::generate_has_module_error_impl(&self.metadata, types_mod_ident);

        quote! {
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            pub mod #mod_ident {
                // Make it easy to access the root via `root_mod` at different levels:
                use super::#mod_ident as root_mod;
                // Identify the pallets composing the static metadata by name.
                pub static PALLETS: [&str; #pallet_names_len] = [ #(#pallet_names,)* ];

                #outer_event
                #( #modules )*
                #types_mod

                /// The default error type returned when there is a runtime issue.
                pub type DispatchError = #types_mod_ident::sp_runtime::DispatchError;
                // Impl HasModuleError on DispatchError so we can pluck out module error details.
                #has_module_error_impl

                pub struct RuntimeApi<T: ::subxt::Config, X> {
                    pub client: ::subxt::Client<T>,
                    marker: ::core::marker::PhantomData<X>,
                }

                impl<T: ::subxt::Config, X> Clone for RuntimeApi<T, X> {
                    fn clone(&self) -> Self {
                        Self { client: self.client.clone(), marker: ::core::marker::PhantomData }
                    }
                }

                impl<T, X> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X>
                where
                    T: ::subxt::Config,
                    X: ::subxt::extrinsic::ExtrinsicParams<T>
                {
                    fn from(client: ::subxt::Client<T>) -> Self {
                        Self { client, marker: ::core::marker::PhantomData }
                    }
                }

                impl<'a, T, X> RuntimeApi<T, X>
                where
                    T: ::subxt::Config,
                    X: ::subxt::extrinsic::ExtrinsicParams<T>,
                {
                    pub fn validate_metadata(&'a self) -> Result<(), ::subxt::MetadataError> {
                        let runtime_metadata_hash = {
                            let locked_metadata = self.client.metadata();
                            let metadata = locked_metadata.read();
                            metadata.metadata_hash(&PALLETS)
                        };
                        if runtime_metadata_hash != [ #(#metadata_hash,)* ] {
                            Err(::subxt::MetadataError::IncompatibleMetadata)
                        } else {
                            Ok(())
                        }
                    }

                    pub fn constants(&'a self) -> ConstantsApi<'a, T> {
                        ConstantsApi { client: &self.client }
                    }

                    pub fn storage(&'a self) -> StorageApi<'a, T> {
                        StorageApi { client: &self.client }
                    }

                    pub fn tx(&'a self) -> TransactionApi<'a, T, X> {
                        TransactionApi { client: &self.client, marker: ::core::marker::PhantomData }
                    }

                    pub fn events(&'a self) -> EventsApi<'a, T> {
                        EventsApi { client: &self.client }
                    }
                }

                pub struct EventsApi<'a, T: ::subxt::Config> {
                    client: &'a ::subxt::Client<T>,
                }

                impl <'a, T: ::subxt::Config> EventsApi<'a, T> {
                    pub async fn at(&self, block_hash: T::Hash) -> Result<::subxt::events::Events<T, Event>, ::subxt::BasicError> {
                        ::subxt::events::at::<T, Event>(self.client, block_hash).await
                    }

                    pub async fn subscribe(&self) -> Result<::subxt::events::EventSubscription<'a, ::subxt::events::EventSub<T::Header>, T, Event>, ::subxt::BasicError> {
                        ::subxt::events::subscribe::<T, Event>(self.client).await
                    }

                    pub async fn subscribe_finalized(&self) -> Result<::subxt::events::EventSubscription<'a, ::subxt::events::FinalizedEventSub<'a, T::Header>, T, Event>, ::subxt::BasicError> {
                        ::subxt::events::subscribe_finalized::<T, Event>(self.client).await
                    }
                }

                pub struct ConstantsApi<'a, T: ::subxt::Config> {
                    client: &'a ::subxt::Client<T>,
                }

                impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                    #(
                        pub fn #pallets_with_constants(&self) -> #pallets_with_constants::constants::ConstantsApi<'a, T> {
                            #pallets_with_constants::constants::ConstantsApi::new(self.client)
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

                pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                    client: &'a ::subxt::Client<T>,
                    marker: ::core::marker::PhantomData<X>,
                }

                impl<'a, T, X> TransactionApi<'a, T, X>
                where
                    T: ::subxt::Config,
                    X: ::subxt::extrinsic::ExtrinsicParams<T>,
                {
                    #(
                        pub fn #pallets_with_calls(&self) -> #pallets_with_calls::calls::TransactionApi<'a, T, X> {
                            #pallets_with_calls::calls::TransactionApi::new(self.client)
                        }
                    )*
                }
            }
        }
    }
}

/// Return a vector of tuples of variant names and corresponding struct definitions.
pub fn generate_structs_from_variants<'a, F>(
    type_gen: &'a TypeGenerator,
    type_id: u32,
    variant_to_struct_name: F,
    error_message_type_name: &str,
) -> Vec<(String, CompositeDef)>
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
                let struct_def = CompositeDef::struct_def(
                    &ty,
                    struct_name.as_ref(),
                    Default::default(),
                    fields,
                    Some(parse_quote!(pub)),
                    type_gen,
                    var.docs(),
                );
                (var.name().to_string(), struct_def)
            })
            .collect()
    } else {
        abort_call_site!(
            "{} type should be an variant/enum type",
            error_message_type_name
        )
    }
}
