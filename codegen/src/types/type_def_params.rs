// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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

use super::TypeParameter;
use crate::types::CompositeDefFieldType;
use quote::quote;
use std::collections::BTreeSet;

/// Represents the set of generic type parameters for generating a type definition e.g. the `T` in
/// `Foo<T>`.
///
/// Additionally this allows generating a `PhantomData` type for any type params which are unused
/// in the type definition itself.
#[derive(Clone, Debug, Default)]
pub struct TypeDefParameters {
    params: Vec<TypeParameter>,
    unused: BTreeSet<TypeParameter>,
}

impl TypeDefParameters {
    /// Create a new [`TypeDefParameters`] instance.
    pub fn new(params: Vec<TypeParameter>) -> Self {
        let unused = params.iter().cloned().collect();
        Self { params, unused }
    }

    /// Update the set of unused type parameters by removing those that are used in the given
    /// fields.
    pub fn update_unused<'a>(
        &mut self,
        fields: impl Iterator<Item = &'a CompositeDefFieldType>,
    ) {
        let mut used_type_params = BTreeSet::new();
        for field in fields {
            field.type_path.parent_type_params(&mut used_type_params)
        }
        for used_type_param in &used_type_params {
            self.unused.remove(used_type_param);
        }
    }

    /// Construct a [`core::marker::PhantomData`] for the type unused type params.
    pub fn unused_params_phantom_data(&self) -> Option<syn::TypePath> {
        if self.unused.is_empty() {
            return None
        }
        let params = if self.unused.len() == 1 {
            let param = self
                .unused
                .iter()
                .next()
                .expect("Checked for exactly one unused param");
            quote! { #param }
        } else {
            let params = self.unused.iter();
            quote! { ( #( #params ), * ) }
        };
        Some(syn::parse_quote! { ::core::marker::PhantomData<#params> })
    }

    /// Returns the set of type parameters.
    pub fn params(&self) -> &[TypeParameter] {
        &self.params
    }
}

impl<'a> quote::ToTokens for TypeDefParameters {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.params.is_empty() {
            let params = &self.params;
            tokens.extend(quote! { < #( #params ),* > })
        }
    }
}
