// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

mod composite_def;
mod derives;
#[cfg(test)]
mod tests;
mod type_def;
mod type_def_params;
mod type_path;

use darling::FromMeta;
use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use proc_macro_error::abort_call_site;
use quote::{
    quote,
    ToTokens,
};
use scale_info::{
    form::PortableForm,
    PortableRegistry,
    Type,
    TypeDef,
};
use std::collections::{
    BTreeMap,
    HashMap,
};

pub use self::{
    composite_def::{
        CompositeDef,
        CompositeDefFieldType,
        CompositeDefFields,
    },
    derives::{
        Derives,
        DerivesRegistry,
    },
    type_def::TypeDefGen,
    type_def_params::TypeDefParameters,
    type_path::{
        TypeParameter,
        TypePath,
        TypePathType,
    },
};

pub type Field = scale_info::Field<PortableForm>;

/// Generate a Rust module containing all types defined in the supplied [`PortableRegistry`].
#[derive(Debug)]
pub struct TypeGenerator<'a> {
    /// The name of the module which will contain the generated types.
    types_mod_ident: Ident,
    /// Registry of type definitions to be transformed into Rust type definitions.
    type_registry: &'a PortableRegistry,
    /// User defined overrides for generated types.
    type_substitutes: HashMap<String, syn::TypePath>,
    /// Set of derives with which to annotate generated types.
    derives: DerivesRegistry,
    /// The `subxt` crate access path in the generated code.
    crate_path: CratePath,
}

impl<'a> TypeGenerator<'a> {
    /// Construct a new [`TypeGenerator`].
    pub fn new(
        type_registry: &'a PortableRegistry,
        root_mod: &'static str,
        type_substitutes: HashMap<String, syn::TypePath>,
        derives: DerivesRegistry,
        crate_path: CratePath,
    ) -> Self {
        let root_mod_ident = Ident::new(root_mod, Span::call_site());
        Self {
            types_mod_ident: root_mod_ident,
            type_registry,
            type_substitutes,
            derives,
            crate_path,
        }
    }

    /// Generate a module containing all types defined in the supplied type registry.
    pub fn generate_types_mod(&self) -> Module {
        let mut root_mod =
            Module::new(self.types_mod_ident.clone(), self.types_mod_ident.clone());

        for ty in self.type_registry.types().iter() {
            if ty.ty().path().namespace().is_empty() {
                // prelude types e.g. Option/Result have no namespace, so we don't generate them
                continue
            }
            self.insert_type(
                ty.ty().clone(),
                ty.ty().path().namespace().to_vec(),
                &self.types_mod_ident,
                &mut root_mod,
            )
        }

        root_mod
    }

    fn insert_type(
        &'a self,
        ty: Type<PortableForm>,
        path: Vec<String>,
        root_mod_ident: &Ident,
        module: &mut Module,
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
            child_mod.types.insert(
                ty.path().clone(),
                TypeDefGen::from_type(ty, self, &self.crate_path),
            );
        } else {
            self.insert_type(ty, path[1..].to_vec(), root_mod_ident, child_mod)
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

    /// Get the type path for a field of a struct or an enum variant, providing any generic
    /// type parameters from the containing type. This is for identifying where a generic type
    /// parameter is used in a field type e.g.
    ///
    /// ```rust
    /// struct S<T> {
    ///     a: T, // `T` is the "parent" type param from the containing type.
    ///     b: Vec<Option<T>>, // nested use of generic type param `T`.
    /// }
    /// ```
    ///
    /// This allows generating the correct generic field type paths.
    ///
    /// # Panics
    ///
    /// If no type with the given id found in the type registry.
    pub fn resolve_field_type_path(
        &self,
        id: u32,
        parent_type_params: &[TypeParameter],
    ) -> TypePath {
        self.resolve_type_path_recurse(id, true, parent_type_params)
    }

    /// Get the type path for the given type identifier.
    ///
    /// # Panics
    ///
    /// If no type with the given id found in the type registry.
    pub fn resolve_type_path(&self, id: u32) -> TypePath {
        self.resolve_type_path_recurse(id, false, &[])
    }

    /// Visit each node in a possibly nested type definition to produce a type path.
    ///
    /// e.g `Result<GenericStruct<NestedGenericStruct<T>>, String>`
    fn resolve_type_path_recurse(
        &self,
        id: u32,
        is_field: bool,
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

        let params = ty
            .type_params()
            .iter()
            .filter_map(|f| {
                f.ty().map(|f| {
                    self.resolve_type_path_recurse(f.id(), false, parent_type_params)
                })
            })
            .collect();

        let ty = match ty.type_def() {
            TypeDef::Composite(_) | TypeDef::Variant(_) => {
                let joined_path = ty.path().segments().join("::");
                if let Some(substitute_type_path) =
                    self.type_substitutes.get(&joined_path)
                {
                    TypePathType::Path {
                        path: substitute_type_path.clone(),
                        params,
                    }
                } else {
                    TypePathType::from_type_def_path(
                        ty.path(),
                        self.types_mod_ident.clone(),
                        params,
                    )
                }
            }
            TypeDef::Primitive(primitive) => {
                TypePathType::Primitive {
                    def: primitive.clone(),
                }
            }
            TypeDef::Array(arr) => {
                TypePathType::Array {
                    len: arr.len() as usize,
                    of: Box::new(self.resolve_type_path_recurse(
                        arr.type_param().id(),
                        false,
                        parent_type_params,
                    )),
                }
            }
            TypeDef::Sequence(seq) => {
                TypePathType::Vec {
                    of: Box::new(self.resolve_type_path_recurse(
                        seq.type_param().id(),
                        false,
                        parent_type_params,
                    )),
                }
            }
            TypeDef::Tuple(tuple) => {
                TypePathType::Tuple {
                    elements: tuple
                        .fields()
                        .iter()
                        .map(|f| {
                            self.resolve_type_path_recurse(
                                f.id(),
                                false,
                                parent_type_params,
                            )
                        })
                        .collect(),
                }
            }
            TypeDef::Compact(compact) => {
                TypePathType::Compact {
                    inner: Box::new(self.resolve_type_path_recurse(
                        compact.type_param().id(),
                        false,
                        parent_type_params,
                    )),
                    is_field,
                    crate_path: self.crate_path.clone(),
                }
            }
            TypeDef::BitSequence(bitseq) => {
                TypePathType::BitVec {
                    bit_order_type: Box::new(self.resolve_type_path_recurse(
                        bitseq.bit_order_type().id(),
                        false,
                        parent_type_params,
                    )),
                    bit_store_type: Box::new(self.resolve_type_path_recurse(
                        bitseq.bit_store_type().id(),
                        false,
                        parent_type_params,
                    )),
                    crate_path: self.crate_path.clone(),
                }
            }
        };

        TypePath::Type(ty)
    }

    /// Returns the derives to be applied to all generated types.
    pub fn default_derives(&self) -> &Derives {
        self.derives.default_derives()
    }

    /// Returns the derives to be applied to a generated type.
    pub fn type_derives(&self, ty: &Type<PortableForm>) -> Derives {
        let joined_path = ty.path().segments().join("::");
        let ty_path: syn::TypePath = syn::parse_str(&joined_path).unwrap_or_else(|e| {
            abort_call_site!("'{}' is an invalid type path: {:?}", joined_path, e,)
        });
        self.derives.resolve(&ty_path)
    }
}

/// Represents a Rust `mod`, containing generated types and child `mod`s.
#[derive(Debug)]
pub struct Module {
    name: Ident,
    root_mod: Ident,
    children: BTreeMap<Ident, Module>,
    types: BTreeMap<scale_info::Path<PortableForm>, TypeDefGen>,
}

impl ToTokens for Module {
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

impl Module {
    /// Create a new [`Module`], with a reference to the root `mod` for resolving type paths.
    pub(crate) fn new(name: Ident, root_mod: Ident) -> Self {
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

    /// Returns this `Module`s child `mod`s.
    pub fn children(&self) -> impl Iterator<Item = (&Ident, &Module)> {
        self.children.iter()
    }

    /// Returns the generated types.
    pub fn types(
        &self,
    ) -> impl Iterator<Item = (&scale_info::Path<PortableForm>, &TypeDefGen)> {
        self.types.iter()
    }

    /// Returns the root `mod` used for resolving type paths.
    pub fn root_mod(&self) -> &Ident {
        &self.root_mod
    }
}

#[derive(Debug, Clone)]
pub struct CratePath(syn::Path);

impl CratePath {
    /// Create a new `CratePath` from a `syn::Path`.
    pub fn new(path: syn::Path) -> Self {
        Self(path)
    }
}

impl Default for CratePath {
    fn default() -> Self {
        Self(syn::parse_quote!(::subxt))
    }
}

impl From<syn::Path> for CratePath {
    fn from(path: syn::Path) -> Self {
        CratePath::new(path)
    }
}

impl ToTokens for CratePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl From<&str> for CratePath {
    fn from(crate_path: &str) -> Self {
        Self(syn::Path::from_string(crate_path).unwrap_or_else(|err| {
            panic!(
                "failed converting {:?} to `syn::Path`: {:?}",
                crate_path, err
            );
        }))
    }
}

impl From<String> for CratePath {
    fn from(crate_path: String) -> Self {
        CratePath::from(crate_path.as_str())
    }
}

impl From<Option<String>> for CratePath {
    fn from(maybe_crate_path: Option<String>) -> Self {
        match maybe_crate_path {
            None => CratePath::default(),
            Some(crate_path) => crate_path.into(),
        }
    }
}
