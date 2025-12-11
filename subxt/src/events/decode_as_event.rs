use scale_decode::DecodeAsFields;

/// This trait can be implemented for any type which implements [`DecodeAsFields`].
/// This adds information to the type about which event it is, which enforces that
/// only the correct event can be decoded into it.
pub trait DecodeAsEvent: DecodeAsFields {
    /// Pallet name.
    const PALLET_NAME: &'static str;
    /// Event name.
    const EVENT_NAME: &'static str;

    /// Returns true if the given pallet and event names match this event.
    fn is_event(pallet: &str, event: &str) -> bool {
        Self::PALLET_NAME == pallet && Self::EVENT_NAME == event
    }
}
