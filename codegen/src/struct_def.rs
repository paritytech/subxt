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

use super::GeneratedTypeDerives;
use crate::types::{
    TypeGenerator,
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

#[derive(Debug)]
pub struct StructDef {
    pub name: syn::Ident,
    pub fields: StructDefFields,
    pub field_visibility: Option<syn::Visibility>,
    pub derives: GeneratedTypeDerives,
}

#[derive(Debug)]
pub enum StructDefFields {
    Named(Vec<(syn::Ident, TypePath)>),
    Unnamed(Vec<TypePath>),
}

impl StructDef {
    pub fn new(
        ident: &str,
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
            StructDefFields::Named(
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
            StructDefFields::Unnamed(
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
            fields,
            field_visibility,
            derives,
        }
    }

    pub fn named_fields(&self) -> Option<&[(syn::Ident, TypePath)]> {
        if let StructDefFields::Named(ref fields) = self.fields {
            Some(fields)
        } else {
            None
        }
    }
}

impl quote::ToTokens for StructDef {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let visibility = &self.field_visibility;
        let derives = &self.derives;
        tokens.extend(match self.fields {
            StructDefFields::Named(ref named_fields) => {
                let fields = named_fields.iter().map(|(name, ty)| {
                    let compact_attr =
                        ty.is_compact().then(|| quote!( #[codec(compact)] ));
                    quote! { #compact_attr #visibility #name: #ty }
                });
                let name = &self.name;
                quote! {
                    #derives
                    pub struct #name {
                        #( #fields ),*
                    }
                }
            }
            StructDefFields::Unnamed(ref unnamed_fields) => {
                let fields = unnamed_fields.iter().map(|ty| {
                    let compact_attr =
                        ty.is_compact().then(|| quote!( #[codec(compact)] ));
                    quote! { #compact_attr #visibility #ty }
                });
                let name = &self.name;
                quote! {
                    #derives
                    pub struct #name (
                        #( #fields ),*
                    );
                }
            }
        })
    }
}
