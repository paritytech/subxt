// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::CodegenError;

use super::{CratePath, Derives, Field, TypeDefParameters, TypeGenerator, TypeParameter, TypePath};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use scale_info::{form::PortableForm, Type, TypeDef, TypeDefPrimitive};

/// Representation of a type which consists of a set of fields. Used to generate Rust code for
/// either a standalone `struct` definition, or an `enum` variant.
///
/// Fields can either be named or unnamed in either case.
#[derive(Debug)]
pub struct CompositeDef {
    /// The name of the `struct`, or the name of the `enum` variant.
    pub name: syn::Ident,
    /// Generate either a standalone `struct` or an `enum` variant.
    pub kind: CompositeDefKind,
    /// The fields of the type, which are either all named or all unnamed.
    pub fields: CompositeDefFields,
    /// Documentation of the composite type as presented in metadata.
    pub docs: Option<TokenStream>,
}

impl CompositeDef {
    /// Construct a definition which will generate code for a standalone `struct`.
    #[allow(clippy::too_many_arguments)]
    pub fn struct_def(
        ty: &Type<PortableForm>,
        ident: &str,
        type_params: TypeDefParameters,
        fields_def: CompositeDefFields,
        field_visibility: Option<syn::Visibility>,
        type_gen: &TypeGenerator,
        docs: &[String],
        crate_path: &CratePath,
    ) -> Result<Self, CodegenError> {
        let mut derives = type_gen.type_derives(ty)?;
        let fields: Vec<_> = fields_def.field_types().collect();

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
                    ty.type_def,
                    TypeDef::Primitive(
                        TypeDefPrimitive::U8
                            | TypeDefPrimitive::U16
                            | TypeDefPrimitive::U32
                            | TypeDefPrimitive::U64
                            | TypeDefPrimitive::U128
                    )
                ) {
                    derives.insert_codec_compact_as(crate_path)
                }
            }
        }

        let name = format_ident!("{}", ident);
        let docs_token = Some(quote! { #( #[doc = #docs ] )* });

        Ok(Self {
            name,
            kind: CompositeDefKind::Struct {
                derives,
                type_params,
                field_visibility,
            },
            fields: fields_def,
            docs: docs_token,
        })
    }

    /// Construct a definition which will generate code for an `enum` variant.
    pub fn enum_variant_def(ident: &str, fields: CompositeDefFields, docs: &[String]) -> Self {
        let name = format_ident!("{}", ident);
        let docs_token = Some(quote! { #( #[doc = #docs ] )* });
        Self {
            name,
            kind: CompositeDefKind::EnumVariant,
            fields,
            docs: docs_token,
        }
    }
}

impl quote::ToTokens for CompositeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let docs = &self.docs;

        let decl = match &self.kind {
            CompositeDefKind::Struct {
                derives,
                type_params,
                field_visibility,
            } => {
                let phantom_data = type_params.unused_params_phantom_data();
                let fields = self
                    .fields
                    .to_struct_field_tokens(phantom_data, field_visibility.as_ref());
                let trailing_semicolon = matches!(
                    self.fields,
                    CompositeDefFields::NoFields | CompositeDefFields::Unnamed(_)
                )
                .then(|| quote!(;));

                quote! {
                    #derives
                    #docs
                    pub struct #name #type_params #fields #trailing_semicolon
                }
            }
            CompositeDefKind::EnumVariant => {
                let fields = self.fields.to_enum_variant_field_tokens();

                quote! {
                    #docs
                    #name #fields
                }
            }
        };
        tokens.extend(decl)
    }
}

/// Which kind of composite type are we generating, either a standalone `struct` or an `enum`
/// variant.
#[derive(Debug)]
pub enum CompositeDefKind {
    /// Composite type comprising a Rust `struct`.
    Struct {
        derives: Derives,
        type_params: TypeDefParameters,
        field_visibility: Option<syn::Visibility>,
    },
    /// Comprises a variant of a Rust `enum`.
    EnumVariant,
}

/// Encapsulates the composite fields, keeping the invariant that all fields are either named or
/// unnamed.
#[derive(Debug)]
pub enum CompositeDefFields {
    NoFields,
    Named(Vec<(syn::Ident, CompositeDefFieldType)>),
    Unnamed(Vec<CompositeDefFieldType>),
}

impl CompositeDefFields {
    /// Construct a new set of composite fields from the supplied [`::scale_info::Field`]s.
    pub fn from_scale_info_fields(
        name: &str,
        fields: &[Field],
        parent_type_params: &[TypeParameter],
        type_gen: &TypeGenerator,
    ) -> Result<Self, CodegenError> {
        if fields.is_empty() {
            return Ok(Self::NoFields);
        }

        let mut named_fields = Vec::new();
        let mut unnamed_fields = Vec::new();

        for field in fields {
            let type_path = type_gen.resolve_field_type_path(
                field.ty.id,
                parent_type_params,
                field.type_name.as_deref(),
            );
            let field_type =
                CompositeDefFieldType::new(field.ty.id, type_path, field.type_name.clone());

            if let Some(name) = &field.name {
                let field_name = format_ident!("{}", name);
                named_fields.push((field_name, field_type))
            } else {
                unnamed_fields.push(field_type)
            }
        }

        if !named_fields.is_empty() && !unnamed_fields.is_empty() {
            return Err(CodegenError::InvalidFields(name.into()));
        }

        let res = if !named_fields.is_empty() {
            Self::Named(named_fields)
        } else {
            Self::Unnamed(unnamed_fields)
        };
        Ok(res)
    }

    /// Returns the set of composite fields.
    pub fn field_types(&self) -> Box<dyn Iterator<Item = &CompositeDefFieldType> + '_> {
        match self {
            Self::NoFields => Box::new([].iter()),
            Self::Named(named_fields) => Box::new(named_fields.iter().map(|(_, f)| f)),
            Self::Unnamed(unnamed_fields) => Box::new(unnamed_fields.iter()),
        }
    }

    /// Generate the code for fields which will compose a `struct`.
    pub fn to_struct_field_tokens(
        &self,
        phantom_data: Option<syn::TypePath>,
        visibility: Option<&syn::Visibility>,
    ) -> TokenStream {
        match self {
            Self::NoFields => {
                if let Some(phantom_data) = phantom_data {
                    quote! { ( #phantom_data ) }
                } else {
                    quote! {}
                }
            }
            Self::Named(ref fields) => {
                let fields = fields.iter().map(|(name, ty)| {
                    let compact_attr = ty.compact_attr();
                    quote! { #compact_attr #visibility #name: #ty }
                });
                let marker = phantom_data.map(|phantom_data| {
                    quote!(
                        #[codec(skip)]
                        #visibility __subxt_unused_type_params: #phantom_data
                    )
                });
                quote!(
                    {
                        #( #fields, )*
                        #marker
                    }
                )
            }
            Self::Unnamed(ref fields) => {
                let fields = fields.iter().map(|ty| {
                    let compact_attr = ty.compact_attr();
                    quote! { #compact_attr #visibility #ty }
                });
                let marker = phantom_data.map(|phantom_data| {
                    quote!(
                        #[codec(skip)]
                        #visibility #phantom_data
                    )
                });
                quote! {
                    (
                        #( #fields, )*
                        #marker
                    )
                }
            }
        }
    }

    /// Generate the code for fields which will compose an `enum` variant.
    pub fn to_enum_variant_field_tokens(&self) -> TokenStream {
        match self {
            Self::NoFields => quote! {},
            Self::Named(ref fields) => {
                let fields = fields.iter().map(|(name, ty)| {
                    let compact_attr = ty.compact_attr();
                    quote! { #compact_attr #name: #ty }
                });
                quote!( { #( #fields, )* } )
            }
            Self::Unnamed(ref fields) => {
                let fields = fields.iter().map(|ty| {
                    let compact_attr = ty.compact_attr();
                    quote! { #compact_attr #ty }
                });
                quote! { ( #( #fields, )* ) }
            }
        }
    }
}

/// Represents a field of a composite type to be generated.
#[derive(Debug)]
pub struct CompositeDefFieldType {
    pub type_id: u32,
    pub type_path: TypePath,
    pub type_name: Option<String>,
}

impl CompositeDefFieldType {
    /// Construct a new [`CompositeDefFieldType`].
    pub fn new(type_id: u32, type_path: TypePath, type_name: Option<String>) -> Self {
        CompositeDefFieldType {
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

    /// Returns the `#[codec(compact)]` attribute if the type is compact.
    fn compact_attr(&self) -> Option<TokenStream> {
        self.type_path
            .is_compact()
            .then(|| quote!( #[codec(compact)] ))
    }
}

impl quote::ToTokens for CompositeDefFieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty_path = &self.type_path;

        if self.is_boxed() {
            tokens.extend(quote! { ::std::boxed::Box<#ty_path> })
        } else {
            tokens.extend(quote! { #ty_path })
        };
    }
}
