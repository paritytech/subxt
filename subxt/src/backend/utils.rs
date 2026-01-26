//! Backend utils.

use super::{StreamOf, StreamOfResults};
use crate::error::BackendError;
use futures::{FutureExt, Stream, StreamExt};
use std::{future::Future, pin::Pin, task::Poll};

/// Spawn a task.
///
/// - On non-wasm targets, this will spawn a task via [`tokio::spawn`].
/// - On wasm targets, this will spawn a task via [`wasm_bindgen_futures::spawn_local`].
#[cfg(feature = "runtime")]
pub(crate) fn spawn<F: std::future::Future + Send + 'static>(future: F) {
    #[cfg(not(target_family = "wasm"))]
    tokio::spawn(async move {
        future.await;
    });
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    wasm_bindgen_futures::spawn_local(async move {
        future.await;
    });
}

/// Retry a future until it doesn't return a disconnected error.
///
/// # Example
///
/// ```rust,no_run,standalone_crate
/// use subxt::backend::utils::retry;
///
/// async fn some_future() -> Result<(), subxt::error::BackendError> {
///    Ok(())
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let result = retry(|| some_future()).await;
/// }
/// ```
pub async fn retry<T, F, R>(mut retry_future: F) -> Result<R, BackendError>
where
    F: FnMut() -> T,
    T: Future<Output = Result<R, BackendError>>,
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
/// ```rust,no_run,standalone_crate
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
pub async fn retry_stream<F, Fut, R>(get_stream: F) -> Result<StreamOfResults<R>, BackendError>
where
    F: Clone + Send + 'static + FnMut() -> Fut,
    Fut: Future<Output = Result<StreamOfResults<R>, BackendError>> + Send,
    R: Send + 'static,
{
    // This returns the stream. On disconnect this is called again.
    let get_stream_with_retry = move || {
        let get_stream = get_stream.clone();
        async move { retry(get_stream).await }.boxed()
    };

    // The extra Box is to encapsulate the retry subscription type
    Ok(StreamOf::new(Box::pin(RetrySubscription {
        state: RetrySubscriptionState::Init,
        resubscribe: get_stream_with_retry,
    })))
}

/// Retry subscription.
struct RetrySubscription<F, R, T> {
    resubscribe: F,
    state: RetrySubscriptionState<R, T>,
}

enum RetrySubscriptionState<R, T> {
    Init,
    Pending(R),
    Stream(StreamOfResults<T>),
    Done,
}

impl<F, R, T> std::marker::Unpin for RetrySubscription<F, R, T> {}

impl<F, R, T> Stream for RetrySubscription<F, R, T>
where
    F: FnMut() -> R,
    R: Future<Output = Result<StreamOfResults<T>, BackendError>> + Unpin,
{
    type Item = Result<T, BackendError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            match &mut self.state {
                RetrySubscriptionState::Init => {
                    self.state = RetrySubscriptionState::Pending((self.resubscribe)());
                }
                RetrySubscriptionState::Stream(s) => match s.poll_next_unpin(cx) {
                    Poll::Ready(Some(Err(err))) => {
                        if err.is_disconnected_will_reconnect() {
                            self.state = RetrySubscriptionState::Init;
                        }
                        return Poll::Ready(Some(Err(err)));
                    }
                    Poll::Ready(None) => {
                        self.state = RetrySubscriptionState::Done;
                        return Poll::Ready(None);
                    }
                    Poll::Ready(Some(Ok(val))) => {
                        return Poll::Ready(Some(Ok(val)));
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                },
                RetrySubscriptionState::Pending(fut) => match fut.poll_unpin(cx) {
                    Poll::Ready(Err(err)) => {
                        if err.is_disconnected_will_reconnect() {
                            self.state = RetrySubscriptionState::Init;
                        }
                        self.state = RetrySubscriptionState::Done;
                        return Poll::Ready(Some(Err(err)));
                    }
                    Poll::Ready(Ok(stream)) => {
                        self.state = RetrySubscriptionState::Stream(stream);
                        continue;
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                },
                RetrySubscriptionState::Done => return Poll::Ready(None),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::StreamOf;

    fn disconnect_err() -> BackendError {
        BackendError::Rpc(subxt_rpcs::Error::DisconnectedWillReconnect(String::new()).into())
    }

    fn custom_err() -> BackendError {
        BackendError::other("")
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
            .collect::<Vec<Result<usize, BackendError>>>()
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
            state: RetrySubscriptionState::Stream(StreamOf::new(Box::pin(stream))),
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
        let resubscribe = Box::new(|| async move { Err(custom_err()) }.boxed());

        let retry_stream = RetrySubscription {
            state: RetrySubscriptionState::Stream(StreamOf::new(Box::pin(stream))),
            resubscribe,
        };

        assert_eq!(retry_stream.count().await, 1);
    }

    #[tokio::test]
    async fn retry_sub_resubscribe_err() {
        let stream = futures::stream::iter([Ok(1), Err(disconnect_err())]);
        let resubscribe = Box::new(|| async move { Err(custom_err()) }.boxed());

        let retry_stream = RetrySubscription {
            state: RetrySubscriptionState::Stream(StreamOf::new(Box::pin(stream))),
            resubscribe,
        };

        let result: Vec<_> = retry_stream.collect().await;

        assert!(matches!(result[0], Ok(r) if r == 1));
        assert!(matches!(result[1], Err(ref e) if e.is_disconnected_will_reconnect()));
        assert!(matches!(result[2], Err(ref e) if matches!(e, BackendError::Other(_))));
    }
}
