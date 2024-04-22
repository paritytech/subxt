//! RPC utils.

use super::StreamOfResults;
use crate::error::Error;
use futures::future::BoxFuture;
use futures::{ready, FutureExt, Stream, StreamExt};
use std::{future::Future, pin::Pin, task::Poll};

/// Resubscribe callback.
pub type ResubscribeGetter<T> = Box<dyn FnMut() -> ResubscribeFuture<T> + Send>;

/// Future that resolves to a subscription stream.
pub type ResubscribeFuture<T> =
    Pin<Box<dyn Future<Output = Result<StreamOfResults<T>, Error>> + Send>>;

pub(crate) enum PendingOrStream<T> {
    Pending(BoxFuture<'static, Result<StreamOfResults<T>, Error>>),
    Stream(StreamOfResults<T>),
}

/// Retry subscription.
pub struct RetrySubscription<T> {
    resubscribe: ResubscribeGetter<T>,
    state: PendingOrStream<T>,
}

impl<T> RetrySubscription<T> {
    /// Create a new retry-able subscription.
    pub fn new(stream: StreamOfResults<T>, resubscribe: ResubscribeGetter<T>) -> Self {
        Self {
            resubscribe,
            state: PendingOrStream::Stream(stream),
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
        enum NextState<T> {
            Stream(StreamOfResults<T>),
            Resubscribe(BoxFuture<'static, Result<StreamOfResults<T>, Error>>),
        }

        loop {
            let next_state = match self.state {
                PendingOrStream::Stream(ref mut s) => match s.poll_next_unpin(cx) {
                    Poll::Ready(Some(Err(e))) => {
                        if e.is_disconnected_will_reconnect() {
                            NextState::Resubscribe((self.resubscribe)())
                        } else {
                            return Poll::Ready(Some(Err(e)));
                        }
                    }
                    other => return other,
                },
                PendingOrStream::Pending(ref mut fut) => match ready!(fut.poll_unpin(cx)) {
                    Ok(stream) => NextState::Stream(stream),
                    Err(e) => {
                        if !e.is_disconnected_will_reconnect() {
                            return Poll::Ready(None);
                        }

                        NextState::Resubscribe((self.resubscribe)())
                    }
                },
            };

            match next_state {
                NextState::Resubscribe(fut) => {
                    self.state = PendingOrStream::Pending(fut);
                }
                NextState::Stream(stream) => {
                    self.state = PendingOrStream::Stream(stream);
                }
            }
        }
    }
}

/// Retry a future until it doesn't return a disconnected error.
pub async fn retry<T, F, R>(mut retry_future: F) -> Result<R, Error>
where
    F: FnMut() -> T,
    T: Future<Output = Result<R, Error>>,
{
    loop {
        match retry_future().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                if !e.is_disconnected_will_reconnect() {
                    return Err(e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::StreamOf;

    fn disconnect_err() -> Error {
        Error::Rpc(crate::error::RpcError::DisconnectedWillReconnect(
            String::new(),
        ))
    }

    fn custom_err() -> Error {
        Error::Other(String::new())
    }

    #[tokio::test]
    async fn retry_stream_works() {
        let stream = futures::stream::iter([Ok(1), Err(disconnect_err())]);

        let resubscribe = Box::new(move || {
            async move { Ok(StreamOf::new(Box::pin(futures::stream::iter([Ok(2)])))) }.boxed()
        });

        let retry_stream = RetrySubscription::new(StreamOf::new(Box::pin(stream)), resubscribe);

        let result: Vec<_> = retry_stream
            .filter_map(|v| async move {
                if let Ok(v) = v {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
            .await;

        // After the subscription gets disconnected
        // we should fetch another element before it's done.
        assert_eq!(result, vec![1, 2]);
    }

    #[tokio::test]
    async fn failed_resubscribe_terminates_stream() {
        let stream = futures::stream::iter([Ok(1)]);
        let resubscribe = Box::new(move || async move { Err(custom_err()) }.boxed());

        let retry_stream = RetrySubscription::new(StreamOf::new(Box::pin(stream)), resubscribe);
        assert_eq!(1, retry_stream.count().await);
    }
}
