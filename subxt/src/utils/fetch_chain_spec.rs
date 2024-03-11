// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::macros::{cfg_jsonrpsee, cfg_jsonrpsee_native, cfg_jsonrpsee_web};
use serde_json::value::RawValue;

/// Possible errors encountered trying to fetch a chain spec from an RPC node.
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum FetchChainspecError {
    #[error("Cannot fetch chain spec: RPC error: {0}.")]
    RpcError(String),
    #[error("Cannot fetch chain spec: Invalid URL.")]
    InvalidUrl,
    #[error("Cannot fetch chain spec: Invalid URL scheme.")]
    InvalidScheme,
    #[error("Cannot fetch chain spec: Handshake error establishing WS connection.")]
    HandshakeError,
}

cfg_jsonrpsee! {
    /// Fetch a chain spec from an RPC node at the given URL.
    pub async fn fetch_chainspec_from_rpc_node(url: impl AsRef<str>) -> Result<Box<RawValue>, FetchChainspecError> {
        use jsonrpsee::core::client::{ClientT, SubscriptionClientT};
        use jsonrpsee::rpc_params;
        use serde_json::value::RawValue;

        let client = jsonrpsee_helpers::client(url.as_ref()).await?;

        let result = client
            .request("sync_state_genSyncSpec", jsonrpsee::rpc_params![true])
            .await
            .map_err(|err| FetchChainspecError::RpcError(err.to_string()))?;

        // Subscribe to the finalized heads of the chain.
        let mut subscription = SubscriptionClientT::subscribe::<Box<RawValue>, _>(
            &client,
            "chain_subscribeFinalizedHeads",
            rpc_params![],
            "chain_unsubscribeFinalizedHeads",
        )
        .await
        .map_err(|err| FetchChainspecError::RpcError(err.to_string()))?;

        // We must ensure that the finalized block of the chain is not the block included
        // in the chainSpec.
        // This is a temporary workaround for: https://github.com/smol-dot/smoldot/issues/1562.
        // The first finalized block that is received might by the finalized block could be the one
        // included in the chainSpec. Decoding the chainSpec for this purpose is too complex.
        let _ = subscription.next().await;
        let _ = subscription.next().await;

        Ok(result)
    }
}

cfg_jsonrpsee_native! {
    mod jsonrpsee_helpers {
        use super::FetchChainspecError;
        use tokio_util::compat::Compat;

        pub use jsonrpsee::{
            client_transport::ws::{self, EitherStream, Url, WsTransportClientBuilder},
            core::client::Client,
        };

        pub type Sender = ws::Sender<Compat<EitherStream>>;
        pub type Receiver = ws::Receiver<Compat<EitherStream>>;

        /// Build WS RPC client from URL
        pub async fn client(url: &str) -> Result<Client, FetchChainspecError> {
            let url = Url::parse(url).map_err(|_| FetchChainspecError::InvalidUrl)?;

                if url.scheme() != "ws" && url.scheme() != "wss" {
                    return Err(FetchChainspecError::InvalidScheme);
                }

                let (sender, receiver) = ws_transport(url).await?;

                Ok(Client::builder()
                .max_buffer_capacity_per_subscription(4096)
                .build_with_tokio(sender, receiver))
            }

            async fn ws_transport(url: Url) -> Result<(Sender, Receiver), FetchChainspecError> {
                WsTransportClientBuilder::default()
                    .build(url)
                    .await
                    .map_err(|_| FetchChainspecError::HandshakeError)
            }
        }
}

cfg_jsonrpsee_web! {
    mod jsonrpsee_helpers {
        use super::FetchChainspecError;
        pub use jsonrpsee::{
            client_transport::web,
            core::client::{Client, ClientBuilder},
        };

        /// Build web RPC client from URL
        pub async fn client(url: &str) -> Result<Client, FetchChainspecError> {
            let (sender, receiver) = web::connect(url)
                .await
                .map_err(|_| FetchChainspecError::HandshakeError)?;

            Ok(ClientBuilder::default()
                .max_buffer_capacity_per_subscription(4096)
                .build_with_wasm(sender, receiver))
        }
    }
}
