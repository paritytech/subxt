//! RPC utils.

use super::{StreamOf, StreamOfResults};
use crate::error::Error;
use futures::future::BoxFuture;
use futures::{ready, FutureExt, Stream, StreamExt};
use std::{future::Future, pin::Pin, task::Poll};

/// Resubscribe callback.
type ResubscribeGetter<T> = Box<dyn FnMut() -> ResubscribeFuture<T> + Send>;

/// Future that resolves to a subscription stream.
type ResubscribeFuture<T> = Pin<Box<dyn Future<Output = Result<StreamOfResults<T>, Error>> + Send>>;

pub(crate) enum PendingOrStream<T> {
    Pending(BoxFuture<'static, Result<StreamOfResults<T>, Error>>),
    Stream(StreamOfResults<T>),
    Closed,
}

impl<T> std::fmt::Debug for PendingOrStream<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PendingOrStream::Pending(_) => write!(f, "Pending"),
            PendingOrStream::Stream(_) => write!(f, "Stream"),
            PendingOrStream::Closed => write!(f, "Closed"),
        }
    }
}

enum NextState<T> {
    Stream(StreamOfResults<T>),
    Resubscribe {
        pending: BoxFuture<'static, Result<StreamOfResults<T>, Error>>,
        err: Error,
    },
    Closed(Option<Error>),
}

impl<T> std::fmt::Debug for NextState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stream(_) => write!(f, "Stream"),
            Self::Resubscribe { .. } => write!(f, "Resubscribe"),
            Self::Closed(_) => write!(f, "Closed"),
        }
    }
}

/// Retry subscription.
struct RetrySubscription<T> {
    resubscribe: ResubscribeGetter<T>,
    state: PendingOrStream<T>,
}

impl<T> std::marker::Unpin for RetrySubscription<T> {}

impl<T> Stream for RetrySubscription<T> {
    type Item = Result<T, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            let next_state = match self.state {
                PendingOrStream::Stream(ref mut s) => match ready!(s.poll_next_unpin(cx)) {
                    Some(Err(err)) => {
                        if !err.is_disconnected_will_reconnect() {
                            NextState::Closed(Some(err))
                        } else {
                            NextState::Resubscribe {
                                pending: (self.resubscribe)(),
                                err,
                            }
                        }
                    }
                    None => NextState::Closed(None),
                    Some(Ok(val)) => return Poll::Ready(Some(Ok(val))),
                },
                PendingOrStream::Pending(ref mut fut) => match ready!(fut.poll_unpin(cx)) {
                    Ok(stream) => NextState::Stream(stream),
                    Err(err) => {
                        if !err.is_disconnected_will_reconnect() {
                            NextState::Closed(Some(err))
                        } else {
                            NextState::Resubscribe {
                                pending: (self.resubscribe)(),
                                err,
                            }
                        }
                    }
                },
                PendingOrStream::Closed => NextState::Closed(None),
            };

            match next_state {
                NextState::Resubscribe { pending, err } => {
                    self.state = PendingOrStream::Pending(pending);
                    return Poll::Ready(Some(Err(err)));
                }
                NextState::Stream(stream) => {
                    self.state = PendingOrStream::Stream(stream);
                }
                NextState::Closed(maybe_err) => {
                    self.state = PendingOrStream::Closed;

                    if let Some(err) = maybe_err {
                        return Poll::Ready(Some(Err(err)));
                    } else {
                        return Poll::Ready(None);
                    }
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
    const REJECTED_MAX_RETRIES: usize = 10;
    let mut rejected_retries = 0;

    loop {
        match retry_future().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                if e.is_disconnected_will_reconnect() {
                    continue;
                }

                // TODO: https://github.com/paritytech/subxt/issues/1567
                // This is a hack because if a reconnection occurs
                // the order of pending calls is not guaranteed.
                //
                // Such that it's possible the a pending future completes
                // before `chainHead_follow` is established with fresh
                // subscription id.
                //
                if e.is_rejected() && rejected_retries < REJECTED_MAX_RETRIES {
                    rejected_retries += 1;
                    continue;
                }

                return Err(e);
            }
        }
    }
}

/// Create a retry stream that will resubscribe on disconnect.
///
/// It's important to note that this function is intended to work only for stateless subscriptions.
/// If the subscription takes input or modifies state, this function should not be used.
pub(crate) async fn retry_stream<F, R>(mut sub_stream: F) -> Result<StreamOfResults<R>, Error>
where
    F: FnMut() -> ResubscribeFuture<R> + Send + 'static + Clone,
    R: Send + 'static,
{
    loop {
        match sub_stream().await {
            Ok(v) => {
                let resubscribe = Box::new(move || {
                    let mut sub_stream = sub_stream.clone();
                    async move {
                        loop {
                            match sub_stream().await {
                                Ok(v) => return Ok(v),
                                Err(e) => {
                                    if e.is_disconnected_will_reconnect() {
                                        continue;
                                    }

                                    return Err(e);
                                }
                            }
                        }
                    }
                    .boxed()
                });

                // The extra Box is to encapsulate the retry subscription type
                return Ok(StreamOf::new(Box::pin(RetrySubscription {
                    state: PendingOrStream::Stream(v),
                    resubscribe,
                })));
            }
            Err(e) => {
                if e.is_disconnected_will_reconnect() {
                    continue;
                }

                return Err(e);
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
        let retry_stream = retry_stream(|| {
            async {
                Ok(StreamOf::new(Box::pin(futures::stream::iter([
                    Ok(1),
                    Err(disconnect_err()),
                ]))))
            }
            .boxed()
        })
        .await
        .unwrap();

        let result = retry_stream
            .take(3)
            .collect::<Vec<Result<usize, Error>>>()
            .await;

        assert!(matches!(result[0], Ok(r) if r == 1));
        assert!(matches!(result[1], Err(ref e) if e.is_disconnected_will_reconnect()));
        assert!(matches!(result[2], Ok(r) if r == 1));
    }

    #[tokio::test]
    async fn retry_sub_works() {
        let stream = futures::stream::iter([Ok(1), Err(disconnect_err())]);

        let resubscribe = Box::new(move || {
            async move { Ok(StreamOf::new(Box::pin(futures::stream::iter([Ok(2)])))) }.boxed()
        });

        let retry_stream = RetrySubscription {
            state: PendingOrStream::Stream(StreamOf::new(Box::pin(stream))),
            resubscribe,
        };

        let result: Vec<_> = retry_stream.collect().await;

        assert!(matches!(result[0], Ok(r) if r == 1));
        assert!(matches!(result[1], Err(ref e) if e.is_disconnected_will_reconnect()));
        assert!(matches!(result[2], Ok(r) if r == 2));
    }

    #[tokio::test]
    async fn retry_sub_err_terminates_stream() {
        let stream = futures::stream::iter([Ok(1)]);
        let resubscribe = Box::new(move || async move { Err(custom_err()) }.boxed());

        let retry_stream = RetrySubscription {
            state: PendingOrStream::Stream(StreamOf::new(Box::pin(stream))),
            resubscribe,
        };

        assert_eq!(retry_stream.count().await, 1);
    }

    #[tokio::test]
    async fn retry_sub_resubscribe_err() {
        let stream = futures::stream::iter([Ok(1), Err(disconnect_err())]);
        let resubscribe = Box::new(move || async move { Err(custom_err()) }.boxed());

        let retry_stream = RetrySubscription {
            state: PendingOrStream::Stream(StreamOf::new(Box::pin(stream))),
            resubscribe,
        };

        let result: Vec<_> = retry_stream.collect().await;

        assert!(matches!(result[0], Ok(r) if r == 1));
        assert!(matches!(result[1], Err(ref e) if e.is_disconnected_will_reconnect()));
        assert!(matches!(result[2], Err(ref e) if matches!(e, Error::Other(_))));
    }
}
