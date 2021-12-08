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
    TypeDefParameters,
    TypePath,
};
use heck::CamelCase as _;
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote};
use scale_info::{
    TypeDef,
    TypeDefPrimitive,
};

#[derive(Debug)]
pub struct CompositeDef {
    pub name: syn::Ident,
    pub kind: CompositeDefKind,
    pub fields: CompositeDefFields,
}

impl CompositeDef {
    pub fn struct_def(
        ident: &str,
        type_params: TypeDefParameters,
        fields: Vec<CompositeDefField>,
        field_visibility: Option<syn::Visibility>,
        type_gen: &TypeGenerator,
    ) -> Self {
        let mut derives = type_gen.derives().clone();

        if fields.len() == 1 {
            // any single field wrapper struct with a concrete unsigned int type can derive
            // CompactAs.
            let field = &fields[0];
            if type_params
                .params()
                .iter()
                .any(|tp| Some(&tp.name.to_string()) == field.type_name())
            {
                let ty = type_gen.resolve_type(field.ty().id());
                if matches!(
                        ty.type_def(),
                        TypeDef::Primitive(
                            TypeDefPrimitive::U8
                                | TypeDefPrimitive::U16
                                | TypeDefPrimitive::U32
                                | TypeDefPrimitive::U64
                                | TypeDefPrimitive::U128
                        )
                    ) {
                    derives.push_codec_compact_as()
                }
            }
        }

        let name = format_ident!("{}", ident.to_camel_case());
        let fields = CompositeDefFields::new(ident, fields);

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
        fields: Vec<CompositeDefField>,
    ) -> Self {
        let name = format_ident!("{}", ident.to_camel_case());
        let fields = CompositeDefFields::new(ident, fields);

        Self {
            name,
            kind: CompositeDefKind::EnumVariant,
            fields,
        }
    }

    pub fn named_fields(&self) -> Option<&[(syn::Ident, CompositeDefField)]> {
        if let CompositeDefFields::Named(ref fields) = self.fields {
            Some(fields)
        } else {
            None
        }
    }
}

impl quote::ToTokens for CompositeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        let decl = match &self.kind {
            CompositeDefKind::Struct {
                derives,
                type_params,
                field_visibility,
            } => {
                let unused_phantom_marker = type_params.unused_params_phantom_data();
                let fields = self
                    .fields
                    .field_tokens(field_visibility.as_ref(), unused_phantom_marker);

                quote! {
                    #derives
                    pub struct #name #type_params #fields
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
        type_params: TypeDefParameters,
        field_visibility: Option<syn::Visibility>,
    },
    /// Comprises a variant of a Rust `enum`
    EnumVariant,
}

#[derive(Debug)]
pub enum CompositeDefFields {
    Named(Vec<(syn::Ident, CompositeDefField)>),
    Unnamed(Vec<CompositeDefField>),
}

impl CompositeDefFields {
    fn new(
        name: &str,
        fields: Vec<CompositeDefField>,
    ) -> Self {
        let named = fields.iter().all(|field| field.name.is_some());
        let unnamed = fields.iter().all(|field| field.name.is_none());

        if named {
            Self::Named(
                fields
                    .into_iter()
                    .map(|field| {
                        let name = field.name.unwrap_or_else(|| {
                            abort_call_site!("All fields should have a name")
                        });
                        (name, field)
                    })
                    .collect(),
            )
        } else if unnamed {
            Self::Unnamed(fields)
        } else {
            abort_call_site!(
                "Struct '{}': Fields should either be all named or all unnamed.",
                name,
            )
        }
    }

    fn field_types(&self) -> Vec<&TypePath> {
        match self {
            Self::Named(fields) => fields.iter().map(|(_, ty)| &ty.type_path).collect(),
            Self::Unnamed(fields) => fields.iter().map(|ty| &ty.type_path).collect(),
        }
    }

    fn field_tokens(
        &self,
        visibility: Option<&syn::Visibility>,
        phantom_data: Option<syn::TypePath>,
    ) -> TokenStream {
        match self {
            CompositeDefFields::Named(named_fields) => {
                let fields = named_fields.iter().map(|(name, field_type)| {
                    let compact_attr = field_type.compact_attr();
                    quote! { #compact_attr #visibility #name: #field_type }
                });
                let marker = phantom_data
                    .map(|phantom_data| quote! ( #[codec(skip)] #visibility __subxt_unused_type_params: #phantom_data ));
                quote! (
                    {
                        #( #fields ),*
                        #marker
                    }
                )
            }
            CompositeDefFields::Unnamed(ref unnamed_fields) => {
                let fields = unnamed_fields.iter().map(|field_type| {
                    let compact_attr = field_type.compact_attr();
                    quote! { #compact_attr #visibility #field_type }
                });
                let marker = phantom_data.map(
                    |phantom_data| quote! ( #[codec(skip)] #visibility #phantom_data ),
                );
                quote! (
                    (
                        #( #fields ),*
                        #marker
                    );
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct CompositeDefField {
    name: Option<syn::Ident>,
    type_path: TypePath,
    type_name: Option<String>,
}

impl CompositeDefField {
    pub fn new(name: Option<syn::Ident>, type_path: TypePath, type_name: Option<String>) -> Self {
        CompositeDefField { name, type_path, type_name }
    }

    fn compact_attr(&self) -> Option<TokenStream> {
        self.type_path.is_compact().then(|| quote!( #[codec(compact)] ))
    }
}

impl quote::ToTokens for CompositeDefField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty_path = &self.type_path;
        if matches!(&self.type_name, Some(ty_name) if ty_name.contains("Box<")) {
            tokens.extend(quote! { ::std::boxed::Box<#ty_path> })
        } else {
            tokens.extend(quote! { #ty_path })
        };
    }
}
