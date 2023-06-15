use futures::stream::StreamExt;
use futures_util::future::{self, Either};
use serde::Deserialize;
use serde_json::value::RawValue;
use std::{collections::HashMap, str::FromStr};
use tokio::sync::{mpsc, oneshot};

use super::LightClientError;
use smoldot_light::{platform::async_std::AsyncStdTcpWebSocket as Platform, ChainId};

const LOG_TARGET: &str = "light-client-background";

/// The response of an RPC method.
pub type MethodResponse = Result<Box<RawValue>, LightClientError>;

/// Message protocol between the front-end client that submits the RPC requests
/// and the backend handler that produces responses from the chain.
///
/// The light client uses a single object [`smoldot_light::JsonRpcResponses`] to
/// handle all requests and subscriptions from a chain. A background task is spawned
/// to multiplex the rpc responses and to provide them back to their rightful submitters.
#[derive(Debug)]
pub enum BackendMessage {
    /// The RPC method request.
    Request {
        /// The method of the request.
        method: String,
        /// The parameters of the request.
        params: String,
        /// Channel used to send back the result.
        sender: oneshot::Sender<MethodResponse>,
    },
    /// The RPC subscription (pub/sub) request.
    Subscription {
        /// The method of the request.
        method: String,
        /// The parameters of the request.
        params: String,
        /// Channel used to send back the subscription ID if successful.
        sub_id: oneshot::Sender<MethodResponse>,
        /// Channel used to send back the notifcations.
        sender: mpsc::Sender<Box<RawValue>>,
    },
}

/// Background task data.
pub struct BackgroundTask {
    /// Smoldot light client implementation that leverages the exposed platform.
    client: smoldot_light::Client<Platform>,
    /// The ID of the chain used to identify the chain protocol (ie. substrate).
    ///
    /// Note: A single chain is supported for a client. This aligns with the subxt's
    /// vision of the Client.
    chain_id: ChainId,
    /// Unique ID for RPC calls.
    request_id: usize,
    /// Map the request ID of a RPC method to the frontend `Sender`.
    requests: HashMap<String, oneshot::Sender<MethodResponse>>,
    /// Subscription calls first need to make a plain RPC method
    /// request to obtain the subscription ID.
    ///
    /// The RPC method request is made in the background and the response should
    /// not be sent back to the user.
    /// Map the request ID of a RPC method to the frontend `Sender`.
    id_to_subscription:
        HashMap<String, (oneshot::Sender<MethodResponse>, mpsc::Sender<Box<RawValue>>)>,
    /// Map the subscription ID to the frontend `Sender`.
    subscriptions: HashMap<String, mpsc::Sender<Box<RawValue>>>,
}

impl BackgroundTask {
    /// Constructs a new [`BackgroundTask`].
    pub fn new(client: smoldot_light::Client<Platform>, chain_id: ChainId) -> BackgroundTask {
        BackgroundTask {
            client,
            chain_id,
            request_id: 1,
            requests: Default::default(),
            id_to_subscription: Default::default(),
            subscriptions: Default::default(),
        }
    }

    /// Fetch and increment the request ID.
    fn next_id(&mut self) -> usize {
        let next = self.request_id;
        self.request_id = self.request_id.wrapping_add(1);
        next
    }

    /// Handle the registration messages received from the user.
    async fn handle_requests(&mut self, message: BackendMessage) {
        match message {
            BackendMessage::Request {
                method,
                params,
                sender,
            } => {
                let id = self.next_id();
                let request = format!(
                    r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":{}}}"#,
                    id, method, params
                );
                let id = id.to_string();

                self.requests.insert(id.clone(), sender);

                let result = self.client.json_rpc_request(request, self.chain_id);
                if let Err(err) = result {
                    tracing::warn!(
                        target: LOG_TARGET,
                        "Cannot send RPC request to lightclient {:?}",
                        err.to_string()
                    );
                    let sender = self
                        .requests
                        .remove(&id)
                        .expect("Channel is inserted above; qed");

                    // Send the error back to frontend.
                    if sender
                        .send(Err(LightClientError::Request(err.to_string())))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send RPC request error to id{:?}",
                            id
                        );
                    }
                }
            }
            BackendMessage::Subscription {
                method,
                params,
                sub_id,
                sender,
            } => {
                // For subscriptions we need to make a plain RPC request to the subscription method.
                // The server will return as a result the subscription ID.
                let id = self.next_id();
                let request = format!(
                    r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":{}}}"#,
                    id, method, params
                );
                let id = id.to_string();

                self.id_to_subscription.insert(id.clone(), (sub_id, sender));

                let result = self.client.json_rpc_request(request, self.chain_id);
                if let Err(err) = result {
                    tracing::warn!(
                        target: LOG_TARGET,
                        "Cannot send RPC request to lightclient {:?}",
                        err.to_string()
                    );
                    let (sub_id, _) = self
                        .id_to_subscription
                        .remove(&id)
                        .expect("Channels are inserted above; qed");

                    // Send the error back to frontend.
                    if sub_id
                        .send(Err(LightClientError::Request(err.to_string())))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send RPC request error to id{:?}",
                            id
                        );
                    }
                }
            }
        };
    }

    /// Parse the response received from the light client and sent it to the appropriate user.
    async fn handle_rpc_response(&mut self, response: String) {
        match RpcResponse::from_str(&response) {
            Ok(RpcResponse::Error { id, error }) => {
                if let Some(sender) = self.requests.remove(&id) {
                    if sender
                        .send(Err(LightClientError::Request(error.to_string())))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            " Cannot send method response to id {:?}",
                            id
                        );
                    }
                } else if let Some((sub_id_sender, _)) = self.id_to_subscription.remove(&id) {
                    if sub_id_sender
                        .send(Err(LightClientError::Request(error.to_string())))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            " Cannot send method response to id {:?}",
                            id
                        );
                    }
                }
            }
            Ok(RpcResponse::Method { id, result }) => {
                // Send the response back.
                if let Some(sender) = self.requests.remove(&id) {
                    if sender.send(Ok(result)).is_err() {
                        tracing::warn!(
                            target: LOG_TARGET,
                            " Cannot send method response to id {:?}",
                            id
                        );
                    }
                } else if let Some((sub_id_sender, sender)) = self.id_to_subscription.remove(&id) {
                    let mut sub_id = result.to_string();
                    sub_id.retain(|ch| ch.is_ascii_digit());
                    tracing::trace!(target: LOG_TARGET, "Received subscription ID: {}", sub_id);

                    if sub_id_sender.send(Ok(result)).is_err() {
                        tracing::warn!(
                            target: LOG_TARGET,
                            " Cannot send method response to id {:?}",
                            id
                        );
                    } else {
                        // Track this subscription ID if send is successful.
                        self.subscriptions.insert(sub_id, sender);
                    }
                }
            }
            Ok(RpcResponse::Subscription { method, id, result }) => {
                // Subxt calls into `author_submitAndWatchExtrinsic`, however the smoldot produces
                // `{"event":"broadcasted","numPeers":1}` and `{"event":"validated"}` which are part
                // of the RPC V2 API. Ignore those events.
                if method == "transaction_unstable_watchEvent"
                    && (result.to_string().contains("broadcasted")
                        || result.to_string().contains("validated"))
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

                        // Remove the sender if the subscription dropped the receiver.
                        self.subscriptions.remove(&id);
                    }
                }
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

                    self.handle_requests(message).await;

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
    Error {
        /// Response ID.
        id: String,
        /// Error.
        error: Box<RawValue>,
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
        #[derive(Deserialize)]
        struct ErrorResponse {
            /// JSON-RPC version.
            #[allow(unused)]
            jsonrpc: String,
            /// Request ID.
            id: String,
            /// Error.
            error: Box<RawValue>,
        }

        // Check if the response can be mapped as an RPC method response.
        let result: Result<Response, _> = serde_json::from_str(response);
        if let Ok(response) = result {
            return Ok(RpcResponse::Method {
                id: response.id,
                result: response.result,
            });
        }

        let result: Result<ResponseNotification, _> = serde_json::from_str(response);
        if let Ok(notification) = result {
            return Ok(RpcResponse::Subscription {
                id: notification.params.subscription,
                method: notification.method,
                result: notification.params.result,
            });
        }

        let error: ErrorResponse = serde_json::from_str(response)?;
        Ok(RpcResponse::Error {
            id: error.id,
            error: error.error,
        })
    }
}
