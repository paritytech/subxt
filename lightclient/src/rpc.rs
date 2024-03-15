// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use serde::Deserialize;
use serde_json::value::RawValue;

/// The RPC response from the light-client.
/// This can either be a response of a method, or a notification from a subscription.
#[derive(Debug, Clone)]
pub enum RpcResponse {
    Method {
        /// Response ID.
        id: String,
        /// The result of the method call.
        result: Box<RawValue>,
    },
    MethodError {
        /// Response ID.
        id: String,
        /// Error.
        error: Box<RawValue>,
    },
    Notification {
        /// RPC method that generated the notification.
        method: String,
        /// Subscription ID.
        subscription_id: String,
        /// Result.
        result: Box<RawValue>,
    },
    NotificationError {
        /// RPC method that generated the notification.
        method: String,
        /// Subscription ID.
        subscription_id: String,
        /// Result.
        error: Box<RawValue>,
    },
}

impl std::str::FromStr for RpcResponse {
    type Err = ();

    fn from_str(response: &str) -> Result<Self, Self::Err> {
        // Valid response
        #[derive(Deserialize, Debug)]
        struct Response {
            #[allow(unused)]
            jsonrpc: String,
            id: String,
            result: Box<RawValue>,
        }

        // Error response
        #[derive(Deserialize)]
        struct ResponseError {
            #[allow(unused)]
            jsonrpc: String,
            id: String,
            error: Box<RawValue>,
        }

        // Valid notification (subscription) response
        #[derive(Deserialize)]
        struct Notification {
            #[allow(unused)]
            jsonrpc: String,
            method: String,
            params: NotificationResultParams,
        }
        #[derive(Deserialize)]
        struct NotificationResultParams {
            subscription: String,
            result: Box<RawValue>,
        }

        // Error notification (subscription) response
        #[derive(Deserialize)]
        struct NotificationError {
            #[allow(unused)]
            jsonrpc: String,
            method: String,
            params: NotificationErrorParams,
        }
        #[derive(Deserialize)]
        struct NotificationErrorParams {
            /// The ID of the subscription.
            subscription: String,
            error: Box<RawValue>,
        }

        // Try deserializing the response payload to one of the above. We can
        // do this more efficiently eg how jsonrpsee_types does.

        let result: Result<Response, _> = serde_json::from_str(response);
        if let Ok(response) = result {
            return Ok(RpcResponse::Method {
                id: response.id,
                result: response.result,
            });
        }
        let result: Result<Notification, _> = serde_json::from_str(response);
        if let Ok(response) = result {
            return Ok(RpcResponse::Notification {
                subscription_id: response.params.subscription,
                method: response.method,
                result: response.params.result,
            });
        }
        let result: Result<ResponseError, _> = serde_json::from_str(response);
        if let Ok(response) = result {
            return Ok(RpcResponse::MethodError {
                id: response.id,
                error: response.error,
            });
        }
        let result: Result<NotificationError, _> = serde_json::from_str(response);
        if let Ok(response) = result {
            return Ok(RpcResponse::NotificationError {
                method: response.method,
                subscription_id: response.params.subscription,
                error: response.params.error,
            });
        }

        // We couldn't decode into any of the above. We could pick one of the above`
        // errors to return, but there's no real point since the string is obviously
        // different from any of them.
        Err(())
    }
}
