use derivative::Derivative;
use std::marker::PhantomData;

use crate::dynamic::DecodedValueThunk;
use crate::metadata::DecodeWithMetadata;

/// This represents the address of a custom value in in the metadata.
/// Anything, that implements the [CustomValueAddress] trait can be used, to fetch
/// custom values from the metadata.
/// The trait is implemented by [str] for dynamic loopup and [StaticAddress] for static queries.
pub trait CustomValueAddress {
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

impl CustomValueAddress for str {
    type Target = DecodedValueThunk;
    type IsDecodable = Yes;

    fn name(&self) -> &str {
        self
    }
}

/// Used to signal whether a [`CustomValueAddress`] can be decoded.
pub struct Yes;

/// A static address to a custom value.
#[derive(Derivative)]
#[derivative(
    Clone(bound = ""),
    Debug(bound = ""),
    Eq(bound = ""),
    Ord(bound = ""),
    PartialEq(bound = ""),
    PartialOrd(bound = "")
)]
pub struct StaticAddress<ReturnTy, IsDecodable> {
    name: &'static str,
    hash: Option<[u8; 32]>,
    phantom: PhantomData<(ReturnTy, IsDecodable)>,
}

impl<ReturnTy, IsDecodable> StaticAddress<ReturnTy, IsDecodable> {
    #[doc(hidden)]
    /// Creates a new StaticAddress.
    pub fn new_static(name: &'static str, hash: [u8; 32]) -> StaticAddress<ReturnTy, IsDecodable> {
        StaticAddress::<ReturnTy, IsDecodable> {
            name,
            hash: Some(hash),
            phantom: PhantomData,
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

impl<R: DecodeWithMetadata, Y> CustomValueAddress for StaticAddress<R, Y> {
    type Target = R;
    type IsDecodable = Y;

    fn name(&self) -> &str {
        self.name
    }

    fn validation_hash(&self) -> Option<[u8; 32]> {
        self.hash
    }
}
