// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Construct addresses to access constants with.

use alloc::borrow::Cow;
use alloc::string::String;
use derive_where::derive_where;
use scale_decode::DecodeAsType;

/// This represents a constant address. Anything implementing this trait
/// can be used to fetch constants.
pub trait Address {
    /// The target type of the value that lives at this address.
    type Target: DecodeAsType;

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

// Any reference to an address is a valid address.
impl<'a, A: Address + ?Sized> Address for &'a A {
    type Target = A::Target;

    fn pallet_name(&self) -> &str {
        A::pallet_name(*self)
    }

    fn constant_name(&self) -> &str {
        A::constant_name(*self)
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        A::validation_hash(*self)
    }
}

// (str, str) and similar are valid addresses.
impl<A: AsRef<str>, B: AsRef<str>> Address for (A, B) {
    type Target = scale_value::Value;

    fn pallet_name(&self) -> &str {
        self.0.as_ref()
    }

    fn constant_name(&self) -> &str {
        self.1.as_ref()
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

/// This represents the address of a constant.
#[derive_where(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct StaticAddress<ReturnTy> {
    pallet_name: Cow<'static, str>,
    constant_name: Cow<'static, str>,
    constant_hash: Option<[u8; 32]>,
    _marker: core::marker::PhantomData<ReturnTy>,
}

/// A dynamic lookup address to access a constant.
pub type DynamicAddress<ReturnTy> = StaticAddress<ReturnTy>;

impl<ReturnTy> StaticAddress<ReturnTy> {
    /// Create a new [`StaticAddress`] to use to look up a constant.
    pub fn new(pallet_name: impl Into<String>, constant_name: impl Into<String>) -> Self {
        Self {
            pallet_name: Cow::Owned(pallet_name.into()),
            constant_name: Cow::Owned(constant_name.into()),
            constant_hash: None,
            _marker: core::marker::PhantomData,
        }
    }

    /// Create a new [`StaticAddress`] that will be validated
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
            _marker: core::marker::PhantomData,
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

impl<ReturnTy: DecodeAsType> Address for StaticAddress<ReturnTy> {
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
pub fn dynamic<ReturnTy: DecodeAsType>(
    pallet_name: impl Into<String>,
    constant_name: impl Into<String>,
) -> DynamicAddress<ReturnTy> {
    DynamicAddress::new(pallet_name, constant_name)
}
