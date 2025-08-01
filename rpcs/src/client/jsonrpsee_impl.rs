// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{RawRpcFuture, RawRpcSubscription, RpcClientT};
use crate::Error;
use futures::stream::{StreamExt, TryStreamExt};
use jsonrpsee::{
    core::{
        client::{Error as JsonrpseeError, Client, ClientT, SubscriptionClientT, SubscriptionKind},
        traits::ToRpcParams,
    },
    types::SubscriptionId,
};
use serde_json::value::RawValue;

/// Construct a `jsonrpsee` RPC client with some sane defaults.
pub async fn client(url: &str) -> Result<Client, Error> {
    jsonrpsee_helpers::client(url).await.map_err(|e| Error::Client(Box::new(e)))
}

struct Params(Option<Box<RawValue>>);

impl ToRpcParams for Params {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
        Ok(self.0)
    }
}

impl RpcClientT for Client {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        Box::pin(async move {
            let res = ClientT::request(self, method, Params(params)).await?;
            Ok(res)
        })
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        Box::pin(async move {
            let stream = SubscriptionClientT::subscribe::<Box<RawValue>, _>(
                self,
                sub,
                Params(params),
                unsub,
            ).await?;

            let id = match stream.kind() {
                SubscriptionKind::Subscription(SubscriptionId::Str(id)) => {
                    Some(id.clone().into_owned())
                }
                _ => None,
            };

            let stream = stream
                .map_err(|e| Error::Client(Box::new(e)))
                .boxed();
            Ok(RawRpcSubscription { stream, id })
        })
    }
}

// Convert a JsonrpseeError into the RPC error in this crate.
// The main reason for this is to capture user errors so that
// they can be represented/handled without casting. 
impl From<JsonrpseeError> for Error {
    fn from(error: JsonrpseeError) -> Self {
        match error {
            JsonrpseeError::Call(e) => {
                Error::User(crate::UserError {
                    code: e.code(),
                    message: e.message().to_owned(),
                    data: e.data().map(|d| d.to_owned())
                })
            },
            e => {
                Error::Client(Box::new(e))
            }
        }
    }
}

// helpers for a jsonrpsee specific RPC client.
#[cfg(all(feature = "jsonrpsee", feature = "native"))]
mod jsonrpsee_helpers {
    pub use jsonrpsee::{
        client_transport::ws::{self, EitherStream, Url, WsTransportClientBuilder},
        core::client::{Client, Error},
    };
    use tokio_util::compat::Compat;

    pub type Sender = ws::Sender<Compat<EitherStream>>;
    pub type Receiver = ws::Receiver<Compat<EitherStream>>;

    /// Build WS RPC client from URL
    pub async fn client(url: &str) -> Result<Client, Error> {
        let (sender, receiver) = ws_transport(url).await?;
        Ok(Client::builder()
            .max_buffer_capacity_per_subscription(4096)
            .build_with_tokio(sender, receiver))
    }

    async fn ws_transport(url: &str) -> Result<(Sender, Receiver), Error> {
        let url = Url::parse(url).map_err(|e| Error::Transport(e.into()))?;
        WsTransportClientBuilder::default()
            .build(url)
            .await
            .map_err(|e| Error::Transport(e.into()))
    }
}

// helpers for a jsonrpsee specific RPC client.
#[cfg(all(feature = "jsonrpsee", feature = "web", target_arch = "wasm32"))]
mod jsonrpsee_helpers {
    pub use jsonrpsee::{
        client_transport::web,
        core::client::{Client, ClientBuilder, Error},
    };

    /// Build web RPC client from URL
    pub async fn client(url: &str) -> Result<Client, Error> {
        let (sender, receiver) = web::connect(url)
            .await
            .map_err(|e| Error::Transport(e.into()))?;
        Ok(ClientBuilder::default()
            .max_buffer_capacity_per_subscription(4096)
            .build_with_wasm(sender, receiver))
    }
}