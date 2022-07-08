// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/// This is returned from constant accesses in the statically generated
/// code, and contains the information needed to find, validate and decode
/// the constant.
pub struct ConstantAddress<'a, ReturnTy> {
    pallet_name: &'a str,
    constant_name: &'a str,
    constant_hash: Option<[u8; 32]>,
    _marker: std::marker::PhantomData<ReturnTy>
}

impl <'a, ReturnTy> ConstantAddress<'a, ReturnTy> {
    /// Create a new [`ConstantAddress`] that will be validated
    /// against node metadata using the hash given.
    pub fn new_with_validation(pallet_name: &'a str, constant_name: &'a str, hash: [u8; 32]) -> Self {
        Self {
            pallet_name,
            constant_name,
            constant_hash: Some(hash),
            _marker: std::marker::PhantomData
        }
    }

    /// Do not validate this constant prior to accessing it.
    pub fn unvalidated(self) -> Self {
        Self {
            pallet_name: self.pallet_name,
            constant_name: self.constant_name,
            constant_hash: None,
            _marker: self._marker
        }
    }

    /// The pallet name.
    pub fn pallet_name(&self) -> &'a str {
        self.pallet_name
    }

    /// The constant name.
    pub fn constant_name(&self) -> &'a str {
        self.constant_name
    }

    /// A hash used for metadata validation.
    pub(super) fn validation_hash(&self) -> Option<[u8; 32]> {
        self.constant_hash
    }
}