// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::error::MetadataError;

use alloc::borrow::ToOwned;
use alloc::sync::Arc;
use frame_decode::extrinsics::{
    ExtrinsicCallInfo, ExtrinsicExtensionInfo, ExtrinsicInfoError,
    ExtrinsicSignatureInfo,
};
use frame_decode::storage::{
    StorageEntry, StorageInfo, StorageInfoError
};
use frame_decode::runtime_apis::{
    RuntimeApi, RuntimeApiInfo, RuntimeApiInfoError
};
use frame_decode::view_functions::{
    ViewFunction, ViewFunctionInfo, ViewFunctionInfoError
};

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

impl frame_decode::storage::StorageTypeInfo for Metadata {
    type TypeId = u32;

    fn storage_info(
        &self,
        pallet_name: &str,
        storage_entry: &str,
    ) -> Result<StorageInfo<'_, Self::TypeId>, StorageInfoError<'_>> {
        self.inner.storage_info(pallet_name, storage_entry)
    }

    fn storage_entries(&self) -> impl Iterator<Item = StorageEntry<'_>> {
        self.inner.storage_entries()
    }
}

impl frame_decode::runtime_apis::RuntimeApiTypeInfo for Metadata {
    type TypeId = u32;

    fn runtime_api_info(
        &self,
        trait_name: &str,
        method_name: &str,
    ) -> Result<RuntimeApiInfo<'_, Self::TypeId>, RuntimeApiInfoError<'_>> {
        self.inner.runtime_api_info(trait_name, method_name)
    }

    fn runtime_apis(&self) -> impl Iterator<Item = RuntimeApi<'_>> {
        self.inner.runtime_apis()
    }
}

impl frame_decode::extrinsics::ExtrinsicTypeInfo for Metadata {
    type TypeId = u32;

    fn extrinsic_call_info(
        &self,
        pallet_index: u8,
        call_index: u8,
    ) -> Result<ExtrinsicCallInfo<'_, Self::TypeId>, ExtrinsicInfoError<'_>> {
        self.inner.extrinsic_call_info(pallet_index, call_index)
    }

    fn extrinsic_signature_info(
        &self,
    ) -> Result<ExtrinsicSignatureInfo<Self::TypeId>, ExtrinsicInfoError<'_>> {
        self.inner.extrinsic_signature_info()
    }

    fn extrinsic_extension_info(
        &self,
        extension_version: Option<u8>,
    ) -> Result<ExtrinsicExtensionInfo<'_, Self::TypeId>, ExtrinsicInfoError<'_>> {
        self.inner.extrinsic_extension_info(extension_version)
    }
}

impl frame_decode::view_functions::ViewFunctionTypeInfo for Metadata {
    type TypeId = u32;

    fn view_function_info(
        &self,
        pallet_name: &str,
        function_name: &str,
    ) -> Result<ViewFunctionInfo<'_, Self::TypeId>, ViewFunctionInfoError<'_>> {
        self.inner.view_function_info(pallet_name, function_name)
    }

    fn view_functions(&self) -> impl Iterator<Item = ViewFunction<'_>> {
        self.inner.view_functions()
    }
}

impl Metadata {
    /// Identical to `metadata.pallet_by_name()`, but returns an error if the pallet is not found.
    pub fn pallet_by_name_err(
        &self,
        name: &str,
    ) -> Result<subxt_metadata::PalletMetadata<'_>, MetadataError> {
        self.pallet_by_name(name)
            .ok_or_else(|| MetadataError::PalletNameNotFound(name.to_owned()))
    }

    /// Identical to `metadata.pallet_by_index()`, but returns an error if the pallet is not found.
    pub fn pallet_by_index_err(
        &self,
        index: u8,
    ) -> Result<subxt_metadata::PalletMetadata<'_>, MetadataError> {
        self.pallet_by_index(index)
            .ok_or(MetadataError::PalletIndexNotFound(index))
    }

    /// Identical to `metadata.runtime_api_trait_by_name()`, but returns an error if the trait is not found.
    pub fn runtime_api_trait_by_name_err(
        &self,
        name: &str,
    ) -> Result<subxt_metadata::RuntimeApiMetadata<'_>, MetadataError> {
        self.runtime_api_trait_by_name(name)
            .ok_or_else(|| MetadataError::RuntimeTraitNotFound(name.to_owned()))
    }

    /// Identical to `metadata.custom().get(name)`, but returns an error if the trait is not found.
    pub fn custom_value_by_name_err(
        &self,
        name: &str,
    ) -> Result<subxt_metadata::CustomValueMetadata<'_>, MetadataError> {
        self.custom()
            .get(name)
            .ok_or_else(|| MetadataError::CustomValueNameNotFound(name.to_owned()))
    }
}

impl From<subxt_metadata::Metadata> for Metadata {
    fn from(md: subxt_metadata::Metadata) -> Self {
        Metadata {
            inner: Arc::new(md),
        }
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
        subxt_metadata::Metadata::decode(input).map(Metadata::from)
    }
}
