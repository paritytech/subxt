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

use super::{
    GeneratedTypeDerives,
    TypeGenerator,
    TypeParameter,
    TypePath,
};
use heck::CamelCase as _;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::{
    format_ident,
    quote,
};
use scale_info::form::PortableForm;
use std::collections::HashSet;

#[derive(Debug)]
pub struct CompositeDef {
    pub name: syn::Ident,
    pub kind: CompositeDefKind,
    pub fields: CompositeDefFields,
    pub field_visibility: Option<syn::Visibility>,
    pub derives: GeneratedTypeDerives,
}

#[derive(Debug)]
pub enum CompositeDefKind {
    /// Composite type comprising a Rust `struct`.
    Struct,
    /// Comprises a variant of a Rust `enum`
    EnumVariant,
}

#[derive(Debug)]
pub enum CompositeDefFields {
    Named(Vec<(syn::Ident, TypePath)>),
    Unnamed(Vec<TypePath>),
}

impl CompositeDef {
    pub fn new(
        ident: &str,
        kind: CompositeDefKind,
        fields: &[scale_info::Field<PortableForm>],
        field_visibility: Option<syn::Visibility>,
        type_gen: &TypeGenerator,
    ) -> Self {
        let name = format_ident!("{}", ident.to_camel_case());
        let fields = fields
            .iter()
            .map(|field| {
                let name = field.name().map(|f| format_ident!("{}", f));
                let ty = type_gen.resolve_type_path(field.ty().id(), &[]);
                (name, ty)
            })
            .collect::<Vec<_>>();

        let named = fields.iter().all(|(name, _)| name.is_some());
        let unnamed = fields.iter().all(|(name, _)| name.is_none());

        let fields = if named {
            CompositeDefFields::Named(
                fields
                    .iter()
                    .map(|(name, field)| {
                        let name = name.as_ref().unwrap_or_else(|| {
                            abort_call_site!("All fields should have a name")
                        });
                        (name.clone(), field.clone())
                    })
                    .collect(),
            )
        } else if unnamed {
            CompositeDefFields::Unnamed(
                fields.iter().map(|(_, field)| field.clone()).collect(),
            )
        } else {
            abort_call_site!(
                "Struct '{}': Fields should either be all named or all unnamed.",
                name,
            )
        };

        let derives = type_gen.derives().clone();

        Self {
            name,
            kind,
            fields,
            field_visibility,
            derives,
        }
    }

    pub fn named_fields(&self) -> Option<&[(syn::Ident, TypePath)]> {
        if let CompositeDefFields::Named(ref fields) = self.fields {
            Some(fields)
        } else {
            None
        }
    }
}

impl quote::ToTokens for CompositeDef {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        fn unused_type_params<'a>(
            type_params: &'a [TypeParameter],
            types: impl Iterator<Item = &'a TypePath>,
        ) -> Vec<TypeParameter> {
            let mut used_type_params = HashSet::new();
            for ty in types {
                ty.parent_type_params(&mut used_type_params)
            }
            let type_params_set: HashSet<_> = type_params.iter().cloned().collect();
            let mut unused = type_params_set
                .difference(&used_type_params)
                .cloned()
                .collect::<Vec<_>>();
            unused.sort();
            unused
        }

        fn ty_toks(ty_name: &str, ty_path: &TypePath) -> TokenStream2 {
            if ty_name.contains("Box<") {
                quote! { ::std::boxed::Box<#ty_path> }
            } else {
                quote! { #ty_path }
            }
        };

        let visibility = &self.field_visibility;
        let derives = &self.derives;
        let name = &self.name;
        let decl_struct = matches!(self.kind, CompositeDefKind::Struct)
            .then(|| quote!( pub struct ));
        tokens.extend(match self.fields {
            CompositeDefFields::Named(ref named_fields) => {
                let fields = named_fields.iter().map(|(name, ty)| {
                    let compact_attr =
                        ty.is_compact().then(|| quote!( #[codec(compact)] ));
                    quote! { #compact_attr #visibility #name: #ty }
                });
                quote! {
                    #derives
                    #decl_struct #name {
                        #( #fields ),*
                    }
                }
            }
            CompositeDefFields::Unnamed(ref unnamed_fields) => {
                let fields = unnamed_fields.iter().map(|ty| {
                    let compact_attr =
                        ty.is_compact().then(|| quote!( #[codec(compact)] ));
                    quote! { #compact_attr #visibility #ty }
                });
                quote! {
                    #derives
                    #decl_struct #name (
                        #( #fields ),*
                    );
                }
            }
        })
    }
}
