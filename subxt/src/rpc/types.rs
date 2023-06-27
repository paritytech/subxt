// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types sent to/from the Substrate RPC interface.

use crate::{metadata::Metadata, Config};
use codec::{Decode, Encode};
use primitive_types::U256;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Subscription types are returned from some calls, so expose it with the rest of the returned types.
pub use super::rpc_client::Subscription;

/// An error dry running an extrinsic.
#[derive(Debug, PartialEq, Eq)]
pub enum DryRunResult {
    /// The transaction could be included in the block and executed.
    Success,
    /// The transaction could be included in the block, but the call failed to dispatch.
    DispatchError(crate::error::DispatchError),
    /// The transaction could not be included in the block.
    TransactionValidityError,
}

/// The bytes representing an error dry running an extrinsic.
pub struct DryRunResultBytes(pub Vec<u8>);

impl DryRunResultBytes {
    /// Attempt to decode the error bytes into a [`DryRunResult`] using the provided [`Metadata`].
    pub fn into_dry_run_result(self, metadata: &Metadata) -> Result<DryRunResult, crate::Error> {
        // dryRun returns an ApplyExtrinsicResult, which is basically a
        // `Result<Result<(), DispatchError>, TransactionValidityError>`.
        let bytes = self.0;
        if bytes[0] == 0 && bytes[1] == 0 {
            // Ok(Ok(())); transaction is valid and executed ok
            Ok(DryRunResult::Success)
        } else if bytes[0] == 0 && bytes[1] == 1 {
            // Ok(Err(dispatch_error)); transaction is valid but execution failed
            let dispatch_error =
                crate::error::DispatchError::decode_from(&bytes[2..], metadata.clone())?;
            Ok(DryRunResult::DispatchError(dispatch_error))
        } else if bytes[0] == 1 {
            // Err(transaction_error); some transaction validity error (we ignore the details at the moment)
            Ok(DryRunResult::TransactionValidityError)
        } else {
            // unable to decode the bytes; they aren't what we expect.
            Err(crate::Error::Unknown(bytes))
        }
    }
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
            .map_err(|e| ::serde::de::Error::custom(format!("Decode error: {e}")))?;
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
    Serialize, Deserialize, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Encode, Decode, Debug,
)]
pub struct StorageKey(#[serde(with = "impl_serde::serialize")] pub Vec<u8>);
impl AsRef<[u8]> for StorageKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Storage data.
#[derive(
    Serialize, Deserialize, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Encode, Decode, Debug,
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

/// The operation could not be processed due to an error.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvent {
    /// Reason of the error.
    pub error: String,
}

/// The runtime specification of the current block.
///
/// This event is generated for:
///   - the first announced block by the follow subscription
///   - blocks that suffered a change in runtime compared with their parents
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersionEvent {
    /// The runtime version.
    pub spec: RuntimeVersion,
}

/// The runtime event generated if the `follow` subscription
/// has set the `runtime_updates` flag.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RuntimeEvent {
    /// The runtime version of this block.
    Valid(RuntimeVersionEvent),
    /// The runtime could not be obtained due to an error.
    Invalid(ErrorEvent),
}

/// Contain information about the latest finalized block.
///
/// # Note
///
/// This is the first event generated by the `follow` subscription
/// and is submitted only once.
///
/// If the `runtime_updates` flag is set, then this event contains
/// the `RuntimeEvent`, otherwise the `RuntimeEvent` is not present.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Initialized<Hash> {
    /// The hash of the latest finalized block.
    pub finalized_block_hash: Hash,
    /// The runtime version of the finalized block.
    ///
    /// # Note
    ///
    /// This is present only if the `runtime_updates` flag is set for
    /// the `follow` subscription.
    pub finalized_block_runtime: Option<RuntimeEvent>,
}

/// Indicate a new non-finalized block.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBlock<Hash> {
    /// The hash of the new block.
    pub block_hash: Hash,
    /// The parent hash of the new block.
    pub parent_block_hash: Hash,
    /// The runtime version of the new block.
    ///
    /// # Note
    ///
    /// This is present only if the `runtime_updates` flag is set for
    /// the `follow` subscription.
    pub new_runtime: Option<RuntimeEvent>,
}

/// Indicate the block hash of the new best block.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestBlockChanged<Hash> {
    /// The block hash of the new best block.
    pub best_block_hash: Hash,
}

/// Indicate the finalized and pruned block hashes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finalized<Hash> {
    /// Block hashes that are finalized.
    pub finalized_block_hashes: Vec<Hash>,
    /// Block hashes that are pruned (removed).
    pub pruned_block_hashes: Vec<Hash>,
}

/// The event generated by the `chainHead_follow` method.
///
/// The events are generated in the following order:
/// 1. Initialized - generated only once to signal the
///      latest finalized block
/// 2. NewBlock - a new block was added.
/// 3. BestBlockChanged - indicate that the best block
///      is now the one from this event. The block was
///      announced priorly with the `NewBlock` event.
/// 4. Finalized - State the finalized and pruned blocks.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
pub enum FollowEvent<Hash> {
    /// The latest finalized block.
    ///
    /// This event is generated only once.
    Initialized(Initialized<Hash>),
    /// A new non-finalized block was added.
    NewBlock(NewBlock<Hash>),
    /// The best block of the chain.
    BestBlockChanged(BestBlockChanged<Hash>),
    /// A list of finalized and pruned blocks.
    Finalized(Finalized<Hash>),
    /// The subscription is dropped and no further events
    /// will be generated.
    Stop,
}

/// The result of a chain head method.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainHeadResult<T> {
    /// Result of the method.
    pub result: T,
}

/// The event generated by the body / call / storage methods.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
pub enum ChainHeadEvent<T> {
    /// The request completed successfully.
    Done(ChainHeadResult<T>),
    /// The resources requested are inaccessible.
    ///
    /// Resubmitting the request later might succeed.
    Inaccessible(ErrorEvent),
    /// An error occurred. This is definitive.
    Error(ErrorEvent),
    /// The provided subscription ID is stale or invalid.
    Disjoint,
}

/// The transaction was broadcasted to a number of peers.
///
/// # Note
///
/// The RPC does not guarantee that the peers have received the
/// transaction.
///
/// When the number of peers is zero, the event guarantees that
/// shutting down the local node will lead to the transaction
/// not being included in the chain.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBroadcasted {
    /// The number of peers the transaction was broadcasted to.
    #[serde(with = "as_string")]
    pub num_peers: usize,
}

/// The transaction was included in a block of the chain.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBlock<Hash> {
    /// The hash of the block the transaction was included into.
    pub hash: Hash,
    /// The index (zero-based) of the transaction within the body of the block.
    #[serde(with = "as_string")]
    pub index: usize,
}

/// The transaction could not be processed due to an error.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionError {
    /// Reason of the error.
    pub error: String,
}

/// The transaction was dropped because of exceeding limits.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDropped {
    /// True if the transaction was broadcasted to other peers and
    /// may still be included in the block.
    pub broadcasted: bool,
    /// Reason of the event.
    pub error: String,
}

/// Possible transaction status events.
///
/// The status events can be grouped based on their kinds as:
///
/// 1. Runtime validated the transaction:
///             - `Validated`
///
/// 2. Inside the `Ready` queue:
///             - `Broadcast`
///
/// 3. Leaving the pool:
///             - `BestChainBlockIncluded`
///             - `Invalid`
///
/// 4. Block finalized:
///             - `Finalized`
///
/// 5. At any time:
///             - `Dropped`
///             - `Error`
///
/// The subscription's stream is considered finished whenever the following events are
/// received: `Finalized`, `Error`, `Invalid` or `Dropped`. However, the user is allowed
/// to unsubscribe at any moment.
#[derive(Debug, Clone, PartialEq, Deserialize)]
// We need to manually specify the trait bounds for the `Hash` trait to ensure `into` and
// `from` still work.
#[serde(bound(deserialize = "Hash: Deserialize<'de> + Clone"))]
#[serde(from = "TransactionEventIR<Hash>")]
pub enum TransactionEvent<Hash> {
    /// The transaction was validated by the runtime.
    Validated,
    /// The transaction was broadcasted to a number of peers.
    Broadcasted(TransactionBroadcasted),
    /// The transaction was included in a best block of the chain.
    ///
    /// # Note
    ///
    /// This may contain `None` if the block is no longer a best
    /// block of the chain.
    BestChainBlockIncluded(Option<TransactionBlock<Hash>>),
    /// The transaction was included in a finalized block.
    Finalized(TransactionBlock<Hash>),
    /// The transaction could not be processed due to an error.
    Error(TransactionError),
    /// The transaction is marked as invalid.
    Invalid(TransactionError),
    /// The client was not capable of keeping track of this transaction.
    Dropped(TransactionDropped),
}

/// Intermediate representation (IR) for the transaction events
/// that handles block events only.
///
/// The block events require a JSON compatible interpretation similar to:
///
/// ```json
/// { event: "EVENT", block: { hash: "0xFF", index: 0 } }
/// ```
///
/// This IR is introduced to circumvent that the block events need to
/// be serialized/deserialized with "tag" and "content", while other
/// events only require "tag".
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event", content = "block")]
enum TransactionEventBlockIR<Hash> {
    /// The transaction was included in the best block of the chain.
    BestChainBlockIncluded(Option<TransactionBlock<Hash>>),
    /// The transaction was included in a finalized block of the chain.
    Finalized(TransactionBlock<Hash>),
}

/// Intermediate representation (IR) for the transaction events
/// that handles non-block events only.
///
/// The non-block events require a JSON compatible interpretation similar to:
///
/// ```json
/// { event: "EVENT", num_peers: 0 }
/// ```
///
/// This IR is introduced to circumvent that the block events need to
/// be serialized/deserialized with "tag" and "content", while other
/// events only require "tag".
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
enum TransactionEventNonBlockIR {
    Validated,
    Broadcasted(TransactionBroadcasted),
    Error(TransactionError),
    Invalid(TransactionError),
    Dropped(TransactionDropped),
}

/// Intermediate representation (IR) used for serialization/deserialization of the
/// [`TransactionEvent`] in a JSON compatible format.
///
/// Serde cannot mix `#[serde(tag = "event")]` with `#[serde(tag = "event", content = "block")]`
/// for specific enum variants. Therefore, this IR is introduced to circumvent this
/// restriction, while exposing a simplified [`TransactionEvent`] for users of the
/// rust ecosystem.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(bound(deserialize = "Hash: Deserialize<'de>"))]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
enum TransactionEventIR<Hash> {
    Block(TransactionEventBlockIR<Hash>),
    NonBlock(TransactionEventNonBlockIR),
}

impl<Hash> From<TransactionEvent<Hash>> for TransactionEventIR<Hash> {
    fn from(value: TransactionEvent<Hash>) -> Self {
        match value {
            TransactionEvent::Validated => {
                TransactionEventIR::NonBlock(TransactionEventNonBlockIR::Validated)
            }
            TransactionEvent::Broadcasted(event) => {
                TransactionEventIR::NonBlock(TransactionEventNonBlockIR::Broadcasted(event))
            }
            TransactionEvent::BestChainBlockIncluded(event) => {
                TransactionEventIR::Block(TransactionEventBlockIR::BestChainBlockIncluded(event))
            }
            TransactionEvent::Finalized(event) => {
                TransactionEventIR::Block(TransactionEventBlockIR::Finalized(event))
            }
            TransactionEvent::Error(event) => {
                TransactionEventIR::NonBlock(TransactionEventNonBlockIR::Error(event))
            }
            TransactionEvent::Invalid(event) => {
                TransactionEventIR::NonBlock(TransactionEventNonBlockIR::Invalid(event))
            }
            TransactionEvent::Dropped(event) => {
                TransactionEventIR::NonBlock(TransactionEventNonBlockIR::Dropped(event))
            }
        }
    }
}

impl<Hash> From<TransactionEventIR<Hash>> for TransactionEvent<Hash> {
    fn from(value: TransactionEventIR<Hash>) -> Self {
        match value {
            TransactionEventIR::NonBlock(status) => match status {
                TransactionEventNonBlockIR::Validated => TransactionEvent::Validated,
                TransactionEventNonBlockIR::Broadcasted(event) => {
                    TransactionEvent::Broadcasted(event)
                }
                TransactionEventNonBlockIR::Error(event) => TransactionEvent::Error(event),
                TransactionEventNonBlockIR::Invalid(event) => TransactionEvent::Invalid(event),
                TransactionEventNonBlockIR::Dropped(event) => TransactionEvent::Dropped(event),
            },
            TransactionEventIR::Block(block) => match block {
                TransactionEventBlockIR::Finalized(event) => TransactionEvent::Finalized(event),
                TransactionEventBlockIR::BestChainBlockIncluded(event) => {
                    TransactionEvent::BestChainBlockIncluded(event)
                }
            },
        }
    }
}

/// Serialize and deserialize helper as string.
mod as_string {
    use super::*;
    use serde::Deserializer;

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<usize, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e| serde::de::Error::custom(format!("Parsing failed: {e}")))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// A util function to assert the result of serialization and deserialization is the same.
    pub fn assert_deser<T>(s: &str, expected: T)
    where
        T: std::fmt::Debug + serde::ser::Serialize + serde::de::DeserializeOwned + PartialEq,
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

        let json = serde_json::to_string(&substrate_runtime_version).expect("serializing failed");
        let val: RuntimeVersion = serde_json::from_str(&json).expect("deserializing failed");

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
            StorageChangeSet as SpStorageChangeSet, StorageData as SpStorageData,
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
