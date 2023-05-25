// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::MetadataError;
use std::sync::Arc;

/// A cheaply clone-able representation of the runtime metadata received from a node.
#[derive(Clone, Debug)]
pub struct Metadata {
    inner: Arc<subxt_metadata::Metadata>,
}

impl std::ops::Deref for Metadata {
    type Target = subxt_metadata::Metadata;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Metadata {
    pub(crate) fn new(md: subxt_metadata::Metadata) -> Self {
        Metadata {
            inner: Arc::new(md),
        }
    }

    pub(crate) fn pallet_by_name_err(
        &self,
        name: &str,
    ) -> Result<subxt_metadata::PalletMetadata, MetadataError> {
        self.pallet_by_name(name)
            .ok_or_else(|| MetadataError::PalletNameNotFound(name.to_owned()))
    }

    pub(crate) fn pallet_by_index_err(
        &self,
        index: u8,
    ) -> Result<subxt_metadata::PalletMetadata, MetadataError> {
        self.pallet_by_index(index)
            .ok_or_else(|| MetadataError::PalletIndexNotFound(index))
    }

    pub(crate) fn runtime_api_trait_by_name_err(
        &self,
        name: &str,
    ) -> Result<subxt_metadata::RuntimeApiMetadata, MetadataError> {
        self.runtime_api_trait_by_name(name)
            .ok_or_else(|| MetadataError::RuntimeTraitNotFound(name.to_owned()))
    }
}

impl From<subxt_metadata::Metadata> for Metadata {
    fn from(md: subxt_metadata::Metadata) -> Self {
        Metadata::new(md)
    }
}

impl codec::Decode for Metadata {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        subxt_metadata::Metadata::decode(input).map(Metadata::new)
    }
}
