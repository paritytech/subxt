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
    Field,
    GeneratedTypeDerives,
    TypeDefParameters,
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
        fields_def: CompositeDefFields,
        field_visibility: Option<syn::Visibility>,
        type_gen: &TypeGenerator,
    ) -> Self {
        let mut derives = type_gen.derives().clone();
        let fields = fields_def.fields();

        if fields.len() == 1 {
            // any single field wrapper struct with a concrete unsigned int type can derive
            // CompactAs.
            let field = &fields[0];
            if !type_params
                .params()
                .iter()
                .any(|tp| Some(tp.original_name.to_string()) == field.type_name)
            {
                let ty = type_gen.resolve_type(field.type_id);
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

        Self {
            name,
            kind: CompositeDefKind::Struct {
                derives,
                type_params,
                field_visibility,
            },
            fields: fields_def,
        }
    }

    pub fn enum_variant_def(ident: &str, fields: CompositeDefFields) -> Self {
        let name = format_ident!("{}", ident);
        Self {
            name,
            kind: CompositeDefKind::EnumVariant,
            fields,
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
                let fields = self.fields.field_tokens(
                    field_visibility.as_ref(),
                    unused_phantom_marker,
                    true,
                );

                quote! {
                    #derives
                    pub struct #name #type_params #fields
                }
            }
            CompositeDefKind::EnumVariant => {
                let fields = self.fields.field_tokens(None, None, false);

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
    /// Comprises a variant of a Rust `enum`.
    EnumVariant,
}

#[derive(Debug)]
pub struct CompositeDefFields {
    named: bool,
    fields: Vec<CompositeDefField>,
}

impl CompositeDefFields {
    pub fn new(name: &str, fields: Vec<CompositeDefField>) -> Self {
        let named = fields.iter().all(|field| field.name.is_some());
        let unnamed = fields.iter().all(|field| field.name.is_none());

        if !named && !unnamed {
            abort_call_site!(
                "Struct '{}': Fields should either be all named or all unnamed.",
                name,
            )
        }

        Self { named, fields }
    }

    pub fn from_scale_info_fields(
        name: &str,
        fields: &[Field],
        parent_type_params: &[TypeParameter],
        type_gen: &TypeGenerator,
    ) -> Self {
        let composite_def_fields = fields
            .iter()
            .map(|field| {
                let name = field.name().map(|f| format_ident!("{}", f));
                let type_path =
                    type_gen.resolve_type_path(field.ty().id(), parent_type_params);
                CompositeDefField::new(
                    name,
                    field.ty().id(),
                    type_path,
                    field.type_name().cloned(),
                )
            })
            .collect();
        Self::new(name, composite_def_fields)
    }

    pub fn fields(&self) -> &[CompositeDefField] {
        &self.fields
    }

    fn field_tokens(
        &self,
        visibility: Option<&syn::Visibility>,
        phantom_data: Option<syn::TypePath>,
        unnamed_trailing_semicolon: bool,
    ) -> TokenStream {
        let trailing_semicolon = unnamed_trailing_semicolon.then(|| quote!(;));
        if self.fields.is_empty() {
            return if let Some(phantom_data) = phantom_data {
                quote! { ( #phantom_data )#trailing_semicolon }
            } else {
                quote! { #trailing_semicolon }
            }
        }
        let fields = self.fields.iter().map(|field| {
            let compact_attr = field
                .type_path
                .is_compact()
                .then(|| quote!( #[codec(compact)] ));
            quote! { #compact_attr #visibility #field }
        });
        if self.named {
            let marker = phantom_data
                .map(|phantom_data| quote!( #[codec(skip)] #visibility __subxt_unused_type_params: #phantom_data ));
            quote!(
                {
                    #( #fields, )*
                    #marker
                }
            )
        } else {
            let marker = phantom_data
                .map(|phantom_data| quote!( #[codec(skip)] #visibility #phantom_data ));
            quote! {
                (
                    #( #fields, )*
                    #marker
                )#trailing_semicolon
            }
        }
    }

    pub fn named_fields(&self) -> Option<impl Iterator<Item = (syn::Ident, &CompositeDefField)>> {
        if self.named {
            Some(self.fields.iter().map(|f| {
                let type_name = f.name.as_ref().expect("All fields have names");
                let ident = format_ident!("{}", type_name);
                (ident, f)
            }))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct CompositeDefField {
    pub name: Option<syn::Ident>,
    pub type_id: u32,
    pub type_path: TypePath,
    pub type_name: Option<String>,
}

impl CompositeDefField {
    pub fn new(
        name: Option<syn::Ident>,
        type_id: u32,
        type_path: TypePath,
        type_name: Option<String>,
    ) -> Self {
        CompositeDefField {
            name,
            type_id,
            type_path,
            type_name,
        }
    }

    /// Returns `true` if the field is a [`::std::boxed::Box`].
    pub fn is_boxed(&self) -> bool {
        // Use the type name to detect a `Box` field.
        // Should be updated once `Box` types are no longer erased:
        // https://github.com/paritytech/scale-info/pull/82
        matches!(&self.type_name, Some(ty_name) if ty_name.contains("Box<"))
    }
}

impl quote::ToTokens for CompositeDefField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Prepend the field name if it is a named field.
        if let Some(ref name) = self.name {
            tokens.extend(quote! { #name: })
        }
        let ty_path = &self.type_path;

        if self.is_boxed() {
            tokens.extend(quote! { ::std::boxed::Box<#ty_path> })
        } else {
            tokens.extend(quote! { #ty_path })
        };
    }
}
