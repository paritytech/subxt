// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A representation of the metadata provided by a substrate based node.
//! This representation is optimized to be used by Subxt and related crates,
//! and is independent of the different versions of metadata that can be
//! provided from a node.
//!
//! Typically, this will be constructed by either:
//!
//! 1. Calling `Metadata::decode()` given some metadata bytes obtained
//!    from a node (this uses [`codec::Decode`]).
//! 2. Obtaining [`frame_metadata::RuntimeMetadataPrefixed`], and then
//!    using `.try_into()` to convert it into [`Metadata`].

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

extern crate alloc;

mod from;
mod utils;

use alloc::borrow::Cow;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use frame_decode::constants::{ConstantEntry, ConstantInfo, ConstantInfoError};
use frame_decode::custom_values::{CustomValue, CustomValueInfo, CustomValueInfoError};
use frame_decode::extrinsics::{
    ExtrinsicCallInfo, ExtrinsicCallInfoArg, ExtrinsicExtensionInfo, ExtrinsicExtensionInfoArg,
    ExtrinsicInfoError, ExtrinsicSignatureInfo,
};
use frame_decode::runtime_apis::{
    RuntimeApiEntry, RuntimeApiInfo, RuntimeApiInfoError, RuntimeApiInput,
};
use frame_decode::storage::{StorageEntry, StorageInfo, StorageInfoError, StorageKeyInfo};
use frame_decode::view_functions::{
    ViewFunctionEntry, ViewFunctionInfo, ViewFunctionInfoError, ViewFunctionInput,
};
use hashbrown::HashMap;
use scale_info::{PortableRegistry, Variant, form::PortableForm};
use utils::{
    ordered_map::OrderedMap,
    validation::{HASH_LEN, get_custom_value_hash},
    variant_index::VariantIndex,
};

pub use frame_decode::storage::StorageHasher;
pub use from::SUPPORTED_METADATA_VERSIONS;
pub use from::TryFromError;
pub use utils::validation::MetadataHasher;

#[cfg(feature = "legacy")]
pub use from::legacy::Error as LegacyFromError;

type CustomMetadataInner = frame_metadata::v15::CustomMetadata<PortableForm>;

/// Metadata is often passed around wrapped in an [`Arc`] so that it can be cloned.
pub type ArcMetadata = Arc<Metadata>;

/// Node metadata. This can be constructed by providing some compatible [`frame_metadata`]
/// which is then decoded into this. We aim to preserve all of the existing information in
/// the incoming metadata while optimizing the format a little for Subxt's use cases.
#[derive(Debug)]
pub struct Metadata {
    /// Type registry containing all types used in the metadata.
    types: PortableRegistry,
    /// Metadata of all the pallets.
    pallets: OrderedMap<String, PalletMetadataInner>,
    /// Find the pallet for a given call index.
    pallets_by_call_index: HashMap<u8, usize>,
    /// Find the pallet for a given event index.
    ///
    /// for modern metadatas, this is the same as pallets_by_call_index,
    /// but for old metadatas this can vary.
    pallets_by_event_index: HashMap<u8, usize>,
    /// Find the pallet for a given error index.
    ///
    /// for modern metadatas, this is the same as pallets_by_call_index,
    /// but for old metadatas this can vary.
    pallets_by_error_index: HashMap<u8, usize>,
    /// Metadata of the extrinsic.
    extrinsic: ExtrinsicMetadata,
    /// The types of the outer enums.
    outer_enums: OuterEnumsMetadata,
    /// The type Id of the `DispatchError` type, which Subxt makes use of.
    dispatch_error_ty: Option<u32>,
    /// Details about each of the runtime API traits.
    apis: OrderedMap<String, RuntimeApiMetadataInner>,
    /// Allows users to add custom types to the metadata. A map that associates a string key to a `CustomValueMetadata`.
    custom: CustomMetadataInner,
}

// Since we've abstracted away from frame-metadatas, we impl this on our custom Metadata
// so that it can be used by `frame-decode` to obtain the relevant extrinsic info.
impl frame_decode::extrinsics::ExtrinsicTypeInfo for Metadata {
    type TypeId = u32;

    fn extrinsic_call_info_by_index(
        &self,
        pallet_index: u8,
        call_index: u8,
    ) -> Result<ExtrinsicCallInfo<'_, Self::TypeId>, ExtrinsicInfoError<'_>> {
        let pallet = self.pallet_by_call_index(pallet_index).ok_or({
            ExtrinsicInfoError::PalletNotFound {
                index: pallet_index,
            }
        })?;

        let call = pallet.call_variant_by_index(call_index).ok_or_else(|| {
            ExtrinsicInfoError::CallNotFound {
                index: call_index,
                pallet_index,
                pallet_name: Cow::Borrowed(pallet.name()),
            }
        })?;

        Ok(ExtrinsicCallInfo {
            call_index,
            pallet_index,
            pallet_name: Cow::Borrowed(pallet.name()),
            call_name: Cow::Borrowed(&call.name),
            args: call
                .fields
                .iter()
                .map(|f| ExtrinsicCallInfoArg {
                    name: Cow::Borrowed(f.name.as_deref().unwrap_or("")),
                    id: f.ty.id,
                })
                .collect(),
        })
    }

    fn extrinsic_call_info_by_name(
        &self,
        pallet_name: &str,
        call_name: &str,
    ) -> Result<ExtrinsicCallInfo<'_, Self::TypeId>, ExtrinsicInfoError<'_>> {
        let pallet = self.pallet_by_name(pallet_name).ok_or({
            ExtrinsicInfoError::PalletNotFoundByName {
                name: Cow::Owned(pallet_name.to_string()),
            }
        })?;

        let call = pallet.call_variant_by_name(call_name).ok_or_else(|| {
            ExtrinsicInfoError::CallNotFoundByName {
                pallet_index: pallet.call_index(),
                pallet_name: Cow::Borrowed(pallet.name()),
                call_name: Cow::Owned(call_name.to_string()),
            }
        })?;

        Ok(ExtrinsicCallInfo {
            call_index: call.index,
            pallet_index: pallet.call_index(),
            pallet_name: Cow::Borrowed(pallet.name()),
            call_name: Cow::Borrowed(&call.name),
            args: call
                .fields
                .iter()
                .map(|f| ExtrinsicCallInfoArg {
                    name: Cow::Borrowed(f.name.as_deref().unwrap_or("")),
                    id: f.ty.id,
                })
                .collect(),
        })
    }

    fn extrinsic_signature_info(
        &self,
    ) -> Result<ExtrinsicSignatureInfo<Self::TypeId>, ExtrinsicInfoError<'_>> {
        Ok(ExtrinsicSignatureInfo {
            address_id: self.extrinsic().address_ty,
            signature_id: self.extrinsic().signature_ty,
        })
    }

    fn extrinsic_extension_version_info(
        &self,
    ) -> Result<impl Iterator<Item = u8>, ExtrinsicInfoError<'_>> {
        Ok(self
            .extrinsic
            .transaction_extensions_by_version
            .keys()
            .copied())
    }

    fn extrinsic_extension_info(
        &self,
        extension_version: Option<u8>,
    ) -> Result<ExtrinsicExtensionInfo<'_, Self::TypeId>, ExtrinsicInfoError<'_>> {
        let extension_version = extension_version.unwrap_or_else(|| {
            // We have some transaction, probably a V4 one with no extension version,
            // but our metadata may support multiple versions. Use the metadata to decide
            // what version to assume we'll decode it as.
            self.extrinsic()
                .transaction_extension_version_to_use_for_decoding()
        });

        let extension_ids = self
            .extrinsic()
            .transaction_extensions_by_version(extension_version)
            .ok_or(ExtrinsicInfoError::ExtrinsicExtensionVersionNotFound { extension_version })?
            .map(|f| ExtrinsicExtensionInfoArg {
                name: Cow::Borrowed(f.identifier()),
                id: f.extra_ty(),
                implicit_id: f.additional_ty(),
            })
            .collect();

        Ok(ExtrinsicExtensionInfo { extension_ids })
    }
}
impl frame_decode::storage::StorageTypeInfo for Metadata {
    type TypeId = u32;

    fn storage_info(
        &self,
        pallet_name: &str,
        storage_entry: &str,
    ) -> Result<StorageInfo<'_, Self::TypeId>, StorageInfoError<'_>> {
        let pallet =
            self.pallet_by_name(pallet_name)
                .ok_or_else(|| StorageInfoError::PalletNotFound {
                    pallet_name: pallet_name.to_string(),
                })?;
        let entry = pallet
            .storage()
            .and_then(|storage| storage.entry_by_name(storage_entry))
            .ok_or_else(|| StorageInfoError::StorageNotFound {
                name: storage_entry.to_string(),
                pallet_name: Cow::Borrowed(pallet.name()),
            })?;

        let info = StorageInfo {
            keys: Cow::Borrowed(&*entry.info.keys),
            value_id: entry.info.value_id,
            default_value: entry
                .info
                .default_value
                .as_ref()
                .map(|def| Cow::Borrowed(&**def)),
            use_old_v9_storage_hashers: false,
        };

        Ok(info)
    }
}
impl frame_decode::storage::StorageEntryInfo for Metadata {
    fn storage_entries(&self) -> impl Iterator<Item = StorageEntry<'_>> {
        self.pallets().flat_map(|pallet| {
            let pallet_name = pallet.name();
            let pallet_iter = core::iter::once(StorageEntry::In(pallet_name.into()));
            let entries_iter = pallet.storage().into_iter().flat_map(|storage| {
                storage
                    .entries()
                    .iter()
                    .map(|entry| StorageEntry::Name(entry.name().into()))
            });

            pallet_iter.chain(entries_iter)
        })
    }
}
impl frame_decode::runtime_apis::RuntimeApiTypeInfo for Metadata {
    type TypeId = u32;

    fn runtime_api_info(
        &self,
        trait_name: &str,
        method_name: &str,
    ) -> Result<RuntimeApiInfo<'_, Self::TypeId>, RuntimeApiInfoError<'_>> {
        let api_trait =
            self.apis
                .get_by_key(trait_name)
                .ok_or_else(|| RuntimeApiInfoError::TraitNotFound {
                    trait_name: trait_name.to_string(),
                })?;
        let api_method = api_trait.methods.get_by_key(method_name).ok_or_else(|| {
            RuntimeApiInfoError::MethodNotFound {
                trait_name: Cow::Borrowed(&api_trait.name),
                method_name: method_name.to_string(),
            }
        })?;

        let info = RuntimeApiInfo {
            inputs: Cow::Borrowed(&api_method.info.inputs),
            output_id: api_method.info.output_id,
        };

        Ok(info)
    }
}
impl frame_decode::runtime_apis::RuntimeApiEntryInfo for Metadata {
    fn runtime_api_entries(&self) -> impl Iterator<Item = RuntimeApiEntry<'_>> {
        self.runtime_api_traits().flat_map(|api_trait| {
            let trait_name = api_trait.name();
            let trait_iter = core::iter::once(RuntimeApiEntry::In(trait_name.into()));
            let method_iter = api_trait
                .methods()
                .map(|method| RuntimeApiEntry::Name(method.name().into()));

            trait_iter.chain(method_iter)
        })
    }
}
impl frame_decode::view_functions::ViewFunctionTypeInfo for Metadata {
    type TypeId = u32;

    fn view_function_info(
        &self,
        pallet_name: &str,
        function_name: &str,
    ) -> Result<ViewFunctionInfo<'_, Self::TypeId>, ViewFunctionInfoError<'_>> {
        let pallet = self.pallet_by_name(pallet_name).ok_or_else(|| {
            ViewFunctionInfoError::PalletNotFound {
                pallet_name: pallet_name.to_string(),
            }
        })?;
        let function = pallet.view_function_by_name(function_name).ok_or_else(|| {
            ViewFunctionInfoError::FunctionNotFound {
                pallet_name: Cow::Borrowed(pallet.name()),
                function_name: function_name.to_string(),
            }
        })?;

        let info = ViewFunctionInfo {
            inputs: Cow::Borrowed(&function.inner.info.inputs),
            output_id: function.inner.info.output_id,
            query_id: *function.query_id(),
        };

        Ok(info)
    }
}
impl frame_decode::view_functions::ViewFunctionEntryInfo for Metadata {
    fn view_function_entries(&self) -> impl Iterator<Item = ViewFunctionEntry<'_>> {
        self.pallets().flat_map(|pallet| {
            let pallet_name = pallet.name();
            let pallet_iter = core::iter::once(ViewFunctionEntry::In(pallet_name.into()));
            let fn_iter = pallet
                .view_functions()
                .map(|function| ViewFunctionEntry::Name(function.name().into()));

            pallet_iter.chain(fn_iter)
        })
    }
}
impl frame_decode::constants::ConstantTypeInfo for Metadata {
    type TypeId = u32;

    fn constant_info(
        &self,
        pallet_name: &str,
        constant_name: &str,
    ) -> Result<ConstantInfo<'_, Self::TypeId>, ConstantInfoError<'_>> {
        let pallet =
            self.pallet_by_name(pallet_name)
                .ok_or_else(|| ConstantInfoError::PalletNotFound {
                    pallet_name: pallet_name.to_string(),
                })?;
        let constant = pallet.constant_by_name(constant_name).ok_or_else(|| {
            ConstantInfoError::ConstantNotFound {
                pallet_name: Cow::Borrowed(pallet.name()),
                constant_name: constant_name.to_string(),
            }
        })?;

        let info = ConstantInfo {
            bytes: &constant.value,
            type_id: constant.ty,
        };

        Ok(info)
    }
}
impl frame_decode::constants::ConstantEntryInfo for Metadata {
    fn constant_entries(&self) -> impl Iterator<Item = ConstantEntry<'_>> {
        self.pallets().flat_map(|pallet| {
            let pallet_name = pallet.name();
            let pallet_iter = core::iter::once(ConstantEntry::In(pallet_name.into()));
            let constant_iter = pallet
                .constants()
                .map(|constant| ConstantEntry::Name(constant.name().into()));

            pallet_iter.chain(constant_iter)
        })
    }
}
impl frame_decode::custom_values::CustomValueTypeInfo for Metadata {
    type TypeId = u32;

    fn custom_value_info(
        &self,
        name: &str,
    ) -> Result<CustomValueInfo<'_, Self::TypeId>, CustomValueInfoError> {
        let custom_value = self
            .custom()
            .get(name)
            .ok_or_else(|| CustomValueInfoError {
                not_found: name.to_string(),
            })?;

        let info = CustomValueInfo {
            bytes: custom_value.data,
            type_id: custom_value.type_id,
        };

        Ok(info)
    }
}
impl frame_decode::custom_values::CustomValueEntryInfo for Metadata {
    fn custom_values(&self) -> impl Iterator<Item = CustomValue<'_>> {
        self.custom.map.keys().map(|name| CustomValue {
            name: Cow::Borrowed(name),
        })
    }
}

impl Metadata {
    /// Metadata tends to be passed around wrapped in an [`Arc`] so that it can be
    /// cheaply cloned. This is a shorthand to return that.
    pub fn arc(self) -> ArcMetadata {
        Arc::new(self)
    }

    /// This is similar to`<Metadata as codec::Decode>::decode(&mut bytes)`, except it
    /// is able to attempt to decode from several types.
    ///
    /// - The default assumption is that metadata is encoded as [`frame_metadata::RuntimeMetadataPrefixed`]. This is the
    ///   expected format that metadata is encoded into, and what the [`codec::Decode`] impl tries.
    /// - if this fails, we also try to decode as [`frame_metadata::RuntimeMetadata`].
    /// - If this all fails, we finally will try to decode as [`frame_metadata::OpaqueMetadata`].
    pub fn decode_from(bytes: &[u8]) -> Result<Self, codec::Error> {
        let metadata = decode_runtime_metadata(bytes)?;
        from_runtime_metadata(metadata)
    }

    /// Convert V16 metadata into [`Metadata`].
    pub fn from_v16(
        metadata: frame_metadata::v16::RuntimeMetadataV16,
    ) -> Result<Self, TryFromError> {
        metadata.try_into()
    }

    /// Convert V15 metadata into [`Metadata`].
    pub fn from_v15(
        metadata: frame_metadata::v15::RuntimeMetadataV15,
    ) -> Result<Self, TryFromError> {
        metadata.try_into()
    }

    /// Convert V14 metadata into [`Metadata`].
    pub fn from_v14(
        metadata: frame_metadata::v14::RuntimeMetadataV14,
    ) -> Result<Self, TryFromError> {
        metadata.try_into()
    }

    /// Convert V13 metadata into [`Metadata`], given the necessary extra type information.
    #[cfg(feature = "legacy")]
    pub fn from_v13(
        metadata: &frame_metadata::v13::RuntimeMetadataV13,
        types: &scale_info_legacy::TypeRegistrySet<'_>,
    ) -> Result<Self, LegacyFromError> {
        from::legacy::from_v13(metadata, types, from::legacy::Opts::compat())
    }

    /// Convert V12 metadata into [`Metadata`], given the necessary extra type information.
    #[cfg(feature = "legacy")]
    pub fn from_v12(
        metadata: &frame_metadata::v12::RuntimeMetadataV12,
        types: &scale_info_legacy::TypeRegistrySet<'_>,
    ) -> Result<Self, LegacyFromError> {
        from::legacy::from_v12(metadata, types, from::legacy::Opts::compat())
    }

    /// Convert V13 metadata into [`Metadata`], given the necessary extra type information.
    #[cfg(feature = "legacy")]
    pub fn from_v11(
        metadata: &frame_metadata::v11::RuntimeMetadataV11,
        types: &scale_info_legacy::TypeRegistrySet<'_>,
    ) -> Result<Self, LegacyFromError> {
        from::legacy::from_v11(metadata, types, from::legacy::Opts::compat())
    }

    /// Convert V13 metadata into [`Metadata`], given the necessary extra type information.
    #[cfg(feature = "legacy")]
    pub fn from_v10(
        metadata: &frame_metadata::v10::RuntimeMetadataV10,
        types: &scale_info_legacy::TypeRegistrySet<'_>,
    ) -> Result<Self, LegacyFromError> {
        from::legacy::from_v10(metadata, types, from::legacy::Opts::compat())
    }

    /// Convert V9 metadata into [`Metadata`], given the necessary extra type information.
    #[cfg(feature = "legacy")]
    pub fn from_v9(
        metadata: &frame_metadata::v9::RuntimeMetadataV9,
        types: &scale_info_legacy::TypeRegistrySet<'_>,
    ) -> Result<Self, LegacyFromError> {
        from::legacy::from_v9(metadata, types, from::legacy::Opts::compat())
    }

    /// Convert V8 metadata into [`Metadata`], given the necessary extra type information.
    #[cfg(feature = "legacy")]
    pub fn from_v8(
        metadata: &frame_metadata::v8::RuntimeMetadataV8,
        types: &scale_info_legacy::TypeRegistrySet<'_>,
    ) -> Result<Self, LegacyFromError> {
        from::legacy::from_v8(metadata, types, from::legacy::Opts::compat())
    }

    /// Access the underlying type registry.
    pub fn types(&self) -> &PortableRegistry {
        &self.types
    }

    /// Mutable access to the underlying type registry.
    pub fn types_mut(&mut self) -> &mut PortableRegistry {
        &mut self.types
    }

    /// The type ID of the `DispatchError` type, if it exists.
    pub fn dispatch_error_ty(&self) -> Option<u32> {
        self.dispatch_error_ty
    }

    /// Return details about the extrinsic format.
    pub fn extrinsic(&self) -> &ExtrinsicMetadata {
        &self.extrinsic
    }

    /// Return details about the outer enums.
    pub fn outer_enums(&self) -> OuterEnumsMetadata {
        self.outer_enums
    }

    /// An iterator over all of the available pallets.
    pub fn pallets(&self) -> impl ExactSizeIterator<Item = PalletMetadata<'_>> {
        self.pallets.values().iter().map(|inner| PalletMetadata {
            inner,
            types: self.types(),
        })
    }

    /// Access a pallet given some call/extrinsic pallet index byte
    pub fn pallet_by_call_index(&self, variant_index: u8) -> Option<PalletMetadata<'_>> {
        let inner = self
            .pallets_by_call_index
            .get(&variant_index)
            .and_then(|i| self.pallets.get_by_index(*i))?;

        Some(PalletMetadata {
            inner,
            types: self.types(),
        })
    }

    /// Access a pallet given some event pallet index byte
    pub fn pallet_by_event_index(&self, variant_index: u8) -> Option<PalletMetadata<'_>> {
        let inner = self
            .pallets_by_event_index
            .get(&variant_index)
            .and_then(|i| self.pallets.get_by_index(*i))?;

        Some(PalletMetadata {
            inner,
            types: self.types(),
        })
    }

    /// Access a pallet given some error pallet index byte
    pub fn pallet_by_error_index(&self, variant_index: u8) -> Option<PalletMetadata<'_>> {
        let inner = self
            .pallets_by_error_index
            .get(&variant_index)
            .and_then(|i| self.pallets.get_by_index(*i))?;

        Some(PalletMetadata {
            inner,
            types: self.types(),
        })
    }

    /// Access a pallet given its name.
    pub fn pallet_by_name(&self, pallet_name: &str) -> Option<PalletMetadata<'_>> {
        let inner = self.pallets.get_by_key(pallet_name)?;

        Some(PalletMetadata {
            inner,
            types: self.types(),
        })
    }

    /// An iterator over all of the runtime APIs.
    pub fn runtime_api_traits(&self) -> impl ExactSizeIterator<Item = RuntimeApiMetadata<'_>> {
        self.apis.values().iter().map(|inner| RuntimeApiMetadata {
            inner,
            types: self.types(),
        })
    }

    /// Access a runtime API trait given its name.
    pub fn runtime_api_trait_by_name(&'_ self, name: &str) -> Option<RuntimeApiMetadata<'_>> {
        let inner = self.apis.get_by_key(name)?;
        Some(RuntimeApiMetadata {
            inner,
            types: self.types(),
        })
    }

    /// Returns custom user defined types
    pub fn custom(&self) -> CustomMetadata<'_> {
        CustomMetadata {
            types: self.types(),
            inner: &self.custom,
        }
    }

    /// Obtain a unique hash representing this metadata or specific parts of it.
    pub fn hasher(&self) -> MetadataHasher<'_> {
        MetadataHasher::new(self)
    }

    /// Get type hash for a type in the registry
    pub fn type_hash(&self, id: u32) -> Option<[u8; HASH_LEN]> {
        self.types.resolve(id)?;
        Some(crate::utils::validation::get_type_hash(&self.types, id))
    }
}

/// Metadata for a specific pallet.
#[derive(Debug, Clone, Copy)]
pub struct PalletMetadata<'a> {
    inner: &'a PalletMetadataInner,
    types: &'a PortableRegistry,
}

impl<'a> PalletMetadata<'a> {
    /// The pallet name.
    pub fn name(&self) -> &'a str {
        &self.inner.name
    }

    /// The index to use for calls in this pallet.
    pub fn call_index(&self) -> u8 {
        self.inner.call_index
    }

    /// The index to use for events in this pallet.
    pub fn event_index(&self) -> u8 {
        self.inner.event_index
    }

    /// The index to use for errors in this pallet.
    pub fn error_index(&self) -> u8 {
        self.inner.error_index
    }

    /// The pallet docs.
    pub fn docs(&self) -> &'a [String] {
        &self.inner.docs
    }

    /// Type ID for the pallet's Call type, if it exists.
    pub fn call_ty_id(&self) -> Option<u32> {
        self.inner.call_ty
    }

    /// Type ID for the pallet's Event type, if it exists.
    pub fn event_ty_id(&self) -> Option<u32> {
        self.inner.event_ty
    }

    /// Type ID for the pallet's Error type, if it exists.
    pub fn error_ty_id(&self) -> Option<u32> {
        self.inner.error_ty
    }

    /// Return metadata about the pallet's storage entries.
    pub fn storage(&self) -> Option<&'a StorageMetadata> {
        self.inner.storage.as_ref()
    }

    /// Return all of the event variants, if an event type exists.
    pub fn event_variants(&self) -> Option<&'a [Variant<PortableForm>]> {
        VariantIndex::get(self.inner.event_ty, self.types)
    }

    /// Return an event variant given it's encoded variant index.
    pub fn event_variant_by_index(&self, variant_index: u8) -> Option<&'a Variant<PortableForm>> {
        self.inner.event_variant_index.lookup_by_index(
            variant_index,
            self.inner.event_ty,
            self.types,
        )
    }

    /// Does this pallet have any view functions?
    pub fn has_view_functions(&self) -> bool {
        !self.inner.view_functions.is_empty()
    }

    /// Return an iterator over the View Functions in this pallet, if any.
    pub fn view_functions(
        &self,
    ) -> impl ExactSizeIterator<Item = ViewFunctionMetadata<'a>> + use<'a> {
        self.inner
            .view_functions
            .values()
            .iter()
            .map(|vf: &'a _| ViewFunctionMetadata {
                inner: vf,
                types: self.types,
            })
    }

    /// Return the view function with a given name, if any
    pub fn view_function_by_name(&self, name: &str) -> Option<ViewFunctionMetadata<'a>> {
        self.inner
            .view_functions
            .get_by_key(name)
            .map(|vf: &'a _| ViewFunctionMetadata {
                inner: vf,
                types: self.types,
            })
    }

    /// Iterate (in no particular order) over the associated type names and type IDs for this pallet.
    pub fn associated_types(&self) -> impl ExactSizeIterator<Item = (&'a str, u32)> + use<'a> {
        self.inner
            .associated_types
            .iter()
            .map(|(name, ty)| (&**name, *ty))
    }

    /// Fetch an associated type ID given the associated type name.
    pub fn associated_type_id(&self, name: &str) -> Option<u32> {
        self.inner.associated_types.get(name).copied()
    }

    /// Return all of the call variants, if a call type exists.
    pub fn call_variants(&self) -> Option<&'a [Variant<PortableForm>]> {
        VariantIndex::get(self.inner.call_ty, self.types)
    }

    /// Return a call variant given it's encoded variant index.
    pub fn call_variant_by_index(&self, variant_index: u8) -> Option<&'a Variant<PortableForm>> {
        self.inner
            .call_variant_index
            .lookup_by_index(variant_index, self.inner.call_ty, self.types)
    }

    /// Return a call variant given it's name.
    pub fn call_variant_by_name(&self, call_name: &str) -> Option<&'a Variant<PortableForm>> {
        self.inner
            .call_variant_index
            .lookup_by_name(call_name, self.inner.call_ty, self.types)
    }

    /// Return all of the error variants, if an error type exists.
    pub fn error_variants(&self) -> Option<&'a [Variant<PortableForm>]> {
        VariantIndex::get(self.inner.error_ty, self.types)
    }

    /// Return an error variant given it's encoded variant index.
    pub fn error_variant_by_index(&self, variant_index: u8) -> Option<&'a Variant<PortableForm>> {
        self.inner.error_variant_index.lookup_by_index(
            variant_index,
            self.inner.error_ty,
            self.types,
        )
    }

    /// Return constant details given the constant name.
    pub fn constant_by_name(&self, name: &str) -> Option<&'a ConstantMetadata> {
        self.inner.constants.get_by_key(name)
    }

    /// An iterator over the constants in this pallet.
    pub fn constants(&self) -> impl ExactSizeIterator<Item = &'a ConstantMetadata> + use<'a> {
        self.inner.constants.values().iter()
    }

    /// Return a hash for the storage entry, or None if it was not found.
    pub fn storage_hash(&self, entry_name: &str) -> Option<[u8; HASH_LEN]> {
        crate::utils::validation::get_storage_hash(self, entry_name)
    }

    /// Return a hash for the constant, or None if it was not found.
    pub fn constant_hash(&self, constant_name: &str) -> Option<[u8; HASH_LEN]> {
        crate::utils::validation::get_constant_hash(self, constant_name)
    }

    /// Return a hash for the call, or None if it was not found.
    pub fn call_hash(&self, call_name: &str) -> Option<[u8; HASH_LEN]> {
        crate::utils::validation::get_call_hash(self, call_name)
    }

    /// Return a hash for the entire pallet.
    pub fn hash(&self) -> [u8; HASH_LEN] {
        crate::utils::validation::get_pallet_hash(*self)
    }
}

#[derive(Debug, Clone)]
struct PalletMetadataInner {
    /// Pallet name.
    name: String,
    /// The index for calls in the pallet.
    call_index: u8,
    /// The index for events in the pallet.
    ///
    /// This is the same as `call_index` for modern metadatas,
    /// but can be different for older metadatas (pre-V12).
    event_index: u8,
    /// The index for errors in the pallet.
    ///
    /// This is the same as `call_index` for modern metadatas,
    /// but can be different for older metadatas (pre-V12).
    error_index: u8,
    /// Pallet storage metadata.
    storage: Option<StorageMetadata>,
    /// Type ID for the pallet Call enum.
    call_ty: Option<u32>,
    /// Call variants by name/u8.
    call_variant_index: VariantIndex,
    /// Type ID for the pallet Event enum.
    event_ty: Option<u32>,
    /// Event variants by name/u8.
    event_variant_index: VariantIndex,
    /// Type ID for the pallet Error enum.
    error_ty: Option<u32>,
    /// Error variants by name/u8.
    error_variant_index: VariantIndex,
    /// Map from constant name to constant details.
    constants: OrderedMap<String, ConstantMetadata>,
    /// Details about each of the pallet view functions.
    view_functions: OrderedMap<String, ViewFunctionMetadataInner>,
    /// Mapping from associated type to type ID describing its shape.
    associated_types: BTreeMap<String, u32>,
    /// Pallet documentation.
    docs: Vec<String>,
}

/// Metadata for the storage entries in a pallet.
#[derive(Debug, Clone)]
pub struct StorageMetadata {
    /// The common prefix used by all storage entries.
    prefix: String,
    /// Map from storage entry name to details.
    entries: OrderedMap<String, StorageEntryMetadata>,
}

impl StorageMetadata {
    /// The common prefix used by all storage entries.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// An iterator over the storage entries.
    pub fn entries(&self) -> &[StorageEntryMetadata] {
        self.entries.values()
    }

    /// Return a specific storage entry given its name.
    pub fn entry_by_name(&self, name: &str) -> Option<&StorageEntryMetadata> {
        self.entries.get_by_key(name)
    }
}

/// Metadata for a single storage entry.
#[derive(Debug, Clone)]
pub struct StorageEntryMetadata {
    /// Variable name of the storage entry.
    name: String,
    /// Information about the storage entry.
    info: StorageInfo<'static, u32>,
    /// Storage entry documentation.
    docs: Vec<String>,
}

impl StorageEntryMetadata {
    /// Name of this entry.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Keys in this storage entry.
    pub fn keys(&self) -> impl ExactSizeIterator<Item = &StorageKeyInfo<u32>> {
        let keys = &*self.info.keys;
        keys.iter()
    }
    /// Value type for this storage entry.
    pub fn value_ty(&self) -> u32 {
        self.info.value_id
    }
    /// The default value, if one exists, for this entry.
    pub fn default_value(&self) -> Option<&[u8]> {
        self.info.default_value.as_deref()
    }
    /// Storage entry documentation.
    pub fn docs(&self) -> &[String] {
        &self.docs
    }
}

/// Metadata for a single constant.
#[derive(Debug, Clone)]
pub struct ConstantMetadata {
    /// Name of the pallet constant.
    name: String,
    /// Type of the pallet constant.
    ty: u32,
    /// Value stored in the constant (SCALE encoded).
    value: Vec<u8>,
    /// Constant documentation.
    docs: Vec<String>,
}

impl ConstantMetadata {
    /// Name of the pallet constant.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Type of the pallet constant.
    pub fn ty(&self) -> u32 {
        self.ty
    }
    /// Value stored in the constant (SCALE encoded).
    pub fn value(&self) -> &[u8] {
        &self.value
    }
    /// Constant documentation.
    pub fn docs(&self) -> &[String] {
        &self.docs
    }
}

/// Metadata for the extrinsic type.
#[derive(Debug, Clone)]
pub struct ExtrinsicMetadata {
    /// The type of the address that signs the extrinsic.
    /// Used to help decode tx signatures.
    address_ty: u32,
    /// The type of the extrinsic's signature.
    /// Used to help decode tx signatures.
    signature_ty: u32,
    /// Which extrinsic versions are supported by this chain.
    supported_versions: Vec<u8>,
    /// The signed extensions in the order they appear in the extrinsic.
    transaction_extensions: Vec<TransactionExtensionMetadataInner>,
    /// Different versions of transaction extensions can exist. Each version
    /// is a u8 which corresponds to the indexes of the transaction extensions
    /// seen in the above Vec, in order, that exist at that version.
    transaction_extensions_by_version: BTreeMap<u8, Vec<u32>>,
}

impl ExtrinsicMetadata {
    /// Which extrinsic versions are supported.
    pub fn supported_versions(&self) -> &[u8] {
        &self.supported_versions
    }

    /// The extra/additional information associated with the extrinsic.
    pub fn transaction_extensions_by_version(
        &self,
        version: u8,
    ) -> Option<impl Iterator<Item = TransactionExtensionMetadata<'_>>> {
        let extension_indexes = self.transaction_extensions_by_version.get(&version)?;
        let iter = extension_indexes.iter().map(|index| {
            let tx_metadata = self
                .transaction_extensions
                .get(*index as usize)
                .expect("transaction extension should exist if index is in transaction_extensions_by_version");

            TransactionExtensionMetadata {
                identifier: &tx_metadata.identifier,
                extra_ty: tx_metadata.extra_ty,
                additional_ty: tx_metadata.additional_ty,
            }
        });

        Some(iter)
    }

    /// When constructing a v5 extrinsic, use this transaction extensions version.
    pub fn transaction_extension_version_to_use_for_encoding(&self) -> u8 {
        *self
            .transaction_extensions_by_version
            .keys()
            .max()
            .expect("At least one version of transaction extensions is expected")
    }

    /// An iterator of the transaction extensions to use when encoding a transaction. Basically equivalent to
    /// `self.transaction_extensions_by_version(self.transaction_extension_version_to_use_for_encoding()).unwrap()`
    pub fn transaction_extensions_to_use_for_encoding(
        &self,
    ) -> impl Iterator<Item = TransactionExtensionMetadata<'_>> {
        let encoding_version = self.transaction_extension_version_to_use_for_encoding();
        self.transaction_extensions_by_version(encoding_version)
            .unwrap()
    }

    /// When presented with a v4 extrinsic that has no version, treat it as being this version.
    pub fn transaction_extension_version_to_use_for_decoding(&self) -> u8 {
        *self
            .transaction_extensions_by_version
            .keys()
            .max()
            .expect("At least one version of transaction extensions is expected")
    }
}

/// Metadata for the signed extensions used by extrinsics.
#[derive(Debug, Clone)]
pub struct TransactionExtensionMetadata<'a> {
    /// The unique transaction extension identifier, which may be different from the type name.
    identifier: &'a str,
    /// The type of the transaction extension, with the data to be included in the extrinsic.
    extra_ty: u32,
    /// The type of the additional signed data, with the data to be included in the signed payload.
    additional_ty: u32,
}

#[derive(Debug, Clone)]
struct TransactionExtensionMetadataInner {
    identifier: String,
    extra_ty: u32,
    additional_ty: u32,
}

impl<'a> TransactionExtensionMetadata<'a> {
    /// The unique signed extension identifier, which may be different from the type name.
    pub fn identifier(&self) -> &'a str {
        self.identifier
    }
    /// The type of the signed extension, with the data to be included in the extrinsic.
    pub fn extra_ty(&self) -> u32 {
        self.extra_ty
    }
    /// The type of the additional signed data, with the data to be included in the signed payload
    pub fn additional_ty(&self) -> u32 {
        self.additional_ty
    }
}

/// Metadata for the outer enums.
#[derive(Debug, Clone, Copy)]
pub struct OuterEnumsMetadata {
    /// The type of the outer call enum.
    call_enum_ty: u32,
    /// The type of the outer event enum.
    event_enum_ty: u32,
    /// The type of the outer error enum.
    error_enum_ty: u32,
}

impl OuterEnumsMetadata {
    /// The type of the outer call enum.
    pub fn call_enum_ty(&self) -> u32 {
        self.call_enum_ty
    }

    /// The type of the outer event enum.
    pub fn event_enum_ty(&self) -> u32 {
        self.event_enum_ty
    }

    /// The type of the outer error enum.
    pub fn error_enum_ty(&self) -> u32 {
        self.error_enum_ty
    }
}

/// Metadata for the available runtime APIs.
#[derive(Debug, Clone, Copy)]
pub struct RuntimeApiMetadata<'a> {
    inner: &'a RuntimeApiMetadataInner,
    types: &'a PortableRegistry,
}

impl<'a> RuntimeApiMetadata<'a> {
    /// Trait name.
    pub fn name(&self) -> &'a str {
        &self.inner.name
    }
    /// Trait documentation.
    pub fn docs(&self) -> &[String] {
        &self.inner.docs
    }
    /// An iterator over the trait methods.
    pub fn methods(&self) -> impl ExactSizeIterator<Item = RuntimeApiMethodMetadata<'a>> + use<'a> {
        self.inner
            .methods
            .values()
            .iter()
            .map(|item| RuntimeApiMethodMetadata {
                trait_name: &self.inner.name,
                inner: item,
                types: self.types,
            })
    }
    /// Get a specific trait method given its name.
    pub fn method_by_name(&self, name: &str) -> Option<RuntimeApiMethodMetadata<'a>> {
        self.inner
            .methods
            .get_by_key(name)
            .map(|item| RuntimeApiMethodMetadata {
                trait_name: &self.inner.name,
                inner: item,
                types: self.types,
            })
    }
    /// Return a hash for the runtime API trait.
    pub fn hash(&self) -> [u8; HASH_LEN] {
        crate::utils::validation::get_runtime_apis_hash(*self)
    }
}

#[derive(Debug, Clone)]
struct RuntimeApiMetadataInner {
    /// Trait name.
    name: String,
    /// Trait methods.
    methods: OrderedMap<String, RuntimeApiMethodMetadataInner>,
    /// Trait documentation.
    docs: Vec<String>,
}

/// Metadata for a single runtime API method.
#[derive(Debug, Clone)]
pub struct RuntimeApiMethodMetadata<'a> {
    trait_name: &'a str,
    inner: &'a RuntimeApiMethodMetadataInner,
    types: &'a PortableRegistry,
}

impl<'a> RuntimeApiMethodMetadata<'a> {
    /// Method name.
    pub fn name(&self) -> &'a str {
        &self.inner.name
    }
    /// Method documentation.
    pub fn docs(&self) -> &[String] {
        &self.inner.docs
    }
    /// Method inputs.
    pub fn inputs(
        &self,
    ) -> impl ExactSizeIterator<Item = &'a RuntimeApiInput<'static, u32>> + use<'a> {
        let inputs = &*self.inner.info.inputs;
        inputs.iter()
    }
    /// Method return type.
    pub fn output_ty(&self) -> u32 {
        self.inner.info.output_id
    }
    /// Return a hash for the method.
    pub fn hash(&self) -> [u8; HASH_LEN] {
        crate::utils::validation::get_runtime_api_hash(self)
    }
}

#[derive(Debug, Clone)]
struct RuntimeApiMethodMetadataInner {
    /// Method name.
    name: String,
    /// Info.
    info: RuntimeApiInfo<'static, u32>,
    /// Method documentation.
    docs: Vec<String>,
}

/// Metadata for the available View Functions. Currently these exist only
/// at the pallet level, but eventually they could exist at the runtime level too.
#[derive(Debug, Clone, Copy)]
pub struct ViewFunctionMetadata<'a> {
    inner: &'a ViewFunctionMetadataInner,
    types: &'a PortableRegistry,
}

impl<'a> ViewFunctionMetadata<'a> {
    /// Method name.
    pub fn name(&self) -> &'a str {
        &self.inner.name
    }
    /// Query ID. This is used to query the function. Roughly, it is constructed by doing
    /// `twox_128(pallet_name) ++ twox_128("fn_name(fnarg_types) -> return_ty")` .
    pub fn query_id(&self) -> &'a [u8; 32] {
        &self.inner.info.query_id
    }
    /// Method documentation.
    pub fn docs(&self) -> &'a [String] {
        &self.inner.docs
    }
    /// Method inputs.
    pub fn inputs(
        &self,
    ) -> impl ExactSizeIterator<Item = &'a ViewFunctionInput<'static, u32>> + use<'a> {
        let inputs = &*self.inner.info.inputs;
        inputs.iter()
    }
    /// Method return type.
    pub fn output_ty(&self) -> u32 {
        self.inner.info.output_id
    }
    /// Return a hash for the method. The query ID of a view function validates it to some
    /// degree, but only takes type _names_ into account. This hash takes into account the
    /// actual _shape_ of each argument and the return type.
    pub fn hash(&self) -> [u8; HASH_LEN] {
        crate::utils::validation::get_view_function_hash(self)
    }
}

#[derive(Debug, Clone)]
struct ViewFunctionMetadataInner {
    /// View function name.
    name: String,
    /// Info.
    info: ViewFunctionInfo<'static, u32>,
    /// Documentation.
    docs: Vec<String>,
}

/// Metadata for a single input parameter to a runtime API method / pallet view function.
#[derive(Debug, Clone)]
pub struct MethodParamMetadata {
    /// Parameter name.
    pub name: String,
    /// Parameter type.
    pub ty: u32,
}

/// Metadata of custom types with custom values, basically the same as `frame_metadata::v15::CustomMetadata<PortableForm>>`.
#[derive(Debug, Clone)]
pub struct CustomMetadata<'a> {
    types: &'a PortableRegistry,
    inner: &'a CustomMetadataInner,
}

impl<'a> CustomMetadata<'a> {
    /// Get a certain [CustomValueMetadata] by its name.
    pub fn get(&self, name: &str) -> Option<CustomValueMetadata<'a>> {
        self.inner
            .map
            .get_key_value(name)
            .map(|(name, e)| CustomValueMetadata {
                types: self.types,
                type_id: e.ty.id,
                data: &e.value,
                name,
            })
    }

    /// Iterates over names (keys) and associated custom values
    pub fn iter(&self) -> impl Iterator<Item = CustomValueMetadata<'a>> + use<'a> {
        self.inner.map.iter().map(|(name, e)| CustomValueMetadata {
            types: self.types,
            type_id: e.ty.id,
            data: &e.value,
            name: name.as_ref(),
        })
    }

    /// Access the underlying type registry.
    pub fn types(&self) -> &PortableRegistry {
        self.types
    }
}

/// Basically the same as `frame_metadata::v15::CustomValueMetadata<PortableForm>>`, but borrowed.
pub struct CustomValueMetadata<'a> {
    types: &'a PortableRegistry,
    type_id: u32,
    data: &'a [u8],
    name: &'a str,
}

impl<'a> CustomValueMetadata<'a> {
    /// Access the underlying type registry.
    pub fn types(&self) -> &PortableRegistry {
        self.types
    }

    /// The scale encoded value
    pub fn bytes(&self) -> &'a [u8] {
        self.data
    }

    /// The type id in the TypeRegistry
    pub fn type_id(&self) -> u32 {
        self.type_id
    }

    /// The name under which the custom value is registered.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Calculates the hash for the CustomValueMetadata.
    pub fn hash(&self) -> [u8; HASH_LEN] {
        get_custom_value_hash(self)
    }
}

// Support decoding metadata from the "wire" format directly into this.
impl codec::Decode for Metadata {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let metadata = frame_metadata::RuntimeMetadataPrefixed::decode(input)?;
        from_runtime_metadata(metadata.1)
    }
}

/// A utility function to decode SCALE encoded metadata. This is much like [`Metadata::decode_from`] but doesn't
/// do the final step of converting the decoded metadata into [`Metadata`].
///
/// - The default assumption is that metadata is encoded as [`frame_metadata::RuntimeMetadataPrefixed`]. This is the
///   expected format that metadata is encoded into.
/// - if this fails, we also try to decode as [`frame_metadata::RuntimeMetadata`].
/// - If this all fails, we also try to decode as [`frame_metadata::OpaqueMetadata`].
pub fn decode_runtime_metadata(
    input: &[u8],
) -> Result<frame_metadata::RuntimeMetadata, codec::Error> {
    use codec::Decode;

    let err = match frame_metadata::RuntimeMetadataPrefixed::decode(&mut &*input) {
        Ok(md) => return Ok(md.1),
        Err(e) => e,
    };

    if let Ok(md) = frame_metadata::RuntimeMetadata::decode(&mut &*input) {
        return Ok(md);
    }

    // frame_metadata::OpaqueMetadata is a vec of bytes. If we can decode the length, AND
    // the length definitely corresponds to the number of remaining bytes, then we try to
    // decode the inner bytes.
    if let Ok(len) = codec::Compact::<u64>::decode(&mut &*input) {
        if input.len() == len.0 as usize {
            return decode_runtime_metadata(input);
        }
    }

    Err(err)
}

/// Convert RuntimeMetadata into Metadata if possible.
fn from_runtime_metadata(
    metadata: frame_metadata::RuntimeMetadata,
) -> Result<Metadata, codec::Error> {
    let metadata = match metadata {
        frame_metadata::RuntimeMetadata::V14(md) => md.try_into(),
        frame_metadata::RuntimeMetadata::V15(md) => md.try_into(),
        frame_metadata::RuntimeMetadata::V16(md) => md.try_into(),
        _ => {
            let reason = alloc::format!(
                "RuntimeMetadata version {} cannot be decoded from",
                metadata.version()
            );
            let e: codec::Error = "Metadata::decode failed: Cannot try_into() to Metadata: unsupported metadata version".into();
            return Err(e.chain(reason));
        }
    };

    metadata.map_err(|reason: TryFromError| {
        let e: codec::Error = "Metadata::decode failed: Cannot try_into() to Metadata".into();
        e.chain(reason.to_string())
    })
}
