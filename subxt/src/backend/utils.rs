//! RPC utils.

use super::{StreamOf, StreamOfResults};
use crate::error::Error;
use futures::future::BoxFuture;
use futures::{FutureExt, Stream, StreamExt};
use std::{future::Future, pin::Pin, task::Poll};

/// Resubscribe callback.
type ResubscribeGetter<T> = Box<dyn FnMut() -> ResubscribeFuture<T> + Send>;

/// Future that resolves to a subscription stream.
type ResubscribeFuture<T> = Pin<Box<dyn Future<Output = Result<StreamOfResults<T>, Error>> + Send>>;

pub(crate) enum PendingOrStream<T> {
    Pending(BoxFuture<'static, Result<StreamOfResults<T>, Error>>),
    Stream(StreamOfResults<T>),
}

impl<T> std::fmt::Debug for PendingOrStream<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PendingOrStream::Pending(_) => write!(f, "Pending"),
            PendingOrStream::Stream(_) => write!(f, "Stream"),
        }
    }
}

/// Retry subscription.
struct RetrySubscription<T> {
    resubscribe: ResubscribeGetter<T>,
    state: Option<PendingOrStream<T>>,
}

impl<T> std::marker::Unpin for RetrySubscription<T> {}

impl<T> Stream for RetrySubscription<T> {
    type Item = Result<T, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            let Some(mut this) = self.state.take() else {
                return Poll::Ready(None);
            };

            match this {
                PendingOrStream::Stream(ref mut s) => match s.poll_next_unpin(cx) {
                    Poll::Ready(Some(Err(err))) => {
                        if err.is_disconnected_will_reconnect() {
                            self.state = Some(PendingOrStream::Pending((self.resubscribe)()));
                        }
                        return Poll::Ready(Some(Err(err)));
                    }
                    Poll::Ready(None) => return Poll::Ready(None),
                    Poll::Ready(Some(Ok(val))) => {
                        self.state = Some(this);
                        return Poll::Ready(Some(Ok(val)));
                    }
                    Poll::Pending => {
                        self.state = Some(this);
                        return Poll::Pending;
                    }
                },
                PendingOrStream::Pending(mut fut) => match fut.poll_unpin(cx) {
                    Poll::Ready(Ok(stream)) => {
                        self.state = Some(PendingOrStream::Stream(stream));
                        continue;
                    }
                    Poll::Ready(Err(err)) => {
                        if err.is_disconnected_will_reconnect() {
                            self.state = Some(PendingOrStream::Pending((self.resubscribe)()));
                        }
                        return Poll::Ready(Some(Err(err)));
                    }
                    Poll::Pending => {
                        self.state = Some(PendingOrStream::Pending(fut));
                        return Poll::Pending;
                    }
                },
            };
        }
    }
}

/// Retry a future until it doesn't return a disconnected error.
///
/// # Example
///
/// ```no_run
/// use subxt::backend::utils::retry;
///
/// async fn some_future() -> Result<(), subxt::error::Error> {
///    Ok(())
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let result = retry(|| some_future()).await;
/// }
/// ```
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
                // This is a hack because, in the event of a disconnection,
                // we may not get the correct subscription ID back on reconnecting.
                //
                // This is because we have a race between this future and the
                // separate chainHead subscription, which runs in a different task.
                // if this future is too quick, it'll be given back an old
                // subscription ID from the chainHead subscription which has yet
                // to reconnect and establish a new subscription ID.
                //
                // In the event of a wrong subscription Id being used, we happen to
                // hand back an `RpcError::LimitReached`, and so can retry when we
                // specifically hit that error to see if we get a new subscription ID
                // eventually.
                if e.is_rpc_limit_reached() && rejected_retries < REJECTED_MAX_RETRIES {
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
///
/// # Example
///
/// ```no_run
/// use subxt::backend::{utils::retry_stream, StreamOf};
/// use futures::future::FutureExt;
///
/// #[tokio::main]
/// async fn main() {
///    retry_stream(|| {
///         // This needs to return a stream of results but if you are using
///         // the subxt backend already it will return StreamOf so you can just
///         // return it directly in the async block below.
///         async move { Ok(StreamOf::new(Box::pin(futures::stream::iter([Ok(2)])))) }.boxed()
///    }).await;
/// }
/// ```
pub async fn retry_stream<F, R>(sub_stream: F) -> Result<StreamOfResults<R>, Error>
where
    F: FnMut() -> ResubscribeFuture<R> + Send + 'static + Clone,
    R: Send + 'static,
{
    let stream = retry(sub_stream.clone()).await?;

    let resubscribe = Box::new(move || {
        let sub_stream = sub_stream.clone();
        async move { retry(sub_stream).await }.boxed()
    });

    // The extra Box is to encapsulate the retry subscription type
    Ok(StreamOf::new(Box::pin(RetrySubscription {
        state: Some(PendingOrStream::Stream(stream)),
        resubscribe,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::StreamOf;

    fn disconnect_err() -> Error {
        Error::Rpc(subxt_rpcs::Error::DisconnectedWillReconnect(String::new()).into())
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
                    Ok(2),
                    Ok(3),
                    Err(disconnect_err()),
                ]))))
            }
            .boxed()
        })
        .await
        .unwrap();

        let result = retry_stream
            .take(5)
            .collect::<Vec<Result<usize, Error>>>()
            .await;

        assert!(matches!(result[0], Ok(r) if r == 1));
        assert!(matches!(result[1], Ok(r) if r == 2));
        assert!(matches!(result[2], Ok(r) if r == 3));
        assert!(matches!(result[3], Err(ref e) if e.is_disconnected_will_reconnect()));
        assert!(matches!(result[4], Ok(r) if r == 1));
    }

    #[tokio::test]
    async fn retry_sub_works() {
        let stream = futures::stream::iter([Ok(1), Err(disconnect_err())]);

        let resubscribe = Box::new(move || {
            async move { Ok(StreamOf::new(Box::pin(futures::stream::iter([Ok(2)])))) }.boxed()
        });

        let retry_stream = RetrySubscription {
            state: Some(PendingOrStream::Stream(StreamOf::new(Box::pin(stream)))),
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
            state: Some(PendingOrStream::Stream(StreamOf::new(Box::pin(stream)))),
            resubscribe,
        };

        assert_eq!(retry_stream.count().await, 1);
    }

    #[tokio::test]
    async fn retry_sub_resubscribe_err() {
        let stream = futures::stream::iter([Ok(1), Err(disconnect_err())]);
        let resubscribe = Box::new(move || async move { Err(custom_err()) }.boxed());

        let retry_stream = RetrySubscription {
            state: Some(PendingOrStream::Stream(StreamOf::new(Box::pin(stream)))),
            resubscribe,
        };

        let result: Vec<_> = retry_stream.collect().await;

        assert!(matches!(result[0], Ok(r) if r == 1));
        assert!(matches!(result[1], Err(ref e) if e.is_disconnected_will_reconnect()));
        assert!(matches!(result[2], Err(ref e) if matches!(e, Error::Other(_))));
    }
}
