mod fetch_metadata;

// easy access to this type needed for fetching metadata:
pub use jsonrpsee::client_transport::ws::Uri;

pub use fetch_metadata::{
    fetch_metadata_bytes, fetch_metadata_bytes_blocking, fetch_metadata_hex,
    fetch_metadata_hex_blocking, FetchMetadataError,
};
