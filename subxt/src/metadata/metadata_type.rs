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

    /// Identical to `metadata.pallet_by_name()`, but returns an error if the pallet is not found.
    pub fn pallet_by_name_err(
        &self,
        name: &str,
    ) -> Result<subxt_metadata::PalletMetadata, MetadataError> {
        self.pallet_by_name(name)
            .ok_or_else(|| MetadataError::PalletNameNotFound(name.to_owned()))
    }

    /// Identical to `metadata.pallet_by_index()`, but returns an error if the pallet is not found.
    pub fn pallet_by_index_err(
        &self,
        index: u8,
    ) -> Result<subxt_metadata::PalletMetadata, MetadataError> {
        self.pallet_by_index(index)
            .ok_or(MetadataError::PalletIndexNotFound(index))
    }

    /// Identical to `metadata.runtime_api_trait_by_name()`, but returns an error if the trait is not found.
    pub fn runtime_api_trait_by_name_err(
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

impl TryFrom<frame_metadata::RuntimeMetadataPrefixed> for Metadata {
    type Error = subxt_metadata::TryFromError;
    fn try_from(value: frame_metadata::RuntimeMetadataPrefixed) -> Result<Self, Self::Error> {
        subxt_metadata::Metadata::try_from(value).map(Metadata::from)
    }
}

impl codec::Decode for Metadata {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        subxt_metadata::Metadata::decode(input).map(Metadata::new)
    }
}
