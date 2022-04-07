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

use std::{
    borrow::Cow,
    collections::HashMap,
    sync::RwLock,
};

/// A cache with the simple goal of storing 32 byte hashes against pallet+item keys
#[derive(Default, Debug)]
pub struct HashCache {
    inner: RwLock<HashMap<PalletItemKey<'static>, [u8; 32]>>,
}

impl HashCache {
    /// get a hash out of the cache by its pallet and item key
    pub fn get(&self, pallet: &str, item: &str) -> Option<[u8; 32]> {
        self.inner
            .read()
            .unwrap()
            .get(&PalletItemKey::new(pallet, item))
            .map(|i| *i)
    }

    /// set a hash in the hash by its pallet and item key
    pub fn insert<P: Into<String>, I: Into<String>>(
        &self,
        pallet: P,
        item: I,
        hash: [u8; 32],
    ) {
        self.inner
            .write()
            .unwrap()
            .insert(PalletItemKey::new(pallet.into(), item.into()), hash);
    }
}

/// This exists so that we can look items up in the cache using &strs, without having to allocate
/// Strings first (as you'd have to do to construct something like an `&(String,String)` key).
#[derive(Debug, PartialEq, Eq, Hash)]
struct PalletItemKey<'a> {
    pallet: Cow<'a, str>,
    item: Cow<'a, str>,
}

impl<'a> PalletItemKey<'a> {
    fn new(pallet: impl Into<Cow<'a, str>>, item: impl Into<Cow<'a, str>>) -> Self {
        PalletItemKey {
            pallet: pallet.into(),
            item: item.into(),
        }
    }
}
