//! RPC utils.

pub use tokio_retry::strategy::*;

use super::{Backend, BlockRef, RuntimeVersion, StreamOfResults};
use crate::backend::TransactionStatus;
use crate::error::Error;
use crate::Config;
use futures::{Stream, StreamExt};
use std::{pin::Pin, task::Poll, time::Duration};
use tokio_retry::{Action, RetryIf};

/// Runtime version subscription.
pub struct RuntimeVersionSubscription<T: Config> {
    pub(crate) backend: Box<dyn Backend<T>>,
    pub(crate) stream: StreamOfResults<RuntimeVersion>,
}

/// Block subscription.
pub struct BlockSubscription<T: Config> {
    pub(crate) backend: Box<dyn Backend<T>>,
    pub(crate) kind: BlockSubscriptionKind,
    pub(crate) stream: StreamOfResults<(T::Header, BlockRef<T::Hash>)>,
}

/// Submit transaction subscription.
pub struct SubmitTransactionSubscription<T: Config> {
    pub(crate) backend: Box<dyn Backend<T>>,
    pub(crate) stream: StreamOfResults<TransactionStatus<T::Hash>>,
}

impl<T: Send + Sync + Config> SubmitTransactionSubscription<T> {
    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next(&mut self) -> Option<Result<TransactionStatus<T::Hash>, Error>> {
        StreamExt::next(self).await
    }

    /// Re-submit the transaction and subscribe.
    pub async fn resubscribe(self, extrinsic: &[u8]) -> Result<Self, Error> {
        self.backend.submit_transaction(extrinsic).await
    }
}

impl<T: Config> std::marker::Unpin for SubmitTransactionSubscription<T> {}

impl<T: Send + Sync + Config> Stream for SubmitTransactionSubscription<T> {
    type Item = Result<TransactionStatus<T::Hash>, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.stream.poll_next_unpin(cx)
    }
}

pub(crate) enum BlockSubscriptionKind {
    All,
    Best,
    Finalized,
}

impl<T: Send + Sync + Config> BlockSubscription<T> {
    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next(&mut self) -> Option<Result<(T::Header, BlockRef<T::Hash>), Error>> {
        StreamExt::next(self).await
    }

    /// Resubscribe to the subscription.
    pub async fn resubscribe(self) -> Result<Self, Error> {
        match self.kind {
            BlockSubscriptionKind::All => self.backend.stream_all_block_headers().await,
            BlockSubscriptionKind::Best => self.backend.stream_best_block_headers().await,
            BlockSubscriptionKind::Finalized => self.backend.stream_finalized_block_headers().await,
        }
    }
}

impl<T: Config> std::marker::Unpin for BlockSubscription<T> {}

impl<T: Send + Sync + Config> Stream for BlockSubscription<T> {
    type Item = Result<(T::Header, BlockRef<T::Hash>), Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.stream.poll_next_unpin(cx)
    }
}

impl<T: Send + Sync + Config> RuntimeVersionSubscription<T> {
    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next(&mut self) -> Option<Result<RuntimeVersion, Error>> {
        StreamExt::next(self).await
    }

    /// Resubscribe to the subscription.
    pub async fn resubscribe(self) -> Result<Self, Error> {
        self.backend.stream_runtime_version().await
    }
}

impl<T: Config> std::marker::Unpin for RuntimeVersionSubscription<T> {}

impl<T: Send + Sync + Config> Stream for RuntimeVersionSubscription<T> {
    type Item = Result<RuntimeVersion, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.stream.poll_next_unpin(cx)
    }
}

/// Retry a future with custom strategy.
pub async fn retry_with_strategy<T, A, I, S>(strategy: S, mut retry_future: A) -> Result<T, Error>
where
    A: Action<Item = T, Error = Error>,
    I: Iterator<Item = Duration>,
    S: IntoIterator<IntoIter = I, Item = Duration>,
{
    RetryIf::spawn(
        strategy,
        || retry_future.run(),
        |err: &Error| err.is_disconnected_will_reconnect(),
    )
    .await
}

/// Retry a future with default strategy.
pub async fn retry<T, A>(retry_future: A) -> Result<T, Error>
where
    A: Action<Item = T, Error = Error>,
{
    retry_with_strategy(
        ExponentialBackoff::from_millis(10)
            .max_delay(Duration::from_secs(60))
            .map(jitter),
        retry_future,
    )
    .await
}
