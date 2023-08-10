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

    /// the name (key) by which the custom value can be accessed in the metadata.
    fn name(&self) -> &str;
}

impl CustomValueAddress for str {
    type Target = DecodedValueThunk;

    fn name(&self) -> &str {
        self
    }
}

/// A static address to a custom value.
pub struct StaticAddress<R> {
    name: &'static str,
    phantom: PhantomData<R>,
}

impl<R> StaticAddress<R> {
    /// Creates a new StaticAddress.
    pub fn new(name: &'static str) -> Self {
        StaticAddress {
            name,
            phantom: PhantomData,
        }
    }
}

impl<R: DecodeWithMetadata> CustomValueAddress for StaticAddress<R> {
    type Target = R;

    fn name(&self) -> &str {
        self.name
    }
}
