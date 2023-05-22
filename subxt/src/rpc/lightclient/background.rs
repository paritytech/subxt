use futures::stream::StreamExt;
use futures_util::future::{self, Either};
use serde::Deserialize;
use serde_json::value::RawValue;
use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};
use tokio::sync::{mpsc, oneshot};

///! The background task of the light client.

/// The number of notifications that are buffered, before the user
/// registers to [`LightClientInner::register_subscription`].
/// Only the first `BUFFER_NUM_NOTIFICATIONS` are buffered, while the
/// others are ignored.
///
/// In practice, the Light Client produces notifications at a lower rate
/// than the substrate full node.
const BUFFER_NUM_NOTIFICATIONS: usize = 16;

const LOG_TARGET: &str = "light-client-background";

/// Message protocol between the front-end client that submits the RPC requests
/// and the backend handler that produces responses from the chain.
///
/// The light client uses a single object [`smoldot_light::JsonRpcResponses`] to
/// handle all requests and subscriptions from a chain. A background task is spawned
/// to multiplex the rpc responses and to provide them back to their rightful submitters.
///
/// This presumes that the request ID for both method calls and subscriptions is unique.
#[derive(Debug)]
pub enum BackendMessage {
    /// The RPC method request.
    Request {
        /// ID of the request, needed to match the result.
        id: String,
        /// Channel used to send back the result.
        sender: oneshot::Sender<Box<RawValue>>,
    },
    /// The RPC subscription (pub/sub) request.
    Subscription {
        /// ID of the subscription, needed to match the result.
        id: String,
        /// Channel used to send back the notifcations.
        sender: mpsc::Sender<Box<RawValue>>,
    },
}

/// Background task data.
#[derive(Default)]
pub struct BackgroundTask {
    /// Map the request ID of a RPC method to the frontend `Sender`.
    requests: HashMap<String, oneshot::Sender<Box<RawValue>>>,
    /// Map the subscription ID to the frontend `Sender`.
    subscriptions: HashMap<String, mpsc::Sender<Box<RawValue>>>,
    /// Notifications are cached for users that did not subscribe yet.
    subscriptions_cache: HashMap<String, Vec<Box<RawValue>>>,
}

impl BackgroundTask {
    /// Constructs a new [`BackgroundTask`].
    pub fn new() -> BackgroundTask {
        BackgroundTask::default()
    }

    /// Handle the registration messages received from the user.
    async fn handle_register(&mut self, message: BackendMessage) {
        match message {
            BackendMessage::Request { id, sender } => {
                self.requests.insert(id, sender);
            }
            BackendMessage::Subscription { id, sender } => {
                // Drain the subscription cache, that holds messages that
                // were not propagated to this subscription yet (because the
                // RPC server produced a notification before the user registered
                // to receive notifications).
                if let Some(cached_responses) = self.subscriptions_cache.remove(&id) {
                    tracing::debug!(
                        target: LOG_TARGET,
                        "Some messages were cached before susbcribing"
                    );

                    for response in cached_responses {
                        if sender.send(response).await.is_err() {
                            tracing::warn!(
                                target: LOG_TARGET,
                                "Cannot send notification to subscription {:?}",
                                id
                            );
                        }
                    }
                }

                self.subscriptions.insert(id, sender);
            }
        };
    }

    /// Parse the response received from the light client and sent it to the appropriate user.
    async fn handle_rpc_response(&mut self, response: String) {
        match RpcResponse::from_str(&response) {
            Ok(RpcResponse::Method { id, result }) => {
                // Send the response back.
                if let Some(sender) = self.requests.remove(&id) {
                    if sender.send(result).is_err() {
                        tracing::warn!(
                            target: LOG_TARGET,
                            " Cannot send method response to id {:?}",
                            id
                        );
                    }
                }
            }
            Ok(RpcResponse::Subscription { method, id, result }) => {
                // Subxt calls into `author_submitAndWatchExtrinsic`, however the smoldot produces
                // `{"event":"broadcasted","numPeers":1}` which is part of the RPC V2 API. Ignore
                // this spurious event.
                if method == "transaction_unstable_watchEvent"
                    && result.to_string().contains("broadcasted")
                {
                    tracing::debug!(target: LOG_TARGET, "Ignoring notification {:?}", result);
                    return;
                }

                if let Some(sender) = self.subscriptions.get_mut(&id) {
                    // Send the current notification response.
                    if sender.send(result).await.is_err() {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send notification to subscription {:?}",
                            id
                        );
                    }
                    return;
                }

                // Subscription ID not registered yet, cache the response.
                // Note: Compiler complains about moving the `result` for `.entry.and_modify(..).or_insert(..)`,
                // because it sees it as used on both closures and apparently cannot determine that only one
                // closure can be executed.
                match self.subscriptions_cache.entry(id) {
                    Entry::Occupied(mut entry) => {
                        let cached_responses: &mut Vec<_> = entry.get_mut();
                        // Do not cache notification if exceeded capacity.
                        if cached_responses.len() > BUFFER_NUM_NOTIFICATIONS {
                            return;
                        }

                        cached_responses.push(result);
                    }
                    Entry::Vacant(entry) => {
                        let mut vec = Vec::with_capacity(BUFFER_NUM_NOTIFICATIONS);
                        vec.push(result);
                        entry.insert(vec);
                    }
                };
            }
            Err(err) => {
                tracing::warn!(target: LOG_TARGET, "cannot decode RPC response {:?}", err);
            }
        }
    }

    /// Perform the main background task:
    /// - receiving user registration for RPC method / subscriptions
    /// - providing the results from the light client back to users.
    pub async fn start_task(
        &mut self,
        backend: mpsc::Receiver<BackendMessage>,
        rpc_responses: smoldot_light::JsonRpcResponses,
    ) {
        let backend_event = tokio_stream::wrappers::ReceiverStream::new(backend);
        let rpc_responses_event =
            futures_util::stream::unfold(rpc_responses, |mut rpc_responses| async {
                rpc_responses
                    .next()
                    .await
                    .map(|result| (result, rpc_responses))
            });

        tokio::pin!(backend_event, rpc_responses_event);

        let mut backend_event_fut = backend_event.next();
        let mut rpc_responses_fut = rpc_responses_event.next();

        loop {
            match future::select(backend_event_fut, rpc_responses_fut).await {
                // Message received from the backend: user registered.
                Either::Left((backend_value, previous_fut)) => {
                    let Some(message) = backend_value else {
                        tracing::trace!(target: LOG_TARGET, "Frontend channel closed");
                        break;
                    };
                    tracing::trace!(
                        target: LOG_TARGET,
                        "Received register message {:?}",
                        message
                    );

                    self.handle_register(message).await;

                    backend_event_fut = backend_event.next();
                    rpc_responses_fut = previous_fut;
                }
                // Message received from rpc handler: lightclient response.
                Either::Right((response, previous_fut)) => {
                    // Smoldot returns `None` if the chain has been removed (which subxt does not remove).
                    let Some(response) = response else {
                        tracing::trace!(target: LOG_TARGET, "Smoldot RPC responses channel closed");
                        break;
                    };
                    tracing::trace!(
                        target: LOG_TARGET,
                        "Received smoldot RPC result {:?}",
                        response
                    );

                    self.handle_rpc_response(response).await;

                    // Advance backend, save frontend.
                    backend_event_fut = previous_fut;
                    rpc_responses_fut = rpc_responses_event.next();
                }
            }
        }

        tracing::trace!(target: LOG_TARGET, "Task closed");
    }
}

/// The RPC response from the light-client.
/// This can either be a response of a method, or a notification from a subscription.
#[derive(Debug, Clone)]
enum RpcResponse {
    Method {
        /// Response ID.
        id: String,
        /// The result of the method call.
        result: Box<RawValue>,
    },
    Subscription {
        /// RPC method that generated the notification.
        method: String,
        /// Subscription ID.
        id: String,
        /// Result.
        result: Box<RawValue>,
    },
}

impl std::str::FromStr for RpcResponse {
    type Err = serde_json::Error;

    fn from_str(response: &str) -> Result<Self, Self::Err> {
        // Helper structures to deserialize from raw RPC strings.
        #[derive(Deserialize, Debug)]
        struct Response {
            /// JSON-RPC version.
            #[allow(unused)]
            jsonrpc: String,
            /// Result.
            result: Box<RawValue>,
            /// Request ID
            id: String,
        }
        #[derive(Deserialize)]
        struct NotificationParams {
            /// The ID of the subscription.
            subscription: String,
            /// Result.
            result: Box<RawValue>,
        }
        #[derive(Deserialize)]
        struct ResponseNotification {
            /// JSON-RPC version.
            #[allow(unused)]
            jsonrpc: String,
            /// RPC method that generated the notification.
            method: String,
            /// Result.
            params: NotificationParams,
        }

        // Check if the response can be mapped as an RPC method response.
        let result: Result<Response, _> = serde_json::from_str(response);
        if let Ok(response) = result {
            return Ok(RpcResponse::Method {
                id: response.id,
                result: response.result,
            });
        }

        let notification: ResponseNotification = serde_json::from_str(response)?;
        Ok(RpcResponse::Subscription {
            id: notification.params.subscription,
            method: notification.method,
            result: notification.params.result,
        })
    }
}
