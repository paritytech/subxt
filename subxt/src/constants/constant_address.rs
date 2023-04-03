// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{dynamic::DecodedValueThunk, metadata::DecodeWithMetadata};
use std::borrow::Cow;

/// This represents a constant address. Anything implementing this trait
/// can be used to fetch constants.
pub trait ConstantAddress {
    /// The target type of the value that lives at this address.
    type Target: DecodeWithMetadata;

    /// The name of the pallet that the constant lives under.
    fn pallet_name(&self) -> &str;

    /// The name of the constant in a given pallet.
    fn constant_name(&self) -> &str;

    /// An optional hash which, if present, will be checked against
    /// the node metadata to confirm that the return type matches what
    /// we are expecting.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

/// This represents the address of a constant.
pub struct Address<ReturnTy> {
    pallet_name: Cow<'static, str>,
    constant_name: Cow<'static, str>,
    constant_hash: Option<[u8; 32]>,
    _marker: std::marker::PhantomData<ReturnTy>,
}

/// The type of address typically used to return dynamic constant values.
pub type DynamicAddress = Address<DecodedValueThunk>;

impl<ReturnTy> Address<ReturnTy> {
    /// Create a new [`Address`] to use to look up a constant.
    pub fn new(pallet_name: impl Into<String>, constant_name: impl Into<String>) -> Self {
        Self {
            pallet_name: Cow::Owned(pallet_name.into()),
            constant_name: Cow::Owned(constant_name.into()),
            constant_hash: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Create a new [`Address`] that will be validated
    /// against node metadata using the hash given.
    #[doc(hidden)]
    pub fn new_static(
        pallet_name: &'static str,
        constant_name: &'static str,
        hash: [u8; 32],
    ) -> Self {
        Self {
            pallet_name: Cow::Borrowed(pallet_name),
            constant_name: Cow::Borrowed(constant_name),
            constant_hash: Some(hash),
            _marker: std::marker::PhantomData,
        }
    }

    /// Do not validate this constant prior to accessing it.
    pub fn unvalidated(self) -> Self {
        Self {
            pallet_name: self.pallet_name,
            constant_name: self.constant_name,
            constant_hash: None,
            _marker: self._marker,
        }
    }
}

impl<ReturnTy: DecodeWithMetadata> ConstantAddress for Address<ReturnTy> {
    type Target = ReturnTy;

    fn pallet_name(&self) -> &str {
        &self.pallet_name
    }

    fn constant_name(&self) -> &str {
        &self.constant_name
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.constant_hash
    }
}

/// Construct a new dynamic constant lookup.
pub fn dynamic(pallet_name: impl Into<String>, constant_name: impl Into<String>) -> DynamicAddress {
    DynamicAddress::new(pallet_name, constant_name)
}
