// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types sent to/from the Substrate RPC interface.

use crate::Config;
use codec::{
    Decode,
    Encode,
};
use primitive_types::U256;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;

// Subscription types are returned from some calls, so expose it with the rest of the returned types.
pub use super::rpc_client::Subscription;

/// Signal what the result of doing a dry run of an extrinsic is.
pub type DryRunResult = Result<(), DryRunError>;

/// An error dry running an extrinsic.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DryRunError {
    /// The extrinsic will not be included in the block
    TransactionValidityError,
    /// The extrinsic will be included in the block, but the call failed to dispatch.
    DispatchError,
}

/// dryRun returns an ApplyExtrinsicResult, which is basically a
/// `Result<Result<(), DispatchError>, TransactionValidityError>`. We want to convert this to
/// a [`DryRunResult`].
///
/// - if `Ok(inner)`, the transaction will be included in the block
/// - if `Ok(Ok(()))`, the transaction will be included and the call will be dispatched
///   successfully
/// - if `Ok(Err(e))`, the transaction will be included but there is some error dispatching
///   the call to the module.
///
/// The errors get a bit involved and have been known to change over time. At the moment
/// then, we will keep things simple here and just decode the Result portion (ie the initial bytes)
/// and ignore the rest.
pub(crate) fn decode_dry_run_result<I: codec::Input>(
    input: &mut I,
) -> Result<DryRunResult, codec::Error> {
    let res = match <Result<Result<(), ()>, ()>>::decode(input)? {
        Ok(Ok(())) => Ok(()),
        Ok(Err(())) => Err(DryRunError::DispatchError),
        Err(()) => Err(DryRunError::TransactionValidityError),
    };
    Ok(res)
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

/// Hex-serialized shim for `Vec<u8>`.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Hash, PartialOrd, Ord, Debug)]
pub struct Bytes(#[serde(with = "impl_serde::serialize")] pub Vec<u8>);
impl std::ops::Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0[..]
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(s: Vec<u8>) -> Self {
        Bytes(s)
    }
}

/// The response from `chain_getBlock`
#[derive(Debug, Deserialize)]
#[serde(bound = "T: Config")]
pub struct ChainBlockResponse<T: Config> {
    /// The block itself.
    pub block: ChainBlock<T>,
    /// Block justification.
    pub justifications: Option<Vec<Justification>>,
}

/// Block details in the [`ChainBlockResponse`].
#[derive(Debug, Deserialize)]
pub struct ChainBlock<T: Config> {
    /// The block header.
    pub header: T::Header,
    /// The accompanying extrinsics.
    pub extrinsics: Vec<ChainBlockExtrinsic>,
}

/// An abstraction over justification for a block's validity under a consensus algorithm.
pub type Justification = (ConsensusEngineId, EncodedJustification);
/// Consensus engine unique ID.
pub type ConsensusEngineId = [u8; 4];
/// The encoded justification specific to a consensus engine.
pub type EncodedJustification = Vec<u8>;

/// Bytes representing an extrinsic in a [`ChainBlock`].
#[derive(Clone, Debug)]
pub struct ChainBlockExtrinsic(pub Vec<u8>);

impl<'a> ::serde::Deserialize<'a> for ChainBlockExtrinsic {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'a>,
    {
        let r = impl_serde::serialize::deserialize(de)?;
        let bytes = Decode::decode(&mut &r[..])
            .map_err(|e| ::serde::de::Error::custom(format!("Decode error: {}", e)))?;
        Ok(ChainBlockExtrinsic(bytes))
    }
}

/// Wrapper for NumberOrHex to allow custom From impls
#[derive(Serialize)]
pub struct BlockNumber(NumberOrHex);

impl From<NumberOrHex> for BlockNumber {
    fn from(x: NumberOrHex) -> Self {
        BlockNumber(x)
    }
}

impl Default for NumberOrHex {
    fn default() -> Self {
        Self::Number(Default::default())
    }
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

impl From<u32> for NumberOrHex {
    fn from(n: u32) -> Self {
        NumberOrHex::Number(n.into())
    }
}

impl From<u64> for NumberOrHex {
    fn from(n: u64) -> Self {
        NumberOrHex::Number(n)
    }
}

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

/// An error type that signals an out-of-range conversion attempt.
#[derive(Debug, thiserror::Error)]
#[error("Out-of-range conversion attempt")]
pub struct TryFromIntError;

impl TryFrom<NumberOrHex> for u32 {
    type Error = TryFromIntError;
    fn try_from(num_or_hex: NumberOrHex) -> Result<u32, Self::Error> {
        num_or_hex
            .into_u256()
            .try_into()
            .map_err(|_| TryFromIntError)
    }
}

impl TryFrom<NumberOrHex> for u64 {
    type Error = TryFromIntError;
    fn try_from(num_or_hex: NumberOrHex) -> Result<u64, Self::Error> {
        num_or_hex
            .into_u256()
            .try_into()
            .map_err(|_| TryFromIntError)
    }
}

impl TryFrom<NumberOrHex> for u128 {
    type Error = TryFromIntError;
    fn try_from(num_or_hex: NumberOrHex) -> Result<u128, Self::Error> {
        num_or_hex
            .into_u256()
            .try_into()
            .map_err(|_| TryFromIntError)
    }
}

impl From<NumberOrHex> for U256 {
    fn from(num_or_hex: NumberOrHex) -> U256 {
        num_or_hex.into_u256()
    }
}

// All unsigned ints can be converted into a BlockNumber:
macro_rules! into_block_number {
    ($($t: ty)+) => {
        $(
            impl From<$t> for BlockNumber {
                fn from(x: $t) -> Self {
                    NumberOrHex::Number(x.into()).into()
                }
            }
        )+
    }
}
into_block_number!(u8 u16 u32 u64);

/// Arbitrary properties defined in the chain spec as a JSON object.
pub type SystemProperties = serde_json::Map<String, serde_json::Value>;

/// Possible transaction status events.
///
/// # Note
///
/// This is copied from `sp-transaction-pool` to avoid a dependency on that crate. Therefore it
/// must be kept compatible with that type from the target substrate version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubstrateTxStatus<Hash, BlockHash> {
    /// Transaction is part of the future queue.
    Future,
    /// Transaction is part of the ready queue.
    Ready,
    /// The transaction has been broadcast to the given peers.
    Broadcast(Vec<String>),
    /// Transaction has been included in block with given hash.
    InBlock(BlockHash),
    /// The block this transaction was included in has been retracted.
    Retracted(BlockHash),
    /// Maximum number of finality watchers has been reached,
    /// old watchers are being removed.
    FinalityTimeout(BlockHash),
    /// Transaction has been finalized by a finality-gadget, e.g GRANDPA
    Finalized(BlockHash),
    /// Transaction has been replaced in the pool, by another transaction
    /// that provides the same tags. (e.g. same (sender, nonce)).
    Usurped(Hash),
    /// Transaction has been dropped from the pool because of the limit.
    Dropped,
    /// Transaction is no longer valid in the current state.
    Invalid,
}

/// This contains the runtime version information necessary to make transactions, as obtained from
/// the RPC call `state_getRuntimeVersion`,
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
    /// Version of the runtime specification. A full-node will not attempt to use its native
    /// runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    /// `spec_version` and `authoring_version` are the same between Wasm and native.
    pub spec_version: u32,

    /// All existing dispatches are fully compatible when this number doesn't change. If this
    /// number changes, then `spec_version` must change, also.
    ///
    /// This number must change when an existing dispatchable (module ID, dispatch ID) is changed,
    /// either through an alteration in its user-level semantics, a parameter
    /// added/removed/changed, a dispatchable being removed, a module being removed, or a
    /// dispatchable/module changing its index.
    ///
    /// It need *not* change when a new module is added or when a dispatchable is added.
    pub transaction_version: u32,

    /// The other fields present may vary and aren't necessary for `subxt`; they are preserved in
    /// this map.
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

/// ReadProof struct returned by the RPC
///
/// # Note
///
/// This is copied from `sc-rpc-api` to avoid a dependency on that crate. Therefore it
/// must be kept compatible with that type from the target substrate version.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadProof<Hash> {
    /// Block hash used to generate the proof
    pub at: Hash,
    /// A proof used to prove that storage entries are included in the storage trie
    pub proof: Vec<Bytes>,
}

/// Statistics of a block returned by the `dev_getBlockStats` RPC.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStats {
    /// The length in bytes of the storage proof produced by executing the block.
    pub witness_len: u64,
    /// The length in bytes of the storage proof after compaction.
    pub witness_compact_len: u64,
    /// Length of the block in bytes.
    ///
    /// This information can also be acquired by downloading the whole block. This merely
    /// saves some complexity on the client side.
    pub block_len: u64,
    /// Number of extrinsics in the block.
    ///
    /// This information can also be acquired by downloading the whole block. This merely
    /// saves some complexity on the client side.
    pub num_extrinsics: u64,
}

/// Storage key.
#[derive(
    Serialize,
    Deserialize,
    Hash,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Clone,
    Encode,
    Decode,
    Debug,
)]
pub struct StorageKey(#[serde(with = "impl_serde::serialize")] pub Vec<u8>);
impl AsRef<[u8]> for StorageKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Storage data.
#[derive(
    Serialize,
    Deserialize,
    Hash,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Clone,
    Encode,
    Decode,
    Debug,
)]
pub struct StorageData(#[serde(with = "impl_serde::serialize")] pub Vec<u8>);
impl AsRef<[u8]> for StorageData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Storage change set
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorageChangeSet<Hash> {
    /// Block hash
    pub block: Hash,
    /// A list of changes
    pub changes: Vec<(StorageKey, Option<StorageData>)>,
}

/// Health struct returned by the RPC
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    /// Number of connected peers
    pub peers: usize,
    /// Is the node syncing
    pub is_syncing: bool,
    /// Should this node have any peers
    ///
    /// Might be false for local chains or when running without discovery.
    pub should_have_peers: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    /// A util function to assert the result of serialization and deserialization is the same.
    pub fn assert_deser<T>(s: &str, expected: T)
    where
        T: std::fmt::Debug
            + serde::ser::Serialize
            + serde::de::DeserializeOwned
            + PartialEq,
    {
        assert_eq!(serde_json::from_str::<T>(s).unwrap(), expected);
        assert_eq!(serde_json::to_string(&expected).unwrap(), s);
    }

    // Check that some A can be serialized and then deserialized into some B.
    pub fn assert_ser_deser<A, B>(a: &A, b: &B)
    where
        A: serde::Serialize,
        B: serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
    {
        let json = serde_json::to_string(a).expect("serializing failed");
        let new_b: B = serde_json::from_str(&json).expect("deserializing failed");

        assert_eq!(b, &new_b);
    }

    #[test]
    fn runtime_version_is_substrate_compatible() {
        use sp_version::RuntimeVersion as SpRuntimeVersion;

        let substrate_runtime_version = SpRuntimeVersion {
            spec_version: 123,
            transaction_version: 456,
            ..Default::default()
        };

        let json = serde_json::to_string(&substrate_runtime_version)
            .expect("serializing failed");
        let val: RuntimeVersion =
            serde_json::from_str(&json).expect("deserializing failed");

        // We ignore any other properties.
        assert_eq!(val.spec_version, 123);
        assert_eq!(val.transaction_version, 456);
    }

    #[test]
    fn runtime_version_handles_arbitrary_params() {
        let val: RuntimeVersion = serde_json::from_str(
            r#"{
                "specVersion": 123,
                "transactionVersion": 456,
                "foo": true,
                "wibble": [1,2,3]
            }"#,
        )
        .expect("deserializing failed");

        let mut m = std::collections::HashMap::new();
        m.insert("foo".to_owned(), serde_json::json!(true));
        m.insert("wibble".to_owned(), serde_json::json!([1, 2, 3]));

        assert_eq!(
            val,
            RuntimeVersion {
                spec_version: 123,
                transaction_version: 456,
                other: m
            }
        );
    }

    #[test]
    fn number_or_hex_deserializes_from_either_repr() {
        assert_deser(r#""0x1234""#, NumberOrHex::Hex(0x1234.into()));
        assert_deser(r#""0x0""#, NumberOrHex::Hex(0.into()));
        assert_deser(r#"5"#, NumberOrHex::Number(5));
        assert_deser(r#"10000"#, NumberOrHex::Number(10000));
        assert_deser(r#"0"#, NumberOrHex::Number(0));
        assert_deser(r#"1000000000000"#, NumberOrHex::Number(1000000000000));
    }

    #[test]
    fn dry_run_result_is_substrate_compatible() {
        use sp_runtime::{
            transaction_validity::{
                InvalidTransaction as SpInvalidTransaction,
                TransactionValidityError as SpTransactionValidityError,
            },
            ApplyExtrinsicResult as SpApplyExtrinsicResult,
            DispatchError as SpDispatchError,
        };

        let pairs = vec![
            // All ok
            (SpApplyExtrinsicResult::Ok(Ok(())), Ok(())),
            // Some transaction error
            (
                SpApplyExtrinsicResult::Err(SpTransactionValidityError::Invalid(
                    SpInvalidTransaction::BadProof,
                )),
                Err(DryRunError::TransactionValidityError),
            ),
            // Some dispatch error
            (
                SpApplyExtrinsicResult::Ok(Err(SpDispatchError::BadOrigin)),
                Err(DryRunError::DispatchError),
            ),
        ];

        for (actual, expected) in pairs {
            let encoded = actual.encode();
            assert_eq!(decode_dry_run_result(&mut &*encoded).unwrap(), expected);
        }
    }

    #[test]
    fn justification_is_substrate_compatible() {
        use sp_runtime::Justification as SpJustification;

        // As much as anything, this just checks that the Justification type
        // is still a tuple as given.
        assert_ser_deser::<SpJustification, Justification>(
            &([1, 2, 3, 4], vec![5, 6, 7, 8]),
            &([1, 2, 3, 4], vec![5, 6, 7, 8]),
        );
    }

    #[test]
    fn storage_types_are_substrate_compatible() {
        use sp_core::storage::{
            StorageChangeSet as SpStorageChangeSet,
            StorageData as SpStorageData,
            StorageKey as SpStorageKey,
        };

        assert_ser_deser(
            &SpStorageKey(vec![1, 2, 3, 4, 5]),
            &StorageKey(vec![1, 2, 3, 4, 5]),
        );
        assert_ser_deser(
            &SpStorageData(vec![1, 2, 3, 4, 5]),
            &StorageData(vec![1, 2, 3, 4, 5]),
        );
        assert_ser_deser(
            &SpStorageChangeSet {
                block: 1u64,
                changes: vec![(SpStorageKey(vec![1]), Some(SpStorageData(vec![2])))],
            },
            &StorageChangeSet {
                block: 1u64,
                changes: vec![(StorageKey(vec![1]), Some(StorageData(vec![2])))],
            },
        );
    }
}
