// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::sync::RwLock;
use std::collections::HashMap;

/// A naive cache with the simple goal of storing 32 byte hashes against root+item keys.
#[derive(Default, Debug)]
pub struct HashCache {
    inner: RwLock<HashMap<String, [u8; 32]>>,
}

impl HashCache {
    /// get a hash out of the cache by its root and item key. If the item doesn't exist,
    /// run the function provided to obtain a hash to insert (or bail with some error on failure).
    pub fn get_or_insert<F>(&self, key: &str, f: F) -> Option<[u8; 32]>
    where
        F: FnOnce() -> Option<[u8; 32]>,
    {
        let maybe_hash = self
            .inner
            .read()
            .expect("shouldn't be poisoned")
            .get(key)
            .copied();

        if let Some(hash) = maybe_hash {
            return Some(hash);
        }

        let hash = f()?;
        self.inner
            .write()
            .expect("shouldn't be poisoned")
            .insert(key.to_string(), hash);

        Some(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_cache_validation() {
        let cache = HashCache::default();

        let item = "Account";
        let mut call_number = 0;
        let value = cache.get_or_insert(item, || -> Option<[u8; 32]> {
            call_number += 1;
            Some([0; 32])
        });

        assert_eq!(
            cache
                .inner
                .read()
                .expect("shouldn't be poisoned")
                .get(item)
                .unwrap(),
            &value.unwrap()
        );
        assert_eq!(value.unwrap(), [0; 32]);
        assert_eq!(call_number, 1);

        // Further calls must be hashed.
        let value = cache.get_or_insert(item, || -> Option<[u8; 32]> {
            call_number += 1;
            Some([0; 32])
        });
        assert_eq!(call_number, 1);
        assert_eq!(value.unwrap(), [0; 32]);
    }
}
