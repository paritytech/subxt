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
