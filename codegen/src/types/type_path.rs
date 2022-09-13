// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use scale_info::{
    form::PortableForm,
    Path,
    Type,
    TypeDef,
    TypeDefPrimitive,
};
use std::collections::BTreeSet;
use syn::parse_quote;

#[derive(Clone, Debug)]
pub enum TypePath {
    Parameter(TypeParameter),
    Type(TypePathType),
}

impl quote::ToTokens for TypePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let syn_type = self.to_syn_type();
        syn_type.to_tokens(tokens)
    }
}

impl TypePath {
    pub(crate) fn to_syn_type(&self) -> syn::Type {
        match self {
            TypePath::Parameter(ty_param) => syn::Type::Path(parse_quote! { #ty_param }),
            TypePath::Type(ty) => ty.to_syn_type(),
        }
    }

    pub(crate) fn is_compact(&self) -> bool {
        matches!(self, Self::Type(ty) if ty.is_compact())
    }

    /// Returns the type parameters in a path which are inherited from the containing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// struct S<T> {
    ///     a: Vec<Option<T>>, // the parent type param here is `T`
    /// }
    /// ```
    pub fn parent_type_params(&self, acc: &mut BTreeSet<TypeParameter>) {
        match self {
            Self::Parameter(type_parameter) => {
                acc.insert(type_parameter.clone());
            }
            Self::Type(type_path) => type_path.parent_type_params(acc),
        }
    }

    /// Gets the vector type parameter if the data is represented as `TypeDef::Sequence`.
    ///
    /// **Note:** Utilized for transforming `std::vec::Vec<T>` into slices `&[T]` for the storage API.
    pub fn vec_type_param(&self) -> Option<&TypePath> {
        let ty = match self {
            TypePath::Type(ty) => ty,
            _ => return None,
        };

        match ty.kind {
            TypePathTypeKind::Vec { ref of } => Some(of),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypePathType {
    pub(super) kind: TypePathTypeKind,
    pub(super) root_mod_ident: Ident,
}

impl TypePathType {
    pub(crate) fn is_compact(&self) -> bool {
        matches!(self.kind, TypePathTypeKind::Compact { .. })
    }

    fn to_syn_type(&self) -> syn::Type {
        match &self.kind {
            TypePathTypeKind::Path { path, params } => {
                let path = if params.is_empty() {
                    parse_quote! { #path }
                } else {
                    parse_quote! { #path< #( #params ),* > }
                };
                syn::Type::Path(path)
            }
            TypePathTypeKind::Vec { of } => {
                let type_path = parse_quote! { ::std::vec::Vec<#of> };
                syn::Type::Path(type_path)
            }
            TypePathTypeKind::Array { len, of } => {
                let array = parse_quote! { [#of; #len] };
                syn::Type::Array(array)
            }
            TypePathTypeKind::Tuple { elements } => {
                let tuple = parse_quote! { (#( # elements, )* ) };
                syn::Type::Tuple(tuple)
            }
            TypePathTypeKind::Primitive { def } => {
                syn::Type::Path(match def {
                    TypeDefPrimitive::Bool => parse_quote!(::core::primitive::bool),
                    TypeDefPrimitive::Char => parse_quote!(::core::primitive::char),
                    TypeDefPrimitive::Str => parse_quote!(::std::string::String),
                    TypeDefPrimitive::U8 => parse_quote!(::core::primitive::u8),
                    TypeDefPrimitive::U16 => parse_quote!(::core::primitive::u16),
                    TypeDefPrimitive::U32 => parse_quote!(::core::primitive::u32),
                    TypeDefPrimitive::U64 => parse_quote!(::core::primitive::u64),
                    TypeDefPrimitive::U128 => parse_quote!(::core::primitive::u128),
                    TypeDefPrimitive::U256 => unimplemented!("not a rust primitive"),
                    TypeDefPrimitive::I8 => parse_quote!(::core::primitive::i8),
                    TypeDefPrimitive::I16 => parse_quote!(::core::primitive::i16),
                    TypeDefPrimitive::I32 => parse_quote!(::core::primitive::i32),
                    TypeDefPrimitive::I64 => parse_quote!(::core::primitive::i64),
                    TypeDefPrimitive::I128 => parse_quote!(::core::primitive::i128),
                    TypeDefPrimitive::I256 => unimplemented!("not a rust primitive"),
                })
            }
            TypePathTypeKind::Compact { inner, is_field } => {
                let path = if *is_field {
                    parse_quote! ( #inner )
                } else {
                    parse_quote! ( ::subxt::ext::codec::Compact<#inner> )
                };
                syn::Type::Path(path)
            }
            TypePathTypeKind::BitVec {
                bit_order_type,
                bit_store_type,
            } => {
                let type_path = parse_quote! { ::subxt::ext::bitvec::vec::BitVec<#bit_store_type, #bit_order_type> };
                syn::Type::Path(type_path)
            }
        }
    }

    /// Returns the type parameters in a path which are inherited from the containing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// struct S<T> {
    ///     a: Vec<Option<T>>, // the parent type param here is `T`
    /// }
    /// ```
    fn parent_type_params(&self, acc: &mut BTreeSet<TypeParameter>) {
        for p in &self.params {
            p.parent_type_params(acc);
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypePathTypeKind {
    Path {
        path: syn::TypePath,
        params: Vec<TypePath>,
    },
    Vec {
        of: Box<TypePath>,
    },
    Array {
        len: usize,
        of: Box<TypePath>,
    },
    Tuple {
        elements: Vec<TypePath>,
    },
    Primitive {
        def: TypeDefPrimitive,
    },
    Compact {
        inner: Box<TypePath>,
        is_field: bool,
    },
    BitVec {
        bit_order_type: Box<TypePath>,
        bit_store_type: Box<TypePath>,
    },
}

impl TypePathTypeKind {
    pub fn from_type_def_path(
        path: &Path<PortableForm>,
        root_mod_ident: Ident,
        params: Vec<TypePath>,
    ) -> Self {
        let path_segments = path.segments();

        let path: syn::TypePath = match path_segments {
            [] => panic!("Type has no ident"),
            [ident] => {
                // paths to prelude types
                match ident.as_str() {
                    "Option" => parse_quote!(::core::option::Option),
                    "Result" => parse_quote!(::core::result::Result),
                    "Cow" => parse_quote!(::std::borrow::Cow),
                    "BTreeMap" => parse_quote!(::std::collections::BTreeMap),
                    "BTreeSet" => parse_quote!(::std::collections::BTreeSet),
                    "Range" => parse_quote!(::core::ops::Range),
                    "RangeInclusive" => parse_quote!(::core::ops::RangeInclusive),
                    ident => panic!("Unknown prelude type '{}'", ident),
                }
            }
            _ => {
                // paths to generated types in the root types module
                let mut ty_path = path_segments
                    .iter()
                    .map(|s| syn::PathSegment::from(format_ident!("{}", s)))
                    .collect::<syn::punctuated::Punctuated<
                        syn::PathSegment,
                        syn::Token![::],
                    >>();
                ty_path.insert(0, syn::PathSegment::from(root_mod_ident));
                parse_quote!( #ty_path )
            }
        };
        Self::Path { path, params }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypeParameter {
    pub(super) concrete_type_id: u32,
    pub(super) original_name: String,
    pub(super) name: Ident,
}

impl quote::ToTokens for TypeParameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens)
    }
}
