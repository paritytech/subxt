// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing extrinsics/transactions that have been submitted to a node.

use std::task::Poll;

use crate::utils::strip_compact_prefix;
use crate::{
    backend::{StreamOfResults, TransactionStatus as BackendTxStatus},
    client::OnlineClientT,
    error::{DispatchError, Error, RpcError, TransactionError},
    events::EventsClient,
    Config,
};
use derivative::Derivative;
use futures::{Stream, StreamExt};

/// This struct represents a subscription to the progress of some transaction.
pub struct TxProgress<T: Config, C> {
    sub: Option<StreamOfResults<BackendTxStatus<T::Hash>>>,
    ext_hash: T::Hash,
    client: C,
}

impl<T: Config, C> std::fmt::Debug for TxProgress<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TxProgress")
            .field("sub", &"<subscription>")
            .field("ext_hash", &self.ext_hash)
            .field("client", &"<client>")
            .finish()
    }
}

// The above type is not `Unpin` by default unless the generic param `T` is,
// so we manually make it clear that Unpin is actually fine regardless of `T`
// (we don't care if this moves around in memory while it's "pinned").
impl<T: Config, C> Unpin for TxProgress<T, C> {}

impl<T: Config, C> TxProgress<T, C> {
    /// Instantiate a new [`TxProgress`] from a custom subscription.
    pub fn new(
        sub: StreamOfResults<BackendTxStatus<T::Hash>>,
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

impl<T, C> TxProgress<T, C>
where
    T: Config,
    C: OnlineClientT<T>,
{
    /// Return the next transaction status when it's emitted. This just delegates to the
    /// [`futures::Stream`] implementation for [`TxProgress`], but allows you to
    /// avoid importing that trait if you don't otherwise need it.
    pub async fn next(&mut self) -> Option<Result<TxStatus<T, C>, Error>> {
        StreamExt::next(self).await
    }

    /// Wait for the transaction to be in a block (but not necessarily finalized), and return
    /// an [`TxInBlock`] instance when this happens, or an error if there was a problem
    /// waiting for this to happen.
    ///
    /// **Note:** consumes `self`. If you'd like to perform multiple actions as the state of the
    /// transaction progresses, use [`TxProgress::next()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid`/`Usurped`/`Dropped` indicate with some
    /// probability that the transaction will not make it into a block but there is no guarantee
    /// that this is true. In those cases the stream is closed however, so you currently have no way to find
    /// out if they finally made it into a block or not.
    pub async fn wait_for_in_block(mut self) -> Result<TxInBlock<T, C>, Error> {
        while let Some(status) = self.next().await {
            match status? {
                // Finalized or otherwise in a block! Return.
                TxStatus::InBestBlock(s) | TxStatus::InFinalizedBlock(s) => return Ok(s),
                // Error scenarios; return the error.
                TxStatus::Error { message } => return Err(TransactionError::Error(message).into()),
                TxStatus::Invalid { message } => {
                    return Err(TransactionError::Invalid(message).into())
                }
                TxStatus::Dropped { message } => {
                    return Err(TransactionError::Dropped(message).into())
                }
                // Ignore anything else and wait for next status event:
                _ => continue,
            }
        }
        Err(RpcError::SubscriptionDropped.into())
    }

    /// Wait for the transaction to be finalized, and return a [`TxInBlock`]
    /// instance when it is, or an error if there was a problem waiting for finalization.
    ///
    /// **Note:** consumes `self`. If you'd like to perform multiple actions as the state of the
    /// transaction progresses, use [`TxProgress::next()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid`/`Usurped`/`Dropped` indicate with some
    /// probability that the transaction will not make it into a block but there is no guarantee
    /// that this is true. In those cases the stream is closed however, so you currently have no way to find
    /// out if they finally made it into a block or not.
    pub async fn wait_for_finalized(mut self) -> Result<TxInBlock<T, C>, Error> {
        while let Some(status) = self.next().await {
            match status? {
                // Finalized! Return.
                TxStatus::InFinalizedBlock(s) => return Ok(s),
                // Error scenarios; return the error.
                TxStatus::Error { message } => return Err(TransactionError::Error(message).into()),
                TxStatus::Invalid { message } => {
                    return Err(TransactionError::Invalid(message).into())
                }
                TxStatus::Dropped { message } => {
                    return Err(TransactionError::Dropped(message).into())
                }
                // Ignore and wait for next status event:
                _ => continue,
            }
        }
        Err(RpcError::SubscriptionDropped.into())
    }

    /// Wait for the transaction to be finalized, and for the transaction events to indicate
    /// that the transaction was successful. Returns the events associated with the transaction,
    /// as well as a couple of other details (block hash and extrinsic hash).
    ///
    /// **Note:** consumes self. If you'd like to perform multiple actions as progress is made,
    /// use [`TxProgress::next()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid`/`Usurped`/`Dropped` indicate with some
    /// probability that the transaction will not make it into a block but there is no guarantee
    /// that this is true. In those cases the stream is closed however, so you currently have no way to find
    /// out if they finally made it into a block or not.
    pub async fn wait_for_finalized_success(
        self,
    ) -> Result<crate::blocks::ExtrinsicEvents<T>, Error> {
        let evs = self.wait_for_finalized().await?.wait_for_success().await?;
        Ok(evs)
    }
}

impl<T: Config, C: Clone> Stream for TxProgress<T, C> {
    type Item = Result<TxStatus<T, C>, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let sub = match self.sub.as_mut() {
            Some(sub) => sub,
            None => return Poll::Ready(None),
        };

        sub.poll_next_unpin(cx).map_ok(|status| {
            match status {
                BackendTxStatus::Validated => TxStatus::Validated,
                BackendTxStatus::Broadcasted { num_peers } => TxStatus::Broadcasted { num_peers },
                BackendTxStatus::InBestBlock { hash } => {
                    TxStatus::InBestBlock(TxInBlock::new(hash, self.ext_hash, self.client.clone()))
                }
                // These stream events mean that nothing further will be sent:
                BackendTxStatus::InFinalizedBlock { hash } => {
                    self.sub = None;
                    TxStatus::InFinalizedBlock(TxInBlock::new(
                        hash,
                        self.ext_hash,
                        self.client.clone(),
                    ))
                }
                BackendTxStatus::Error { message } => {
                    self.sub = None;
                    TxStatus::Error { message }
                }
                BackendTxStatus::Invalid { message } => {
                    self.sub = None;
                    TxStatus::Invalid { message }
                }
                BackendTxStatus::Dropped { message } => {
                    self.sub = None;
                    TxStatus::Dropped { message }
                }
            }
        })
    }
}

/// Possible transaction statuses returned from our [`TxProgress::next()`] call.
#[derive(Derivative)]
#[derivative(Debug(bound = "C: std::fmt::Debug"))]
pub enum TxStatus<T: Config, C> {
    /// Transaction is part of the future queue.
    Validated,
    /// The transaction has been broadcast to other nodes.
    Broadcasted {
        /// Number of peers it's been broadcast to.
        num_peers: u32,
    },
    /// Transaction has been included in block with given hash.
    InBestBlock(TxInBlock<T, C>),
    /// Transaction has been finalized by a finality-gadget, e.g GRANDPA
    InFinalizedBlock(TxInBlock<T, C>),
    /// Something went wrong in the node.
    Error {
        /// Human readable message; what went wrong.
        message: String,
    },
    /// Transaction is invalid (bad nonce, signature etc).
    Invalid {
        /// Human readable message; why was it invalid.
        message: String,
    },
    /// The transaction was dropped.
    Dropped {
        /// Human readable message; why was it dropped.
        message: String,
    },
}

impl<T: Config, C> TxStatus<T, C> {
    /// A convenience method to return the finalized details. Returns
    /// [`None`] if the enum variant is not [`TxStatus::InFinalizedBlock`].
    pub fn as_finalized(&self) -> Option<&TxInBlock<T, C>> {
        match self {
            Self::InFinalizedBlock(val) => Some(val),
            _ => None,
        }
    }

    /// A convenience method to return the best block details. Returns
    /// [`None`] if the enum variant is not [`TxStatus::InBestBlock`].
    pub fn as_in_block(&self) -> Option<&TxInBlock<T, C>> {
        match self {
            Self::InBestBlock(val) => Some(val),
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

impl<T: Config, C> TxInBlock<T, C> {
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
}

impl<T: Config, C: OnlineClientT<T>> TxInBlock<T, C> {
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
    pub async fn wait_for_success(&self) -> Result<crate::blocks::ExtrinsicEvents<T>, Error> {
        let events = self.fetch_events().await?;

        // Try to find any errors; return the first one we encounter.
        for ev in events.iter() {
            let ev = ev?;
            if ev.pallet_name() == "System" && ev.variant_name() == "ExtrinsicFailed" {
                let dispatch_error =
                    DispatchError::decode_from(ev.field_bytes(), self.client.metadata())?;
                return Err(dispatch_error.into());
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
    pub async fn fetch_events(&self) -> Result<crate::blocks::ExtrinsicEvents<T>, Error> {
        let block_body = self
            .client
            .backend()
            .block_body(self.block_hash)
            .await?
            .ok_or(Error::Transaction(TransactionError::BlockNotFound))?;

        let extrinsic_idx = block_body
            .iter()
            .position(|ext| {
                use crate::config::Hasher;
                let Ok((_, stripped)) = strip_compact_prefix(ext) else {
                    return false;
                };
                let hash = T::Hasher::hash_of(&stripped);
                hash == self.ext_hash
            })
            // If we successfully obtain the block hash we think contains our
            // extrinsic, the extrinsic should be in there somewhere..
            .ok_or(Error::Transaction(TransactionError::BlockNotFound))?;

        let events = EventsClient::new(self.client.clone())
            .at(self.block_hash)
            .await?;

        Ok(crate::blocks::ExtrinsicEvents::new(
            self.ext_hash,
            extrinsic_idx as u32,
            events,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        backend::{StreamOfResults, TransactionStatus},
        client::{OfflineClientT, OnlineClientT},
        tx::TxProgress,
        Config, Error, SubstrateConfig,
    };

    type MockTxProgress = TxProgress<SubstrateConfig, MockClient>;
    type MockHash = <SubstrateConfig as Config>::Hash;
    type MockSubstrateTxStatus = TransactionStatus<MockHash>;

    /// a mock client to satisfy trait bounds in tests
    #[derive(Clone, Debug)]
    struct MockClient;

    impl OfflineClientT<SubstrateConfig> for MockClient {
        fn metadata(&self) -> crate::Metadata {
            unimplemented!("just a mock impl to satisfy trait bounds")
        }

        fn genesis_hash(&self) -> MockHash {
            unimplemented!("just a mock impl to satisfy trait bounds")
        }

        fn runtime_version(&self) -> crate::backend::RuntimeVersion {
            unimplemented!("just a mock impl to satisfy trait bounds")
        }
    }

    impl OnlineClientT<SubstrateConfig> for MockClient {
        fn backend(&self) -> &dyn crate::backend::Backend<SubstrateConfig> {
            unimplemented!("just a mock impl to satisfy trait bounds")
        }
    }

    #[tokio::test]
    async fn wait_for_finalized_returns_err_when_error() {
        let tx_progress = mock_tx_progress(vec![
            MockSubstrateTxStatus::Broadcasted { num_peers: 2 },
            MockSubstrateTxStatus::Error {
                message: "err".into(),
            },
        ]);
        let finalized_result = tx_progress.wait_for_finalized().await;
        assert!(matches!(
            finalized_result,
            Err(Error::Transaction(crate::error::TransactionError::Error(e))) if e == "err"
        ));
    }

    #[tokio::test]
    async fn wait_for_finalized_returns_err_when_invalid() {
        let tx_progress = mock_tx_progress(vec![
            MockSubstrateTxStatus::Broadcasted { num_peers: 2 },
            MockSubstrateTxStatus::Invalid {
                message: "err".into(),
            },
        ]);
        let finalized_result = tx_progress.wait_for_finalized().await;
        assert!(matches!(
            finalized_result,
            Err(Error::Transaction(crate::error::TransactionError::Invalid(e))) if e == "err"
        ));
    }

    #[tokio::test]
    async fn wait_for_finalized_returns_err_when_dropped() {
        let tx_progress = mock_tx_progress(vec![
            MockSubstrateTxStatus::Broadcasted { num_peers: 2 },
            MockSubstrateTxStatus::Dropped {
                message: "err".into(),
            },
        ]);
        let finalized_result = tx_progress.wait_for_finalized().await;
        assert!(matches!(
            finalized_result,
            Err(Error::Transaction(crate::error::TransactionError::Dropped(e))) if e == "err"
        ));
    }

    fn mock_tx_progress(statuses: Vec<MockSubstrateTxStatus>) -> MockTxProgress {
        let sub = create_substrate_tx_status_subscription(statuses);
        TxProgress::new(sub, MockClient, Default::default())
    }

    fn create_substrate_tx_status_subscription(
        elements: Vec<MockSubstrateTxStatus>,
    ) -> StreamOfResults<MockSubstrateTxStatus> {
        let results = elements.into_iter().map(Ok);
        let stream = Box::pin(futures::stream::iter(results));
        let sub: StreamOfResults<MockSubstrateTxStatus> = StreamOfResults::new(stream);
        sub
    }
}
