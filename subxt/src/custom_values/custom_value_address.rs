use crate::dynamic::DecodedValueThunk;
use crate::metadata::DecodeWithMetadata;

/// This represents the address of a custom value in in the metadata.
/// Anything, that implements the [CustomValueAddress] trait can be used, to fetch
/// custom values from the metadata.
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
