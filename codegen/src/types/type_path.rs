// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::CratePath;

use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use scale_info::{form::PortableForm, Path, TypeDefPrimitive};
use std::collections::BTreeSet;
use syn::parse_quote;

/// An opaque struct representing a type path. The main usage of this is
/// to spit out as tokens in some `quote!{ ... }` macro; the inner structure
/// should be unimportant.
#[derive(Clone, Debug)]
pub struct TypePath(TypePathInner);

#[derive(Clone, Debug)]
pub enum TypePathInner {
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
    /// Construct a [`TypePath`] from a [`TypeParameter`]
    pub fn from_parameter(param: TypeParameter) -> TypePath {
        TypePath(TypePathInner::Parameter(param))
    }

    /// Construct a [`TypePath`] from a [`TypeParameter`]
    pub fn from_type(ty: TypePathType) -> TypePath {
        TypePath(TypePathInner::Type(ty))
    }

    /// Construct a [`TypePath`] from a [`syn::TypePath`]
    pub fn from_syn_path(path: syn::Path) -> TypePath {
        // Note; this doesn't parse the parameters or anything, but since nothing external
        // can inspect this structure, and the ToTokens impl works either way, it should be ok.
        TypePath(TypePathInner::Type(TypePathType::Path {
            path,
            params: Vec::new(),
        }))
    }

    pub(crate) fn to_syn_type(&self) -> syn::Type {
        match &self.0 {
            TypePathInner::Parameter(ty_param) => syn::Type::Path(parse_quote! { #ty_param }),
            TypePathInner::Type(ty) => ty.to_syn_type(),
        }
    }

    pub(crate) fn is_compact(&self) -> bool {
        matches!(&self.0, TypePathInner::Type(ty) if ty.is_compact())
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
        match &self.0 {
            TypePathInner::Parameter(type_parameter) => {
                acc.insert(type_parameter.clone());
            }
            TypePathInner::Type(type_path) => type_path.parent_type_params(acc),
        }
    }

    /// Gets the vector type parameter if the data is represented as `TypeDef::Sequence`.
    ///
    /// **Note:** Utilized for transforming `std::vec::Vec<T>` into slices `&[T]` for the storage API.
    pub fn vec_type_param(&self) -> Option<&TypePath> {
        let ty = match &self.0 {
            TypePathInner::Type(ty) => ty,
            _ => return None,
        };

        match ty {
            TypePathType::Vec { of } => Some(of),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypePathType {
    Path {
        path: syn::Path,
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
        crate_path: CratePath,
    },
    BitVec {
        bit_order_type: Box<TypePath>,
        bit_store_type: Box<TypePath>,
        crate_path: CratePath,
    },
}

impl TypePathType {
    pub fn from_type_def_path(
        path: &Path<PortableForm>,
        root_mod_ident: Ident,
        params: Vec<TypePath>,
    ) -> Self {
        let path_segments = &*path.segments;

        let path: syn::Path = match path_segments {
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
                    "NonZeroI8" => parse_quote!(::core::num::NonZeroI8),
                    "NonZeroU8" => parse_quote!(::core::num::NonZeroU8),
                    "NonZeroI16" => parse_quote!(::core::num::NonZeroI16),
                    "NonZeroU16" => parse_quote!(::core::num::NonZeroU16),
                    "NonZeroI32" => parse_quote!(::core::num::NonZeroI32),
                    "NonZeroU32" => parse_quote!(::core::num::NonZeroU32),
                    "NonZeroI64" => parse_quote!(::core::num::NonZeroI64),
                    "NonZeroU64" => parse_quote!(::core::num::NonZeroU64),
                    "NonZeroI128" => parse_quote!(::core::num::NonZeroI128),
                    "NonZeroU128" => parse_quote!(::core::num::NonZeroU128),
                    "NonZeroIsize" => parse_quote!(::core::num::NonZeroIsize),
                    "NonZeroUsize" => parse_quote!(::core::num::NonZeroUsize),
                    ident => panic!("Unknown prelude type '{ident}'"),
                }
            }
            _ => {
                // paths to generated types in the root types module
                let mut ty_path = path_segments
                    .iter()
                    .map(|s| syn::PathSegment::from(format_ident!("{}", s)))
                    .collect::<syn::punctuated::Punctuated<syn::PathSegment, syn::Token![::]>>();
                ty_path.insert(0, syn::PathSegment::from(root_mod_ident));
                parse_quote!( #ty_path )
            }
        };
        Self::Path { path, params }
    }

    /// Visits a type path, collecting all the generic type parameters from the containing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// struct S<T> {
    ///     a: Vec<Option<T>>, // the parent type param here is `T`
    /// }
    /// ```
    fn parent_type_params(&self, acc: &mut BTreeSet<TypeParameter>) {
        match self {
            TypePathType::Path { params, .. } => {
                for p in params {
                    p.parent_type_params(acc)
                }
            }
            TypePathType::Vec { of } => of.parent_type_params(acc),
            TypePathType::Array { of, .. } => of.parent_type_params(acc),
            TypePathType::Tuple { elements } => {
                for e in elements {
                    e.parent_type_params(acc)
                }
            }
            TypePathType::Primitive { .. } => (),
            TypePathType::Compact { inner, .. } => inner.parent_type_params(acc),
            TypePathType::BitVec {
                bit_order_type,
                bit_store_type,
                crate_path: _,
            } => {
                bit_order_type.parent_type_params(acc);
                bit_store_type.parent_type_params(acc);
            }
        }
    }

    pub(crate) fn is_compact(&self) -> bool {
        matches!(self, TypePathType::Compact { .. })
    }

    fn to_syn_type(&self) -> syn::Type {
        match &self {
            TypePathType::Path { path, params } => {
                let path = if params.is_empty() {
                    parse_quote! { #path }
                } else {
                    parse_quote! { #path< #( #params ),* > }
                };
                syn::Type::Path(path)
            }
            TypePathType::Vec { of } => {
                let type_path = parse_quote! { ::std::vec::Vec<#of> };
                syn::Type::Path(type_path)
            }
            TypePathType::Array { len, of } => {
                let array = parse_quote! { [#of; #len] };
                syn::Type::Array(array)
            }
            TypePathType::Tuple { elements } => {
                let tuple = parse_quote! { (#( # elements, )* ) };
                syn::Type::Tuple(tuple)
            }
            TypePathType::Primitive { def } => syn::Type::Path(match def {
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
            }),
            TypePathType::Compact {
                inner,
                is_field,
                crate_path,
            } => {
                let path = if *is_field {
                    // compact fields can use the inner compact type directly and be annotated with
                    // the `compact` attribute e.g. `#[codec(compact)] my_compact_field: u128`
                    parse_quote! ( #inner )
                } else {
                    parse_quote! ( #crate_path::ext::codec::Compact<#inner> )
                };
                syn::Type::Path(path)
            }
            TypePathType::BitVec {
                bit_order_type,
                bit_store_type,
                crate_path,
            } => {
                let type_path = parse_quote! { #crate_path::utils::bits::DecodedBits<#bit_store_type, #bit_order_type> };
                syn::Type::Path(type_path)
            }
        }
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
