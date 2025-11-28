use crate::backend::utils::retry;
use crate::backend::StorageResponse;
use crate::config::{Config, HashFor, RpcConfigFor};
use crate::error::BackendError;
use futures::{Future, FutureExt, Stream, StreamExt};
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};
use super::LegacyRpcMethods;

/// This provides a stream of values given some prefix `key`. It
/// internally manages pagination and such.
#[allow(clippy::type_complexity)]
pub struct StorageFetchDescendantKeysStream<T: Config> {
    methods: LegacyRpcMethods<RpcConfigFor<T>>,
    key: Vec<u8>,
    at: HashFor<T>,
    // How many entries to ask for each time.
    storage_page_size: u32,
    // What key do we start paginating from? None = from the beginning.
    pagination_start_key: Option<Vec<u8>>,
    // Keys, future and cached:
    keys_fut:
        Option<Pin<Box<dyn Future<Output = Result<Vec<Vec<u8>>, BackendError>> + Send + 'static>>>,
    // Set to true when we're done:
    done: bool,
}

impl <T: Config> StorageFetchDescendantKeysStream<T> {
    /// Fetch descendant keys.
    pub fn new(
        methods: LegacyRpcMethods<RpcConfigFor<T>>,
        key: Vec<u8>,
        at: HashFor<T>,
        storage_page_size: u32,
    ) -> Self {
        StorageFetchDescendantKeysStream {
            methods,
            key,
            at,
            storage_page_size,
            pagination_start_key: None,
            keys_fut: None,
            done: false,
        }
    }
}

impl<T: Config> std::marker::Unpin for StorageFetchDescendantKeysStream<T> {}

impl<T: Config> Stream for StorageFetchDescendantKeysStream<T> {
    type Item = Result<Vec<Vec<u8>>, BackendError>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();
        loop {
            // We're already done.
            if this.done {
                return Poll::Ready(None);
            }

            // Poll future to fetch next keys.
            if let Some(mut keys_fut) = this.keys_fut.take() {
                match keys_fut.poll_unpin(cx) {
                    Poll::Ready(Ok(mut keys)) => {
                        if this.pagination_start_key.is_some()
                            && keys.first() == this.pagination_start_key.as_ref()
                        {
                            // Currently, Smoldot returns the "start key" as the first key in the input
                            // (see https://github.com/smol-dot/smoldot/issues/1692), whereas Substrate doesn't.
                            // We don't expect the start key to be returned either (since it was the last key of prev
                            // iteration), so remove it if we see it. This `remove()` method isn't very efficient but
                            // this will be a non issue with the RPC V2 APIs or if Smoldot aligns with Substrate anyway.
                            keys.remove(0);
                        }
                        if keys.is_empty() {
                            // No keys left; we're done!
                            this.done = true;
                            return Poll::Ready(None);
                        }
                        // The last key is where we want to paginate from next time.
                        this.pagination_start_key = keys.last().cloned();
                        // return all of the keys from this run.
                        return Poll::Ready(Some(Ok(keys)));
                    }
                    Poll::Ready(Err(e)) => {
                        if e.is_disconnected_will_reconnect() {
                            // Loop around and try again. No more keys_fut as it was taken,
                            // so we'll ask for the keys again from the last good pagination_start_key.
                            continue;
                        }

                        // Error getting keys? Return it.
                        return Poll::Ready(Some(Err(e)));
                    },
                    Poll::Pending => {
                        this.keys_fut = Some(keys_fut);
                        return Poll::Pending;
                    }
                }
            }

            // Else, we don't have a fut to get keys yet so start one going.
            let methods = this.methods.clone();
            let key = this.key.clone();
            let at = this.at;
            let storage_page_size = this.storage_page_size;
            let pagination_start_key = this.pagination_start_key.clone();
            let keys_fut = async move {
                let keys = methods
                    .state_get_keys_paged(
                        &key,
                        storage_page_size,
                        pagination_start_key.as_deref(),
                        Some(at),
                    )
                    .await?;
                Ok(keys)
            };
            this.keys_fut = Some(Box::pin(keys_fut));
        }
    }
}

/// This provides a stream of values given some stream of keys.
#[allow(clippy::type_complexity)]
pub struct StorageFetchDescendantValuesStream<T: Config> {
    // Stream of keys.
    keys_stream: StorageFetchDescendantKeysStream<T>,
    // Keys back from the stream which we are currently trying to fetch results for:
    keys: Vec<Vec<u8>>,
    // A future which will resolve to the resulting values:
    results_fut: Option<
        Pin<
            Box<
                dyn Future<Output = Result<Option<VecDeque<(Vec<u8>, Vec<u8>)>>, BackendError>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    // Once we get values back we put them here and hand them back one by one to the caller.
    results: VecDeque<(Vec<u8>, Vec<u8>)>,
}

impl <T: Config> StorageFetchDescendantValuesStream<T> {
    /// Fetch descendant values.
    pub fn new(
        methods: LegacyRpcMethods<RpcConfigFor<T>>,
        key: Vec<u8>,
        at: HashFor<T>,
        storage_page_size: u32,
    ) -> Self {
        StorageFetchDescendantValuesStream {
            keys_stream: StorageFetchDescendantKeysStream {
                methods,
                key,
                at,
                storage_page_size,
                pagination_start_key: None,
                keys_fut: None,
                done: false,
            },
            keys: Default::default(),
            results_fut: None,
            results: Default::default(),
        }
    }
}

impl<T: Config> Stream for StorageFetchDescendantValuesStream<T> {
    type Item = Result<StorageResponse, BackendError>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();

        loop {
            // If we have results back, return them one by one
            if let Some((key, value)) = this.results.pop_front() {
                let res = StorageResponse { key, value };
                return Poll::Ready(Some(Ok(res)));
            }

            // If we're waiting on the next results then poll that future:
            if let Some(mut results_fut) = this.results_fut.take() {
                match results_fut.poll_unpin(cx) {
                    Poll::Ready(Ok(Some(results))) => {
                        // Clear keys once result comes back.
                        this.keys = Vec::new();
                        this.results = results;
                        continue;
                    }
                    Poll::Ready(Ok(None)) => {
                        // Clear keys once result comes back.
                        this.keys = Vec::new();
                        // But no results back for these keys we we just skip them.
                        continue;
                    }
                    Poll::Ready(Err(e)) => {
                        if e.is_disconnected_will_reconnect() {
                            // Don't replace the `results_fut` since we got disconnected, and loop around.
                            // This will cause us to try re-fetching results for the current keys.
                            continue;
                        }

                        return Poll::Ready(Some(Err(e)))
                    }
                    Poll::Pending => {
                        this.results_fut = Some(results_fut);
                        return Poll::Pending;
                    }
                }
            }

            // If we have keys ready to fetch results for, then line up a results future to get them.
            // The keys stream handles disconnections internally for us.
            if !this.keys.is_empty() {
                let methods = this.keys_stream.methods.clone();
                let at = this.keys_stream.at;
                let keys = this.keys.clone();
                let results_fut = async move {
                    let keys = keys.iter().map(|k| &**k);
                    let values = retry(|| async {
                        let res = methods
                            .state_query_storage_at(keys.clone(), Some(at))
                            .await?;
                        Ok(res)
                    })
                    .await?;
                    let values: VecDeque<_> = values
                        .into_iter()
                        .flat_map(|v| {
                            v.changes.into_iter().filter_map(|(k, v)| {
                                let v = v?;
                                Some((k.0, v.0))
                            })
                        })
                        .collect();
                    Ok(Some(values))
                };

                this.results_fut = Some(Box::pin(results_fut));
                continue;
            }

            // We have no keys yet so wait for those first.
            match this.keys_stream.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(keys))) => {
                    this.keys = keys;
                    continue;
                }
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e))),
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
