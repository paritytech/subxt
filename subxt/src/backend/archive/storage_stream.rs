use crate::config::{Config, HashFor, RpcConfigFor};
use crate::error::BackendError;
use futures::{FutureExt, Stream, StreamExt};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use subxt_rpcs::Error as RpcError;
use subxt_rpcs::methods::ChainHeadRpcMethods;
use subxt_rpcs::methods::chain_head::{
    ArchiveStorageEvent, ArchiveStorageEventItem, ArchiveStorageQuery, ArchiveStorageSubscription,
};

pub struct ArchiveStorageStream<T: Config> {
    at: HashFor<T>,
    methods: ChainHeadRpcMethods<RpcConfigFor<T>>,
    query_queue: VecDeque<ArchiveStorageQuery<Vec<u8>>>,
    state: Option<StreamState<T>>,
}

enum StreamState<T: Config> {
    GetSubscription {
        current_query: ArchiveStorageQuery<Vec<u8>>,
        sub_fut: Pin<
            Box<
                dyn Future<Output = Result<ArchiveStorageSubscription<HashFor<T>>, RpcError>>
                    + Send
                    + 'static,
            >,
        >,
    },
    RunSubscription {
        current_query: ArchiveStorageQuery<Vec<u8>>,
        sub: ArchiveStorageSubscription<HashFor<T>>,
    },
}

impl<T: Config> ArchiveStorageStream<T> {
    /// Fetch descendant keys.
    pub fn new(
        at: HashFor<T>,
        methods: ChainHeadRpcMethods<RpcConfigFor<T>>,
        query_queue: VecDeque<ArchiveStorageQuery<Vec<u8>>>,
    ) -> Self {
        Self {
            at,
            methods,
            query_queue,
            state: None,
        }
    }
}

impl<T: Config> std::marker::Unpin for ArchiveStorageStream<T> {}

impl<T: Config> Stream for ArchiveStorageStream<T> {
    type Item = Result<ArchiveStorageEventItem<HashFor<T>>, BackendError>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();

        loop {
            match this.state.take() {
                // No state yet so initialise!
                None => {
                    // Nothing left; we're done.
                    let Some(query) = this.query_queue.pop_front() else {
                        return Poll::Ready(None);
                    };

                    let at = this.at;
                    let methods = this.methods.clone();
                    let current_query = query.clone();
                    let sub_fut = async move {
                        let query = std::iter::once(ArchiveStorageQuery {
                            key: query.key.as_ref(),
                            query_type: query.query_type,
                            pagination_start_key: query.pagination_start_key.as_deref(),
                        });

                        methods.archive_v1_storage(at, query, None).await
                    };

                    this.state = Some(StreamState::GetSubscription {
                        current_query,
                        sub_fut: Box::pin(sub_fut),
                    });
                }
                // We're getting our subscription stream for the current query.
                Some(StreamState::GetSubscription {
                    current_query,
                    mut sub_fut,
                }) => {
                    match sub_fut.poll_unpin(cx) {
                        Poll::Ready(Ok(sub)) => {
                            this.state = Some(StreamState::RunSubscription { current_query, sub });
                        }
                        Poll::Ready(Err(e)) => {
                            if e.is_disconnected_will_reconnect() {
                                // Push the query back onto the queue to try again
                                this.query_queue.push_front(current_query);
                                continue;
                            }

                            this.state = None;
                            return Poll::Ready(Some(Err(e.into())));
                        }
                        Poll::Pending => {
                            this.state = Some(StreamState::GetSubscription {
                                current_query,
                                sub_fut,
                            });
                            return Poll::Pending;
                        }
                    }
                }
                // Running the subscription and returning results.
                Some(StreamState::RunSubscription {
                    current_query,
                    mut sub,
                }) => {
                    match sub.poll_next_unpin(cx) {
                        Poll::Ready(Some(Ok(val))) => {
                            match val {
                                ArchiveStorageEvent::Item(item) => {
                                    this.state = Some(StreamState::RunSubscription {
                                        current_query: ArchiveStorageQuery {
                                            key: current_query.key,
                                            query_type: current_query.query_type,
                                            // In the event of error, we resume from the last seen value.
                                            // At the time of writing, it's not clear if paginationStartKey
                                            // starts from the key itself or the first key after it:
                                            // https://github.com/paritytech/json-rpc-interface-spec/issues/176
                                            pagination_start_key: Some(item.key.0.clone()),
                                        },
                                        sub,
                                    });

                                    // We treat `paginationStartKey` as being the key we want results to begin _after_.
                                    // So, if we see a value that's <= it, ignore the value.
                                    let ignore_this_value = current_query
                                        .pagination_start_key
                                        .as_ref()
                                        .is_some_and(|k| item.key.0.cmp(k).is_le());

                                    if ignore_this_value {
                                        continue;
                                    }

                                    return Poll::Ready(Some(Ok(item)));
                                }
                                ArchiveStorageEvent::Error(e) => {
                                    this.state = None;
                                    return Poll::Ready(Some(Err(BackendError::other(e.error))));
                                }
                                ArchiveStorageEvent::Done => {
                                    this.state = None;
                                    continue;
                                }
                            }
                        }
                        Poll::Ready(Some(Err(e))) => {
                            if e.is_disconnected_will_reconnect() {
                                // Put the current query back into the queue and retry.
                                // We've been keeping it uptodate as needed.
                                this.query_queue.push_front(current_query);
                                this.state = None;
                                continue;
                            }

                            this.state = None;
                            return Poll::Ready(Some(Err(e.into())));
                        }
                        Poll::Ready(None) => {
                            this.state = None;
                            continue;
                        }
                        Poll::Pending => {
                            this.state = Some(StreamState::RunSubscription { current_query, sub });
                            return Poll::Pending;
                        }
                    }
                }
            }
        }
    }
}
