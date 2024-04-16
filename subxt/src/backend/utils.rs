//! RPC utils.

use futures::future::BoxFuture;
pub use tokio_retry::strategy::ExponentialBackoff;

use super::{RuntimeVersion, StreamOfResults};
use crate::error::Error;
use futures::{ready, FutureExt, Stream, StreamExt};
use std::{future::Future, pin::Pin, task::Poll, time::Duration};
use tokio_retry::{Action, RetryIf};

/// Resubscribe callback.
pub type ResubscribeGetter<T> = Box<dyn FnMut() -> ResubscribeFuture<T> + Send>;

/// Future that resolves to a subscription stream.
pub type ResubscribeFuture<T> =
    Pin<Box<dyn Future<Output = Result<StreamOfResults<T>, Error>> + Send>>;

pub(crate) enum WaitingOrStream {
    Waiting(BoxFuture<'static, StreamOfResults<RuntimeVersion>>),
    Stream(StreamOfResults<RuntimeVersion>),
}

/// Retry subscription.
pub struct RetrySubscription<T> {
    pub(crate) resubscribe: Option<ResubscribeGetter<T>>,
    pub(crate) stream: Option<StreamOfResults<T>>,
    pub(crate) pending: Option<BoxFuture<'static, Result<StreamOfResults<T>, Error>>>,
}

impl<T> RetrySubscription<T> {
    /// Create a new retry-able subscription.
    ///
    /// The stream itself re-starts the subscription
    /// if the connection was closed.
    pub fn new(stream: StreamOfResults<T>) -> Self {
        Self {
            stream: Some(stream),
            resubscribe: None,
            pending: None,
        }
    }

    /// Create a new retry-able subscription.
    ///
    /// The callback is invoked if a reconnection occurs.
    pub fn with_resubscribe_callback(
        stream: StreamOfResults<T>,
        resubscribe: ResubscribeGetter<T>,
    ) -> Self {
        Self {
            stream: Some(stream),
            resubscribe: Some(resubscribe),
            pending: None,
        }
    }

    /// Returns the next item in the stream. This is just a wrapper around
    /// [`StreamExt::next()`] so that you can avoid the extra import.
    pub async fn next(&mut self) -> Option<Result<T, Error>> {
        StreamExt::next(self).await
    }
}

impl<T> std::marker::Unpin for RetrySubscription<T> {}

impl<T> Stream for RetrySubscription<T> {
    type Item = Result<T, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            // Poll the stream.
            let need_resubscribe = match self.stream.as_mut() {
                Some(s) => match s.poll_next_unpin(cx) {
                    Poll::Ready(Some(Err(e))) => {
                        if e.is_disconnected_will_reconnect() {
                            true
                        } else {
                            return Poll::Ready(Some(Err(e)));
                        }
                    }
                    other => return other,
                },
                None => false,
            };

            if need_resubscribe {
                self.stream = None;

                let Some(f) = self.resubscribe.as_mut() else {
                    tracing::error!("No callback configured for RetrySubscription that emitted Error::DisconnectedWillReconnect; This a bug please file an issue");
                    return Poll::Ready(None);
                };

                self.pending = Some(f());
            }

            // Poll the resubscription.
            let not_pending = if let Some(p) = self.pending.as_mut() {
                if let Ok(stream) = ready!(p.poll_unpin(cx)) {
                    self.stream = Some(stream);
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if not_pending {
                self.pending = None;
            }
        }
    }
}

/// Retry a future with custom strategy.
pub async fn retry_with_strategy<T, A>(
    strategy: tokio_retry::strategy::ExponentialBackoff,
    mut retry_future: A,
) -> Result<T, Error>
where
    A: Action<Item = T, Error = Error>,
{
    RetryIf::spawn(
        strategy,
        || retry_future.run(),
        |err: &Error| err.is_disconnected_will_reconnect(),
    )
    .await
}

/// Retry policy
///
/// Does nothing if a non-reconnecting rpc client hasn't been enabled.
#[derive(Debug, Clone)]
pub struct RetryPolicy(tokio_retry::strategy::ExponentialBackoff);

impl Default for RetryPolicy {
    fn default() -> Self {
        Self(ExponentialBackoff::from_millis(10).max_delay(Duration::from_secs(60)))
    }
}

impl RetryPolicy {
    /// Create new retry policy
    pub fn new(policy: ExponentialBackoff) -> Self {
        Self(policy)
    }

    /// Run a retry-able call.
    pub async fn call<T, A>(&self, retry_future: A) -> Result<T, Error>
    where
        A: Action<Item = T, Error = Error>,
    {
        retry_with_strategy(self.0.clone(), retry_future).await
    }
}
