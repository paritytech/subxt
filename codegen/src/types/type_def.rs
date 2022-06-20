// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use super::{
    CompositeDef,
    CompositeDefFields,
    Derives,
    TypeDefParameters,
    TypeGenerator,
    TypeParameter,
};
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use scale_info::{
    form::PortableForm,
    Type,
    TypeDef,
};
use syn::parse_quote;

/// Generates a Rust `struct` or `enum` definition based on the supplied [`scale-info::Type`].
///
/// Field type paths are resolved via the `TypeGenerator`, which contains the registry of all
/// generated types in the module.
#[derive(Debug)]
pub struct TypeDefGen {
    /// The type parameters of the type to be generated
    type_params: TypeDefParameters,
    /// The derives with which to annotate the generated type.
    derives: Derives,
    /// The kind of type to be generated.
    ty_kind: TypeDefGenKind,
    /// Type documentation.
    ty_docs: TokenStream,
}

impl TypeDefGen {
    /// Construct a type definition for codegen from the given [`scale_info::Type`].
    pub fn from_type(ty: Type<PortableForm>, type_gen: &TypeGenerator) -> Self {
        let derives = type_gen.type_derives(&ty);

        let type_params = ty
            .type_params()
            .iter()
            .enumerate()
            .filter_map(|(i, tp)| {
                match tp.ty() {
                    Some(ty) => {
                        let tp_name = format_ident!("_{}", i);
                        Some(TypeParameter {
                            concrete_type_id: ty.id(),
                            original_name: tp.name().clone(),
                            name: tp_name,
                        })
                    }
                    None => None,
                }
            })
            .collect::<Vec<_>>();

        let mut type_params = TypeDefParameters::new(type_params);

        let ty_kind = match ty.type_def() {
            TypeDef::Composite(composite) => {
                let type_name = ty.path().ident().expect("structs should have a name");
                let fields = CompositeDefFields::from_scale_info_fields(
                    &type_name,
                    composite.fields(),
                    type_params.params(),
                    type_gen,
                );
                type_params.update_unused(fields.field_types());
                let composite_def = CompositeDef::struct_def(
                    &ty,
                    &type_name,
                    type_params.clone(),
                    fields,
                    Some(parse_quote!(pub)),
                    type_gen,
                    ty.docs(),
                );
                TypeDefGenKind::Struct(composite_def)
            }
            TypeDef::Variant(variant) => {
                let type_name = ty.path().ident().expect("variants should have a name");
                let variants = variant
                    .variants()
                    .iter()
                    .map(|v| {
                        let fields = CompositeDefFields::from_scale_info_fields(
                            v.name(),
                            v.fields(),
                            type_params.params(),
                            type_gen,
                        );
                        type_params.update_unused(fields.field_types());
                        let variant_def =
                            CompositeDef::enum_variant_def(v.name(), fields, v.docs());
                        (v.index(), variant_def)
                    })
                    .collect();

                TypeDefGenKind::Enum(type_name, variants)
            }
            _ => TypeDefGenKind::BuiltIn,
        };

        let docs = ty.docs();
        let ty_docs = quote! { #( #[doc = #docs ] )* };

        Self {
            type_params,
            derives,
            ty_kind,
            ty_docs,
        }
    }
}

impl quote::ToTokens for TypeDefGen {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.ty_kind {
            TypeDefGenKind::Struct(composite) => composite.to_tokens(tokens),
            TypeDefGenKind::Enum(type_name, variants) => {
                let mut variants = variants
                    .iter()
                    .map(|(index, def)| {
                        let index = proc_macro2::Literal::u8_unsuffixed(*index);
                        quote! {
                            #[codec(index = #index)]
                            #def
                        }
                    })
                    .collect::<Vec<_>>();

                if let Some(phantom) = self.type_params.unused_params_phantom_data() {
                    variants.push(quote! {
                        __Ignore(#phantom)
                    })
                }

                let enum_ident = format_ident!("{}", type_name);
                let type_params = &self.type_params;
                let derives = &self.derives;
                let docs = &self.ty_docs;
                let ty_toks = quote! {
                    #derives
                    #docs
                    pub enum #enum_ident #type_params {
                        #( #variants, )*
                    }
                };
                tokens.extend(ty_toks);
            }
            TypeDefGenKind::BuiltIn => (), /* all built-in types should already be in scope */
        }
    }
}

#[derive(Debug)]
pub enum TypeDefGenKind {
    Struct(CompositeDef),
    Enum(String, Vec<(u8, CompositeDef)>),
    BuiltIn,
}
