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
    Span,
    TokenStream as TokenStream2,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
    ToTokens,
};
use scale_info::{
    form::PortableForm,
    Field,
    PortableRegistry,
    Type,
    TypeDef,
    TypeDefPrimitive,
};
use std::collections::{
    BTreeMap,
    HashMap,
    HashSet,
};

#[derive(Debug)]
pub struct TypeGenerator<'a> {
    root_mod_ident: Ident,
    type_registry: &'a PortableRegistry,
    type_substitutes: HashMap<String, syn::TypePath>,
}

impl<'a> TypeGenerator<'a> {
    /// Construct a new [`TypeGenerator`].
    pub fn new(
        type_registry: &'a PortableRegistry,
        root_mod: &'static str,
        type_substitutes: HashMap<String, syn::TypePath>,
    ) -> Self {
        let root_mod_ident = Ident::new(root_mod, Span::call_site());
        Self {
            root_mod_ident,
            type_registry,
            type_substitutes,
        }
    }

    /// Generate a module containing all types defined in the supplied type registry.
    pub fn generate_types_mod(&'a self) -> Module<'a> {
        let mut root_mod =
            Module::new(self.root_mod_ident.clone(), self.root_mod_ident.clone());

        for (id, ty) in self.type_registry.types().iter().enumerate() {
            if ty.ty().path().namespace().is_empty() {
                // prelude types e.g. Option/Result have no namespace, so we don't generate them
                continue
            }
            self.insert_type(
                ty.ty().clone(),
                id as u32,
                ty.ty().path().namespace().to_vec(),
                &self.root_mod_ident,
                &mut root_mod,
            )
        }

        root_mod
    }

    fn insert_type(
        &'a self,
        ty: Type<PortableForm>,
        id: u32,
        path: Vec<String>,
        root_mod_ident: &Ident,
        module: &mut Module<'a>,
    ) {
        let joined_path = path.join("::");
        if self.type_substitutes.contains_key(&joined_path) {
            return
        }

        let segment = path.first().expect("path has at least one segment");
        let mod_ident = Ident::new(segment, Span::call_site());

        let child_mod = module
            .children
            .entry(mod_ident.clone())
            .or_insert_with(|| Module::new(mod_ident, root_mod_ident.clone()));

        if path.len() == 1 {
            child_mod
                .types
                .insert(ty.path().clone(), ModuleType { ty, type_gen: self });
        } else {
            self.insert_type(ty, id, path[1..].to_vec(), root_mod_ident, child_mod)
        }
    }

    /// # Panics
    ///
    /// If no type with the given id found in the type registry.
    pub fn resolve_type(&self, id: u32) -> Type<PortableForm> {
        self.type_registry
            .resolve(id)
            .unwrap_or_else(|| panic!("No type with id {} found", id))
            .clone()
    }

    /// # Panics
    ///
    /// If no type with the given id found in the type registry.
    pub fn resolve_type_path(
        &self,
        id: u32,
        parent_type_params: &[TypeParameter],
    ) -> TypePath {
        if let Some(parent_type_param) = parent_type_params
            .iter()
            .find(|tp| tp.concrete_type_id == id)
        {
            return TypePath::Parameter(parent_type_param.clone())
        }

        let mut ty = self.resolve_type(id);

        if ty.path().ident() == Some("Cow".to_string()) {
            ty = self.resolve_type(
                ty.type_params()[0]
                    .ty()
                    .expect("type parameters to Cow are not expected to be skipped; qed")
                    .id(),
            )
        }

        let params_type_ids = match ty.type_def() {
            TypeDef::Array(arr) => vec![arr.type_param().id()],
            TypeDef::Sequence(seq) => vec![seq.type_param().id()],
            TypeDef::Tuple(tuple) => tuple.fields().iter().map(|f| f.id()).collect(),
            TypeDef::Compact(compact) => vec![compact.type_param().id()],
            TypeDef::BitSequence(seq) => {
                vec![seq.bit_order_type().id(), seq.bit_store_type().id()]
            }
            _ => {
                ty.type_params()
                    .iter()
                    .filter_map(|f| f.ty().map(|f| f.id()))
                    .collect()
            }
        };

        let params = params_type_ids
            .iter()
            .map(|tp| self.resolve_type_path(*tp, parent_type_params))
            .collect::<Vec<_>>();

        let joined_path = ty.path().segments().join("::");
        if let Some(substitute_type_path) = self.type_substitutes.get(&joined_path) {
            TypePath::Substitute(TypePathSubstitute {
                path: substitute_type_path.clone(),
                params,
            })
            // todo: add tests for this type substitution
        } else {
            TypePath::Type(TypePathType {
                ty,
                params,
                root_mod_ident: self.root_mod_ident.clone(),
            })
        }
    }
}

#[derive(Debug)]
pub struct Module<'a> {
    name: Ident,
    root_mod: Ident,
    children: BTreeMap<Ident, Module<'a>>,
    types: BTreeMap<scale_info::Path<scale_info::form::PortableForm>, ModuleType<'a>>,
}

impl<'a> ToTokens for Module<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let root_mod = &self.root_mod;
        let modules = self.children.values();
        let types = self.types.values().clone();

        tokens.extend(quote! {
            pub mod #name {
                use super::#root_mod;

                #( #modules )*
                #( #types )*
            }
        })
    }
}

impl<'a> Module<'a> {
    pub fn new(name: Ident, root_mod: Ident) -> Self {
        Self {
            name,
            root_mod,
            children: BTreeMap::new(),
            types: BTreeMap::new(),
        }
    }

    /// Returns the module ident.
    pub fn ident(&self) -> &Ident {
        &self.name
    }
}

#[derive(Debug)]
pub struct ModuleType<'a> {
    type_gen: &'a TypeGenerator<'a>,
    ty: Type<PortableForm>,
}

impl<'a> quote::ToTokens for ModuleType<'a> {
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
            let path = syn::parse_quote! { #ty #type_params};
            syn::Type::Path(path)
        });

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
                            Some(quote!( , ::codec::CompactAs ))
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
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode #derive_as_compact)]
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
                    variants.push(quote! { #variant_name #fields });
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
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
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

impl<'a> ModuleType<'a> {
    fn composite_fields(
        &self,
        fields: &'a [Field<PortableForm>],
        type_params: &'a [TypeParameter],
        is_struct: bool,
    ) -> (TokenStream2, Vec<TypeParameter>) {
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
                // todo [AJ] remove this hack once scale-info can represent Box somehow
                quote! { std::boxed::Box<#ty_path> }
            } else if ty_name.contains("BTreeMap<") {
                // todo [AJ] remove this hack and add namespaces or import prelude types
                quote! { std::collections::#ty_path }
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
                        // todo: [AJ] figure out way to ensure AsCompact generated for target type in scale_info.
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
                    match ty_name {
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

    fn phantom_data(params: &[TypeParameter]) -> TokenStream2 {
        let params = if params.len() == 1 {
            let param = &params[0];
            quote! { #param }
        } else {
            quote! { ( #( #params ), * ) }
        };
        quote! ( ::core::marker::PhantomData<#params> )
    }
}

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
            TypePath::Parameter(ty_param) => {
                syn::Type::Path(syn::parse_quote! { #ty_param })
            }
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
    fn parent_type_params(&self, acc: &mut HashSet<TypeParameter>) {
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
    ty: Type<PortableForm>,
    params: Vec<TypePath>,
    root_mod_ident: Ident,
}

impl TypePathType {
    pub(crate) fn is_compact(&self) -> bool {
        matches!(self.ty.type_def(), TypeDef::Compact(_))
    }

    fn to_syn_type(&self) -> syn::Type {
        let params = &self.params;
        match self.ty.type_def() {
            TypeDef::Composite(_) | TypeDef::Variant(_) => {
                let mut ty_path = self
                    .ty
                    .path()
                    .segments()
                    .iter()
                    .map(|s| syn::PathSegment::from(format_ident!("{}", s)))
                    .collect::<syn::punctuated::Punctuated<syn::PathSegment, syn::Token![::]>>();
                if !self.ty.path().namespace().is_empty() {
                    // types without a namespace are assumed to be globally in scope e.g. `Option`s
                    ty_path
                        .insert(0, syn::PathSegment::from(self.root_mod_ident.clone()));
                }

                let params = &self.params;
                let path = if params.is_empty() {
                    syn::parse_quote! { #ty_path }
                } else {
                    syn::parse_quote! { #ty_path< #( #params ),* > }
                };
                syn::Type::Path(path)
            }
            TypeDef::Sequence(_) => {
                let type_param = &self.params[0];
                let type_path = syn::parse_quote! { Vec<#type_param> };
                syn::Type::Path(type_path)
            }
            TypeDef::Array(array) => {
                let array_type = &self.params[0];
                let array_len = array.len() as usize;
                let array = syn::parse_quote! { [#array_type; #array_len] };
                syn::Type::Array(array)
            }
            TypeDef::Tuple(_) => {
                let tuple = syn::parse_quote! { (#( # params, )* ) };
                syn::Type::Tuple(tuple)
            }
            TypeDef::Primitive(primitive) => {
                let primitive = match primitive {
                    TypeDefPrimitive::Bool => "bool",
                    TypeDefPrimitive::Char => "char",
                    TypeDefPrimitive::Str => "String",
                    TypeDefPrimitive::U8 => "u8",
                    TypeDefPrimitive::U16 => "u16",
                    TypeDefPrimitive::U32 => "u32",
                    TypeDefPrimitive::U64 => "u64",
                    TypeDefPrimitive::U128 => "u128",
                    TypeDefPrimitive::U256 => unimplemented!("not a rust primitive"),
                    TypeDefPrimitive::I8 => "i8",
                    TypeDefPrimitive::I16 => "i16",
                    TypeDefPrimitive::I32 => "i32",
                    TypeDefPrimitive::I64 => "i64",
                    TypeDefPrimitive::I128 => "i128",
                    TypeDefPrimitive::I256 => unimplemented!("not a rust primitive"),
                };
                let ident = format_ident!("{}", primitive);
                let path = syn::parse_quote! { #ident };
                syn::Type::Path(path)
            }
            TypeDef::Compact(_) => {
                // todo: change the return type of this method to include info that it is compact
                // and should be annotated with #[compact] for fields
                let compact_type = &self.params[0];
                syn::Type::Path(syn::parse_quote! ( #compact_type ))
            }
            TypeDef::BitSequence(_) => {
                let bit_order_type = &self.params[0];
                let bit_store_type = &self.params[1];

                let type_path = syn::parse_quote! { ::subxt::bitvec::vec::BitVec<#bit_order_type, #bit_store_type> };

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
    concrete_type_id: u32,
    name: proc_macro2::Ident,
}

impl quote::ToTokens for TypeParameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens)
    }
}

#[derive(Clone, Debug)]
pub struct TypePathSubstitute {
    path: syn::TypePath,
    params: Vec<TypePath>,
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
            syn::parse_quote! ( #substitute_path< #( #params ),* > )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use scale_info::{
        meta_type,
        Registry,
        TypeInfo,
    };

    const MOD_PATH: &'static [&'static str] = &["subxt_codegen", "types", "tests"];

    fn get_mod<'a>(
        module: &'a Module,
        path_segs: &[&'static str],
    ) -> Option<&'a Module<'a>> {
        let (mod_name, rest) = path_segs.split_first()?;
        let mod_ident = Ident::new(mod_name, Span::call_site());
        let module = module.children.get(&mod_ident)?;
        if rest.is_empty() {
            Some(module)
        } else {
            get_mod(module, rest)
        }
    }

    #[test]
    fn generate_struct_with_primitives() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            a: bool,
            b: u32,
            c: char,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<S>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct S {
                        pub a: bool,
                        pub b: u32,
                        pub c: char,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generate_struct_with_a_struct_field() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Parent {
            a: bool,
            b: Child,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Child {
            a: i32,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<Parent>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct Child {
                        pub a: i32,
                    }

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct Parent {
                        pub a: bool,
                        pub b: root::subxt_codegen::types::tests::Child,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generate_tuple_struct() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Parent(bool, Child);

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Child(i32);

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<Parent>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct Child(pub i32,);

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct Parent(pub bool, pub root::subxt_codegen::types::tests::Child,);
                }
            }
            .to_string()
        )
    }

    #[test]
    fn derive_compact_as_for_uint_wrapper_structs() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Su8 {
            a: u8,
        }
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct TSu8(u8);
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Su16 {
            a: u16,
        }
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct TSu16(u16);
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Su32 {
            a: u32,
        }
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct TSu32(u32);
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Su64 {
            a: u64,
        }
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct TSu64(u64);
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Su128 {
            a: u128,
        }
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct TSu128(u128);

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<Su8>());
        registry.register_type(&meta_type::<TSu8>());
        registry.register_type(&meta_type::<Su16>());
        registry.register_type(&meta_type::<TSu16>());
        registry.register_type(&meta_type::<Su32>());
        registry.register_type(&meta_type::<TSu32>());
        registry.register_type(&meta_type::<Su64>());
        registry.register_type(&meta_type::<TSu64>());
        registry.register_type(&meta_type::<Su128>());
        registry.register_type(&meta_type::<TSu128>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct Su128 { pub a: u128, }

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct Su16 { pub a: u16, }

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct Su32 { pub a: u32, }

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct Su64 { pub a: u64, }

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct Su8 { pub a: u8, }

                     #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct TSu128(pub u128,);

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct TSu16(pub u16,);

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct TSu32(pub u32,);

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct TSu64(pub u64,);

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct TSu8(pub u8,);
                }
            }
                .to_string()
        )
    }

    #[test]
    fn generate_enum() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        enum E {
            A,
            B(bool),
            C { a: u32 },
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<E>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub enum E {
                        A,
                        B (bool,),
                        C { a: u32, },
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generate_array_field() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            a: [u8; 32],
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<S>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct S {
                        pub a: [u8; 32usize],
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn option_fields() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            a: Option<bool>,
            b: Option<u32>,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<S>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct S {
                        pub a: Option<bool>,
                        pub b: Option<u32>,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn box_fields_struct() {
        // todo: [AJ] remove hack for Box and make no_std compatible using `alloc::Box`

        use std::boxed::Box;

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            a: std::boxed::Box<bool>,
            b: Box<u32>,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<S>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct S {
                        pub a: std::boxed::Box<bool>,
                        pub b: std::boxed::Box<u32>,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn box_fields_enum() {
        use std::boxed::Box;

        #[allow(unused)]
        #[derive(TypeInfo)]
        enum E {
            A(Box<bool>),
            B { a: Box<u32> },
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<E>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub enum E {
                        A(std::boxed::Box<bool>,),
                        B { a: std::boxed::Box<u32>, },
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn range_fields() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            a: core::ops::Range<u32>,
            b: core::ops::RangeInclusive<u32>,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<S>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct S {
                        pub a: ::core::ops::Range<u32>,
                        pub b: ::core::ops::RangeInclusive<u32>,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generics() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Foo<T> {
            a: T,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Bar {
            b: Foo<u32>,
            c: Foo<u8>,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<Bar>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct Bar {
                        pub b: root::subxt_codegen::types::tests::Foo<u32>,
                        pub c: root::subxt_codegen::types::tests::Foo<u8>,
                    }
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct Foo<_0> {
                        pub a: _0,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generics_nested() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Foo<T, U> {
            a: T,
            b: Option<(T, U)>,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Bar<T> {
            b: Foo<T, u32>,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<Bar<bool>>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct Bar<_0> {
                        pub b: root::subxt_codegen::types::tests::Foo<_0, u32>,
                    }

                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct Foo<_0, _1> {
                        pub a: _0,
                        pub b: Option<(_0, _1,)>,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generate_bitvec() {
        use bitvec::{
            order::{
                Lsb0,
                Msb0,
            },
            vec::BitVec,
        };

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            lsb: BitVec<Lsb0, u8>,
            msb: BitVec<Msb0, u16>,
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<S>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct S {
                        pub lsb: ::subxt::bitvec::vec::BitVec<root::bitvec::order::Lsb0, u8>,
                        pub msb: ::subxt::bitvec::vec::BitVec<root::bitvec::order::Msb0, u16>,
                    }
                }
            }
            .to_string()
        )
    }

    #[test]
    fn generics_with_alias_adds_phantom_data_marker() {
        trait Trait {
            type Type;
        }

        impl Trait for bool {
            type Type = u32;
        }

        type Foo<T> = <T as Trait>::Type;
        type Bar<T, U> = (<T as Trait>::Type, <U as Trait>::Type);

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct NamedFields<T: Trait> {
            b: Foo<T>,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct UnnamedFields<T: Trait, U: Trait>(Bar<T, U>);

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<NamedFields<bool>>());
        registry.register_type(&meta_type::<UnnamedFields<bool, bool>>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode, ::codec::CompactAs)]
                    pub struct NamedFields<_0> {
                        pub b: u32,
                        #[codec(skip)] pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
                    }
                    #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                    pub struct UnnamedFields<_0, _1> (
                        pub (u32, u32,),
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1)>,
                    );
                }
            }
            .to_string()
        )
    }

    #[test]
    fn modules() {
        mod modules {
            pub mod a {
                #[allow(unused)]
                #[derive(scale_info::TypeInfo)]
                pub struct Foo {}

                pub mod b {
                    #[allow(unused)]
                    #[derive(scale_info::TypeInfo)]
                    pub struct Bar {
                        a: super::Foo,
                    }
                }
            }

            pub mod c {
                #[allow(unused)]
                #[derive(scale_info::TypeInfo)]
                pub struct Foo {
                    a: super::a::b::Bar,
                }
            }
        }

        let mut registry = Registry::new();
        registry.register_type(&meta_type::<modules::c::Foo>());
        let portable_types: PortableRegistry = registry.into();

        let type_gen = TypeGenerator::new(&portable_types, "root", Default::default());
        let types = type_gen.generate_types_mod();
        let tests_mod = get_mod(&types, MOD_PATH).unwrap();

        assert_eq!(
            tests_mod.into_token_stream().to_string(),
            quote! {
                pub mod tests {
                    use super::root;
                    pub mod modules {
                        use super::root;
                        pub mod a {
                            use super::root;

                            pub mod b {
                                use super::root;

                                #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                                pub struct Bar {
                                    pub a: root::subxt_codegen::types::tests::modules::a::Foo,
                                }
                            }

                            #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                            pub struct Foo {}
                        }

                        pub mod c {
                            use super::root;

                            #[derive(Debug, Eq, PartialEq, ::codec::Encode, ::codec::Decode)]
                            pub struct Foo {
                                pub a: root::subxt_codegen::types::tests::modules::a::b::Bar,
                            }
                        }
                    }
                }
            }
            .to_string()
        )
    }
}
