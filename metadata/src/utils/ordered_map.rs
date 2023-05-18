// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::collections::HashMap;

/// A minimal, append-only ordered map to let one search for
/// things by key or get the values in insert order.
pub struct OrderedMap<K,V> {
    values: Vec<V>,
    map: HashMap<K, usize>
}

impl <K, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            map: Default::default()
        }
    }
}

impl <K, V> OrderedMap<K, V>
where
    K: PartialEq + Eq + std::hash::Hash
{
    /// Create a new, empty [`OrderedMap`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Push/insert an item to the end of the map.
    pub fn push_insert(&mut self, key: K, value: V) {
        let idx = self.values.len();
        self.values.push(value);
        self.map.insert(key, idx);
    }

    /// Get an item by its key.
    pub fn get_by_key<Q>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized
    {
        self.map
            .get(key)
            .and_then(|&v| self.values.get(v))
    }

    /// Get an item by its index.
    pub fn get_by_index(&self, i: usize) -> Option<&V> {
        self.values.get(i)
    }

    /// Access the underlying values.
    pub fn values(&self) -> &[V] {
        &self.values
    }

    /// Return the underlying values.
    pub fn into_values(self) -> Vec<V> {
        self.values
    }
}

impl <K, V> FromIterator<(K, V)> for OrderedMap<K, V>
where
    K: PartialEq + Eq + std::hash::Hash
{
    fn from_iter<T: IntoIterator<Item = (K,V)>>(iter: T) -> Self {
        let mut map = OrderedMap::new();
        for (k,v) in iter {
            map.push_insert(k, v)
        }
        map
    }
}