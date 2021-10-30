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

#[cfg(test)]
mod tests;
mod type_def;
mod type_path;

use super::GeneratedTypeDerives;
use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
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
    type_def::TypeDefGen,
    type_path::{
        TypeParameter,
        TypePath,
        TypePathSubstitute,
        TypePathType,
    },
};

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
    derives: GeneratedTypeDerives,
}

impl<'a> TypeGenerator<'a> {
    /// Construct a new [`TypeGenerator`].
    pub fn new(
        type_registry: &'a PortableRegistry,
        root_mod: &'static str,
        type_substitutes: HashMap<String, syn::TypePath>,
        derives: GeneratedTypeDerives,
    ) -> Self {
        let root_mod_ident = Ident::new(root_mod, Span::call_site());
        Self {
            types_mod_ident: root_mod_ident,
            type_registry,
            type_substitutes,
            derives,
        }
    }

    /// Generate a module containing all types defined in the supplied type registry.
    pub fn generate_types_mod(&'a self) -> Module<'a> {
        let mut root_mod =
            Module::new(self.types_mod_ident.clone(), self.types_mod_ident.clone());

        for (id, ty) in self.type_registry.types().iter().enumerate() {
            if ty.ty().path().namespace().is_empty() {
                // prelude types e.g. Option/Result have no namespace, so we don't generate them
                continue
            }
            self.insert_type(
                ty.ty().clone(),
                id as u32,
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
                .insert(ty.path().clone(), TypeDefGen { ty, type_gen: self });
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
                root_mod_ident: self.types_mod_ident.clone(),
            })
        }
    }

    /// Returns the derives with which all generated type will be decorated.
    pub fn derives(&self) -> &GeneratedTypeDerives {
        &self.derives
    }
}

#[derive(Debug)]
pub struct Module<'a> {
    name: Ident,
    root_mod: Ident,
    children: BTreeMap<Ident, Module<'a>>,
    types: BTreeMap<scale_info::Path<scale_info::form::PortableForm>, TypeDefGen<'a>>,
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
