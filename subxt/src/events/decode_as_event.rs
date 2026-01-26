use scale_decode::DecodeAsFields;

/// This trait can be implemented for any type which implements [`DecodeAsFields`].
/// This adds information to the type about which event it is, which enforces that
/// only the correct event can be decoded into it.
pub trait DecodeAsEvent: DecodeAsFields {
    /// Returns true if the given pallet and event names match this event.
    fn is_event(pallet: &str, event: &str) -> bool;
}
