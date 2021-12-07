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
use proc_macro2::TokenStream;
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
}

impl CompositeDef {
    pub fn struct_def(
        ident: &str,
        type_params: &[TypeParameter],
        fields: &[scale_info::Field<PortableForm>],
        field_visibility: Option<syn::Visibility>,
        type_gen: &TypeGenerator,
    ) -> Self {
        let name = format_ident!("{}", ident.to_camel_case());
        let derives = type_gen.derives().clone();
        let type_params = type_params.iter().cloned().collect();
        let fields = CompositeDefFields::new(ident, type_gen, fields);

        Self {
            name,
            kind: CompositeDefKind::Struct {
                derives,
                type_params,
                field_visibility,
            },
            fields,
        }
    }

    pub fn enum_variant_def(
        ident: &str,
        fields: &[scale_info::Field<PortableForm>],
        type_gen: &TypeGenerator,
    ) -> Self {
        let name = format_ident!("{}", ident.to_camel_case());
        let fields = CompositeDefFields::new(ident, type_gen, fields);

        Self {
            name,
            kind: CompositeDefKind::EnumVariant,
            fields,
        }
    }

    pub fn named_fields(&self) -> Option<&[(syn::Ident, TypePath, Option<String>)]> {
        if let CompositeDefFields::Named(ref fields) = self.fields {
            Some(fields)
        } else {
            None
        }
    }
}

impl quote::ToTokens for CompositeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        fn unused_type_params_phantom_data<'a>(
            type_params: &'a [TypeParameter],
            types: impl Iterator<Item = &'a TypePath>,
        ) -> Option<syn::TypePath> {
            if type_params.is_empty() {
                return None
            }
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

            if !unused.is_empty() {
                Some(super::phantom_data(&unused))
            } else {
                None
            }
        }

        let name = &self.name;

        let decl = match &self.kind {
            CompositeDefKind::Struct {
                derives,
                type_params,
                field_visibility,
            } => {
                let unused_params_marker = unused_type_params_phantom_data(
                    type_params,
                    self.fields.field_types().iter(),
                );

                let fields = self
                    .fields
                    .field_tokens(field_visibility.as_ref(), unused_params_marker);

                quote! {
                    #derives
                    pub struct #name #fields
                }
            }
            CompositeDefKind::EnumVariant => {
                let fields = self.fields.field_tokens(None, None);

                quote! {
                    #name #fields
                }
            }
        };
        tokens.extend(decl)
    }
}

#[derive(Debug)]
pub enum CompositeDefKind {
    /// Composite type comprising a Rust `struct`.
    Struct {
        derives: GeneratedTypeDerives,
        type_params: Vec<TypeParameter>,
        field_visibility: Option<syn::Visibility>,
    },
    /// Comprises a variant of a Rust `enum`
    EnumVariant,
}

#[derive(Debug)]
pub enum CompositeDefFields {
    Named(Vec<(syn::Ident, TypePath, Option<String>)>),
    Unnamed(Vec<(TypePath, Option<String>)>),
}

impl CompositeDefFields {
    fn new(
        name: &str,
        type_gen: &TypeGenerator,
        fields: &[scale_info::Field<PortableForm>],
    ) -> Self {
        let fields = fields
            .iter()
            .map(|field| {
                let name = field.name().map(|f| format_ident!("{}", f));
                let ty = type_gen.resolve_type_path(field.ty().id(), &[]);
                (name, ty, field.type_name())
            })
            .collect::<Vec<_>>();

        let named = fields.iter().all(|(name, _, _)| name.is_some());
        let unnamed = fields.iter().all(|(name, _, _)| name.is_none());

        if named {
            Self::Named(
                fields
                    .iter()
                    .map(|(name, field, type_name)| {
                        let name = name.as_ref().unwrap_or_else(|| {
                            abort_call_site!("All fields should have a name")
                        });
                        (name.clone(), field.clone(), type_name.cloned())
                    })
                    .collect(),
            )
        } else if unnamed {
            Self::Unnamed(
                fields
                    .iter()
                    .map(|(_, field, type_name)| (field.clone(), type_name.cloned()))
                    .collect(),
            )
        } else {
            abort_call_site!(
                "Struct '{}': Fields should either be all named or all unnamed.",
                name,
            )
        }
    }

    fn field_types(&self) -> Vec<TypePath> {
        match self {
            Self::Named(fields) => fields.iter().map(|(_, ty, _)| ty.clone()).collect(),
            Self::Unnamed(fields) => fields.iter().map(|(ty, _)| ty.clone()).collect(),
        }
    }

    fn field_tokens(
        &self,
        visibility: Option<&syn::Visibility>,
        phantom_data: Option<syn::TypePath>,
    ) -> TokenStream {
        fn ty_path(ty_name: &Option<String>, ty_path: &TypePath) -> syn::TypePath {
            if let Some(ty_name) = ty_name {
                if ty_name.contains("Box<") {
                    return syn::parse_quote! { ::std::boxed::Box<#ty_path> }
                }
            }
            syn::parse_quote! { #ty_path }
        }

        match self {
            CompositeDefFields::Named(named_fields) => {
                let fields = named_fields.iter().map(|(name, ty, ty_name)| {
                    let compact_attr =
                        ty.is_compact().then(|| quote!( #[codec(compact)] ));
                    let ty = ty_path(ty_name, ty);
                    quote! { #compact_attr #visibility #name: #ty }
                });
                let marker = phantom_data
                    .map(|phantom_data| quote! ( #[codec(skip)] #visibility __subxt_unused_type_params: #phantom_data ));
                quote! (
                    {
                        #( #fields ),*
                        #marker,
                    }
                )
            }
            CompositeDefFields::Unnamed(ref unnamed_fields) => {
                let fields = unnamed_fields.iter().map(|(ty, ty_name)| {
                    let compact_attr =
                        ty.is_compact().then(|| quote!( #[codec(compact)] ));
                    let ty = ty_path(ty_name, ty);
                    quote! { #compact_attr #visibility #ty }
                });
                let marker = phantom_data.map(
                    |phantom_data| quote! ( #[codec(skip)] #visibility #phantom_data ),
                );
                quote! (
                    (
                        #( #fields ),*
                    )
                )
            }
        }
    }
}
