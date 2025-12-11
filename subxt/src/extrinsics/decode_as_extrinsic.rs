use scale_decode::DecodeAsFields;

/// This trait can be implemented for any type which implements [`DecodeAsFields`].
/// This adds information to the type about which extrinsic it is, which enforces that
/// only the correct extrinsic can be decoded into it.
pub trait DecodeAsExtrinsic: DecodeAsFields {
    /// Pallet name.
    const PALLET_NAME: &'static str;
    /// Call name.
    const CALL_NAME: &'static str;

    /// Returns true if the given pallet and call names match this extrinsic.
    fn is_extrinsic(pallet: &str, call: &str) -> bool {
        Self::PALLET_NAME == pallet && Self::CALL_NAME == call
    }
}
