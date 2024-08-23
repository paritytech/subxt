// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::backend::rpc::reconnecting_rpc_client::{RpcClientBuilder, RpcError};
use jsonrpsee::core::client::Client;
use std::sync::Arc;

#[cfg(feature = "native")]
pub use tokio::spawn;

#[cfg(feature = "web")]
pub use wasm_bindgen_futures::spawn_local as spawn;

#[cfg(feature = "native")]
pub async fn ws_client<P>(
    url: &str,
    builder: &RpcClientBuilder<P>,
) -> Result<Arc<Client>, RpcError> {
    use jsonrpsee::ws_client::WsClientBuilder;

    let RpcClientBuilder {
        max_request_size,
        max_response_size,
        ping_config,
        headers,
        max_redirections,
        id_kind,
        max_concurrent_requests,
        max_log_len,
        request_timeout,
        connection_timeout,
        ..
    } = builder;

    let mut ws_client_builder = WsClientBuilder::new()
        .max_request_size(*max_request_size)
        .max_response_size(*max_response_size)
        .set_headers(headers.clone())
        .max_redirections(*max_redirections as usize)
        .max_buffer_capacity_per_subscription(tokio::sync::Semaphore::MAX_PERMITS)
        .max_concurrent_requests(*max_concurrent_requests as usize)
        .set_max_logging_length(*max_log_len)
        .set_tcp_no_delay(true)
        .request_timeout(*request_timeout)
        .connection_timeout(*connection_timeout)
        .id_format(*id_kind);

    if let Some(ping) = ping_config {
        ws_client_builder = ws_client_builder.enable_ws_ping(*ping);
    }

    let client = ws_client_builder.build(url).await?;

    Ok(Arc::new(client))
}

#[cfg(feature = "web")]
pub async fn ws_client<P>(
    url: &str,
    builder: &RpcClientBuilder<P>,
) -> Result<Arc<Client>, RpcError> {
    use jsonrpsee::wasm_client::WasmClientBuilder;

    let RpcClientBuilder {
        id_kind,
        max_concurrent_requests,
        max_log_len,
        request_timeout,
        ..
    } = builder;

    let ws_client_builder = WasmClientBuilder::new()
        .max_buffer_capacity_per_subscription(tokio::sync::Semaphore::MAX_PERMITS)
        .max_concurrent_requests(*max_concurrent_requests as usize)
        .set_max_logging_length(*max_log_len)
        .request_timeout(*request_timeout)
        .id_format(*id_kind);

    let client = ws_client_builder.build(url).await?;

    Ok(Arc::new(client))
}
