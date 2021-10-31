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
    Type,
    TypeDef,
    TypeDefPrimitive,
};
use std::collections::HashSet;
use syn::parse_quote;

#[derive(Clone, Debug)]
pub enum TypePath {
    Parameter(TypeParameter),
    Type(TypePathType),
    Substitute(TypePathSubstitute),
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
            TypePath::Substitute(sub) => sub.to_syn_type(),
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
    pub fn parent_type_params(&self, acc: &mut HashSet<TypeParameter>) {
        match self {
            Self::Parameter(type_parameter) => {
                acc.insert(type_parameter.clone());
            }
            Self::Type(type_path) => type_path.parent_type_params(acc),
            Self::Substitute(sub) => sub.parent_type_params(acc),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypePathType {
    pub(super) ty: Type<PortableForm>,
    pub(super) params: Vec<TypePath>,
    pub(super) root_mod_ident: Ident,
}

impl TypePathType {
    pub(crate) fn is_compact(&self) -> bool {
        matches!(self.ty.type_def(), TypeDef::Compact(_))
    }

    fn to_syn_type(&self) -> syn::Type {
        let params = &self.params;
        match self.ty.type_def() {
            TypeDef::Composite(_) | TypeDef::Variant(_) => {
                let path_segments = self.ty.path().segments();

                let ty_path: syn::TypePath = match path_segments {
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
                        ty_path.insert(
                            0,
                            syn::PathSegment::from(self.root_mod_ident.clone()),
                        );
                        parse_quote!( #ty_path )
                    }
                };

                let params = &self.params;
                let path = if params.is_empty() {
                    parse_quote! { #ty_path }
                } else {
                    parse_quote! { #ty_path< #( #params ),* > }
                };
                syn::Type::Path(path)
            }
            TypeDef::Sequence(_) => {
                let type_param = &self.params[0];
                let type_path = parse_quote! { ::std::vec::Vec<#type_param> };
                syn::Type::Path(type_path)
            }
            TypeDef::Array(array) => {
                let array_type = &self.params[0];
                let array_len = array.len() as usize;
                let array = parse_quote! { [#array_type; #array_len] };
                syn::Type::Array(array)
            }
            TypeDef::Tuple(_) => {
                let tuple = parse_quote! { (#( # params, )* ) };
                syn::Type::Tuple(tuple)
            }
            TypeDef::Primitive(primitive) => {
                let path = match primitive {
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
                };
                syn::Type::Path(path)
            }
            TypeDef::Compact(_) => {
                // todo: change the return type of this method to include info that it is compact
                // and should be annotated with #[compact] for fields
                let compact_type = &self.params[0];
                syn::Type::Path(parse_quote! ( #compact_type ))
            }
            TypeDef::BitSequence(_) => {
                let bit_order_type = &self.params[0];
                let bit_store_type = &self.params[1];

                let type_path = parse_quote! { ::subxt::bitvec::vec::BitVec<#bit_order_type, #bit_store_type> };

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
    fn parent_type_params(&self, acc: &mut HashSet<TypeParameter>) {
        for p in &self.params {
            p.parent_type_params(acc);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypeParameter {
    pub(super) concrete_type_id: u32,
    pub(super) name: proc_macro2::Ident,
}

impl quote::ToTokens for TypeParameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens)
    }
}

#[derive(Clone, Debug)]
pub struct TypePathSubstitute {
    pub(super) path: syn::TypePath,
    pub(super) params: Vec<TypePath>,
}

impl quote::ToTokens for TypePathSubstitute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.params.is_empty() {
            self.path.to_tokens(tokens)
        } else {
            let substitute_path = &self.path;
            let params = &self.params;
            tokens.extend(quote! {
                #substitute_path< #( #params ),* >
            })
        }
    }
}

impl TypePathSubstitute {
    fn parent_type_params(&self, acc: &mut HashSet<TypeParameter>) {
        for p in &self.params {
            p.parent_type_params(acc);
        }
    }

    fn to_syn_type(&self) -> syn::Type {
        if self.params.is_empty() {
            syn::Type::Path(self.path.clone())
        } else {
            let substitute_path = &self.path;
            let params = &self.params;
            parse_quote! ( #substitute_path< #( #params ),* > )
        }
    }
}
