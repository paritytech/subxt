use scale_decode::DecodeAsFields;

/// Trait to uniquely identify the extrinsic's identity from the runtime metadata.
///
/// Generated API structures that represent an extrinsic implement this trait.
///
/// The trait is utilized to decode emitted extrinsics from a block, via obtaining the
/// form of the `Extrinsic` from the metadata.
pub trait StaticExtrinsic: DecodeAsFields {
    /// Pallet name.
    const PALLET: &'static str;
    /// Call name.
    const CALL: &'static str;

    /// Returns true if the given pallet and call names match this extrinsic.
    fn is_extrinsic(pallet: &str, call: &str) -> bool {
        Self::PALLET == pallet && Self::CALL == call
    }
}
