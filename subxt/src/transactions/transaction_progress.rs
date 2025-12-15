use crate::backend::BlockRef;
use crate::backend::{StreamOfResults, TransactionStatus as BackendTransactionStatus};
use crate::client::{BlockNumberOrRef, OnlineClientAtBlockT};
use crate::config::{Config, HashFor};
use crate::error::{
    DispatchError, TransactionEventsError, TransactionFinalizedSuccessError,
    TransactionProgressError, TransactionStatusError,
};
use crate::extrinsics::ExtrinsicEvents;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

/// A stream representing the progress of some transaction. Events can be
/// streamed and acted on using [`TransactionProgress::next()`], or the helper
/// functions [`TransactionProgress::wait_for_finalized`] and
/// [`TransactionProgress::wait_for_finalized_success`] can be used to wait
/// for completion.
#[derive(Debug)]
pub struct TransactionProgress<'atblock, T: Config, C> {
    sub: Option<StreamOfResults<BackendTransactionStatus<HashFor<T>>>>,
    ext_hash: HashFor<T>,
    client: &'atblock C,
}

// The above type is not `Unpin` by default unless the generic param `T` is,
// so we manually make it clear that Unpin is actually fine regardless of `T`
// (we don't care if this moves around in memory while it's "pinned").
impl<'atblock, T: Config, C> Unpin for TransactionProgress<'atblock, T, C> {}

impl<'atblock, T: Config, C> TransactionProgress<'atblock, T, C> {
    /// Instantiate a new [`TransactionProgress`] from a custom subscription.
    pub fn new(
        sub: StreamOfResults<BackendTransactionStatus<HashFor<T>>>,
        client: &'atblock C,
        ext_hash: HashFor<T>,
    ) -> Self {
        Self {
            sub: Some(sub),
            client,
            ext_hash,
        }
    }

    /// Return the hash of the extrinsic.
    pub fn extrinsic_hash(&self) -> HashFor<T> {
        self.ext_hash
    }
}

impl<'atblock, T, C> TransactionProgress<'atblock, T, C>
where
    T: Config,
    C: OnlineClientAtBlockT<T>,
{
    /// Return the next transaction status when it's emitted. This just delegates to the
    /// [`futures::Stream`] implementation for [`TransactionProgress`], but allows you to
    /// avoid importing that trait if you don't otherwise need it.
    pub async fn next(
        &mut self,
    ) -> Option<Result<TransactionStatus<'atblock, T, C>, TransactionProgressError>> {
        StreamExt::next(self).await
    }

    /// Wait for the transaction to be finalized, and return a [`TransactionInBlock`]
    /// instance when it is, or an error if there was a problem waiting for finalization.
    ///
    /// **Note:** consumes `self`. If you'd like to perform multiple actions as the state of the
    /// transaction progresses, use [`TxProgress::next()`] instead.
    ///
    /// **Note:** transaction statuses like `Invalid`/`Usurped`/`Dropped` indicate with some
    /// probability that the transaction will not make it into a block but there is no guarantee
    /// that this is true. In those cases the stream is closed however, so you currently have no way to find
    /// out if they finally made it into a block or not.
    pub async fn wait_for_finalized(
        mut self,
    ) -> Result<TransactionInBlock<'atblock, T, C>, TransactionProgressError> {
        while let Some(status) = self.next().await {
            match status? {
                // Finalized! Return.
                TransactionStatus::InFinalizedBlock(s) => return Ok(s),
                // Error scenarios; return the error.
                TransactionStatus::Error { message } => {
                    return Err(TransactionStatusError::Error(message).into());
                }
                TransactionStatus::Invalid { message } => {
                    return Err(TransactionStatusError::Invalid(message).into());
                }
                TransactionStatus::Dropped { message } => {
                    return Err(TransactionStatusError::Dropped(message).into());
                }
                // Ignore and wait for next status event:
                _ => continue,
            }
        }
        Err(TransactionProgressError::UnexpectedEndOfTransactionStatusStream)
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
    ) -> Result<ExtrinsicEvents<T>, TransactionFinalizedSuccessError> {
        let evs = self.wait_for_finalized().await?.wait_for_success().await?;
        Ok(evs)
    }
}

// TransactionProgress is a stream of transaction events
impl<'atblock, T: Config, C: Clone> Stream for TransactionProgress<'atblock, T, C> {
    type Item = Result<TransactionStatus<'atblock, T, C>, TransactionProgressError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let sub = match self.sub.as_mut() {
            Some(sub) => sub,
            None => return Poll::Ready(None),
        };

        sub.poll_next_unpin(cx)
            .map_err(TransactionProgressError::CannotGetNextProgressUpdate)
            .map_ok(|status| {
                match status {
                    BackendTransactionStatus::Validated => TransactionStatus::Validated,
                    BackendTransactionStatus::Broadcasted => TransactionStatus::Broadcasted,
                    BackendTransactionStatus::NoLongerInBestBlock => {
                        TransactionStatus::NoLongerInBestBlock
                    }
                    BackendTransactionStatus::InBestBlock { hash } => {
                        TransactionStatus::InBestBlock(TransactionInBlock::new(
                            hash,
                            self.ext_hash,
                            self.client,
                        ))
                    }
                    // These stream events mean that nothing further will be sent:
                    BackendTransactionStatus::InFinalizedBlock { hash } => {
                        self.sub = None;
                        TransactionStatus::InFinalizedBlock(TransactionInBlock::new(
                            hash,
                            self.ext_hash,
                            self.client,
                        ))
                    }
                    BackendTransactionStatus::Error { message } => {
                        self.sub = None;
                        TransactionStatus::Error { message }
                    }
                    BackendTransactionStatus::Invalid { message } => {
                        self.sub = None;
                        TransactionStatus::Invalid { message }
                    }
                    BackendTransactionStatus::Dropped { message } => {
                        self.sub = None;
                        TransactionStatus::Dropped { message }
                    }
                }
            })
    }
}

/// Possible transaction statuses returned from our [`TransactionProgress::next()`] call.
#[derive(Debug)]
pub enum TransactionStatus<'atblock, T: Config, C> {
    /// Transaction is part of the future queue.
    Validated,
    /// The transaction has been broadcast to other nodes.
    Broadcasted,
    /// Transaction is no longer in a best block.
    NoLongerInBestBlock,
    /// Transaction has been included in block with given hash.
    InBestBlock(TransactionInBlock<'atblock, T, C>),
    /// Transaction has been finalized by a finality-gadget, e.g GRANDPA
    InFinalizedBlock(TransactionInBlock<'atblock, T, C>),
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

impl<'atblock, T: Config, C> TransactionStatus<'atblock, T, C> {
    /// A convenience method to return the finalized details. Returns
    /// [`None`] if the enum variant is not [`TxStatus::InFinalizedBlock`].
    pub fn as_finalized(&self) -> Option<&TransactionInBlock<'atblock, T, C>> {
        match self {
            Self::InFinalizedBlock(val) => Some(val),
            _ => None,
        }
    }

    /// A convenience method to return the best block details. Returns
    /// [`None`] if the enum variant is not [`TxStatus::InBestBlock`].
    pub fn as_in_block(&self) -> Option<&TransactionInBlock<'atblock, T, C>> {
        match self {
            Self::InBestBlock(val) => Some(val),
            _ => None,
        }
    }
}

/// This struct represents a transaction that has made it into a block.
#[derive(Debug)]
pub struct TransactionInBlock<'atblock, T: Config, C> {
    block_ref: BlockRef<HashFor<T>>,
    ext_hash: HashFor<T>,
    client: &'atblock C,
}

impl<'atblock, T: Config, C> TransactionInBlock<'atblock, T, C> {
    pub(crate) fn new(
        block_ref: BlockRef<HashFor<T>>,
        ext_hash: HashFor<T>,
        client: &'atblock C,
    ) -> Self {
        Self {
            block_ref,
            ext_hash,
            client,
        }
    }

    /// Return the hash of the block that the transaction has made it into.
    pub fn block_hash(&self) -> HashFor<T> {
        self.block_ref.hash()
    }

    /// Return the hash of the extrinsic that was submitted.
    pub fn extrinsic_hash(&self) -> HashFor<T> {
        self.ext_hash
    }
}

impl<'atblock, T: Config, C: OnlineClientAtBlockT<T>> TransactionInBlock<'atblock, T, C> {
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
    pub async fn wait_for_success(&self) -> Result<ExtrinsicEvents<T>, TransactionEventsError> {
        let events = self.fetch_events().await?;

        // Try to find any errors; return the first one we encounter.
        for (ev_idx, ev) in events.iter().enumerate() {
            let ev = ev.map_err(|e| TransactionEventsError::CannotDecodeEventInBlock {
                event_index: ev_idx,
                block_hash: self.block_hash().into(),
                error: e,
            })?;

            if ev.pallet_name() == "System" && ev.event_name() == "ExtrinsicFailed" {
                let dispatch_error =
                    DispatchError::decode_from(ev.field_bytes(), self.client.metadata()).map_err(
                        |e| TransactionEventsError::CannotDecodeDispatchError {
                            error: e,
                            bytes: ev.field_bytes().to_vec(),
                        },
                    )?;
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
    pub async fn fetch_events(&self) -> Result<ExtrinsicEvents<T>, TransactionEventsError> {
        // Create a client at the block the TX made it into:
        let tx_block_ref = BlockNumberOrRef::BlockRef(self.block_ref.clone());
        let at_tx_block = self
            .client
            .at_block(tx_block_ref)
            .await
            .map_err(TransactionEventsError::CannotInstantiateClientAtBlock)?;

        let hasher = at_tx_block.client.hasher();

        let block_body = at_tx_block
            .client
            .backend()
            .block_body(self.block_ref.hash())
            .await
            .map_err(|e| TransactionEventsError::CannotFetchBlockBody {
                block_hash: self.block_hash().into(),
                error: e,
            })?
            .ok_or_else(|| TransactionEventsError::BlockNotFound {
                block_hash: self.block_hash().into(),
            })?;

        let extrinsic_index = block_body
            .iter()
            .position(|ext| {
                use crate::config::Hasher;
                let hash = hasher.hash(&ext);
                hash == self.ext_hash
            })
            // If we successfully obtain the block hash we think contains our
            // extrinsic, the extrinsic should be in there somewhere..
            .ok_or_else(|| TransactionEventsError::CannotFindTransactionInBlock {
                block_hash: self.block_hash().into(),
                transaction_hash: self.ext_hash.into(),
            })?;

        let events =
            ExtrinsicEvents::fetch(&at_tx_block.client, self.extrinsic_hash(), extrinsic_index)
                .await
                .map_err(
                    |e| TransactionEventsError::CannotFetchEventsForTransaction {
                        block_hash: self.block_hash().into(),
                        transaction_hash: self.ext_hash.into(),
                        error: e,
                    },
                )?;

        Ok(events)
    }
}
