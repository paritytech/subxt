use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::{ oneshot, mpsc };
use serde_json::value::RawValue;
use super::rpc::RpcResponse;

const LOG_TARGET: &str = "subxt-light-client-background";

pub struct Receiver {
    shared: SharedState
}

// Tomorrow:
// - use backend from lightclient actually, but strip out per chain things so that we spin up a new backend per chain.
//   then we can do similarly to before but with slightly simpler backend.
// - We need to send RPC messages from backend and from LightClientRpc (because backend needs to be able to unsubscribe).
//   so maybe wrap `Arc<Mutex<Client>>` in some nice struct that makes it easy to send messages from it.

impl Receiver {
    pub fn new(rpc_responses: smoldot_light::JsonRpcResponses) -> (Receiver, ReceiverDriver) {
        let shared = SharedState::default();

        let recv = Receiver {
            shared: shared.clone()
        };
        let recv_driver = ReceiverDriver {
            rpc_responses,
            shared,
        };

        (recv, recv_driver)
    }
}

pub struct ReceiverDriver {
    rpc_responses: smoldot_light::JsonRpcResponses,
    shared: SharedState
}

impl ReceiverDriver {
    pub async fn start(&mut self) {
        while let Some(json) = self.rpc_responses.next().await {
            match RpcResponse::from_str(&json) {
                Ok(RpcResponse::Method { id, result }) => {
                    let Ok(id) = id.parse::<usize>() else {
                        tracing::warn!(target: LOG_TARGET, "Cannot send response. Id={id} chain={chain_id:?} is not a valid number");
                        return;
                    };

                    // If response_channels contains ID,
                    // - if ID is pending Subscription, then parse response as subscription ID and add to subscriptions list
                    // - if ID is response then just send back the response.

                    // Send the response back.
                    if let Some(sender) = chain_data.requests.remove(&id) {
                        if sender.send(Ok(result)).is_err() {
                            tracing::warn!(
                                target: LOG_TARGET,
                                "Cannot send method response to id={id} chain={chain_id:?}",
                            );
                        }
                    } else if let Some(pending_subscription) = chain_data.id_to_subscription.remove(&id)
                    {
                        let Ok(sub_id) = result
                            .get()
                            .trim_start_matches('"')
                            .trim_end_matches('"')
                            .parse::<usize>()
                        else {
                            tracing::warn!(
                                target: LOG_TARGET,
                                "Subscription id={result} chain={chain_id:?} is not a valid number",
                            );
                            return;
                        };

                        tracing::trace!(target: LOG_TARGET, "Received subscription id={sub_id} chain={chain_id:?}");

                        let (sub_id_sender, active_subscription) = pending_subscription.into_parts();
                        if sub_id_sender.send(Ok(result)).is_err() {
                            tracing::warn!(
                                target: LOG_TARGET,
                                "Cannot send method response to id={id} chain={chain_id:?}",
                            );

                            return;
                        }

                        // Track this subscription ID if send is successful.
                        chain_data.subscriptions.insert(sub_id, active_subscription);
                    } else {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Response id={id} chain={chain_id:?} is not tracked",
                        );
                    }
                }
                Ok(RpcResponse::Subscription { method, id, result }) => {
                    let Ok(id) = id.parse::<usize>() else {
                        tracing::warn!(target: LOG_TARGET, "Cannot send subscription. Id={id} chain={chain_id:?} is not a valid number");
                        return;
                    };

                    // id is subscription ID (keep as string).
                    // look up subscription channel and send response.
                    // if sub channel fails then .... aaahh... unsubscribe. Need client here to do this.

                    let Some(subscription_state) = chain_data.subscriptions.get_mut(&id) else {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Subscription response id={id} chain={chain_id:?} method={method} is not tracked",
                        );
                        return;
                    };
                    if subscription_state.sender.send(result).is_ok() {
                        // Nothing else to do, user is informed about the notification.
                        return;
                    }

                    // User dropped the receiver, unsubscribe from the method and remove internal tracking.
                    let Some(subscription_state) = chain_data.subscriptions.remove(&id) else {
                        // State is checked to be some above, so this should never happen.
                        return;
                    };
                    // Make a call to unsubscribe from this method.
                    let unsub_id = chain_data.next_id();
                    let request = format!(
                        r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":["{}"]}}"#,
                        unsub_id, subscription_state.unsubscribe_method, id
                    );

                    if let Err(err) = self.client.json_rpc_request(request, chain_id) {
                        tracing::warn!(
                            target: LOG_TARGET,
                            "Failed to unsubscribe id={id:?} chain={chain_id:?} method={:?} err={err:?}", subscription_state.unsubscribe_method
                        );
                    } else {
                        tracing::debug!(target: LOG_TARGET,"Unsubscribe id={id:?} chain={chain_id:?} method={:?}", subscription_state.unsubscribe_method);
                    }
                }
                Ok(RpcResponse::Error { id, error }) => {
                    let Ok(id) = id.parse::<usize>() else {
                        tracing::warn!(target: LOG_TARGET, "Cannot send error; response is invalid (ID is not a number): {json}");
                        return;
                    };

                    if let Some(sender) = chain_data.requests.remove(&id) {
                        if sender
                            .send(Err(LightClientRpcError::Request(error.to_string())))
                            .is_err()
                        {
                            tracing::warn!(
                                target: LOG_TARGET,
                                "Cannot send method response to id={id} chain={chain_id:?}",
                            );
                        }
                    } else if let Some(subscription_id_state) =
                        chain_data.id_to_subscription.remove(&id)
                    {
                        if subscription_id_state
                            .sub_id_sender
                            .send(Err(LightClientRpcError::Request(error.to_string())))
                            .is_err()
                        {
                            tracing::warn!(
                                target: LOG_TARGET,
                                "Cannot send method response to id {id} chain={chain_id:?}",
                            );
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(target: LOG_TARGET, "error deserializing JSON-RPC response from Smoldot: {e}");
                }
            }
        }

        // No more responses; clear shared state to indicate this
        self.shared.clear();
    }
}

#[derive(Clone, Default)]
struct SharedState {
    inner: Arc<Mutex<SharedStateInner>>
}

impl SharedState {
    fn clear(&self) {
        self.inner
            .lock()
            .expect("lock is poisoned")
            .response_channels
            .clear();
    }
    fn send(&self, id: usize, message: Box<RawValue>) {

    }
}

#[derive(Default)]
struct SharedStateInner {
    response_channels: HashMap<usize, ResponseChannel>,
    subscription_channels: HashMap<String, mpsc::Sender<Box<RawValue>>>
}

enum ResponseChannel {
    PendingSubscription(mpsc::Sender<Box<RawValue>>),
    MethodCall(oneshot::Sender<Box<RawValue>>)
}