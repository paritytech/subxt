use serde_json::value::RawValue;
use serde::Deserialize;

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
