// Copyright 2019-2025 Parity Technologies (UK) Ltd.
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

mod from_into;
mod utils;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use frame_decode::extrinsics::{
    ExtrinsicCallInfo, ExtrinsicExtensionInfo, ExtrinsicInfoArg, ExtrinsicInfoError,
    ExtrinsicSignatureInfo,
};
use hashbrown::HashMap;
use scale_info::{form::PortableForm, PortableRegistry, Variant};
use utils::{
    ordered_map::OrderedMap, 
    validation::outer_enum_hashes::OuterEnumHashes, 
    variant_index::VariantIndex,
    validation::{get_custom_value_hash, HASH_LEN},
};

type ArcStr = Arc<str>;

pub use from_into::TryFromError;
pub use utils::validation::MetadataHasher;

type CustomMetadataInner = frame_metadata::v15::CustomMetadata<PortableForm>;

/// Utility functions to help with metadata related things.
pub mod utilities {
    pub use crate::utils::outer_enums::OuterEnums;
}

/// Node metadata. This can be constructed by providing some compatible [`frame_metadata`]
/// which is then decoded into this. We aim to preserve all of the existing information in
/// the incoming metadata while optimizing the format a little for Subxt's use cases.
#[derive(Debug, Clone)]
pub struct Metadata {
    /// Type registry containing all types used in the metadata.
    types: PortableRegistry,
    /// Metadata of all the pallets.
    pallets: OrderedMap<ArcStr, PalletMetadataInner>,
    /// Find the location in the pallet Vec by pallet index.
    pallets_by_index: HashMap<u8, usize>,
    /// Metadata of the extrinsic.
    extrinsic: ExtrinsicMetadata,
    /// The types of the outer enums.
    outer_enums: OuterEnumsMetadata,
    /// The type Id of the `DispatchError` type, which Subxt makes use of.
    dispatch_error_ty: Option<u32>,
    /// Details about each of the runtime API traits.
    apis: OrderedMap<ArcStr, RuntimeApiMetadataInner>,
    /// Allows users to add custom types to the metadata. A map that associates a string key to a `CustomValueMetadata`.
    custom: CustomMetadataInner,
}

// Since we've abstracted away from frame-metadatas, we impl this on our custom Metadata
// so that it can be used by `frame-decode` to obtain the relevant extrinsic info.
impl frame_decode::extrinsics::ExtrinsicTypeInfo for Metadata {
    type TypeId = u32;

    fn get_call_info(
        &self,
        pallet_index: u8,
        call_index: u8,
    ) -> Result<ExtrinsicCallInfo<'_, Self::TypeId>, ExtrinsicInfoError<'_>> {
        let pallet = self.pallet_by_index(pallet_index).ok_or({
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
            pallet_name: Cow::Borrowed(pallet.name()),
            call_name: Cow::Borrowed(&call.name),
            args: call
                .fields
                .iter()
                .map(|f| ExtrinsicInfoArg {
                    name: Cow::Borrowed(f.name.as_deref().unwrap_or("")),
                    id: f.ty.id,
                })
                .collect(),
        })
    }

    fn get_signature_info(
        &self,
    ) -> Result<ExtrinsicSignatureInfo<Self::TypeId>, ExtrinsicInfoError<'_>> {
        Ok(ExtrinsicSignatureInfo {
            address_id: self.extrinsic().address_ty,
            signature_id: self.extrinsic().signature_ty,
        })
    }

    fn get_extension_info(
        &self,
        extension_version: Option<u8>,
    ) -> Result<ExtrinsicExtensionInfo<'_, Self::TypeId>, ExtrinsicInfoError<'_>> {
        let extension_version = extension_version.unwrap_or_else(|| {
            // We have some transaction, probably a V4 one with no extension version,
            // but our metadata may support multiple versions. Use the metadata to decide
            // what version to assume we'll decode it as.
            self.extrinsic().transaction_extension_version_to_use_for_decoding()
        });

        let extension_ids = self
            .extrinsic()
            .transaction_extensions_by_version(extension_version)
            .ok_or_else(|| ExtrinsicInfoError::ExtrinsicExtensionVersionNotFound { extension_version })?
            .map(|f| ExtrinsicInfoArg {
                name: Cow::Borrowed(f.identifier()),
                id: f.extra_ty(),
            })
            .collect();

        Ok(ExtrinsicExtensionInfo { extension_ids })
    }
}

impl Metadata {
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

    /// Access a pallet given its encoded variant index.
    pub fn pallet_by_index(&self, variant_index: u8) -> Option<PalletMetadata<'_>> {
        let inner = self
            .pallets_by_index
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
    pub fn hasher(&self) -> MetadataHasher {
        MetadataHasher::new(self)
    }

    /// Filter out any pallets and/or runtime_apis that we don't want to keep, retaining only those that we do.
    /// Note:
    /// only filter by `pallet`s will not lead to significant metadata size reduction because the return types are kept to ensure that those can be decoded.
    ///
    pub fn retain<F, G>(&mut self, pallet_filter: F, api_filter: G)
    where
        F: FnMut(&str) -> bool,
        G: FnMut(&str) -> bool,
    {
        utils::retain::retain_metadata(self, pallet_filter, api_filter);
    }

    /// Get type hash for a type in the registry
    pub fn type_hash(&self, id: u32) -> Option<[u8; HASH_LEN]> {
        self.types.resolve(id)?;
        Some(crate::utils::validation::get_type_hash(
            &self.types,
            id,
            &OuterEnumHashes::empty(),
        ))
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

    /// The pallet index.
    pub fn index(&self) -> u8 {
        self.inner.index
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

    /// Return an iterator over the View Functions in this pallet, if any.
    pub fn view_functions(&self) -> impl ExactSizeIterator<Item = PalletViewFunctionMetadata<'a>> {
        self.inner.view_functions.iter().map(|vf: &'a _| {
            PalletViewFunctionMetadata {
                inner: vf,
                types: self.types
            }
        })
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
    pub fn constants(&self) -> impl ExactSizeIterator<Item = &'a ConstantMetadata> {
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
        crate::utils::validation::get_pallet_hash(*self, &OuterEnumHashes::empty())
    }
}

#[derive(Debug, Clone)]
struct PalletMetadataInner {
    /// Pallet name.
    name: ArcStr,
    /// Pallet index.
    index: u8,
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
    constants: OrderedMap<ArcStr, ConstantMetadata>,
    /// Details about each of the pallet view functions.
    view_functions: Vec<PalletViewFunctionMetadataInner>,
    /// Pallet documentation.
    docs: Vec<String>,
}

/// Metadata for the storage entries in a pallet.
#[derive(Debug, Clone)]
pub struct StorageMetadata {
    /// The common prefix used by all storage entries.
    prefix: String,
    /// Map from storage entry name to details.
    entries: OrderedMap<ArcStr, StorageEntryMetadata>,
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
    name: ArcStr,
    /// An `Option` modifier of that storage entry.
    modifier: StorageEntryModifier,
    /// Type of the value stored in the entry.
    entry_type: StorageEntryType,
    /// Default value (SCALE encoded).
    default: Vec<u8>,
    /// Storage entry documentation.
    docs: Vec<String>,
}

impl StorageEntryMetadata {
    /// Name of this entry.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Is the entry value optional or does it have a default value.
    pub fn modifier(&self) -> StorageEntryModifier {
        self.modifier
    }
    /// Type of the storage entry.
    pub fn entry_type(&self) -> &StorageEntryType {
        &self.entry_type
    }
    /// The SCALE encoded default value for this entry.
    pub fn default_bytes(&self) -> &[u8] {
        &self.default
    }
    /// Storage entry documentation.
    pub fn docs(&self) -> &[String] {
        &self.docs
    }
}

/// The type of a storage entry.
#[derive(Debug, Clone)]
pub enum StorageEntryType {
    /// Plain storage entry (just the value).
    Plain(u32),
    /// A storage map.
    Map {
        /// One or more hashers, should be one hasher per key element.
        hashers: Vec<StorageHasher>,
        /// The type of the key, can be a tuple with elements for each of the hashers.
        key_ty: u32,
        /// The type of the value.
        value_ty: u32,
    },
}

impl StorageEntryType {
    /// The type of the value.
    pub fn value_ty(&self) -> u32 {
        match self {
            StorageEntryType::Map { value_ty, .. } | StorageEntryType::Plain(value_ty) => *value_ty,
        }
    }

    /// The type of the key, can be a tuple with elements for each of the hashers. None for a Plain storage entry.
    pub fn key_ty(&self) -> Option<u32> {
        match self {
            StorageEntryType::Map { key_ty, .. } => Some(*key_ty),
            StorageEntryType::Plain(_) => None,
        }
    }
}

/// Hasher used by storage maps.
#[derive(Debug, Clone, Copy)]
pub enum StorageHasher {
    /// 128-bit Blake2 hash.
    Blake2_128,
    /// 256-bit Blake2 hash.
    Blake2_256,
    /// Multiple 128-bit Blake2 hashes concatenated.
    Blake2_128Concat,
    /// 128-bit XX hash.
    Twox128,
    /// 256-bit XX hash.
    Twox256,
    /// Multiple 64-bit XX hashes concatenated.
    Twox64Concat,
    /// Identity hashing (no hashing).
    Identity,
}

impl StorageHasher {
    /// The hash produced by a [`StorageHasher`] can have these two components, in order:
    ///
    /// 1. A fixed size hash. (not present for [`StorageHasher::Identity`]).
    /// 2. The SCALE encoded key that was used as an input to the hasher (only present for
    ///    [`StorageHasher::Twox64Concat`], [`StorageHasher::Blake2_128Concat`] or [`StorageHasher::Identity`]).
    ///
    /// This function returns the number of bytes used to represent the first of these.
    pub fn len_excluding_key(&self) -> usize {
        match self {
            StorageHasher::Blake2_128Concat => 16,
            StorageHasher::Twox64Concat => 8,
            StorageHasher::Blake2_128 => 16,
            StorageHasher::Blake2_256 => 32,
            StorageHasher::Twox128 => 16,
            StorageHasher::Twox256 => 32,
            StorageHasher::Identity => 0,
        }
    }

    /// Returns true if the key used to produce the hash is appended to the hash itself.
    pub fn ends_with_key(&self) -> bool {
        matches!(
            self,
            StorageHasher::Blake2_128Concat | StorageHasher::Twox64Concat | StorageHasher::Identity
        )
    }
}

/// Is the storage entry optional, or does it have a default value.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StorageEntryModifier {
    /// The storage entry returns an `Option<T>`, with `None` if the key is not present.
    Optional,
    /// The storage entry returns `T::Default` if the key is not present.
    Default,
}

/// Metadata for a single constant.
#[derive(Debug, Clone)]
pub struct ConstantMetadata {
    /// Name of the pallet constant.
    name: ArcStr,
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
    pub fn transaction_extensions_by_version(&self, version: u8) -> Option<impl Iterator<Item = TransactionExtensionMetadata<'_>>> {
        let extension_indexes = self.transaction_extensions_by_version.get(&version)?;
        let iter = extension_indexes.iter().map(|index| {
            let tx_metadata = self
                .transaction_extensions
                .get(*index as usize)
                .expect("transaction extension should exist if index is in transaction_extensions_by_version");

            TransactionExtensionMetadata {
                identifier: &*tx_metadata.identifier,
                extra_ty: tx_metadata.extra_ty,
                additional_ty: tx_metadata.additional_ty,
            }
        });

        Some(iter)
    }

    /// When constructing a v5 extrinsic, use this transaction extensions version.
    pub fn transaction_extension_version_to_use_for_encoding(&self) -> u8 {
        *self.transaction_extensions_by_version.keys().max().unwrap()
    }

    /// When presented with a v4 extrinsic that has no version, treat it as being this version.
    pub fn transaction_extension_version_to_use_for_decoding(&self) -> u8 {
        *self.transaction_extensions_by_version.keys().max().unwrap()
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

impl <'a> TransactionExtensionMetadata<'a> {
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
    pub fn methods(&self) -> impl ExactSizeIterator<Item = RuntimeApiMethodMetadata<'a>> {
        self.inner.methods.values().iter().map(|item| {
            RuntimeApiMethodMetadata {
                trait_name: &self.inner.name,
                inner: item,
                types: self.types
            }
        })
    }
    /// Get a specific trait method given its name.
    pub fn method_by_name(&self, name: &str) -> Option<RuntimeApiMethodMetadata<'a>> {
        self.inner.methods.get_by_key(name).map(|item| {
            RuntimeApiMethodMetadata {
                trait_name: &self.inner.name,
                inner: item,
                types: self.types
            }
        })
    }
    /// Return a hash for the runtime API trait.
    pub fn hash(&self) -> [u8; HASH_LEN] {
        crate::utils::validation::get_runtime_apis_hash(*self, &OuterEnumHashes::empty())
    }
}

#[derive(Debug, Clone)]
struct RuntimeApiMetadataInner {
    /// Trait name.
    name: ArcStr,
    /// Trait methods.
    methods: OrderedMap<ArcStr, RuntimeApiMethodMetadataInner>,
    /// Trait documentation.
    docs: Vec<String>,
}

/// Metadata for a single runtime API method.
#[derive(Debug, Clone)]
pub struct RuntimeApiMethodMetadata<'a> {
    trait_name: &'a str,
    inner: &'a RuntimeApiMethodMetadataInner,
    types: &'a PortableRegistry
}

impl <'a> RuntimeApiMethodMetadata<'a> {
    /// Method name.
    pub fn name(&self) -> &'a str {
        &self.inner.name
    }
    /// Method documentation.
    pub fn docs(&self) -> &[String] {
        &self.inner.docs
    }
    /// Method inputs.
    pub fn inputs(&self) -> impl ExactSizeIterator<Item = &MethodParamMetadata> {
        self.inner.inputs.iter()
    }
    /// Method return type.
    pub fn output_ty(&self) -> u32 {
        self.inner.output_ty
    }
    /// Return a hash for the method.
    pub fn hash(&self) -> [u8; HASH_LEN] {
        crate::utils::validation::get_runtime_api_hash(self, &OuterEnumHashes::empty())
    }
}

#[derive(Debug, Clone)]
struct RuntimeApiMethodMetadataInner {
    /// Method name.
    name: ArcStr,
    /// Method parameters.
    inputs: Vec<MethodParamMetadata>,
    /// Method output type.
    output_ty: u32,
    /// Method documentation.
    docs: Vec<String>,
}

/// Metadata for the available pallet View Functions.
#[derive(Debug, Clone, Copy)]
pub struct PalletViewFunctionMetadata<'a> {
    inner: &'a PalletViewFunctionMetadataInner,
    types: &'a PortableRegistry,
}

impl<'a> PalletViewFunctionMetadata<'a> {
    /// Method name.
    pub fn name(&self) -> &str {
        &self.inner.name
    }
    /// Query ID. This is used to query the function. Roughly, it is constructed by doing
    /// `twox_128(pallet_name) ++ twox_128("fn_name(fnarg_types) -> return_ty")` .
    pub fn query_id(&self) -> [u8; 32] {
        self.inner.query_id
    }
    /// Method documentation.
    pub fn docs(&self) -> &[String] {
        &self.inner.docs
    }
    /// Method inputs.
    pub fn inputs(&self) -> impl ExactSizeIterator<Item = &MethodParamMetadata> {
        self.inner.inputs.iter()
    }
    /// Method return type.
    pub fn output_ty(&self) -> u32 {
        self.inner.output_ty
    }
}

#[derive(Debug, Clone)]
struct PalletViewFunctionMetadataInner {
    /// View function name.
    name: String,
    /// View function query ID.
    query_id: [u8; 32],
    /// Input types.
    inputs: Vec<MethodParamMetadata>,
    /// Output type.
    output_ty: u32,
    /// Documentation.
    docs: Vec<String>
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
    pub fn iter(&self) -> impl Iterator<Item = CustomValueMetadata> {
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
        get_custom_value_hash(self, &OuterEnumHashes::empty())
    }
}

// Support decoding metadata from the "wire" format directly into this.
// Errors may be lost in the case that the metadata content is somehow invalid.
impl codec::Decode for Metadata {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let metadata = frame_metadata::RuntimeMetadataPrefixed::decode(input)?;
        let metadata = match metadata.1 {
            frame_metadata::RuntimeMetadata::V14(md) => md.try_into(),
            frame_metadata::RuntimeMetadata::V15(md) => md.try_into(),
            _ => return Err("Cannot try_into() to Metadata: unsupported metadata version".into()),
        };

        metadata.map_err(|_e| "Cannot try_into() to Metadata.".into())
    }
}
