// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::prelude::*;
use borrow::ToOwned;
use derive_more::Display;
use string::String;
use sync::Arc;

/// A cheaply clone-able representation of the runtime metadata received from a node.
#[derive(Clone, Debug)]
pub struct Metadata {
    inner: Arc<subxt_metadata::Metadata>,
}

impl core::ops::Deref for Metadata {
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

/// Something went wrong trying to access details in the metadata.
#[derive(Clone, Debug, PartialEq, Display)]
#[non_exhaustive]
pub enum MetadataError {
    /// The DispatchError type isn't available in the metadata
    #[display(fmt = "The DispatchError type isn't available")]
    DispatchErrorNotFound,
    /// Type not found in metadata.
    #[display(fmt = "Type with ID {_0} not found")]
    TypeNotFound(u32),
    /// Pallet not found (index).
    #[display(fmt = "Pallet with index {_0} not found")]
    PalletIndexNotFound(u8),
    /// Pallet not found (name).
    #[display(fmt = "Pallet with name {_0} not found")]
    PalletNameNotFound(String),
    /// Variant not found.
    #[display(fmt = "Variant with index {_0} not found")]
    VariantIndexNotFound(u8),
    /// Constant not found.
    #[display(fmt = "Constant with name {_0} not found")]
    ConstantNameNotFound(String),
    /// Call not found.
    #[display(fmt = "Call with name {_0} not found")]
    CallNameNotFound(String),
    /// Runtime trait not found.
    #[display(fmt = "Runtime trait with name {_0} not found")]
    RuntimeTraitNotFound(String),
    /// Runtime method not found.
    #[display(fmt = "Runtime method with name {_0} not found")]
    RuntimeMethodNotFound(String),
    /// Call type not found in metadata.
    #[display(fmt = "Call type not found in pallet with index {_0}")]
    CallTypeNotFoundInPallet(u8),
    /// Event type not found in metadata.
    #[display(fmt = "Event type not found in pallet with index {_0}")]
    EventTypeNotFoundInPallet(u8),
    /// Storage details not found in metadata.
    #[display(fmt = "Storage details not found in pallet with name {_0}")]
    StorageNotFoundInPallet(String),
    /// Storage entry not found.
    #[display(fmt = "Storage entry {_0} not found")]
    StorageEntryNotFound(String),
    /// The generated interface used is not compatible with the node.
    #[display(fmt = "The generated code is not compatible with the node")]
    IncompatibleCodegen,
    /// Custom value not found.
    #[display(fmt = "Custom value with name {_0} not found")]
    CustomValueNameNotFound(String),
}
