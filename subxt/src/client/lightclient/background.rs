// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use futures::stream::StreamExt;
use futures_util::future::{self, Either};
use serde::Deserialize;
use serde_json::value::RawValue;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tokio::sync::{mpsc, oneshot};

use super::LightClientError;
use smoldot_light::{platform::default::DefaultPlatform as Platform, ChainId};

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
pub enum FromSubxt {
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
        sender: mpsc::UnboundedSender<Box<RawValue>>,
    },
}

/// Background task data.
pub struct BackgroundTask {
    /// Smoldot light client implementation that leverages the exposed platform.
    client: smoldot_light::Client<Arc<Platform>>,
    /// The ID of the chain used to identify the chain protocol (ie. substrate).
    ///
    /// Note: A single chain is supported for a client. This aligns with the subxt's
    /// vision of the Client.
    chain_id: ChainId,
    /// Unique ID for RPC calls.
    request_id: usize,
    /// Map the request ID of a RPC method to the frontend `Sender`.
    requests: HashMap<usize, oneshot::Sender<MethodResponse>>,
    /// Subscription calls first need to make a plain RPC method
    /// request to obtain the subscription ID.
    ///
    /// The RPC method request is made in the background and the response should
    /// not be sent back to the user.
    /// Map the request ID of a RPC method to the frontend `Sender`.
    id_to_subscription: HashMap<
        usize,
        (
            oneshot::Sender<MethodResponse>,
            mpsc::UnboundedSender<Box<RawValue>>,
        ),
    >,
    /// Map the subscription ID to the frontend `Sender`.
    subscriptions: HashMap<usize, mpsc::UnboundedSender<Box<RawValue>>>,
}

impl BackgroundTask {
    /// Constructs a new [`BackgroundTask`].
    pub fn new(client: smoldot_light::Client<Arc<Platform>>, chain_id: ChainId) -> BackgroundTask {
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
    async fn handle_requests(&mut self, message: FromSubxt) {
        match message {
            FromSubxt::Request {
                method,
                params,
                sender,
            } => {
                let id = self.next_id();
                let request = format!(
                    r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":{}}}"#,
                    id, method, params
                );

                self.requests.insert(id, sender);

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
                            "Cannot send RPC request error to id={id}",
                        );
                    }
                }
            }
            FromSubxt::Subscription {
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

                self.id_to_subscription.insert(id, (sub_id, sender));

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
                            "Cannot send RPC request error to id={id}",
                        );
                    }
                }
            }
        };
    }

    /// Parse the response received from the light client and sent it to the appropriate user.
    fn handle_rpc_response(&mut self, response: String) {
        match RpcResponse::from_str(&response) {
            Ok(RpcResponse::Error { id, error }) => {
                let Ok(id) = id.parse::<usize>() else {
                    tracing::warn!(target: LOG_TARGET, "Cannot send error. Id={id} is not a valid number");
                    return
                };

                if let Some(sender) = self.requests.remove(&id) {
                    if sender
                        .send(Err(LightClientError::Request(error.to_string())))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send method response to id={id}",
                        );
                    }
                } else if let Some((sub_id_sender, _)) = self.id_to_subscription.remove(&id) {
                    if sub_id_sender
                        .send(Err(LightClientError::Request(error.to_string())))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send method response to id {:?}",
                            id
                        );
                    }
                }
            }
            Ok(RpcResponse::Method { id, result }) => {
                let Ok(id) = id.parse::<usize>() else {
                    tracing::warn!(target: LOG_TARGET, "Cannot send response. Id={id} is not a valid number");
                    return
                };

                // Send the response back.
                if let Some(sender) = self.requests.remove(&id) {
                    if sender.send(Ok(result)).is_err() {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send method response to id={id}",
                        );
                    }
                } else if let Some((sub_id_sender, sender)) = self.id_to_subscription.remove(&id) {
                    let Ok(sub_id) = result
                        .get()
                        .trim_start_matches('"')
                        .trim_end_matches('"')
                        .parse::<usize>() else {
                            tracing::warn!(
                                target: LOG_TARGET,
                                "Subscription id={result} is not a valid number",
                            );
                            return;
                    };

                    tracing::trace!(target: LOG_TARGET, "Received subscription id={sub_id}");

                    if sub_id_sender.send(Ok(result)).is_err() {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send method response to id={id}",
                        );
                    } else {
                        // Track this subscription ID if send is successful.
                        self.subscriptions.insert(sub_id, sender);
                    }
                }
            }
            Ok(RpcResponse::Subscription { method, id, result }) => {
                let Ok(id) = id.parse::<usize>() else {
                    tracing::warn!(target: LOG_TARGET, "Cannot send subscription. Id={id} is not a valid number");
                    return
                };

                if let Some(sender) = self.subscriptions.get_mut(&id) {
                    // Send the current notification response.
                    if sender.send(result).is_err() {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send notification to subscription id={id} method={method}",
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
    /// - receiving requests from subxt RPC method / subscriptions
    /// - provides the results from the light client back to users.
    pub async fn start_task(
        &mut self,
        from_subxt: mpsc::UnboundedReceiver<FromSubxt>,
        from_node: smoldot_light::JsonRpcResponses,
    ) {
        let from_subxt_event = tokio_stream::wrappers::UnboundedReceiverStream::new(from_subxt);
        let from_node_event = futures_util::stream::unfold(from_node, |mut from_node| async {
            from_node.next().await.map(|result| (result, from_node))
        });

        tokio::pin!(from_subxt_event, from_node_event);

        let mut from_subxt_event_fut = from_subxt_event.next();
        let mut from_node_event_fut = from_node_event.next();

        loop {
            match future::select(from_subxt_event_fut, from_node_event_fut).await {
                // Message received from subxt.
                Either::Left((subxt_message, previous_fut)) => {
                    let Some(message) = subxt_message else {
                        tracing::trace!(target: LOG_TARGET, "Subxt channel closed");
                        break;
                    };
                    tracing::trace!(
                        target: LOG_TARGET,
                        "Received register message {:?}",
                        message
                    );

                    self.handle_requests(message).await;

                    from_subxt_event_fut = from_subxt_event.next();
                    from_node_event_fut = previous_fut;
                }
                // Message received from rpc handler: lightclient response.
                Either::Right((node_message, previous_fut)) => {
                    // Smoldot returns `None` if the chain has been removed (which subxt does not remove).
                    let Some(response) = node_message else {
                        tracing::trace!(target: LOG_TARGET, "Smoldot RPC responses channel closed");
                        break;
                    };
                    tracing::trace!(
                        target: LOG_TARGET,
                        "Received smoldot RPC result {:?}",
                        response
                    );

                    self.handle_rpc_response(response);

                    // Advance backend, save frontend.
                    from_subxt_event_fut = previous_fut;
                    from_node_event_fut = from_node_event.next();
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
