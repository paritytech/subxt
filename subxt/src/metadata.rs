use std::sync::Arc;

// Re-export everything from subxt-metadata here.
pub use subxt_metadata::*;

/// A cheaply clonable version of our [`Metadata`].
pub type ArcMetadata = Arc<Metadata>;
