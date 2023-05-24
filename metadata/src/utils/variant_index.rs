// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use scale_info::{form::PortableForm, PortableRegistry, TypeDef, Variant};
use std::collections::HashMap;

/// Given some type ID and type registry, build a couple of
/// indexes to look up variants by index or name. If the ID provided
/// is not a variant, the index will be empty.
///
/// API optimized for dealing with the `Option<u32>` variant type IDs
/// that we get in metadata pallets.
#[derive(Debug, Clone)]
pub struct VariantIndex {
    by_name: HashMap<String, usize>,
    by_index: HashMap<u8, usize>,
}

impl VariantIndex {
    /// Build indexes from the optional variant ID.
    pub fn build(variant_id: Option<u32>, types: &PortableRegistry) -> Self {
        let Some(variants) = Self::get(variant_id, types) else {
            return Self::empty()
        };

        let mut by_name = HashMap::new();
        let mut by_index = HashMap::new();
        for (pos, variant) in variants.iter().enumerate() {
            by_name.insert(variant.name.to_owned(), pos);
            by_index.insert(variant.index, pos);
        }

        Self { by_name, by_index }
    }

    /// Build an empty index.
    pub fn empty() -> Self {
        Self {
            by_name: Default::default(),
            by_index: Default::default(),
        }
    }

    /// Get the variants we're pointing at; None if this isn't possible.
    pub fn get(
        variant_id: Option<u32>,
        types: &PortableRegistry,
    ) -> Option<&[Variant<PortableForm>]> {
        let Some(variant_id) = variant_id else {
            return None
        };
        let TypeDef::Variant(v) = &types.resolve(variant_id)?.type_def else {
            return None
        };
        Some(&v.variants)
    }

    /// Lookup a variant by name; `None` if the type is not a variant or name isn't found.
    pub fn lookup_by_name<'a, K>(
        &self,
        name: &K,
        variant_id: Option<u32>,
        types: &'a PortableRegistry,
    ) -> Option<&'a Variant<PortableForm>>
    where
        String: std::borrow::Borrow<K>,
        K: std::hash::Hash + Eq + ?Sized,
    {
        let pos = *self.by_name.get(name)?;
        let variants = Self::get(variant_id, types)?;
        variants.get(pos)
    }

    /// Lookup a variant by index; `None` if the type is not a variant or index isn't found.
    pub fn lookup_by_index<'a>(
        &self,
        index: u8,
        variant_id: Option<u32>,
        types: &'a PortableRegistry,
    ) -> Option<&'a Variant<PortableForm>> {
        let pos = *self.by_index.get(&index)?;
        let variants = Self::get(variant_id, types)?;
        variants.get(pos)
    }
}
