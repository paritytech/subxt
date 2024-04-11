//! RPC utils.

use futures::future::BoxFuture;
pub use tokio_retry::strategy::*;

use super::{Backend, BlockRef, RuntimeVersion, StreamOfResults};
use crate::error::Error;
use crate::Config;
use crate::{backend::TransactionStatus, error::RpcError};
use futures::{FutureExt, Stream, StreamExt};
use std::{future::Future, pin::Pin, task::Poll, time::Duration};
use tokio_retry::{Action, RetryIf};

/// ..
pub type ResubscribeGetter<T> = Box<dyn FnMut() -> ResubscribeFuture<T> + Send>;

/// ...
pub type ResubscribeFuture<T> =
    Pin<Box<dyn Future<Output = Result<StreamOfResults<T>, Error>> + Send>>;

enum WaitingOrStream {
    Waiting(BoxFuture<'static, StreamOfResults<RuntimeVersion>>),
    Stream(StreamOfResults<RuntimeVersion>),
}

/// Runtime version subscription.
pub struct RuntimeVersionSubscription {
    pub(crate) resubscribe: ResubscribeGetter<RuntimeVersion>,
    pub(crate) stream: StreamOfResults<RuntimeVersion>,
}

/// Block subscription.
pub struct BlockSubscription<T: Config> {
    pub(crate) resubscribe: ResubscribeGetter<(T::Header, BlockRef<T::Hash>)>,
    pub(crate) stream: StreamOfResults<(T::Header, BlockRef<T::Hash>)>,
}

/// Submit transaction subscription.
pub struct SubmitTransactionSubscription<T: Config> {
    pub(crate) stream: StreamOfResults<TransactionStatus<T::Hash>>,
}

impl<T: Send + Sync + Config> SubmitTransactionSubscription<T> {
    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next(&mut self) -> Option<Result<TransactionStatus<T::Hash>, Error>> {
        StreamExt::next(self).await
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

impl<T: Send + Sync + Config> BlockSubscription<T> {
    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next(&mut self) -> Option<Result<(T::Header, BlockRef<T::Hash>), Error>> {
        loop {
            match self.stream.next().await {
                Some(Err(e)) => {
                    if e.is_disconnected_will_reconnect() {
                        self.stream = match (self.resubscribe)().await {
                            Ok(s) => s,
                            Err(e) => break Some(Err(e)),
                        };
                    } else {
                        break Some(Err(e));
                    }
                }
                other => break other,
            }
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
        todo!("write a proper poll impl");
        //self.stream.poll_next_unpin(cx)
    }
}

impl RuntimeVersionSubscription {
    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next(&mut self) -> Option<Result<RuntimeVersion, Error>> {
        loop {
            match self.stream.next().await {
                Some(Err(e)) => {
                    if e.is_disconnected_will_reconnect() {
                        self.stream = match (self.resubscribe)().await {
                            Ok(s) => s,
                            Err(e) => break Some(Err(e)),
                        };
                    } else {
                        break Some(Err(e));
                    }
                }
                other => break other,
            }
        }
    }
}

impl std::marker::Unpin for RuntimeVersionSubscription {}

impl Stream for RuntimeVersionSubscription {
    type Item = Result<RuntimeVersion, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        todo!("write a proper poll impl");
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
