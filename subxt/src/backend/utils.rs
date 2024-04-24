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
    const REJECTED_MAX_RETRIES: usize = 10;
    let mut rejected_retries = 0;

    loop {
        match retry_future().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                if e.is_disconnected_will_reconnect() {
                    continue;
                }

                // This applies only to the rpc v2 method calls/unstable backend.
                //
                // Once the backend sees `Error::DisconnectedWillReconnect` it will
                // update the state, and subsequent `subscribe` will not proceed until
                // a new subscription ID is obtained.
                //
                // It's still possible that subscription ID could be read before
                // the backend has updated the state, but it is quite unlikely.
                //
                // Thus, we retry the call a few times to be on the safe-side.
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
    use futures::TryStreamExt;

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

        let result: Vec<_> = retry_stream.take(2).try_collect().await.unwrap();

        assert_eq!(result, vec![1, 1], "Disconnect error should be ignored");
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

        assert_eq!(
            result,
            vec![1, 2],
            "Stream should contain values from both subscription and resubscription"
        );
    }

    #[tokio::test]
    async fn retry_sub_resubscribe_err_terminates_stream() {
        let stream = futures::stream::iter([Ok(1)]);
        let resubscribe = Box::new(move || async move { Err(custom_err()) }.boxed());

        let retry_stream = RetrySubscription {
            state: PendingOrStream::Stream(StreamOf::new(Box::pin(stream))),
            resubscribe,
        };
        assert_eq!(
            1,
            retry_stream.count().await,
            "Stream should terminate after custom error"
        );
    }
}
