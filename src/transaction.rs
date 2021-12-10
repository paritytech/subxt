// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

use sp_core::storage::StorageKey;
use sp_runtime::traits::Hash;
pub use sp_runtime::traits::SignedExtension;
pub use sp_version::RuntimeVersion;

use crate::{
    client::Client,
    error::{
        Error,
        TransactionError,
    },
    rpc::SubstrateTransactionStatus,
    subscription::SystemEvents,
    Config,
    Phase,
};
use jsonrpsee::types::{
    Error as RpcError,
    Subscription as RpcSubscription,
};

/// This struct represents a subscription to the progress of some transaction, and is
/// returned from [`crate::SubmittableExtrinsic::sign_and_submit_then_watch()`].
#[derive(Debug)]
pub struct TransactionProgress<'client, T: Config> {
    sub: Option<RpcSubscription<SubstrateTransactionStatus<T::Hash, T::Hash>>>,
    ext_hash: T::Hash,
    client: &'client Client<T>,
}

impl<'client, T: Config> TransactionProgress<'client, T> {
    pub(crate) fn new(
        sub: RpcSubscription<SubstrateTransactionStatus<T::Hash, T::Hash>>,
        client: &'client Client<T>,
        ext_hash: T::Hash,
    ) -> Self {
        Self {
            sub: Some(sub),
            client,
            ext_hash,
        }
    }

    /// Return the next transaction status when it's emitted.
    pub async fn next(&mut self) -> Result<Option<TransactionStatus<'client, T>>, Error> {
        // Return `None` if the subscription has been dropped:
        let sub = match &mut self.sub {
            Some(sub) => sub,
            None => return Ok(None),
        };

        // Return the next item otherwise:
        let res = sub.next().await?;
        Ok(res.map(|status| {
            match status {
                SubstrateTransactionStatus::Future => TransactionStatus::Future,
                SubstrateTransactionStatus::Ready => TransactionStatus::Ready,
                SubstrateTransactionStatus::Broadcast(peers) => {
                    TransactionStatus::Broadcast(peers)
                }
                SubstrateTransactionStatus::InBlock(hash) => {
                    TransactionStatus::InBlock(TransactionInBlock {
                        block_hash: hash,
                        ext_hash: self.ext_hash,
                        client: self.client,
                    })
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
                    TransactionStatus::Finalized(TransactionInBlock {
                        block_hash: hash,
                        ext_hash: self.ext_hash,
                        client: self.client,
                    })
                }
            }
        }))
    }

    /// Wait for the transaction to be in a block (but not necessarily finalized), and return
    /// an [`TransactionInBlock`] instance when this happens, or an error if there was a problem
    /// waiting for this to happen.
    ///
    /// **Note:** consumes `self`. If you'd like to perform multiple actions as the state of the
    /// transaction progresses, use [`TransactionProgress::next()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid` and `Usurped` are ignored, because while they
    /// may well indicate with some probability that the transaction will not make it into a block,
    /// there is no guarantee that this is true. Thus, we prefer to "play it safe" here. Use the lower
    /// level [`TransactionProgress::next()`] API if you'd like to handle these statuses yourself.
    pub async fn wait_for_in_block(
        mut self,
    ) -> Result<TransactionInBlock<'client, T>, Error> {
        while let Some(status) = self.next().await? {
            match status {
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
    /// transaction progresses, use [`TransactionProgress::next()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid` and `Usurped` are ignored, because while they
    /// may well indicate with some probability that the transaction will not make it into a block,
    /// there is no guarantee that this is true. Thus, we prefer to "play it safe" here. Use the lower
    /// level [`TransactionProgress::next()`] API if you'd like to handle these statuses yourself.
    pub async fn wait_for_finalized(
        mut self,
    ) -> Result<TransactionInBlock<'client, T>, Error> {
        while let Some(status) = self.next().await? {
            match status {
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
    /// use [`TransactionProgress::next()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid` and `Usurped` are ignored, because while they
    /// may well indicate with some probability that the transaction will not make it into a block,
    /// there is no guarantee that this is true. Thus, we prefer to "play it safe" here. Use the lower
    /// level [`TransactionProgress::next()`] API if you'd like to handle these statuses yourself.
    pub async fn wait_for_finalized_success(self) -> Result<TransactionEvents<T>, Error> {
        let evs = self.wait_for_finalized().await?.wait_for_success().await?;
        Ok(evs)
    }
}

//* Dev note: The below is adapted from the substrate docs on `TransactionStatus`, which this
//* enum was adapted from (and which is an exact copy of `SubstrateTransactionStatus` in this crate).
//* Note that the number of finality watchers is, at the time of writing, found in the constant
//* `MAX_FINALITY_WATCHERS` in the `sc_transaction_pool` crate.
//*
/// Possible transaction statuses returned from our [`TransactionProgress::next()`] call.
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
#[derive(Debug)]
pub enum TransactionStatus<'client, T: Config> {
    /// The transaction is part of the "future" queue.
    Future,
    /// The transaction is part of the "ready" queue.
    Ready,
    /// The transaction has been broadcast to the given peers.
    Broadcast(Vec<String>),
    /// The transaction has been included in a block with given hash.
    InBlock(TransactionInBlock<'client, T>),
    /// The block this transaction was included in has been retracted,
    /// probably because it did not make it onto the blocks which were
    /// finalized.
    Retracted(T::Hash),
    /// A block containing the transaction did not reach finality within 512
    /// blocks, and so the subscription has ended.
    FinalityTimeout(T::Hash),
    /// The transaction has been finalized by a finality-gadget, e.g GRANDPA.
    Finalized(TransactionInBlock<'client, T>),
    /// The transaction has been replaced in the pool by another transaction
    /// that provides the same tags. (e.g. same (sender, nonce)).
    Usurped(T::Hash),
    /// The transaction has been dropped from the pool because of the limit.
    Dropped,
    /// The transaction is no longer valid in the current state.
    Invalid,
}

impl<'client, T: Config> TransactionStatus<'client, T> {
    /// A convenience method to return the `Finalized` details. Returns
    /// [`None`] if the enum variant is not [`TransactionStatus::Finalized`].
    pub fn as_finalized(&self) -> Option<&TransactionInBlock<'client, T>> {
        match self {
            Self::Finalized(val) => Some(val),
            _ => None,
        }
    }

    /// A convenience method to return the `InBlock` details. Returns
    /// [`None`] if the enum variant is not [`TransactionStatus::InBlock`].
    pub fn as_in_block(&self) -> Option<&TransactionInBlock<'client, T>> {
        match self {
            Self::InBlock(val) => Some(val),
            _ => None,
        }
    }
}

/// This struct represents a transaction that has made it into a block.
#[derive(Debug)]
pub struct TransactionInBlock<'client, T: Config> {
    block_hash: T::Hash,
    ext_hash: T::Hash,
    client: &'client Client<T>,
}

impl<'client, T: Config> TransactionInBlock<'client, T> {
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
    pub async fn wait_for_success(&self) -> Result<TransactionEvents<T>, Error> {
        let events = self.fetch_events().await?;

        // Try to find any errors; return the first one we encounter.
        for ev in events.as_slice() {
            if &ev.pallet == "System" && &ev.variant == "ExtrinsicFailed" {
                use codec::Decode;
                let dispatch_error = sp_runtime::DispatchError::decode(&mut &*ev.data)?;
                let runtime_error = crate::RuntimeError::from_dispatch(
                    self.client.metadata(),
                    dispatch_error,
                )?;
                return Err(runtime_error.into())
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
    pub async fn fetch_events(&self) -> Result<TransactionEvents<T>, Error> {
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

        let raw_events = self
            .client
            .rpc()
            .storage(
                &StorageKey::from(SystemEvents::new()),
                Some(self.block_hash),
            )
            .await?
            .map(|s| s.0)
            .unwrap_or_else(Vec::new);

        let events = self
            .client
            .events_decoder()
            .decode_events(&mut &*raw_events)?
            .into_iter()
            .filter(move |(phase, _raw)| {
                phase == &Phase::ApplyExtrinsic(extrinsic_idx as u32)
            })
            .map(|(_phase, event)| event)
            .collect();

        Ok(TransactionEvents {
            block_hash: self.block_hash,
            ext_hash: self.ext_hash,
            events,
        })
    }
}

/// This represents the events related to our transaction.
/// We can iterate over the events, or look for a specific one.
#[derive(Debug)]
pub struct TransactionEvents<T: Config> {
    block_hash: T::Hash,
    ext_hash: T::Hash,
    events: Vec<crate::RawEvent>,
}

impl<T: Config> TransactionEvents<T> {
    /// Return the hash of the block that the transaction has made it into.
    pub fn block_hash(&self) -> T::Hash {
        self.block_hash
    }

    /// Return the hash of the extrinsic.
    pub fn extrinsic_hash(&self) -> T::Hash {
        self.ext_hash
    }

    /// Return a slice of the returned events.
    pub fn as_slice(&self) -> &[crate::RawEvent] {
        &self.events
    }

    /// Find all of the events matching the event type provided as a generic parameter. This
    /// will return an error if a matching event is found but cannot be properly decoded.
    pub fn find_events<E: crate::Event>(&self) -> Result<Vec<E>, Error> {
        self.events
            .iter()
            .filter_map(|e| e.as_event::<E>().map_err(Into::into).transpose())
            .collect()
    }

    /// Find the first event that matches the event type provided as a generic parameter. This
    /// will return an error if a matching event is found but cannot be properly decoded.
    ///
    /// Use [`TransactionEvents::find_events`], or iterate over [`TransactionEvents`] yourself
    /// if you'd like to handle multiple events of the same type.
    pub fn find_first_event<E: crate::Event>(&self) -> Result<Option<E>, Error> {
        self.events
            .iter()
            .filter_map(|e| e.as_event::<E>().transpose())
            .next()
            .transpose()
            .map_err(Into::into)
    }

    /// Find an event. Returns true if it was found.
    pub fn has_event<E: crate::Event>(&self) -> Result<bool, Error> {
        Ok(self.find_first_event::<E>()?.is_some())
    }
}

impl<T: Config> std::ops::Deref for TransactionEvents<T> {
    type Target = [crate::RawEvent];
    fn deref(&self) -> &Self::Target {
        &self.events
    }
}
