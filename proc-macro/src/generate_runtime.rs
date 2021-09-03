// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use codec::Decode;
use crate::{TokenStream2, TypeGenerator};
use frame_metadata::{
    v14::RuntimeMetadataV14, PalletCallMetadata, RuntimeMetadata, RuntimeMetadataPrefixed,
};
use heck::SnakeCase as _;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote};
use scale_info::form::PortableForm;
use scale_info::prelude::string::ToString;
use std::{
    fs,
    io::Read,
    path,
};

pub fn generate_runtime_types<P>(item_mod: syn::ItemMod, path: P) -> TokenStream2
where
    P: AsRef<path::Path>,
{
    let mut file = fs::File::open(&path)
        .unwrap_or_else(|e| abort_call_site!("Failed to open {}: {}", path.as_ref().to_string_lossy(), e));

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .unwrap_or_else(|e| abort_call_site!("Failed to read metadata file: {}", e));

    let metadata = frame_metadata::RuntimeMetadataPrefixed::decode(&mut &bytes[..])
        .unwrap_or_else(|e| abort_call_site!("Failed to decode metadata: {}", e));

    let generator = RuntimeGenerator::new(metadata);
    generator.generate_runtime(item_mod)
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

    pub fn generate_runtime(&self, item_mod: syn::ItemMod) -> TokenStream2 {
        let type_gen = TypeGenerator::new(&self.metadata.types, "__types");
        let types_mod = type_gen.generate_types_mod();
        let types_mod_ident = types_mod.ident();
        let modules = self.metadata.pallets.iter().map(|pallet| {
            let mod_name = format_ident!("{}", pallet.name.to_string().to_snake_case());
            let mut calls = Vec::new();
            for call in &pallet.calls {
                let call_structs = self.generate_call_structs(&type_gen, call);
                calls.extend(call_structs)
            }

            let event = if let Some(ref event) = pallet.event {
                let event_type = type_gen.resolve_type_path(event.ty.id(), &[]);
                quote! {
                    pub type Event = #event_type;
                }
            } else {
                quote! {}
            };

            let calls = if !calls.is_empty() {
                quote! {
                    pub mod calls {
                        use super::#types_mod_ident;
                        #( #calls )*
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                pub mod #mod_name {
                    use super::#types_mod_ident;
                    #calls
                    #event
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
            #[derive(Debug, ::codec::Encode, ::codec::Decode)]
            pub enum Event {
                #( #outer_event_variants )*
            }
        };

        let mod_ident = item_mod.ident;
        quote! {
            #[allow(dead_code, unused_imports, non_camel_case_types)]
            pub mod #mod_ident {
                #outer_event
                #( #modules )*
                #types_mod
            }
        }
    }

    fn generate_call_structs(
        &self,
        type_gen: &TypeGenerator,
        call: &PalletCallMetadata<PortableForm>,
    ) -> Vec<TokenStream2> {
        let ty = call.ty;
        let name = type_gen.resolve_type_path(ty.id(), &[]);
        use crate::generate_types::TypePath;
        match name {
            TypePath::Parameter(_) => panic!("Call type should be a Type"),
            TypePath::Type(ref ty) => {
                let ty = ty.ty();

                let type_def = ty.type_def();
                if let scale_info::TypeDef::Variant(variant) = type_def {
                    variant
                        .variants()
                        .iter()
                        .map(|var| {
                            use heck::CamelCase;
                            let name = format_ident!("{}", var.name().to_string().to_camel_case());
                            let args = var.fields().iter().filter_map(|field| {
                                field.name().map(|name| {
                                    let name = format_ident!("{}", name);
                                    let ty = type_gen.resolve_type_path(field.ty().id(), &[]);
                                    quote! { #name: #ty }
                                })
                            });

                            quote! {
                                #[derive(Debug, ::codec::Encode, ::codec::Decode)]
                                pub struct #name {
                                    #( #args ),*
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                } else {
                    panic!("Call type should be an variant/enum type")
                }
            }
        }
    }
}