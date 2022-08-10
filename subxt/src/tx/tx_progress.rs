// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing extrinsics/transactions that have been submitted to a node.

use std::task::Poll;

use crate::{
    client::OnlineClientT,
    error::{
        DispatchError,
        Error,
        TransactionError,
    },
    events::{
        self,
        EventDetails,
        Events,
        EventsClient,
        Phase,
        StaticEvent,
    },
    rpc::SubstrateTxStatus,
    Config,
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
use sp_runtime::traits::Hash;

pub use sp_runtime::traits::SignedExtension;

/// This struct represents a subscription to the progress of some transaction.
#[derive(Derivative)]
#[derivative(Debug(bound = "C: std::fmt::Debug"))]
pub struct TxProgress<T: Config, C> {
    sub: Option<RpcSubscription<SubstrateTxStatus<T::Hash, T::Hash>>>,
    ext_hash: T::Hash,
    client: C,
}

// The above type is not `Unpin` by default unless the generic param `T` is,
// so we manually make it clear that Unpin is actually fine regardless of `T`
// (we don't care if this moves around in memory while it's "pinned").
impl<T: Config, C> Unpin for TxProgress<T, C> {}

impl<T: Config, C> TxProgress<T, C> {
    /// Instantiate a new [`TxProgress`] from a custom subscription.
    pub fn new(
        sub: RpcSubscription<SubstrateTxStatus<T::Hash, T::Hash>>,
        client: C,
        ext_hash: T::Hash,
    ) -> Self {
        Self {
            sub: Some(sub),
            client,
            ext_hash,
        }
    }

    /// Return the hash of the extrinsic.
    pub fn extrinsic_hash(&self) -> T::Hash {
        self.ext_hash
    }
}

impl<T: Config, C: OnlineClientT<T>> TxProgress<T, C> {
    /// Return the next transaction status when it's emitted. This just delegates to the
    /// [`futures::Stream`] implementation for [`TxProgress`], but allows you to
    /// avoid importing that trait if you don't otherwise need it.
    pub async fn next_item(&mut self) -> Option<Result<TxStatus<T, C>, Error>> {
        self.next().await
    }

    /// Wait for the transaction to be in a block (but not necessarily finalized), and return
    /// an [`TxInBlock`] instance when this happens, or an error if there was a problem
    /// waiting for this to happen.
    ///
    /// **Note:** consumes `self`. If you'd like to perform multiple actions as the state of the
    /// transaction progresses, use [`TxProgress::next_item()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid` and `Usurped` are ignored, because while they
    /// may well indicate with some probability that the transaction will not make it into a block,
    /// there is no guarantee that this is true. Thus, we prefer to "play it safe" here. Use the lower
    /// level [`TxProgress::next_item()`] API if you'd like to handle these statuses yourself.
    pub async fn wait_for_in_block(mut self) -> Result<TxInBlock<T, C>, Error> {
        while let Some(status) = self.next_item().await {
            match status? {
                // Finalized or otherwise in a block! Return.
                TxStatus::InBlock(s) | TxStatus::Finalized(s) => return Ok(s),
                // Error scenarios; return the error.
                TxStatus::FinalityTimeout(_) => {
                    return Err(TransactionError::FinalitySubscriptionTimeout.into())
                }
                // Ignore anything else and wait for next status event:
                _ => continue,
            }
        }
        Err(RpcError::Custom("RPC subscription dropped".into()).into())
    }

    /// Wait for the transaction to be finalized, and return a [`TxInBlock`]
    /// instance when it is, or an error if there was a problem waiting for finalization.
    ///
    /// **Note:** consumes `self`. If you'd like to perform multiple actions as the state of the
    /// transaction progresses, use [`TxProgress::next_item()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid` and `Usurped` are ignored, because while they
    /// may well indicate with some probability that the transaction will not make it into a block,
    /// there is no guarantee that this is true. Thus, we prefer to "play it safe" here. Use the lower
    /// level [`TxProgress::next_item()`] API if you'd like to handle these statuses yourself.
    pub async fn wait_for_finalized(mut self) -> Result<TxInBlock<T, C>, Error> {
        while let Some(status) = self.next_item().await {
            match status? {
                // Finalized! Return.
                TxStatus::Finalized(s) => return Ok(s),
                // Error scenarios; return the error.
                TxStatus::FinalityTimeout(_) => {
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
    /// use [`TxProgress::next_item()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid` and `Usurped` are ignored, because while they
    /// may well indicate with some probability that the transaction will not make it into a block,
    /// there is no guarantee that this is true. Thus, we prefer to "play it safe" here. Use the lower
    /// level [`TxProgress::next_item()`] API if you'd like to handle these statuses yourself.
    pub async fn wait_for_finalized_success(self) -> Result<TxEvents<T>, Error> {
        let evs = self.wait_for_finalized().await?.wait_for_success().await?;
        Ok(evs)
    }
}

impl<T: Config, C: OnlineClientT<T>> Stream for TxProgress<T, C> {
    type Item = Result<TxStatus<T, C>, Error>;

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
                    SubstrateTxStatus::Future => TxStatus::Future,
                    SubstrateTxStatus::Ready => TxStatus::Ready,
                    SubstrateTxStatus::Broadcast(peers) => TxStatus::Broadcast(peers),
                    SubstrateTxStatus::InBlock(hash) => {
                        TxStatus::InBlock(TxInBlock::new(
                            hash,
                            self.ext_hash,
                            self.client.clone(),
                        ))
                    }
                    SubstrateTxStatus::Retracted(hash) => TxStatus::Retracted(hash),
                    SubstrateTxStatus::Usurped(hash) => TxStatus::Usurped(hash),
                    SubstrateTxStatus::Dropped => TxStatus::Dropped,
                    SubstrateTxStatus::Invalid => TxStatus::Invalid,
                    // Only the following statuses are actually considered "final" (see the substrate
                    // docs on `TxStatus`). Basically, either the transaction makes it into a
                    // block, or we eventually give up on waiting for it to make it into a block.
                    // Even `Dropped`/`Invalid`/`Usurped` transactions might make it into a block eventually.
                    //
                    // As an example, a transaction that is `Invalid` on one node due to having the wrong
                    // nonce might still be valid on some fork on another node which ends up being finalized.
                    // Equally, a transaction `Dropped` from one node may still be in the transaction pool,
                    // and make it into a block, on another node. Likewise with `Usurped`.
                    SubstrateTxStatus::FinalityTimeout(hash) => {
                        self.sub = None;
                        TxStatus::FinalityTimeout(hash)
                    }
                    SubstrateTxStatus::Finalized(hash) => {
                        self.sub = None;
                        TxStatus::Finalized(TxInBlock::new(
                            hash,
                            self.ext_hash,
                            self.client.clone(),
                        ))
                    }
                }
            })
    }
}

//* Dev note: The below is adapted from the substrate docs on `TxStatus`, which this
//* enum was adapted from (and which is an exact copy of `SubstrateTxStatus` in this crate).
//* Note that the number of finality watchers is, at the time of writing, found in the constant
//* `MAX_FINALITY_WATCHERS` in the `sc_transaction_pool` crate.
//*
/// Possible transaction statuses returned from our [`TxProgress::next_item()`] call.
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
#[derivative(Debug(bound = "C: std::fmt::Debug"))]
pub enum TxStatus<T: Config, C> {
    /// The transaction is part of the "future" queue.
    Future,
    /// The transaction is part of the "ready" queue.
    Ready,
    /// The transaction has been broadcast to the given peers.
    Broadcast(Vec<String>),
    /// The transaction has been included in a block with given hash.
    InBlock(TxInBlock<T, C>),
    /// The block this transaction was included in has been retracted,
    /// probably because it did not make it onto the blocks which were
    /// finalized.
    Retracted(T::Hash),
    /// A block containing the transaction did not reach finality within 512
    /// blocks, and so the subscription has ended.
    FinalityTimeout(T::Hash),
    /// The transaction has been finalized by a finality-gadget, e.g GRANDPA.
    Finalized(TxInBlock<T, C>),
    /// The transaction has been replaced in the pool by another transaction
    /// that provides the same tags. (e.g. same (sender, nonce)).
    Usurped(T::Hash),
    /// The transaction has been dropped from the pool because of the limit.
    Dropped,
    /// The transaction is no longer valid in the current state.
    Invalid,
}

impl<T: Config, C> TxStatus<T, C> {
    /// A convenience method to return the `Finalized` details. Returns
    /// [`None`] if the enum variant is not [`TxStatus::Finalized`].
    pub fn as_finalized(&self) -> Option<&TxInBlock<T, C>> {
        match self {
            Self::Finalized(val) => Some(val),
            _ => None,
        }
    }

    /// A convenience method to return the `InBlock` details. Returns
    /// [`None`] if the enum variant is not [`TxStatus::InBlock`].
    pub fn as_in_block(&self) -> Option<&TxInBlock<T, C>> {
        match self {
            Self::InBlock(val) => Some(val),
            _ => None,
        }
    }
}

/// This struct represents a transaction that has made it into a block.
#[derive(Derivative)]
#[derivative(Debug(bound = "C: std::fmt::Debug"))]
pub struct TxInBlock<T: Config, C> {
    block_hash: T::Hash,
    ext_hash: T::Hash,
    client: C,
}

impl<T: Config, C: OnlineClientT<T>> TxInBlock<T, C> {
    pub(crate) fn new(block_hash: T::Hash, ext_hash: T::Hash, client: C) -> Self {
        Self {
            block_hash,
            ext_hash,
            client,
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
    /// You can use [`TxInBlock::fetch_events`] instead if you'd like to
    /// work with multiple "error" events.
    ///
    /// **Note:** This has to download block details from the node and decode events
    /// from them.
    pub async fn wait_for_success(&self) -> Result<TxEvents<T>, Error> {
        let events = self.fetch_events().await?;

        // Try to find any errors; return the first one we encounter.
        for ev in events.iter() {
            let ev = ev?;
            if ev.pallet_name() == "System" && ev.variant_name() == "ExtrinsicFailed" {
                let dispatch_error =
                    DispatchError::decode_from(ev.field_bytes(), &self.client.metadata());
                return Err(dispatch_error.into())
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
    pub async fn fetch_events(&self) -> Result<TxEvents<T>, Error> {
        let block = self
            .client
            .rpc()
            .block(Some(self.block_hash))
            .await?
            .ok_or(Error::Transaction(TransactionError::BlockHashNotFound))?;

        let extrinsic_idx = block.block.extrinsics
            .iter()
            .position(|ext| {
                let hash = T::Hashing::hash_of(ext);
                hash == self.ext_hash
            })
            // If we successfully obtain the block hash we think contains our
            // extrinsic, the extrinsic should be in there somewhere..
            .ok_or(Error::Transaction(TransactionError::BlockHashNotFound))?;

        let events = EventsClient::new(self.client.clone())
            .at(Some(self.block_hash))
            .await?;

        Ok(TxEvents {
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
pub struct TxEvents<T: Config> {
    ext_hash: T::Hash,
    ext_idx: u32,
    events: Events<T>,
}

impl<T: Config> TxEvents<T> {
    /// Return the hash of the block that the transaction has made it into.
    pub fn block_hash(&self) -> T::Hash {
        self.events.block_hash()
    }

    /// Return the hash of the extrinsic.
    pub fn extrinsic_hash(&self) -> T::Hash {
        self.ext_hash
    }

    /// Return all of the events in the block that the transaction made it into.
    pub fn all_events_in_block(&self) -> &events::Events<T> {
        &self.events
    }

    /// Iterate over all of the raw events associated with this transaction.
    ///
    /// This works in the same way that [`events::Events::iter()`] does, with the
    /// exception that it filters out events not related to the submitted extrinsic.
    pub fn iter(&self) -> impl Iterator<Item = Result<EventDetails, Error>> + '_ {
        self.events.iter().filter(|ev| {
            ev.as_ref()
                .map(|ev| ev.phase() == Phase::ApplyExtrinsic(self.ext_idx))
                .unwrap_or(true) // Keep any errors.
        })
    }

    /// Find all of the transaction events matching the event type provided as a generic parameter.
    ///
    /// This works in the same way that [`events::Events::find()`] does, with the
    /// exception that it filters out events not related to the submitted extrinsic.
    pub fn find<Ev: StaticEvent>(&self) -> impl Iterator<Item = Result<Ev, Error>> + '_ {
        self.iter().filter_map(|ev| {
            ev.and_then(|ev| ev.as_event::<Ev>().map_err(Into::into))
                .transpose()
        })
    }

    /// Iterate through the transaction events using metadata to dynamically decode and skip
    /// them, and return the first event found which decodes to the provided `Ev` type.
    ///
    /// This works in the same way that [`events::Events::find_first()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn find_first<Ev: StaticEvent>(&self) -> Result<Option<Ev>, Error> {
        self.find::<Ev>().next().transpose()
    }

    /// Find an event in those associated with this transaction. Returns true if it was found.
    ///
    /// This works in the same way that [`events::Events::has()`] does, with the
    /// exception that it ignores events not related to the submitted extrinsic.
    pub fn has<Ev: StaticEvent>(&self) -> Result<bool, Error> {
        Ok(self.find::<Ev>().next().transpose()?.is_some())
    }
}
