// Copyright 2019-2023 Parity Technologies (UK) Ltd.
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

#![deny(missing_docs)]

mod from_into;
mod utils;

use scale_info::{form::PortableForm, PortableRegistry, Variant};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use utils::ordered_map::OrderedMap;
use utils::variant_index::VariantIndex;

type ArcStr = Arc<str>;

pub use from_into::TryFromError;
pub use utils::validation::MetadataHasher;

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
    /// The type ID of the `Runtime` type.
    runtime_ty: u32,
    /// The types of the outer enums.
    outer_enums: OuterEnumsMetadata,
    /// The type Id of the `DispatchError` type, which Subxt makes use of.
    dispatch_error_ty: Option<u32>,
    /// Details about each of the runtime API traits.
    apis: OrderedMap<ArcStr, RuntimeApiMetadataInner>,
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

    /// The type ID of the `Runtime` type.
    pub fn runtime_ty(&self) -> u32 {
        self.runtime_ty
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

    /// Obtain a unique hash representing this metadata or specific parts of it.
    pub fn hasher(&self) -> MetadataHasher {
        MetadataHasher::new(self)
    }

    /// Filter out any pallets that we don't want to keep, retaining only those that we do.
    pub fn retain<F, G>(&mut self, pallet_filter: F, api_filter: G)
    where
        F: FnMut(&str) -> bool,
        G: FnMut(&str) -> bool,
    {
        utils::retain::retain_metadata(self, pallet_filter, api_filter);
    }

    /// Get type hash for a type in the registry
    pub fn type_hash(&self, id: u32) -> Option<[u8; 32]> {
        self.types.resolve(id)?;
        Some(crate::utils::validation::get_type_hash(
            &self.types,
            id,
            &mut HashSet::<u32>::new(),
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
    pub fn storage_hash(&self, entry_name: &str) -> Option<[u8; 32]> {
        crate::utils::validation::get_storage_hash(self, entry_name)
    }

    /// Return a hash for the constant, or None if it was not found.
    pub fn constant_hash(&self, constant_name: &str) -> Option<[u8; 32]> {
        crate::utils::validation::get_constant_hash(self, constant_name)
    }

    /// Return a hash for the call, or None if it was not found.
    pub fn call_hash(&self, call_name: &str) -> Option<[u8; 32]> {
        crate::utils::validation::get_call_hash(self, call_name)
    }

    /// Return a hash for the entire pallet.
    pub fn hash(&self) -> [u8; 32] {
        crate::utils::validation::get_pallet_hash(*self)
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
    /// The type of the address that signs the extrinsic
    address_ty: u32,
    /// The type of the outermost Call enum.
    call_ty: u32,
    /// The type of the extrinsic's signature.
    signature_ty: u32,
    /// The type of the outermost Extra enum.
    extra_ty: u32,
    /// Extrinsic version.
    version: u8,
    /// The signed extensions in the order they appear in the extrinsic.
    signed_extensions: Vec<SignedExtensionMetadata>,
}

impl ExtrinsicMetadata {
    /// The type of the address that signs the extrinsic
    pub fn address_ty(&self) -> u32 {
        self.address_ty
    }

    /// The type of the outermost Call enum.
    pub fn call_ty(&self) -> u32 {
        self.call_ty
    }
    /// The type of the extrinsic's signature.
    pub fn signature_ty(&self) -> u32 {
        self.signature_ty
    }
    /// The type of the outermost Extra enum.
    pub fn extra_ty(&self) -> u32 {
        self.extra_ty
    }

    /// Extrinsic version.
    pub fn version(&self) -> u8 {
        self.version
    }

    /// The extra/additional information associated with the extrinsic.
    pub fn signed_extensions(&self) -> &[SignedExtensionMetadata] {
        &self.signed_extensions
    }
}

/// Metadata for the signed extensions used by extrinsics.
#[derive(Debug, Clone)]
pub struct SignedExtensionMetadata {
    /// The unique signed extension identifier, which may be different from the type name.
    identifier: String,
    /// The type of the signed extension, with the data to be included in the extrinsic.
    extra_ty: u32,
    /// The type of the additional signed data, with the data to be included in the signed payload
    additional_ty: u32,
}

impl SignedExtensionMetadata {
    /// The unique signed extension identifier, which may be different from the type name.
    pub fn identifier(&self) -> &str {
        &self.identifier
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
    pub fn methods(&self) -> impl ExactSizeIterator<Item = &'a RuntimeApiMethodMetadata> {
        self.inner.methods.values().iter()
    }
    /// Get a specific trait method given its name.
    pub fn method_by_name(&self, name: &str) -> Option<&'a RuntimeApiMethodMetadata> {
        self.inner.methods.get_by_key(name)
    }
    /// Return a hash for the constant, or None if it was not found.
    pub fn method_hash(&self, method_name: &str) -> Option<[u8; 32]> {
        crate::utils::validation::get_runtime_api_hash(self, method_name)
    }

    /// Return a hash for the runtime API trait.
    pub fn hash(&self) -> [u8; 32] {
        crate::utils::validation::get_runtime_trait_hash(*self)
    }
}

#[derive(Debug, Clone)]
struct RuntimeApiMetadataInner {
    /// Trait name.
    name: ArcStr,
    /// Trait methods.
    methods: OrderedMap<ArcStr, RuntimeApiMethodMetadata>,
    /// Trait documentation.
    docs: Vec<String>,
}

/// Metadata for a single runtime API method.
#[derive(Debug, Clone)]
pub struct RuntimeApiMethodMetadata {
    /// Method name.
    name: ArcStr,
    /// Method parameters.
    inputs: Vec<RuntimeApiMethodParamMetadata>,
    /// Method output type.
    output_ty: u32,
    /// Method documentation.
    docs: Vec<String>,
}

impl RuntimeApiMethodMetadata {
    /// Method name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Method documentation.
    pub fn docs(&self) -> &[String] {
        &self.docs
    }
    /// Method inputs.
    pub fn inputs(&self) -> impl ExactSizeIterator<Item = &RuntimeApiMethodParamMetadata> {
        self.inputs.iter()
    }
    /// Method return type.
    pub fn output_ty(&self) -> u32 {
        self.output_ty
    }
}

/// Metadata for a single input parameter to a runtime API method.
#[derive(Debug, Clone)]
pub struct RuntimeApiMethodParamMetadata {
    /// Parameter name.
    pub name: String,
    /// Parameter type.
    pub ty: u32,
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

// Metadata can be encoded, too. It will encode into a format that's compatible with what
// Subxt requires, and that it can be decoded back from. The actual specifics of the format
// can change over time.
impl codec::Encode for Metadata {
    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        let m: frame_metadata::v15::RuntimeMetadataV15 = self.clone().into();
        let m: frame_metadata::RuntimeMetadataPrefixed = m.into();
        m.encode_to(dest)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use codec::{Decode, Encode};

    fn load_metadata() -> Vec<u8> {
        std::fs::read("../artifacts/polkadot_metadata_full.scale").unwrap()
    }

    // We don't expect to lose any information converting back and forth between
    // our own representation and the latest version emitted from a node that we can
    // work with.
    #[test]
    fn is_isomorphic_to_v15() {
        let bytes = load_metadata();

        // Decode into our metadata struct:
        let metadata = Metadata::decode(&mut &*bytes).unwrap();

        // Convert into v15 metadata:
        let v15: frame_metadata::v15::RuntimeMetadataV15 = metadata.into();
        let prefixed = frame_metadata::RuntimeMetadataPrefixed::from(v15);

        // Re-encode that:
        let new_bytes = prefixed.encode();

        // The bytes should be identical:
        assert_eq!(bytes, new_bytes);
    }
}
