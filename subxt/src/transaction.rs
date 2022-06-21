// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use std::task::Poll;

use crate::PhantomDataSendSync;
use codec::Decode;
use sp_runtime::traits::Hash;
pub use sp_runtime::traits::SignedExtension;

use crate::{
    client::Client,
    error::{
        BasicError,
        Error,
        HasModuleError,
        ModuleError,
        RuntimeError,
        TransactionError,
    },
    events::{
        self,
        EventDetails,
        Events,
        RawEventDetails,
    },
    rpc::SubstrateTransactionStatus,
    Config,
    Phase,
};
use derivative::Derivative;
use futures::{
    Stream,
    StreamExt,
};
use jsonrpsee::core::{
    client::Subscription as RpcSubscription,
    Error as RpcError,
};

/// This struct represents a subscription to the progress of some transaction, and is
/// returned from [`crate::SubmittableExtrinsic::sign_and_submit_then_watch()`].
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct TransactionProgress<'client, T: Config, E, Evs> {
    sub: Option<RpcSubscription<SubstrateTransactionStatus<T::Hash, T::Hash>>>,
    ext_hash: T::Hash,
    client: &'client Client<T>,
    _error: PhantomDataSendSync<(E, Evs)>,
}

// The above type is not `Unpin` by default unless the generic param `T` is,
// so we manually make it clear that Unpin is actually fine regardless of `T`
// (we don't care if this moves around in memory while it's "pinned").
impl<'client, T: Config, E, Evs> Unpin for TransactionProgress<'client, T, E, Evs> {}

impl<'client, T: Config, E: Decode + HasModuleError, Evs: Decode>
    TransactionProgress<'client, T, E, Evs>
{
    /// Instantiate a new [`TransactionProgress`] from a custom subscription.
    pub fn new(
        sub: RpcSubscription<SubstrateTransactionStatus<T::Hash, T::Hash>>,
        client: &'client Client<T>,
        ext_hash: T::Hash,
    ) -> Self {
        Self {
            sub: Some(sub),
            client,
            ext_hash,
            _error: PhantomDataSendSync::new(),
        }
    }

    /// Return the next transaction status when it's emitted. This just delegates to the
    /// [`futures::Stream`] implementation for [`TransactionProgress`], but allows you to
    /// avoid importing that trait if you don't otherwise need it.
    pub async fn next_item(
        &mut self,
    ) -> Option<Result<TransactionStatus<'client, T, E, Evs>, BasicError>> {
        self.next().await
    }

    /// Wait for the transaction to be in a block (but not necessarily finalized), and return
    /// an [`TransactionInBlock`] instance when this happens, or an error if there was a problem
    /// waiting for this to happen.
    ///
    /// **Note:** consumes `self`. If you'd like to perform multiple actions as the state of the
    /// transaction progresses, use [`TransactionProgress::next_item()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid` and `Usurped` are ignored, because while they
    /// may well indicate with some probability that the transaction will not make it into a block,
    /// there is no guarantee that this is true. Thus, we prefer to "play it safe" here. Use the lower
    /// level [`TransactionProgress::next_item()`] API if you'd like to handle these statuses yourself.
    pub async fn wait_for_in_block(
        mut self,
    ) -> Result<TransactionInBlock<'client, T, E, Evs>, BasicError> {
        while let Some(status) = self.next_item().await {
            match status? {
                // Finalized or otherwise in a block! Return.
                TransactionStatus::InBlock(s) | TransactionStatus::Finalized(s) => {
                    return Ok(s)
                }
                // Error scenarios; return the error.
                TransactionStatus::FinalityTimeout(_) => {
                    return Err(TransactionError::FinalitySubscriptionTimeout.into())
                }
                // Ignore anything else and wait for next status event:
                _ => continue,
            }
        }
        Err(RpcError::Custom("RPC subscription dropped".into()).into())
    }

    /// Wait for the transaction to be finalized, and return a [`TransactionInBlock`]
    /// instance when it is, or an error if there was a problem waiting for finalization.
    ///
    /// **Note:** consumes `self`. If you'd like to perform multiple actions as the state of the
    /// transaction progresses, use [`TransactionProgress::next_item()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid` and `Usurped` are ignored, because while they
    /// may well indicate with some probability that the transaction will not make it into a block,
    /// there is no guarantee that this is true. Thus, we prefer to "play it safe" here. Use the lower
    /// level [`TransactionProgress::next_item()`] API if you'd like to handle these statuses yourself.
    pub async fn wait_for_finalized(
        mut self,
    ) -> Result<TransactionInBlock<'client, T, E, Evs>, BasicError> {
        while let Some(status) = self.next_item().await {
            match status? {
                // Finalized! Return.
                TransactionStatus::Finalized(s) => return Ok(s),
                // Error scenarios; return the error.
                TransactionStatus::FinalityTimeout(_) => {
                    return Err(TransactionError::FinalitySubscriptionTimeout.into())
                }
                // Ignore and wait for next status event:
                _ => continue,
            }
        }
        Err(RpcError::Custom("RPC subscription dropped".into()).into())
    }

    /// Wait for the transaction to be finalized, and for the transaction events to indicate
    /// that the transaction was successful. Returns the events associated with the transaction,
    /// as well as a couple of other details (block hash and extrinsic hash).
    ///
    /// **Note:** consumes self. If you'd like to perform multiple actions as progress is made,
    /// use [`TransactionProgress::next_item()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid` and `Usurped` are ignored, because while they
    /// may well indicate with some probability that the transaction will not make it into a block,
    /// there is no guarantee that this is true. Thus, we prefer to "play it safe" here. Use the lower
    /// level [`TransactionProgress::next_item()`] API if you'd like to handle these statuses yourself.
    pub async fn wait_for_finalized_success(
        self,
    ) -> Result<TransactionEvents<T, Evs>, Error<E>> {
        let evs = self.wait_for_finalized().await?.wait_for_success().await?;
        Ok(evs)
    }
}

impl<'client, T: Config, E: Decode + HasModuleError, Evs: Decode> Stream
    for TransactionProgress<'client, T, E, Evs>
{
    type Item = Result<TransactionStatus<'client, T, E, Evs>, BasicError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let sub = match self.sub.as_mut() {
            Some(sub) => sub,
            None => return Poll::Ready(None),
        };

        sub.poll_next_unpin(cx)
            .map_err(|e| e.into())
            .map_ok(|status| {
                match status {
                    SubstrateTransactionStatus::Future => TransactionStatus::Future,
                    SubstrateTransactionStatus::Ready => TransactionStatus::Ready,
                    SubstrateTransactionStatus::Broadcast(peers) => {
                        TransactionStatus::Broadcast(peers)
                    }
                    SubstrateTransactionStatus::InBlock(hash) => {
                        TransactionStatus::InBlock(TransactionInBlock::new(
                            hash,
                            self.ext_hash,
                            self.client,
                        ))
                    }
                    SubstrateTransactionStatus::Retracted(hash) => {
                        TransactionStatus::Retracted(hash)
                    }
                    SubstrateTransactionStatus::Usurped(hash) => {
                        TransactionStatus::Usurped(hash)
                    }
                    SubstrateTransactionStatus::Dropped => TransactionStatus::Dropped,
                    SubstrateTransactionStatus::Invalid => TransactionStatus::Invalid,
                    // Only the following statuses are actually considered "final" (see the substrate
                    // docs on `TransactionStatus`). Basically, either the transaction makes it into a
                    // block, or we eventually give up on waiting for it to make it into a block.
                    // Even `Dropped`/`Invalid`/`Usurped` transactions might make it into a block eventually.
                    //
                    // As an example, a transaction that is `Invalid` on one node due to having the wrong
                    // nonce might still be valid on some fork on another node which ends up being finalized.
                    // Equally, a transaction `Dropped` from one node may still be in the transaction pool,
                    // and make it into a block, on another node. Likewise with `Usurped`.
                    SubstrateTransactionStatus::FinalityTimeout(hash) => {
                        self.sub = None;
                        TransactionStatus::FinalityTimeout(hash)
                    }
                    SubstrateTransactionStatus::Finalized(hash) => {
                        self.sub = None;
                        TransactionStatus::Finalized(TransactionInBlock::new(
                            hash,
                            self.ext_hash,
                            self.client,
                        ))
                    }
                }
            })
    }
}

//* Dev note: The below is adapted from the substrate docs on `TransactionStatus`, which this
//* enum was adapted from (and which is an exact copy of `SubstrateTransactionStatus` in this crate).
//* Note that the number of finality watchers is, at the time of writing, found in the constant
//* `MAX_FINALITY_WATCHERS` in the `sc_transaction_pool` crate.
//*
/// Possible transaction statuses returned from our [`TransactionProgress::next_item()`] call.
///
/// These status events can be grouped based on their kinds as:
///
/// 1. Entering/Moving within the pool:
///    - `Future`
///    - `Ready`
/// 2. Inside `Ready` queue:
///    - `Broadcast`
/// 3. Leaving the pool:
///    - `InBlock`
///    - `Invalid`
///    - `Usurped`
///    - `Dropped`
/// 4. Re-entering the pool:
///    - `Retracted`
/// 5. Block finalized:
///    - `Finalized`
///    - `FinalityTimeout`
///
/// The events will always be received in the order described above, however
/// there might be cases where transactions alternate between `Future` and `Ready`
/// pool, and are `Broadcast` in the meantime.
///
/// Note that there are conditions that may cause transactions to reappear in the pool:
///
/// 1. Due to possible forks, the transaction that ends up being included
///    in one block may later re-enter the pool or be marked as invalid.
/// 2. A transaction that is `Dropped` at one point may later re-enter the pool if
///    some other transactions are removed.
/// 3. `Invalid` transactions may become valid at some point in the future.
///    (Note that runtimes are encouraged to use `UnknownValidity` to inform the
///    pool about such cases).
/// 4. `Retracted` transactions might be included in a future block.
///
/// The stream is considered finished only when either the `Finalized` or `FinalityTimeout`
/// event is triggered. You are however free to unsubscribe from notifications at any point.
/// The first one will be emitted when the block in which the transaction was included gets
/// finalized. The `FinalityTimeout` event will be emitted when the block did not reach finality
/// within 512 blocks. This either indicates that finality is not available for your chain,
/// or that finality gadget is lagging behind.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub enum TransactionStatus<'client, T: Config, E: Decode, Evs: Decode> {
    /// The transaction is part of the "future" queue.
    Future,
    /// The transaction is part of the "ready" queue.
    Ready,
    /// The transaction has been broadcast to the given peers.
    Broadcast(Vec<String>),
    /// The transaction has been included in a block with given hash.
    InBlock(TransactionInBlock<'client, T, E, Evs>),
    /// The block this transaction was included in has been retracted,
    /// probably because it did not make it onto the blocks which were
    /// finalized.
    Retracted(T::Hash),
    /// A block containing the transaction did not reach finality within 512
    /// blocks, and so the subscription has ended.
    FinalityTimeout(T::Hash),
    /// The transaction has been finalized by a finality-gadget, e.g GRANDPA.
    Finalized(TransactionInBlock<'client, T, E, Evs>),
    /// The transaction has been replaced in the pool by another transaction
    /// that provides the same tags. (e.g. same (sender, nonce)).
    Usurped(T::Hash),
    /// The transaction has been dropped from the pool because of the limit.
    Dropped,
    /// The transaction is no longer valid in the current state.
    Invalid,
}

impl<'client, T: Config, E: Decode, Evs: Decode> TransactionStatus<'client, T, E, Evs> {
    /// A convenience method to return the `Finalized` details. Returns
    /// [`None`] if the enum variant is not [`TransactionStatus::Finalized`].
    pub fn as_finalized(&self) -> Option<&TransactionInBlock<'client, T, E, Evs>> {
        match self {
            Self::Finalized(val) => Some(val),
            _ => None,
        }
    }

    /// A convenience method to return the `InBlock` details. Returns
    /// [`None`] if the enum variant is not [`TransactionStatus::InBlock`].
    pub fn as_in_block(&self) -> Option<&TransactionInBlock<'client, T, E, Evs>> {
        match self {
            Self::InBlock(val) => Some(val),
            _ => None,
        }
    }
}

/// This struct represents a transaction that has made it into a block.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct TransactionInBlock<'client, T: Config, E: Decode, Evs: Decode> {
    block_hash: T::Hash,
    ext_hash: T::Hash,
    client: &'client Client<T>,
    _error: PhantomDataSendSync<(E, Evs)>,
}

impl<'client, T: Config, E: Decode + HasModuleError, Evs: Decode>
    TransactionInBlock<'client, T, E, Evs>
{
    pub(crate) fn new(
        block_hash: T::Hash,
        ext_hash: T::Hash,
        client: &'client Client<T>,
    ) -> Self {
        Self {
            block_hash,
            ext_hash,
            client,
            _error: PhantomDataSendSync::new(),
        }
    }

    /// Return the hash of the block that the transaction has made it into.
    pub fn block_hash(&self) -> T::Hash {
        self.block_hash
    }

    /// Return the hash of the extrinsic that was submitted.
    pub fn extrinsic_hash(&self) -> T::Hash {
        self.ext_hash
    }

    /// Fetch the events associated with this transaction. If the transaction
    /// was successful (ie no `ExtrinsicFailed`) events were found, then we return
    /// the events associated with it. If the transaction was not successful, or
    /// something else went wrong, we return an error.
    ///
    /// **Note:** If multiple `ExtrinsicFailed` errors are returned (for instance
    /// because a pallet chooses to emit one as an event, which is considered
    /// abnormal behaviour), it is not specified which of the errors is returned here.
    /// You can use [`TransactionInBlock::fetch_events`] instead if you'd like to
    /// work with multiple "error" events.
    ///
    /// **Note:** This has to download block details from the node and decode events
    /// from them.
    pub async fn wait_for_success(&self) -> Result<TransactionEvents<T, Evs>, Error<E>> {
        let events = self.fetch_events().await?;

        // Try to find any errors; return the first one we encounter.
        for ev in events.iter_raw() {
            let ev = ev?;
            if &ev.pallet == "System" && &ev.variant == "ExtrinsicFailed" {
                let dispatch_error = E::decode(&mut &*ev.bytes)?;
                if let Some(error_data) = dispatch_error.module_error_data() {
                    // Error index is utilized as the first byte from the error array.
                    let locked_metadata = self.client.metadata();
                    let metadata = locked_metadata.read();
                    let details = metadata
                        .error(error_data.pallet_index, error_data.error_index())?;
                    return Err(Error::Module(ModuleError {
                        pallet: details.pallet().to_string(),
                        error: details.error().to_string(),
                        description: details.description().to_vec(),
                        error_data,
                    }))
                } else {
                    return Err(Error::Runtime(RuntimeError(dispatch_error)))
                }
            }
        }

        Ok(events)
    }

    /// Fetch all of the events associated with this transaction. This succeeds whether
    /// the transaction was a success or not; it's up to you to handle the error and
    /// success events however you prefer.
    ///
    /// **Note:** This has to download block details from the node and decode events
    /// from them.
    pub async fn fetch_events(&self) -> Result<TransactionEvents<T, Evs>, BasicError> {
        let block = self
            .client
            .rpc()
            .block(Some(self.block_hash))
            .await?
            .ok_or(BasicError::Transaction(TransactionError::BlockHashNotFound))?;

        let extrinsic_idx = block.block.extrinsics
            .iter()
            .position(|ext| {
                let hash = T::Hashing::hash_of(ext);
                hash == self.ext_hash
            })
            // If we successfully obtain the block hash we think contains our
            // extrinsic, the extrinsic should be in there somewhere..
            .ok_or(BasicError::Transaction(TransactionError::BlockHashNotFound))?;

        let events = events::at::<T, Evs>(self.client, self.block_hash).await?;

        Ok(TransactionEvents {
            ext_hash: self.ext_hash,
            ext_idx: extrinsic_idx as u32,
            events,
        })
    }
}

/// This represents the events related to our transaction.
/// We can iterate over the events, or look for a specific one.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct TransactionEvents<T: Config, Evs: Decode> {
    ext_hash: T::Hash,
    ext_idx: u32,
    events: Events<T, Evs>,
}

impl<T: Config, Evs: Decode> TransactionEvents<T, Evs> {
    /// Return the hash of the block that the transaction has made it into.
    pub fn block_hash(&self) -> T::Hash {
        self.events.block_hash()
    }

    /// Return the hash of the extrinsic.
    pub fn extrinsic_hash(&self) -> T::Hash {
        self.ext_hash
    }

    /// Return all of the events in the block that the transaction made it into.
    pub fn all_events_in_block(&self) -> &events::Events<T, Evs> {
        &self.events
    }

    /// Iterate over the statically decoded events associated with this transaction.
    ///
    /// This works in the same way that [`events::Events::iter()`] does, with the
    /// exception that it filters out events not related to the submitted extrinsic.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Result<EventDetails<Evs>, BasicError>> + '_ {
        self.events.iter().filter(|ev| {
            ev.as_ref()
                .map(|ev| ev.phase == Phase::ApplyExtrinsic(self.ext_idx))
                .unwrap_or(true) // Keep any errors
        })
    }

    /// Iterate over all of the raw events associated with this transaction.
    ///
    /// This works in the same way that [`events::Events::iter_raw()`] does, with the
    /// exception that it filters out events not related to the submitted extrinsic.
    pub fn iter_raw(
        &self,
    ) -> impl Iterator<Item = Result<RawEventDetails, BasicError>> + '_ {
        self.events.iter_raw().filter(|ev| {
            ev.as_ref()
                .map(|ev| ev.phase == Phase::ApplyExtrinsic(self.ext_idx))
                .unwrap_or(true) // Keep any errors.
        })
    }

    /// Find all of the transaction events matching the event type provided as a generic parameter.
    ///
    /// This works in the same way that [`events::Events::find()`] does, with the
    /// exception that it filters out events not related to the submitted extrinsic.
    pub fn find<Ev: crate::Event>(
        &self,
    ) -> impl Iterator<Item = Result<Ev, BasicError>> + '_ {
        self.iter_raw().filter_map(|ev| {
            ev.and_then(|ev| ev.as_event::<Ev>().map_err(Into::into))
                .transpose()
        })
    }

    /// Iterate through the transaction events using metadata to dynamically decode and skip
    /// them, and return the first event found which decodes to the provided `Ev` type.
    ///
    /// This works in the same way that [`events::Events::find_first()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn find_first<Ev: crate::Event>(&self) -> Result<Option<Ev>, BasicError> {
        self.find::<Ev>().next().transpose()
    }

    /// Find an event in those associated with this transaction. Returns true if it was found.
    ///
    /// This works in the same way that [`events::Events::has()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn has<Ev: crate::Event>(&self) -> Result<bool, BasicError> {
        Ok(self.find::<Ev>().next().transpose()?.is_some())
    }
}
