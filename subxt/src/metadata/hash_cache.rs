// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use parking_lot::RwLock;
use std::{borrow::Cow, collections::HashMap};

/// A cache with the simple goal of storing 32 byte hashes against root+item keys
#[derive(Default, Debug)]
pub struct HashCache {
    inner: RwLock<HashMap<RootItemKey<'static>, [u8; 32]>>,
}

impl HashCache {
    /// get a hash out of the cache by its root and item key. If the item doesn't exist,
    /// run the function provided to obtain a hash to insert (or bail with some error on failure).
    pub fn get_or_insert<F, E>(&self, root: &str, item: &str, f: F) -> Result<[u8; 32], E>
    where
        F: FnOnce() -> Result<[u8; 32], E>,
    {
        let maybe_hash = self
            .inner
            .read()
            .get(&RootItemKey::new(root, item))
            .copied();

        if let Some(hash) = maybe_hash {
            return Ok(hash);
        }

        let hash = f()?;
        self.inner
            .write()
            .insert(RootItemKey::new(root.to_string(), item.to_string()), hash);

        Ok(hash)
    }
}

/// This exists so that we can look items up in the cache using &strs, without having to allocate
/// Strings first (as you'd have to do to construct something like an `&(String,String)` key).
#[derive(Debug, PartialEq, Eq, Hash)]
struct RootItemKey<'a> {
    pallet: Cow<'a, str>,
    item: Cow<'a, str>,
}

impl<'a> RootItemKey<'a> {
    fn new(pallet: impl Into<Cow<'a, str>>, item: impl Into<Cow<'a, str>>) -> Self {
        RootItemKey {
            pallet: pallet.into(),
            item: item.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_cache_validation() {
        let cache = HashCache::default();

        let pallet = "System";
        let item = "Account";
        let mut call_number = 0;
        let value = cache.get_or_insert(pallet, item, || -> Result<[u8; 32], ()> {
            call_number += 1;
            Ok([0; 32])
        });

        assert_eq!(
            cache
                .inner
                .read()
                .get(&RootItemKey::new(pallet, item))
                .unwrap(),
            &value.unwrap()
        );
        assert_eq!(value.unwrap(), [0; 32]);
        assert_eq!(call_number, 1);

        // Further calls must be hashed.
        let value = cache.get_or_insert(pallet, item, || -> Result<[u8; 32], ()> {
            call_number += 1;
            Ok([0; 32])
        });
        assert_eq!(call_number, 1);
        assert_eq!(value.unwrap(), [0; 32]);
    }
}
