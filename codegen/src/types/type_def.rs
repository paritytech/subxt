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
    TypeGenerator,
    TypeParameter,
    TypePath,
};
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use scale_info::{
    form::PortableForm,
    Field,
    Type,
    TypeDef,
    TypeDefPrimitive,
};
use std::collections::HashSet;
use syn::parse_quote;

/// Generates a Rust `struct` or `enum` definition based on the supplied [`scale-info::Type`].
///
/// Field type paths are resolved via the `TypeGenerator`, which contains the registry of all
/// generated types in the module.
#[derive(Debug)]
pub struct TypeDefGen<'a> {
    /// The type generation context, allows resolving of type paths for the fields of the
    /// generated type.
    pub(super) type_gen: &'a TypeGenerator<'a>,
    /// Contains the definition of the type to be generated.
    pub(super) ty: Type<PortableForm>,
}

impl<'a> quote::ToTokens for TypeDefGen<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let type_params = self
            .ty
            .type_params()
            .iter()
            .enumerate()
            .filter_map(|(i, tp)| {
                match tp.ty() {
                    Some(ty) => {
                        let tp_name = format_ident!("_{}", i);
                        Some(TypeParameter {
                            concrete_type_id: ty.id(),
                            name: tp_name,
                        })
                    }
                    None => None,
                }
            })
            .collect::<Vec<_>>();

        let type_name = self.ty.path().ident().map(|ident| {
            let type_params = if !type_params.is_empty() {
                quote! { < #( #type_params ),* > }
            } else {
                quote! {}
            };
            let ty = format_ident!("{}", ident);
            let path = parse_quote! { #ty #type_params};
            syn::Type::Path(path)
        });

        let derives = self.type_gen.derives();

        match self.ty.type_def() {
            TypeDef::Composite(composite) => {
                let type_name = type_name.expect("structs should have a name");
                let (fields, _) =
                    self.composite_fields(composite.fields(), &type_params, true);
                let derive_as_compact = if composite.fields().len() == 1 {
                    // any single field wrapper struct with a concrete unsigned int type can derive
                    // CompactAs.
                    let field = &composite.fields()[0];
                    if !self
                        .ty
                        .type_params()
                        .iter()
                        .any(|tp| Some(tp.name()) == field.type_name())
                    {
                        let ty = self.type_gen.resolve_type(field.ty().id());
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
                            Some(quote!( #[derive(::subxt::codec::CompactAs)] ))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                let ty_toks = quote! {
                    #derive_as_compact
                    #derives
                    pub struct #type_name #fields
                };
                tokens.extend(ty_toks);
            }
            TypeDef::Variant(variant) => {
                let type_name = type_name.expect("variants should have a name");
                let mut variants = Vec::new();
                let mut used_type_params = HashSet::new();
                let type_params_set: HashSet<_> = type_params.iter().cloned().collect();

                for v in variant.variants() {
                    let variant_name = format_ident!("{}", v.name());
                    let (fields, unused_type_params) = if v.fields().is_empty() {
                        let unused = type_params_set.iter().cloned().collect::<Vec<_>>();
                        (quote! {}, unused)
                    } else {
                        self.composite_fields(v.fields(), &type_params, false)
                    };
                    let index = proc_macro2::Literal::u8_unsuffixed(v.index());
                    variants.push(quote! {
                        #[codec(index = #index)]
                        #variant_name #fields
                    });
                    let unused_params_set = unused_type_params.iter().cloned().collect();
                    let used_params = type_params_set.difference(&unused_params_set);

                    for used_param in used_params {
                        used_type_params.insert(used_param.clone());
                    }
                }

                let unused_type_params = type_params_set
                    .difference(&used_type_params)
                    .cloned()
                    .collect::<Vec<_>>();
                if !unused_type_params.is_empty() {
                    let phantom = Self::phantom_data(&unused_type_params);
                    variants.push(quote! {
                        __Ignore(#phantom)
                    })
                }

                let ty_toks = quote! {
                    #derives
                    pub enum #type_name {
                        #( #variants, )*
                    }
                };
                tokens.extend(ty_toks);
            }
            _ => (), // all built-in types should already be in scope
        }
    }
}

impl<'a> TypeDefGen<'a> {
    fn composite_fields(
        &self,
        fields: &'a [Field<PortableForm>],
        type_params: &'a [TypeParameter],
        is_struct: bool,
    ) -> (TokenStream, Vec<TypeParameter>) {
        let named = fields.iter().all(|f| f.name().is_some());
        let unnamed = fields.iter().all(|f| f.name().is_none());

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

        let ty_toks = |ty_name: &str, ty_path: &TypePath| {
            if ty_name.contains("Box<") {
                quote! { ::std::boxed::Box<#ty_path> }
            } else {
                quote! { #ty_path }
            }
        };

        if named {
            let fields = fields
                .iter()
                .map(|field| {
                    let name = format_ident!(
                        "{}",
                        field.name().expect("named field without a name")
                    );
                    let ty = self
                        .type_gen
                        .resolve_type_path(field.ty().id(), type_params);
                    (name, ty, field.type_name())
                })
                .collect::<Vec<_>>();

            let mut fields_tokens = fields
                .iter()
                .map(|(name, ty, ty_name)| {
                    let field_type = match ty_name {
                        Some(ty_name) => {
                            let ty = ty_toks(ty_name, ty);
                            if is_struct {
                                quote! ( pub #name: #ty )
                            } else {
                                quote! ( #name: #ty )
                            }
                        }
                        None => {
                            quote! ( #name: #ty )
                        }
                    };
                    if ty.is_compact() {
                        quote!( #[codec(compact)] #field_type  )
                    } else {
                        quote!( #field_type  )
                    }
                })
                .collect::<Vec<_>>();

            let unused_params =
                unused_type_params(type_params, fields.iter().map(|(_, ty, _)| ty));

            if is_struct && !unused_params.is_empty() {
                let phantom = Self::phantom_data(&unused_params);
                fields_tokens.push(quote! {
                    #[codec(skip)] pub __subxt_unused_type_params: #phantom
                })
            }

            let fields = quote! {
                {
                    #( #fields_tokens, )*
                }
            };
            (fields, unused_params)
        } else if unnamed {
            let type_paths = fields
                .iter()
                .map(|field| {
                    let ty = self
                        .type_gen
                        .resolve_type_path(field.ty().id(), type_params);
                    (ty, field.type_name())
                })
                .collect::<Vec<_>>();
            let mut fields_tokens = type_paths
                .iter()
                .map(|(ty, ty_name)| {
                    let field_type = match ty_name {
                        Some(ty_name) => {
                            let ty = ty_toks(ty_name, ty);
                            if is_struct {
                                quote! { pub #ty }
                            } else {
                                quote! { #ty }
                            }
                        }
                        None => {
                            quote! { #ty }
                        }
                    };
                    if ty.is_compact() {
                        quote!( #[codec(compact)] #field_type  )
                    } else {
                        quote!( #field_type  )
                    }
                })
                .collect::<Vec<_>>();

            let unused_params =
                unused_type_params(type_params, type_paths.iter().map(|(ty, _)| ty));

            if is_struct && !unused_params.is_empty() {
                let phantom_data = Self::phantom_data(&unused_params);
                fields_tokens.push(quote! { #[codec(skip)] pub #phantom_data })
            }

            let fields = quote! { ( #( #fields_tokens, )* ) };
            let fields_tokens = if is_struct {
                // add a semicolon for tuple structs
                quote! { #fields; }
            } else {
                fields
            };

            (fields_tokens, unused_params)
        } else {
            panic!("Fields must be either all named or all unnamed")
        }
    }

    fn phantom_data(params: &[TypeParameter]) -> TokenStream {
        let params = if params.len() == 1 {
            let param = &params[0];
            quote! { #param }
        } else {
            quote! { ( #( #params ), * ) }
        };
        quote! ( ::core::marker::PhantomData<#params> )
    }
}
