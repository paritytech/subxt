// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Substrate specific configuration

use super::{Config, DefaultExtrinsicParams, DefaultExtrinsicParamsBuilder, Hasher, Header};
use crate::config::Hash;
use crate::metadata::{ArcMetadata, Metadata};
use crate::utils::RangeMap;
pub use crate::utils::{AccountId32, MultiAddress, MultiSignature};
use codec::{Decode, Encode};
pub use primitive_types::{H256, U256};
use scale_info_legacy::{ChainTypeRegistry, TypeRegistrySet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

/// Construct a [`SubstrateConfig`] using this.
pub struct SubstrateConfigBuilder {
    legacy_types: Option<ChainTypeRegistry>,
    spec_and_transaction_version_for_block_number: RangeMap<u64, (u32, u32)>,
    genesis_hash: Option<H256>,
    metadata_for_spec_version: Mutex<HashMap<u32, ArcMetadata>>,
    use_old_v9_hashers_before_spec_version: u32,
}

impl Default for SubstrateConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SubstrateConfigBuilder {
    /// Create a new builder to construct a [`SubstrateConfig`] from.
    pub fn new() -> Self {
        SubstrateConfigBuilder {
            legacy_types: None,
            genesis_hash: None,
            spec_and_transaction_version_for_block_number: RangeMap::empty(),
            metadata_for_spec_version: Mutex::new(HashMap::new()),
            use_old_v9_hashers_before_spec_version: 0,
        }
    }

    /// Set the genesis hash for this chain.
    pub fn set_genesis_hash(mut self, genesis_hash: H256) -> Self {
        self.genesis_hash = Some(genesis_hash);
        self
    }

    /// Set the legacy types to use for this configuration. This enables support for
    /// blocks produced by Runtimes that emit metadata older than V14.
    pub fn set_legacy_types(mut self, legacy_types: ChainTypeRegistry) -> Self {
        self.legacy_types = Some(legacy_types);
        self
    }

    /// Set the metadata to be used for decoding blocks at the given spec versions.
    pub fn set_metadata_for_spec_versions(
        self,
        ranges: impl IntoIterator<Item = (u32, ArcMetadata)>,
    ) -> Self {
        let mut map = self.metadata_for_spec_version.lock().unwrap();
        for (spec_version, metadata) in ranges.into_iter() {
            map.insert(spec_version, metadata);
        }
        drop(map);
        self
    }

    /// Given an iterator of block ranges to spec version of the form `(start, end, spec_version)`, add them
    /// to this configuration.
    pub fn set_spec_version_for_block_ranges(
        mut self,
        ranges: impl IntoIterator<Item = SpecVersionForRange>,
    ) -> Self {
        let mut m = RangeMap::builder();
        for version_for_range in ranges.into_iter() {
            let start = version_for_range.block_range.start;
            let end = version_for_range.block_range.end;
            let spec_version = version_for_range.spec_version;
            let transaction_version = version_for_range.transaction_version;
            m = m.add_range(start, end, (spec_version, transaction_version));
        }
        self.spec_and_transaction_version_for_block_number = m.build();
        self
    }

    /// The storage hasher encoding/decoding changed during V9 metadata. By default we support the "new" version
    /// of things. We can use this option to support the old version of things prior to a given spec version.
    pub fn use_old_v9_hashers_before_spec_version(mut self, spec_version: u32) -> Self {
        self.use_old_v9_hashers_before_spec_version = spec_version;
        self
    }

    /// Construct the [`SubstrateConfig`] from this builder.
    pub fn build(self) -> SubstrateConfig {
        SubstrateConfig {
            inner: Arc::new(SubstrateConfigInner {
                legacy_types: self.legacy_types,
                spec_and_transaction_version_for_block_number: self
                    .spec_and_transaction_version_for_block_number,
                metadata_for_spec_version: self.metadata_for_spec_version,
            }),
        }
    }
}

/// Define a spec version for a range of blocks. The new spec version is expected
/// to begin at the first block in the range and end just prior to the last block
/// in the range.
pub struct SpecVersionForRange {
    /// The block range that this spec version applies to. Inclusive of the start
    /// and exclusive of the enc.
    pub block_range: std::ops::Range<u64>,
    /// The spec version at this block range.
    pub spec_version: u32,
    /// The transaction version at this block range.
    pub transaction_version: u32,
}

/// Configuration that's suitable for standard Substrate chains (ie those
/// that have not customized the block hash type).
#[derive(Debug, Clone)]
pub struct SubstrateConfig {
    inner: Arc<SubstrateConfigInner>,
}

#[derive(Debug)]
struct SubstrateConfigInner {
    legacy_types: Option<ChainTypeRegistry>,
    spec_and_transaction_version_for_block_number: RangeMap<u64, (u32, u32)>,
    metadata_for_spec_version: Mutex<HashMap<u32, ArcMetadata>>,
}

impl Default for SubstrateConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl SubstrateConfig {
    /// Create a new, default, [`SubstrateConfig`]. This does not
    /// support working with historic (pre-V14) types. If you want this,
    /// then use [`SubstrateConfig::builder()`] and then provide legacy
    /// types via [`SubstrateConfigBuilder::set_legacy_types()`].
    pub fn new() -> Self {
        Self::builder().build()
    }

    /// Build a new [`SubstrateConfig`].
    pub fn builder() -> SubstrateConfigBuilder {
        SubstrateConfigBuilder::new()
    }
}

impl Config for SubstrateConfig {
    type AccountId = AccountId32;
    type Address = MultiAddress<Self::AccountId, u32>;
    type Signature = MultiSignature;
    type Hasher = DynamicHasher256;
    type Header = SubstrateHeader<<Self::Hasher as Hasher>::Hash>;
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
    type AssetId = u32;

    fn legacy_types_for_spec_version(&'_ self, spec_version: u32) -> Option<TypeRegistrySet<'_>> {
        self.inner
            .legacy_types
            .as_ref()
            .map(|types| types.for_spec_version(spec_version as u64))
    }

    fn spec_and_transaction_version_for_block_number(
        &self,
        block_number: u64,
    ) -> Option<(u32, u32)> {
        self.inner
            .spec_and_transaction_version_for_block_number
            .get(block_number)
            .copied()
    }

    fn metadata_for_spec_version(&self, spec_version: u32) -> Option<ArcMetadata> {
        self.inner
            .metadata_for_spec_version
            .lock()
            .unwrap()
            .get(&spec_version)
            .cloned()
    }

    fn set_metadata_for_spec_version(&self, spec_version: u32, metadata: ArcMetadata) {
        self.inner
            .metadata_for_spec_version
            .lock()
            .unwrap()
            .insert(spec_version, metadata);
    }
}

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for the default substrate node.
pub type SubstrateExtrinsicParams<T> = DefaultExtrinsicParams<T>;

/// A builder which leads to [`SubstrateExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type SubstrateExtrinsicParamsBuilder<T> = DefaultExtrinsicParamsBuilder<T>;

/// A hasher (ie implements [`Hasher`]) which hashes values using the blaks2_256 algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlakeTwo256;

impl Hasher for BlakeTwo256 {
    type Hash = H256;

    fn new(_metadata: &Metadata) -> Self {
        Self
    }

    fn hash(&self, s: &[u8]) -> Self::Hash {
        sp_crypto_hashing::blake2_256(s).into()
    }
}

/// A hasher (ie implements [`Hasher`]) which inspects the runtime metadata to decide how to
/// hash types, falling back to blake2_256 if the hasher information is not available.
///
/// Currently this hasher supports only `BlakeTwo256` and `Keccak256` hashing methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynamicHasher256(HashType);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HashType {
    // Most chains use this:
    BlakeTwo256,
    // Chains like Hyperbridge use this (tends to be eth compatible chains)
    Keccak256,
    // If we don't have V16 metadata, we'll emit this and default to BlakeTwo256.
    Unknown,
}

impl Hasher for DynamicHasher256 {
    type Hash = H256;

    fn new(metadata: &Metadata) -> Self {
        // Determine the Hash associated type used for the current chain, if possible.
        let Some(system_pallet) = metadata.pallet_by_name("System") else {
            return Self(HashType::Unknown);
        };
        let Some(hash_ty_id) = system_pallet.associated_type_id("Hashing") else {
            return Self(HashType::Unknown);
        };

        let ty = metadata
            .types()
            .resolve(hash_ty_id)
            .expect("Type information for 'Hashing' associated type should be in metadata");

        let hash_type = match ty.path.ident().as_deref().unwrap_or("") {
            "BlakeTwo256" => HashType::BlakeTwo256,
            "Keccak256" => HashType::Keccak256,
            _ => HashType::Unknown,
        };

        Self(hash_type)
    }

    fn hash(&self, s: &[u8]) -> Self::Hash {
        match self.0 {
            HashType::BlakeTwo256 | HashType::Unknown => sp_crypto_hashing::blake2_256(s).into(),
            HashType::Keccak256 => sp_crypto_hashing::keccak_256(s).into(),
        }
    }
}

/// A generic Substrate header type, adapted from `sp_runtime::generic::Header`.
/// The block number and hasher can be configured to adapt this for other nodes.
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstrateHeader<Hash> {
    /// The parent hash.
    pub parent_hash: Hash,
    /// The block number.
    #[serde(
        serialize_with = "serialize_number",
        deserialize_with = "deserialize_number"
    )]
    #[codec(compact)]
    pub number: u64,
    /// The state trie merkle root
    pub state_root: Hash,
    /// The merkle root of the extrinsics.
    pub extrinsics_root: Hash,
    /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
    pub digest: Digest,
}

impl<H> Header for SubstrateHeader<H>
where
    H: Hash,
    SubstrateHeader<H>: Encode + Decode,
{
    fn number(&self) -> u64 {
        self.number
    }
}

/// Generic header digest. From `sp_runtime::generic::digest`.
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct Digest {
    /// A list of digest items.
    pub logs: Vec<DigestItem>,
}

/// Digest item that is able to encode/decode 'system' digest items and
/// provide opaque access to other items. From `sp_runtime::generic::digest`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DigestItem {
    /// A pre-runtime digest.
    ///
    /// These are messages from the consensus engine to the runtime, although
    /// the consensus engine can (and should) read them itself to avoid
    /// code and state duplication. It is erroneous for a runtime to produce
    /// these, but this is not (yet) checked.
    ///
    /// NOTE: the runtime is not allowed to panic or fail in an `on_initialize`
    /// call if an expected `PreRuntime` digest is not present. It is the
    /// responsibility of a external block verifier to check this. Runtime API calls
    /// will initialize the block without pre-runtime digests, so initialization
    /// cannot fail when they are missing.
    PreRuntime(ConsensusEngineId, Vec<u8>),

    /// A message from the runtime to the consensus engine. This should *never*
    /// be generated by the native code of any consensus engine, but this is not
    /// checked (yet).
    Consensus(ConsensusEngineId, Vec<u8>),

    /// Put a Seal on it. This is only used by native code, and is never seen
    /// by runtimes.
    Seal(ConsensusEngineId, Vec<u8>),

    /// Some other thing. Unsupported and experimental.
    Other(Vec<u8>),

    /// An indication for the light clients that the runtime execution
    /// environment is updated.
    ///
    /// Currently this is triggered when:
    /// 1. Runtime code blob is changed or
    /// 2. `heap_pages` value is changed.
    RuntimeEnvironmentUpdated,
}

// From sp_runtime::generic, DigestItem enum indexes are encoded using this:
#[repr(u32)]
#[derive(Encode, Decode)]
enum DigestItemType {
    Other = 0u32,
    Consensus = 4u32,
    Seal = 5u32,
    PreRuntime = 6u32,
    RuntimeEnvironmentUpdated = 8u32,
}
impl Encode for DigestItem {
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();

        match self {
            Self::Consensus(val, data) => {
                DigestItemType::Consensus.encode_to(&mut v);
                (val, data).encode_to(&mut v);
            }
            Self::Seal(val, sig) => {
                DigestItemType::Seal.encode_to(&mut v);
                (val, sig).encode_to(&mut v);
            }
            Self::PreRuntime(val, data) => {
                DigestItemType::PreRuntime.encode_to(&mut v);
                (val, data).encode_to(&mut v);
            }
            Self::Other(val) => {
                DigestItemType::Other.encode_to(&mut v);
                val.encode_to(&mut v);
            }
            Self::RuntimeEnvironmentUpdated => {
                DigestItemType::RuntimeEnvironmentUpdated.encode_to(&mut v);
            }
        }

        v
    }
}
impl Decode for DigestItem {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let item_type: DigestItemType = Decode::decode(input)?;
        match item_type {
            DigestItemType::PreRuntime => {
                let vals: (ConsensusEngineId, Vec<u8>) = Decode::decode(input)?;
                Ok(Self::PreRuntime(vals.0, vals.1))
            }
            DigestItemType::Consensus => {
                let vals: (ConsensusEngineId, Vec<u8>) = Decode::decode(input)?;
                Ok(Self::Consensus(vals.0, vals.1))
            }
            DigestItemType::Seal => {
                let vals: (ConsensusEngineId, Vec<u8>) = Decode::decode(input)?;
                Ok(Self::Seal(vals.0, vals.1))
            }
            DigestItemType::Other => Ok(Self::Other(Decode::decode(input)?)),
            DigestItemType::RuntimeEnvironmentUpdated => Ok(Self::RuntimeEnvironmentUpdated),
        }
    }
}

/// Consensus engine unique ID. From `sp_runtime::ConsensusEngineId`.
pub type ConsensusEngineId = [u8; 4];

impl serde::Serialize for DigestItem {
    fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.using_encoded(|bytes| impl_serde::serialize::serialize(bytes, seq))
    }
}

impl<'a> serde::Deserialize<'a> for DigestItem {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let r = impl_serde::serialize::deserialize(de)?;
        Decode::decode(&mut &r[..])
            .map_err(|e| serde::de::Error::custom(format!("Decode error: {e}")))
    }
}

fn serialize_number<S, T: Copy + Into<U256>>(val: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let u256: U256 = (*val).into();
    serde::Serialize::serialize(&u256, s)
}

fn deserialize_number<'a, D, T: TryFrom<U256>>(d: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'a>,
{
    // At the time of writing, Smoldot gives back block numbers in numeric rather
    // than hex format. So let's support deserializing from both here:
    let number_or_hex = NumberOrHex::deserialize(d)?;
    let u256 = number_or_hex.into_u256();
    TryFrom::try_from(u256).map_err(|_| serde::de::Error::custom("Try from failed"))
}

/// A number type that can be serialized both as a number or a string that encodes a number in a
/// string.
///
/// We allow two representations of the block number as input. Either we deserialize to the type
/// that is specified in the block type or we attempt to parse given hex value.
///
/// The primary motivation for having this type is to avoid overflows when using big integers in
/// JavaScript (which we consider as an important RPC API consumer).
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum NumberOrHex {
    /// The number represented directly.
    Number(u64),
    /// Hex representation of the number.
    Hex(U256),
}

impl NumberOrHex {
    /// Converts this number into an U256.
    pub fn into_u256(self) -> U256 {
        match self {
            NumberOrHex::Number(n) => n.into(),
            NumberOrHex::Hex(h) => h,
        }
    }
}

impl From<NumberOrHex> for U256 {
    fn from(num_or_hex: NumberOrHex) -> U256 {
        num_or_hex.into_u256()
    }
}

macro_rules! into_number_or_hex {
    ($($t: ty)+) => {
        $(
            impl From<$t> for NumberOrHex {
                fn from(x: $t) -> Self {
                    NumberOrHex::Number(x.into())
                }
            }
        )+
    }
}
into_number_or_hex!(u8 u16 u32 u64);

impl From<u128> for NumberOrHex {
    fn from(n: u128) -> Self {
        NumberOrHex::Hex(n.into())
    }
}

impl From<U256> for NumberOrHex {
    fn from(n: U256) -> Self {
        NumberOrHex::Hex(n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Smoldot returns numeric block numbers in the header at the time of writing;
    // ensure we can deserialize them properly.
    #[test]
    fn can_deserialize_numeric_block_number() {
        let numeric_block_number_json = r#"
            {
                "digest": {
                    "logs": []
                },
                "extrinsicsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "number": 4,
                "parentHash": "0xcb2690b2c85ceab55be03fc7f7f5f3857e7efeb7a020600ebd4331e10be2f7a5",
                "stateRoot": "0x0000000000000000000000000000000000000000000000000000000000000000"
            }
        "#;

        let header: SubstrateHeader<H256> =
            serde_json::from_str(numeric_block_number_json).expect("valid block header");
        assert_eq!(header.number(), 4);
    }

    // Substrate returns hex block numbers; ensure we can also deserialize those OK.
    #[test]
    fn can_deserialize_hex_block_number() {
        let numeric_block_number_json = r#"
            {
                "digest": {
                    "logs": []
                },
                "extrinsicsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "number": "0x04",
                "parentHash": "0xcb2690b2c85ceab55be03fc7f7f5f3857e7efeb7a020600ebd4331e10be2f7a5",
                "stateRoot": "0x0000000000000000000000000000000000000000000000000000000000000000"
            }
        "#;

        let header: SubstrateHeader<H256> =
            serde_json::from_str(numeric_block_number_json).expect("valid block header");
        assert_eq!(header.number(), 4);
    }
}
