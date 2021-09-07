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

use crate::{
    TokenStream2,
    TypeGenerator,
    TypePath,
};
use codec::Decode;
use darling::FromMeta;
use frame_metadata::{
    v14::RuntimeMetadataV14,
    PalletCallMetadata,
    PalletMetadata,
    PalletStorageMetadata,
    RuntimeMetadata,
    RuntimeMetadataPrefixed,
    StorageEntryMetadata,
    StorageEntryType,
    StorageHasher,
};
use heck::SnakeCase as _;
use proc_macro_error::{
    abort,
    abort_call_site,
};
use quote::{
    format_ident,
    quote,
};
use scale_info::{
    form::PortableForm,
    prelude::string::ToString,
    TypeDef,
};
use std::{
    collections::HashMap,
    fs,
    io::Read,
    path,
};
use syn::spanned::Spanned as _;

pub fn generate_runtime_types<P>(item_mod: syn::ItemMod, path: P) -> TokenStream2
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
    generator.generate_runtime(item_mod)
}

#[derive(Debug, FromMeta)]
#[darling(rename_all = "snake_case")]
enum Subxt {
    SubstituteType(String),
}

impl Subxt {
    fn substitute_type(&self) -> String {
        match self {
            Self::SubstituteType(path) => path.clone(),
        }
    }
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
        let type_substitutes = Self::parse_type_substitutes(&item_mod);
        let type_gen =
            TypeGenerator::new(&self.metadata.types, "__types", type_substitutes);
        let types_mod = type_gen.generate_types_mod();
        let types_mod_ident = types_mod.ident();
        let modules = self.metadata.pallets.iter().map(|pallet| {
            let mod_name = format_ident!("{}", pallet.name.to_string().to_snake_case());

            let calls = if let Some(ref calls) = pallet.calls {
                let call_structs = self.generate_call_structs(&type_gen, pallet, calls);
                quote! {
                    pub mod calls {
                        use super::#types_mod_ident;
                        #( #call_structs )*
                    }
                }
            } else {
                quote!()
            };

            let event = if let Some(ref event) = pallet.event {
                let event_type = type_gen.resolve_type_path(event.ty.id(), &[]);
                quote! {
                    pub type Event = #event_type;
                }
            } else {
                quote!()
            };

            let storage = if let Some(ref storage) = pallet.storage {
                let storage_types = storage
                    .entries
                    .iter()
                    .map(|entry| {
                        self.generate_storage_entry_types(&type_gen, &pallet, entry)
                    })
                    .collect::<Vec<_>>();
                quote! {
                    pub mod storage {
                        use super::#types_mod_ident;
                        #( #storage_types )*
                    }
                }
            } else {
                quote!()
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

        // todo: [AJ] keep all other code items from decorated mod?
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

    fn parse_type_substitutes(item_mod: &syn::ItemMod) -> HashMap<String, syn::TypePath> {
        if let Some(ref content) = item_mod.content {
            content
                .1
                .iter()
                .filter_map(|item| {
                    if let syn::Item::Use(use_) = item {
                        let substitute_attrs = use_
                            .attrs
                            .iter()
                            .map(|attr| {
                                let meta = attr.parse_meta().unwrap_or_else(|e| {
                                    abort!(attr.span(), "Error parsing attribute: {}", e)
                                });
                                let substitute_type_args =
                                    Subxt::from_meta(&meta).unwrap(); // todo
                                substitute_type_args
                            })
                            .collect::<Vec<_>>();
                        if substitute_attrs.len() > 1 {
                            abort!(
                                use_.attrs[0].span(),
                                "Duplicate `substitute_type` attributes"
                            )
                        }
                        substitute_attrs.iter().next().map(|attr| {
                            let substitute_type = attr.substitute_type();
                            let use_path = &use_.tree;
                            let use_type_path = syn::parse_quote!( #use_path );
                            (substitute_type.to_string(), use_type_path)
                        })
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            HashMap::new()
        }
    }

    fn generate_call_structs(
        &self,
        type_gen: &TypeGenerator,
        pallet: &PalletMetadata<PortableForm>,
        call: &PalletCallMetadata<PortableForm>,
    ) -> Vec<TokenStream2> {
        let ty = call.ty;
        let name = type_gen.resolve_type_path(ty.id(), &[]);
        match name {
            TypePath::Parameter(_) => panic!("Call type should be a Parameter"),
            TypePath::Substitute(_) => panic!("Call type should not be a Substitute"),
            TypePath::Type(ref ty) => {
                let ty = ty.ty();

                let type_def = ty.type_def();
                if let scale_info::TypeDef::Variant(variant) = type_def {
                    variant
                        .variants()
                        .iter()
                        .map(|var| {
                            use heck::CamelCase;
                            let name = format_ident!(
                                "{}",
                                var.name().to_string().to_camel_case()
                            );
                            let args = var.fields().iter().filter_map(|field| {
                                field.name().map(|name| {
                                    let name = format_ident!("{}", name);
                                    let ty =
                                        type_gen.resolve_type_path(field.ty().id(), &[]);
                                    quote! { pub #name: #ty }
                                })
                            });

                            let pallet_name = &pallet.name;
                            let function_name = var.name().to_string();

                            quote! {
                                #[derive(Debug, ::codec::Encode, ::codec::Decode)]
                                pub struct #name {
                                    #( #args ),*
                                }

                                impl ::subxt::Call for #name {
                                    const PALLET: &'static str = #pallet_name;
                                    const FUNCTION: &'static str = #function_name;
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                } else {
                    abort_call_site!("Call type should be an variant/enum type")
                }
            }
        }
    }

    fn generate_storage_entry_types(
        &self,
        type_gen: &TypeGenerator,
        pallet: &PalletMetadata<PortableForm>,
        storage_entry: &StorageEntryMetadata<PortableForm>,
    ) -> TokenStream2 {
        let entry_struct_ident = format_ident!("{}", storage_entry.name);
        let (entry_struct, key_impl) = match storage_entry.ty {
            StorageEntryType::Plain(_) => {
                let entry_struct = quote!( pub struct #entry_struct_ident; );
                let key_impl = quote!(::subxt::StorageKey::Plain);
                (entry_struct, key_impl)
            }
            StorageEntryType::Map {
                ref key,
                ref hashers,
                ..
            } => {
                let key_ty = self.metadata.types.resolve(key.id()).unwrap_or_else(|| {
                    abort_call_site!("Failed to resolve storage key type")
                });
                let hashers = hashers.iter().map(|hasher| {
                    let hasher = match hasher {
                        StorageHasher::Blake2_128 => "Blake2_128",
                        StorageHasher::Blake2_256 => "Blake2_256",
                        StorageHasher::Blake2_128Concat => "Blake2_128Concat",
                        StorageHasher::Twox128 => "Twox128",
                        StorageHasher::Twox256 => "Twox256",
                        StorageHasher::Twox64Concat => "Twox64Concat",
                        StorageHasher::Identity => "Identity",
                    };
                    quote!( ::subxt::StorageHasher::#hasher )
                }).collect::<Vec<_>>();
                match key_ty.type_def() {
                    TypeDef::Tuple(tuple) => {
                        let fields = tuple
                            .fields()
                            .iter()
                            .map(|f| type_gen.resolve_type_path(f.id(), &[]));
                        let entry_struct = quote! {
                            pub struct #entry_struct_ident( #( #fields ),* );
                        };
                        let keys = (0..tuple.fields().len())
                            .into_iter()
                            .zip(hashers)
                            .map(|(field, hasher)| {
                                quote!( ::subxt::StorageMapKey::new(self.#field, #hasher) )
                            });
                        let key_impl = quote! {
                            ::subxt::StorageKey::Map(
                                vec![ #( #keys ),* ]
                            )
                        };
                        (entry_struct, key_impl)
                    }
                    TypeDef::Composite(composite) => {
                        // todo: [AJ] extract this pattern also used in ModuleType::composite_fields?
                        let named = composite.fields().iter().all(|f| f.name().is_some());
                        let unnamed =
                            composite.fields().iter().all(|f| f.name().is_none());

                        if named {
                            let fields = composite
                                .fields()
                                .iter()
                                .map(|f| {
                                    let field_name = format_ident!(
                                        "{}",
                                        f.name().expect("field is named")
                                    );
                                    let field_type =
                                        type_gen.resolve_type_path(f.ty().id(), &[]);
                                    (field_name, field_type)
                                })
                                .collect::<Vec<_>>();
                            let fields_def =
                                fields.iter().map(|(name, ty)| quote!( pub #name: #ty));
                            let entry_struct = quote! {
                                pub struct #entry_struct_ident (
                                    #( #fields_def, )*
                                )
                            };
                            let keys = fields
                                .iter()
                                .zip(hashers)
                                .map(|((field, _), hasher)| {
                                    quote!( ::subxt::StorageMapKey::new(self.#field, #hasher) )
                                });
                            let key_impl = quote! {
                                ::subxt::StorageKey::Map(
                                    vec![ #( #keys ),* ]
                                )
                            };
                            (entry_struct, key_impl)
                        } else if unnamed {
                            let fields = composite.fields().iter().map(|f| {
                                type_gen.resolve_type_path(f.ty().id(), &[])
                            }).collect::<Vec<_>>();
                            let fields_def = fields.iter().map(|field_type| quote!( pub #field_type ));
                            let entry_struct = quote! {
                                pub struct #entry_struct_ident {
                                    #( #fields, )*
                                }
                            };
                            let keys = fields
                                .iter()
                                .zip(hashers)
                                .map(|(field, hasher)| {
                                    quote!( ::subxt::StorageMapKey::new(self.#field, #hasher) )
                                });
                            let key_impl = quote! {
                                ::subxt::StorageKey::Map(
                                    vec![ #( #keys ),* ]
                                )
                            };
                            (entry_struct, key_impl)
                        } else {
                            abort_call_site!(
                                "Fields must be either all named or all unnamed"
                            )
                        }
                    }
                    _ => {
                        let ty_path = type_gen.resolve_type_path(key.id(), &[]);
                        let entry_struct = quote! {
                            pub struct #entry_struct_ident(#ty_path);
                        };
                        let hasher = hashers.get(0).unwrap_or_else(|| abort_call_site!("No hasher found for single key"));
                        let key_impl = quote! {
                            ::subxt::StorageKey::Map(
                                vec![ ::subxt::StorageMapKey::new(self.0, #hasher) ]
                            )
                        };
                        (entry_struct, key_impl)
                    }
                }
            }
        };
        let pallet_name = &pallet.name;
        let storage_name = &storage_entry.name;
        let value_ty = match storage_entry.ty {
            StorageEntryType::Plain(ref ty) => ty,
            StorageEntryType::Map { ref value, .. } => value,
        };
        let value_ty_path = type_gen.resolve_type_path(value_ty.id(), &[]);

        quote! {
            #entry_struct

            impl ::subxt::StorageEntry for #entry_struct_ident {
                const PALLET: &'static str = #pallet_name;
                const STORAGE: &'static str = #storage_name;
                type Value = #value_ty_path;
                fn key(&self) -> ::subxt::StorageKey {
                    #key_impl
                }
            }
        }
    }
}
