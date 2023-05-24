// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::collections::HashMap;

/// A minimal ordered map to let one search for
/// things by key or get the values in insert order.
#[derive(Debug, Clone)]
pub struct OrderedMap<K, V> {
    values: Vec<V>,
    map: HashMap<K, usize>,
}

impl<K, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            map: Default::default(),
        }
    }
}

impl<K, V> OrderedMap<K, V>
where
    K: PartialEq + Eq + std::hash::Hash,
{
    /// Create a new, empty [`OrderedMap`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of entries in the map.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Is the map empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Retain specific entries.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&V) -> bool,
    {
        let values = std::mem::take(&mut self.values);
        let map = std::mem::take(&mut self.map);

        // Filter the values, storing a map from old to new positions:
        let mut new_values = Vec::new();
        let mut old_pos_to_new_pos = HashMap::new();
        for (pos, value) in values.into_iter().enumerate().filter(|(_, v)| f(v)) {
            old_pos_to_new_pos.insert(pos, new_values.len());
            new_values.push(value);
        }

        // Update the values now we've filtered them:
        self.values = new_values;

        // Rebuild the map using the new positions:
        self.map = map
            .into_iter()
            .filter_map(|(k, v)| old_pos_to_new_pos.get(&v).map(|v2| (k, *v2)))
            .collect();
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
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.map.get(key).and_then(|&v| self.values.get(v))
    }

    /// Get an item by its index.
    pub fn get_by_index(&self, i: usize) -> Option<&V> {
        self.values.get(i)
    }

    /// Access the underlying values.
    pub fn values(&self) -> &[V] {
        &self.values
    }

    /// Mutable access to the underlying values.
    pub fn values_mut(&mut self) -> &mut [V] {
        &mut self.values
    }

    /// Return the underlying values.
    pub fn into_values(self) -> Vec<V> {
        self.values
    }
}

impl<K, V> FromIterator<(K, V)> for OrderedMap<K, V>
where
    K: PartialEq + Eq + std::hash::Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = OrderedMap::new();
        for (k, v) in iter {
            map.push_insert(k, v)
        }
        map
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retain() {
        let mut m = OrderedMap::from_iter([(1, 'a'), (2, 'b'), (3, 'c')]);

        m.retain(|v| *v != 'b');

        assert_eq!(m.get_by_key(&1), Some(&'a'));
        assert_eq!(m.get_by_key(&2), None);
        assert_eq!(m.get_by_key(&3), Some(&'c'));

        assert_eq!(m.values(), &['a', 'c'])
    }
}
