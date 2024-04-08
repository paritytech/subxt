// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Construct addresses to access custom values with.

use derive_where::derive_where;
use crate::dynamic::DecodedValueThunk;
use crate::metadata::DecodeWithMetadata;

/// Use this with [`ConstantvalueAddress::IsDecodable`].
pub use crate::utils::Yes;

/// This represents the address of a custom value in in the metadata.
/// Anything that implements it can be used to fetch custom values from the metadata.
/// The trait is implemented by [`str`] for dynamic lookup and [`StaticAddress`] for static queries.
pub trait AddressT {
    /// The type of the custom value.
    type Target: DecodeWithMetadata;
    /// Should be set to `Yes` for Dynamic values and static values that have a valid type.
    /// Should be `()` for custom values, that have an invalid type id.
    type IsDecodable;

    /// the name (key) by which the custom value can be accessed in the metadata.
    fn name(&self) -> &str;

    /// An optional hash which, if present, can be checked against node metadata.
    fn validation_hash(&self) -> Option<[u8; 32]> {
        None
    }
}

impl AddressT for str {
    type Target = DecodedValueThunk;
    type IsDecodable = Yes;

    fn name(&self) -> &str {
        self
    }
}

/// A static address to a custom value.
#[derive_where(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct StaticAddress<ReturnTy, IsDecodable> {
    name: &'static str,
    hash: Option<[u8; 32]>,
    phantom: core::marker::PhantomData<(ReturnTy, IsDecodable)>,
}

impl<ReturnTy, IsDecodable> StaticAddress<ReturnTy, IsDecodable> {
    #[doc(hidden)]
    /// Creates a new StaticAddress.
    pub fn new_static(name: &'static str, hash: [u8; 32]) -> StaticAddress<ReturnTy, IsDecodable> {
        StaticAddress::<ReturnTy, IsDecodable> {
            name,
            hash: Some(hash),
            phantom: core::marker::PhantomData,
        }
    }

    /// Do not validate this custom value prior to accessing it.
    pub fn unvalidated(self) -> Self {
        Self {
            name: self.name,
            hash: None,
            phantom: self.phantom,
        }
    }
}

impl<R: DecodeWithMetadata, Y> AddressT for StaticAddress<R, Y> {
    type Target = R;
    type IsDecodable = Y;

    fn name(&self) -> &str {
        self.name
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.hash
    }
}
