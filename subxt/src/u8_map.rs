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

use std::iter::FromIterator;

/// A map where the key is a `u8`. Allows for constant-time access
/// with no hashing overhead.
///
/// **Note:** This map can only store 255 entries;
/// one slot is reserved for "item not found".
#[derive(Debug, Clone)]
pub struct U8Map<T> {
    indexes: [u8; 256],
    items: Vec<T>,
}

const MAX_ENTRIES: usize = u8::MAX as usize;

impl<V> U8Map<V> {
    /// Create a new, empty [`U8Map`].
    pub fn new() -> Self {
        U8Map {
            // `u8::MAX` in a slot == "item not found".
            indexes: [u8::MAX; 256],
            items: Vec::new(),
        }
    }

    /// Insert a value at some location, and return the
    /// value previously stored there if one exists.
    ///
    /// # Panics
    ///
    /// This map can only store 255 entries (as aone entry is reserved
    /// for book keeping), and will panic on attempting to insert more.
    pub fn insert(&mut self, key: u8, value: V) -> Option<V> {
        let idx = self.indexes[key as usize];
        if idx == u8::MAX {
            // No entry in the slot; push new one to vec.
            if self.items.len() == MAX_ENTRIES {
                panic!("U8Map can only store `u8::MAX - 1` (255) entries; it's run out of space!");
            }
            let loc = self.items.len() as u8;
            self.items.push(value);
            self.indexes[key as usize] = loc;
            None
        } else {
            // Existing entry found; replace it and return original.
            let item = self
                .items
                .get_mut(idx as usize)
                .expect("item must exist if in indexes");
            let old_value = std::mem::replace(item, value);
            Some(old_value)
        }
    }

    /// Get the value at a given location, `None` if not found.
    pub fn get(&self, key: u8) -> Option<&V> {
        let idx = self.indexes[key as usize];
        if idx == u8::MAX {
            None
        } else {
            let item = self
                .items
                .get(idx as usize)
                .expect("item must exist if in indexes");
            Some(item)
        }
    }
}

impl<V> FromIterator<(u8, V)> for U8Map<V> {
    fn from_iter<T: IntoIterator<Item = (u8, V)>>(iter: T) -> Self {
        let mut map = U8Map::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn insert_and_retrieve() {
        let mut m = U8Map::new();

        m.insert(123, "123");
        m.insert(10, "10");

        assert_eq!(m.get(123), Some(&"123"));
        assert_eq!(m.get(10), Some(&"10"));
        assert_eq!(m.get(124), None);
    }

    #[test]
    fn collect_255_vals() {
        let m: U8Map<String> = (0..255u8).map(|i| (i, i.to_string())).collect();
        for i in 0..255u8 {
            assert_eq!(m.get(i), Some(&i.to_string()));
        }
    }

    #[test]
    #[should_panic]
    fn collect_256_vals() {
        let m: U8Map<String> = (0..=255u8).map(|i| (i, i.to_string())).collect();
        for i in 0..=255u8 {
            assert_eq!(m.get(i), Some(&i.to_string()));
        }
    }

    #[test]
    fn test_replacing() {
        let mut m = U8Map::new();

        assert_eq!(m.insert(123, "one"), None);
        assert_eq!(m.insert(123, "two"), Some("one"));
        assert_eq!(m.insert(123, "three"), Some("two"));

        assert_eq!(m.get(123), Some(&"three"));
    }
}
