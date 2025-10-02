// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Construct addresses to access custom values with.

use derive_where::derive_where;
use scale_decode::DecodeAsType;
use alloc::borrow::Cow;

/// Use this with [`Address::IsDecodable`].
pub use crate::utils::{No, Maybe, NoMaybe};

/// This represents the address of a custom value in the metadata.
/// Anything that implements it can be used to fetch custom values from the metadata.
/// The trait is implemented by [`str`] for dynamic lookup and [`StaticAddress`] for static queries.
pub trait Address {
    /// The type of the custom value.
    type Target: DecodeAsType;
    /// Should be set to `Yes` for Dynamic values and static values that have a valid type.
    /// Should be `No` for custom values, that have an invalid type id.
    type IsDecodable: NoMaybe;

    /// the name (key) by which the custom value can be accessed in the metadata.
    fn name(&self) -> &str;

    /// An optional hash which, if present, can be checked against node metadata.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

/// A static address to a custom value.
#[derive_where(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct StaticAddress<ReturnTy, IsDecodable> {
    name: Cow<'static, str>,
    hash: Option<[u8; 32]>,
    marker: core::marker::PhantomData<(ReturnTy, IsDecodable)>,
}

/// A dynamic address to a custom value.
pub type DynamicAddress<ReturnTy> = StaticAddress<ReturnTy, Maybe>;

impl<ReturnTy, IsDecodable> StaticAddress<ReturnTy, IsDecodable> {
    #[doc(hidden)]
    /// Creates a new StaticAddress.
    pub fn new_static(name: &'static str, hash: [u8; 32]) -> Self {
        Self {
            name: Cow::Borrowed(name),
            hash: Some(hash),
            marker: core::marker::PhantomData,
        }
    }

    /// Create a new [`StaticAddress`]
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            hash: None,
            marker: core::marker::PhantomData,
        }
    }

    /// Do not validate this custom value prior to accessing it.
    pub fn unvalidated(self) -> Self {
        Self {
            name: self.name,
            hash: None,
            marker: self.marker,
        }
    }
}

impl<Target: DecodeAsType, IsDecodable: NoMaybe> Address for StaticAddress<Target, IsDecodable> {
    type Target = Target;
    type IsDecodable = IsDecodable;

    fn name(&self) -> &str {
        &self.name
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.hash
    }
}

// Support plain strings for looking up custom values (but prefer `dynamic` if you want to pick the return type)
impl Address for str {
    type Target = scale_value::Value;
    type IsDecodable = Maybe;

    fn name(&self) -> &str {
        self
    }
}

/// Construct a new dynamic custom value lookup.
pub fn dynamic<ReturnTy: DecodeAsType>(custom_value_name: impl Into<Cow<'static, str>>) -> DynamicAddress<ReturnTy> {
    DynamicAddress::new(custom_value_name)
}
