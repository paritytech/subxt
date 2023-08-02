use crate::client::OfflineClientT;
use crate::{Config, Metadata};
use scale_info::form::PortableForm;
use std::borrow::Borrow;

/// A client for accessing custom types.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct CustomTypesClient {
    metadata: Metadata,
}

impl CustomTypesClient {
    /// Create a new [`ConstantsClient`].
    pub fn new(metadata: Metadata) -> Self {
        Self { metadata }
    }
}

impl CustomTypesClient {
    pub fn get(&self, key: impl Borrow<str>) -> Option<&CustomValueMetadata> {
        self.metadata.custom_metadata().map.get(key)
    }
}
