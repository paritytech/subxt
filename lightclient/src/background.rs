// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::rpc::RpcResponse;
use crate::shared_client::SharedClient;
use crate::{JsonRpcError, LightClientRpcError};
use futures::{stream::StreamExt, FutureExt};
use serde_json::value::RawValue;
use smoldot_light::platform::PlatformRef;
use std::{collections::HashMap, str::FromStr};
use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::UnboundedReceiverStream;

const LOG_TARGET: &str = "subxt-light-client-background-task";

/// The response of an RPC method.
pub type MethodResponse = Result<Box<RawValue>, LightClientRpcError>;

/// The response back from an RPC subscription call; a subscription ID and channel that
/// subscription notifications will be received on.
pub type SubscriptionResponse = Result<
    (
        SubscriptionId,
        mpsc::UnboundedReceiver<Result<Box<RawValue>, JsonRpcError>>,
    ),
    LightClientRpcError,
>;

/// Type of subscription IDs we can get back.
pub type SubscriptionId = String;

/// Message protocol between the front-end client that submits the RPC requests
/// and the background task which fetches responses from Smoldot.
#[derive(Debug)]
pub enum Message {
    /// The RPC method request.
    Request {
        /// The method of the request.
        method: String,
        /// The parameters of the request.
        params: Option<Box<RawValue>>,
        /// Channel used to send back the method response.
        sender: oneshot::Sender<MethodResponse>,
    },
    /// The RPC subscription (pub/sub) request.
    Subscription {
        /// The method of the request.
        method: String,
        /// The method to unsubscribe.
        unsubscribe_method: String,
        /// The parameters of the request.
        params: Option<Box<RawValue>>,
        /// Channel used to send back the subscription response.
        sender: oneshot::Sender<SubscriptionResponse>,
    },
}

/// A handle to communicate with the background task.
pub struct BackgroundTaskHandle {
    to_backend: mpsc::UnboundedSender<Message>,
}

impl BackgroundTaskHandle {
    /// Make an RPC request via the background task.
    pub async fn request(&self, method: String, params: Option<Box<RawValue>>) -> MethodResponse {
        let (tx, rx) = oneshot::channel();
        self.to_backend
            .send(Message::Request {
                method,
                params,
                sender: tx,
            })
            .map_err(|_e| LightClientRpcError::BackgroundTaskDropped)?;

        match rx.await {
            Err(_e) => Err(LightClientRpcError::BackgroundTaskDropped),
            Ok(response) => response,
        }
    }

    /// Subscribe to some RPC method via the background task.
    pub async fn subscribe(
        &self,
        method: String,
        params: Option<Box<RawValue>>,
        unsubscribe_method: String,
    ) -> SubscriptionResponse {
        let (tx, rx) = oneshot::channel();
        self.to_backend
            .send(Message::Subscription {
                method,
                params,
                unsubscribe_method,
                sender: tx,
            })
            .map_err(|_e| LightClientRpcError::BackgroundTaskDropped)?;

        match rx.await {
            Err(_e) => Err(LightClientRpcError::BackgroundTaskDropped),
            Ok(response) => response,
        }
    }
}

/// A background task which runs with [`BackgroundTask::run()`] and manages messages
/// coming to/from Smoldot.
#[allow(clippy::type_complexity)]
pub struct BackgroundTask<TPlatform: PlatformRef, TChain> {
    channels: BackgroundTaskChannels,
    data: BackgroundTaskData<TPlatform, TChain>,
}

impl<TPlatform: PlatformRef, TChain> BackgroundTask<TPlatform, TChain> {
    /// Constructs a new [`BackgroundTask`].
    pub(crate) fn new(
        client: SharedClient<TPlatform, TChain>,
        chain_id: smoldot_light::ChainId,
        from_back: smoldot_light::JsonRpcResponses,
    ) -> (BackgroundTask<TPlatform, TChain>, BackgroundTaskHandle) {
        let (tx, rx) = mpsc::unbounded_channel();

        let bg_task = BackgroundTask {
            channels: BackgroundTaskChannels {
                from_front: UnboundedReceiverStream::new(rx),
                from_back,
            },
            data: BackgroundTaskData {
                client,
                chain_id,
                last_request_id: 0,
                pending_subscriptions: HashMap::new(),
                requests: HashMap::new(),
                subscriptions: HashMap::new(),
            },
        };

        let bg_handle = BackgroundTaskHandle { to_backend: tx };

        (bg_task, bg_handle)
    }

    /// Run the background task, which:
    /// - Forwards messages/subscription requests to Smoldot from the front end.
    /// - Forwards responses back from Smoldot to the front end.
    pub async fn run(self) {
        let chain_id = self.data.chain_id;
        let mut channels = self.channels;
        let mut data = self.data;

        loop {
            tokio::pin! {
                let from_front_fut = channels.from_front.next().fuse();
                let from_back_fut = channels.from_back.next().fuse();
            }

            futures::select! {
                // Message coming from the front end/client.
                front_message = from_front_fut => {
                    let Some(message) = front_message else {
                        tracing::trace!(target: LOG_TARGET, "Subxt channel closed");
                        break;
                    };
                    tracing::trace!(
                        target: LOG_TARGET,
                        "Received register message {:?}",
                        message
                    );

                    data.handle_requests(message).await;
                },
                // Message coming from Smoldot.
                back_message = from_back_fut => {
                    let Some(back_message) = back_message else {
                        tracing::trace!(target: LOG_TARGET, "Smoldot RPC responses channel closed");
                        break;
                    };
                    tracing::trace!(
                        target: LOG_TARGET,
                        "Received smoldot RPC chain {:?} result {:?}",
                        chain_id, back_message
                    );

                    data.handle_rpc_response(back_message);
                }
            }
        }

        tracing::trace!(target: LOG_TARGET, "Task closed");
    }
}

struct BackgroundTaskChannels {
    /// Messages sent into this background task from the front end.
    from_front: UnboundedReceiverStream<Message>,
    /// Messages sent into the background task from Smoldot.
    from_back: smoldot_light::JsonRpcResponses,
}

struct BackgroundTaskData<TPlatform: PlatformRef, TChain> {
    /// A smoldot light client that can be shared.
    client: SharedClient<TPlatform, TChain>,
    /// Knowing the chain ID helps with debugging, but isn't overwise necessary.
    chain_id: smoldot_light::ChainId,
    /// Know which Id to use next for new requests/subscriptions.
    last_request_id: usize,
    /// Map the request ID of a RPC method to the frontend `Sender`.
    requests: HashMap<usize, oneshot::Sender<MethodResponse>>,
    /// Subscription calls first need to make a plain RPC method
    /// request to obtain the subscription ID.
    ///
    /// The RPC method request is made in the background and the response should
    /// not be sent back to the user.
    /// Map the request ID of a RPC method to the frontend `Sender`.
    pending_subscriptions: HashMap<usize, PendingSubscription>,
    /// Map the subscription ID to the frontend `Sender`.
    ///
    /// The subscription ID is entirely generated by the node (smoldot). Therefore, it is
    /// possible for two distinct subscriptions of different chains to have the same subscription ID.
    subscriptions: HashMap<String, ActiveSubscription>,
}

/// The state needed to resolve the subscription ID and send
/// back the response to frontend.
struct PendingSubscription {
    /// Send the method response ID back to the user.
    ///
    /// It contains the subscription ID if successful, or an JSON RPC error object.
    response_sender: oneshot::Sender<SubscriptionResponse>,
    /// The unsubscribe method to call when the user drops the receiver
    /// part of the channel.
    unsubscribe_method: String,
}

/// The state of the subscription.
struct ActiveSubscription {
    /// Channel to send the subscription notifications back to frontend.
    notification_sender: mpsc::UnboundedSender<Result<Box<RawValue>, JsonRpcError>>,
    /// The unsubscribe method to call when the user drops the receiver
    /// part of the channel.
    unsubscribe_method: String,
}

impl<TPlatform: PlatformRef, TChain> BackgroundTaskData<TPlatform, TChain> {
    /// Fetch and increment the request ID.
    fn next_id(&mut self) -> usize {
        self.last_request_id = self.last_request_id.wrapping_add(1);
        self.last_request_id
    }

    /// Handle the registration messages received from the user.
    async fn handle_requests(&mut self, message: Message) {
        match message {
            Message::Request {
                method,
                params,
                sender,
            } => {
                let id = self.next_id();
                let chain_id = self.chain_id;

                let params = match &params {
                    Some(params) => params.get(),
                    None => "null",
                };
                let request = format!(
                    r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":{}}}"#,
                    id, method, params
                );

                self.requests.insert(id, sender);
                tracing::trace!(target: LOG_TARGET, "Tracking request id={id} chain={chain_id:?}");

                let result = self.client.json_rpc_request(request, chain_id);
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
                        .send(Err(LightClientRpcError::SmoldotError(err.to_string())))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send RPC request error to id={id}",
                        );
                    }
                } else {
                    tracing::trace!(target: LOG_TARGET, "Submitted to smoldot request with id={id}");
                }
            }
            Message::Subscription {
                method,
                unsubscribe_method,
                params,
                sender,
            } => {
                let id = self.next_id();
                let chain_id = self.chain_id;

                // For subscriptions we need to make a plain RPC request to the subscription method.
                // The server will return as a result the subscription ID.
                let params = match &params {
                    Some(params) => params.get(),
                    None => "null",
                };
                let request = format!(
                    r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":{}}}"#,
                    id, method, params
                );

                tracing::trace!(target: LOG_TARGET, "Tracking subscription request id={id} chain={chain_id:?}");
                let pending_subscription = PendingSubscription {
                    response_sender: sender,
                    unsubscribe_method,
                };
                self.pending_subscriptions.insert(id, pending_subscription);

                let result = self.client.json_rpc_request(request, chain_id);
                if let Err(err) = result {
                    tracing::warn!(
                        target: LOG_TARGET,
                        "Cannot send RPC request to lightclient {:?}",
                        err.to_string()
                    );
                    let subscription_id_state = self
                        .pending_subscriptions
                        .remove(&id)
                        .expect("Channels are inserted above; qed");

                    // Send the error back to frontend.
                    if subscription_id_state
                        .response_sender
                        .send(Err(LightClientRpcError::SmoldotError(err.to_string())))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send RPC request error to id={id}",
                        );
                    }
                } else {
                    tracing::trace!(target: LOG_TARGET, "Submitted to smoldot subscription request with id={id}");
                }
            }
        };
    }

    /// Parse the response received from the light client and sent it to the appropriate user.
    fn handle_rpc_response(&mut self, response: String) {
        let chain_id = self.chain_id;
        tracing::trace!(target: LOG_TARGET, "Received from smoldot response='{response}' chain={chain_id:?}");

        match RpcResponse::from_str(&response) {
            Ok(RpcResponse::Method { id, result }) => {
                let Ok(id) = id.parse::<usize>() else {
                    tracing::warn!(target: LOG_TARGET, "Cannot send response. Id={id} chain={chain_id:?} is not a valid number");
                    return;
                };

                // Send the response back.
                if let Some(sender) = self.requests.remove(&id) {
                    if sender.send(Ok(result)).is_err() {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send method response to id={id} chain={chain_id:?}",
                        );
                    }
                } else if let Some(pending_subscription) = self.pending_subscriptions.remove(&id) {
                    let Ok(sub_id) = serde_json::from_str::<SubscriptionId>(result.get()) else {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Subscription id='{result}' chain={chain_id:?} is not a valid string",
                        );
                        return;
                    };

                    tracing::trace!(target: LOG_TARGET, "Received subscription id={sub_id} chain={chain_id:?}");

                    let (sub_tx, sub_rx) = mpsc::unbounded_channel();

                    // Send the method response and a channel to receive notifications back.
                    if pending_subscription
                        .response_sender
                        .send(Ok((sub_id.clone(), sub_rx)))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send subscription ID response to id={id} chain={chain_id:?}",
                        );
                        return;
                    }

                    // Store the other end of the notif channel to send future subscription notifications to.
                    self.subscriptions.insert(
                        sub_id,
                        ActiveSubscription {
                            notification_sender: sub_tx,
                            unsubscribe_method: pending_subscription.unsubscribe_method,
                        },
                    );
                } else {
                    tracing::warn!(
                        target: LOG_TARGET,
                        "Response id={id} chain={chain_id:?} is not tracked",
                    );
                }
            }
            Ok(RpcResponse::MethodError { id, error }) => {
                let Ok(id) = id.parse::<usize>() else {
                    tracing::warn!(target: LOG_TARGET, "Cannot send error. Id={id} chain={chain_id:?} is not a valid number");
                    return;
                };

                if let Some(sender) = self.requests.remove(&id) {
                    if sender
                        .send(Err(LightClientRpcError::JsonRpcError(JsonRpcError(error))))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send method response to id={id} chain={chain_id:?}",
                        );
                    }
                } else if let Some(subscription_id_state) = self.pending_subscriptions.remove(&id) {
                    if subscription_id_state
                        .response_sender
                        .send(Err(LightClientRpcError::JsonRpcError(JsonRpcError(error))))
                        .is_err()
                    {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Cannot send method response to id {id} chain={chain_id:?}",
                        );
                    }
                }
            }
            Ok(RpcResponse::Notification {
                method,
                subscription_id,
                result,
            }) => {
                let Some(active_subscription) = self.subscriptions.get_mut(&subscription_id) else {
                    tracing::warn!(
                        target: LOG_TARGET,
                        "Subscription response id={subscription_id} chain={chain_id:?} method={method} is not tracked",
                    );
                    return;
                };
                if active_subscription
                    .notification_sender
                    .send(Ok(result))
                    .is_err()
                {
                    self.unsubscribe(&subscription_id, chain_id);
                }
            }
            Ok(RpcResponse::NotificationError {
                method,
                subscription_id,
                error,
            }) => {
                let Some(active_subscription) = self.subscriptions.get_mut(&subscription_id) else {
                    tracing::warn!(
                        target: LOG_TARGET,
                        "Subscription error id={subscription_id} chain={chain_id:?} method={method} is not tracked",
                    );
                    return;
                };
                if active_subscription
                    .notification_sender
                    .send(Err(JsonRpcError(error)))
                    .is_err()
                {
                    self.unsubscribe(&subscription_id, chain_id);
                }
            }
            Err(err) => {
                tracing::warn!(target: LOG_TARGET, "cannot decode RPC response {:?}", err);
            }
        }
    }

    // Unsubscribe from a subscription.
    fn unsubscribe(&mut self, subscription_id: &str, chain_id: smoldot_light::ChainId) {
        let Some(active_subscription) = self.subscriptions.remove(subscription_id) else {
            // Subscription doesn't exist so nothing more to do.
            return;
        };

        // Build a call to unsubscribe from this method.
        let unsub_id = self.next_id();
        let request = format!(
            r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":["{}"]}}"#,
            unsub_id, active_subscription.unsubscribe_method, subscription_id
        );

        // Submit it.
        if let Err(err) = self.client.json_rpc_request(request, chain_id) {
            tracing::warn!(
                target: LOG_TARGET,
                "Failed to unsubscribe id={subscription_id} chain={chain_id:?} method={:?} err={err:?}", active_subscription.unsubscribe_method
            );
        } else {
            tracing::debug!(target: LOG_TARGET,"Unsubscribe id={subscription_id} chain={chain_id:?} method={:?}", active_subscription.unsubscribe_method);
        }
    }
}
