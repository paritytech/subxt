// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Client for embedding substrate nodes.

#[cfg(test)]
mod tests;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod substrate {}

use async_std::task;
use futures::{
    channel::mpsc,
    future::{
        select,
        FutureExt,
    },
    sink::SinkExt,
    stream::StreamExt,
};
use jsonrpsee::core::{
    async_trait,
    client::{
        Client as JsonRpcClient,
        ClientBuilder as JsonRpcClientBuilder,
        TransportReceiverT,
        TransportSenderT,
    },
};
use sc_network::config::TransportConfig;
pub use sc_service::{
    config::{
        DatabaseSource,
        KeystoreConfig,
        WasmExecutionMethod,
    },
    Error as ServiceError,
};
use sc_service::{
    config::{
        NetworkConfiguration,
        TelemetryEndpoints,
    },
    ChainSpec,
    Configuration,
    KeepBlocks,
    RpcHandlers,
    RpcSession,
    TaskManager,
};
use thiserror::Error;

/// Error thrown by the client.
#[derive(Debug, Error)]
pub enum SubxtClientError {
    /// Failed to parse json rpc message.
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    /// Channel closed.
    #[error("{0}")]
    Mpsc(#[from] mpsc::SendError),
}

/// Sending end.
pub struct Sender(mpsc::UnboundedSender<String>);

/// Receiving end
pub struct Receiver(mpsc::UnboundedReceiver<String>);

#[async_trait]
impl TransportSenderT for Sender {
    type Error = SubxtClientError;

    async fn send(&mut self, msg: String) -> Result<(), Self::Error> {
        log::info!("send: {:?}", msg);
        self.0.send(msg).await?;
        Ok(())
    }
}

#[async_trait]
impl TransportReceiverT for Receiver {
    type Error = SubxtClientError;

    async fn receive(&mut self) -> Result<String, Self::Error> {
        let msg = self.0.next().await.expect("channel should be open");
        log::info!("rx: {:?}", msg);
        Ok(msg)
    }
}

/// Client for an embedded substrate node.
pub struct SubxtClient {
    sender: Sender,
    receiver: Receiver,
}

impl SubxtClient {
    /// Create a new client.
    pub fn new(mut task_manager: TaskManager, rpc: RpcHandlers) -> Self {
        let (to_back, from_front) = mpsc::unbounded();
        let (to_front, from_back) = mpsc::unbounded();

        let session = RpcSession::new(to_front.clone());
        task::spawn(
            select(
                Box::pin(from_front.for_each(move |message: String| {
                    let rpc = rpc.clone();
                    let session = session.clone();
                    let mut to_front = to_front.clone();
                    async move {
                        let response = rpc.rpc_query(&session, &message).await;
                        if let Some(response) = response {
                            to_front.send(response).await.ok();
                        }
                    }
                })),
                Box::pin(async move {
                    task_manager.future().await.ok();
                }),
            )
            .map(drop),
        );

        Self {
            sender: Sender(to_back),
            receiver: Receiver(from_back),
        }
    }

    /// Creates a new client from a config.
    pub fn from_config<C: ChainSpec + 'static>(
        config: SubxtClientConfig<C>,
        builder: impl Fn(Configuration) -> Result<(TaskManager, RpcHandlers), ServiceError>,
    ) -> Result<Self, ServiceError> {
        let config = config.into_service_config();
        let (task_manager, rpc_handlers) = (builder)(config)?;
        Ok(Self::new(task_manager, rpc_handlers))
    }
}

impl From<SubxtClient> for JsonRpcClient {
    fn from(client: SubxtClient) -> Self {
        JsonRpcClientBuilder::default()
            .request_timeout(std::time::Duration::from_secs(5 * 60))
            .build(client.sender, client.receiver)
    }
}

/// Role of the node.
#[derive(Clone, Copy, Debug)]
pub enum Role {
    /// Light client.
    Light,
    /// A full node (mainly used for testing purposes).
    Authority(sp_keyring::AccountKeyring),
}

impl From<Role> for sc_service::Role {
    fn from(role: Role) -> Self {
        match role {
            Role::Light => Self::Light,
            Role::Authority(_) => Self::Authority,
        }
    }
}

impl From<Role> for Option<String> {
    fn from(role: Role) -> Self {
        match role {
            Role::Light => None,
            Role::Authority(key) => Some(key.to_seed()),
        }
    }
}

/// Client configuration.
#[derive(Clone)]
pub struct SubxtClientConfig<C: ChainSpec + 'static> {
    /// Name of the implementation.
    pub impl_name: &'static str,
    /// Version of the implementation.
    pub impl_version: &'static str,
    /// Author of the implementation.
    pub author: &'static str,
    /// Copyright start year.
    pub copyright_start_year: i32,
    /// Database configuration.
    pub db: DatabaseSource,
    /// Keystore configuration.
    pub keystore: KeystoreConfig,
    /// Chain specification.
    pub chain_spec: C,
    /// Role of the node.
    pub role: Role,
    /// Enable telemetry on the given port.
    pub telemetry: Option<u16>,
    /// Wasm execution method
    pub wasm_method: WasmExecutionMethod,
    /// Handle to the tokio runtime. Will be used to spawn futures by the task manager.
    pub tokio_handle: tokio::runtime::Handle,
}

impl<C: ChainSpec + 'static> SubxtClientConfig<C> {
    /// Creates a service configuration.
    pub fn into_service_config(self) -> Configuration {
        let mut network = NetworkConfiguration::new(
            format!("{} (subxt client)", self.chain_spec.name()),
            "unknown",
            Default::default(),
            None,
        );
        network.boot_nodes = self.chain_spec.boot_nodes().to_vec();
        network.transport = TransportConfig::Normal {
            enable_mdns: true,
            allow_private_ipv4: true,
        };
        let telemetry_endpoints = if let Some(port) = self.telemetry {
            let endpoints = TelemetryEndpoints::new(vec![(
                format!("/ip4/127.0.0.1/tcp/{}/ws", port),
                0,
            )])
            .expect("valid config; qed");
            Some(endpoints)
        } else {
            None
        };
        let service_config = Configuration {
            network,
            impl_name: self.impl_name.to_string(),
            impl_version: self.impl_version.to_string(),
            chain_spec: Box::new(self.chain_spec),
            role: self.role.into(),
            database: self.db,
            keystore: self.keystore,
            max_runtime_instances: 8,
            announce_block: true,
            dev_key_seed: self.role.into(),
            telemetry_endpoints,
            tokio_handle: self.tokio_handle,
            default_heap_pages: Default::default(),
            disable_grandpa: true,
            execution_strategies: Default::default(),
            force_authoring: true,
            keep_blocks: KeepBlocks::All,
            keystore_remote: Default::default(),
            offchain_worker: Default::default(),
            prometheus_config: Default::default(),
            rpc_cors: Default::default(),
            rpc_http: Default::default(),
            rpc_ipc: Default::default(),
            rpc_ws: Default::default(),
            rpc_ws_max_connections: Default::default(),
            rpc_methods: Default::default(),
            rpc_max_payload: Default::default(),
            runtime_cache_size: Default::default(),
            state_cache_child_ratio: Default::default(),
            state_cache_size: Default::default(),
            tracing_receiver: Default::default(),
            tracing_targets: Default::default(),
            transaction_pool: Default::default(),
            wasm_method: self.wasm_method,
            base_path: Default::default(),
            informant_output_format: Default::default(),
            state_pruning: Default::default(),
            transaction_storage: sc_client_db::TransactionStorageMode::BlockBody,
            wasm_runtime_overrides: Default::default(),
            ws_max_out_buffer_capacity: Default::default(),
        };

        log::info!("{}", service_config.impl_name);
        log::info!("✌️  version {}", service_config.impl_version);
        log::info!("❤️  by {}, {}", self.author, self.copyright_start_year);
        log::info!(
            "📋 Chain specification: {}",
            service_config.chain_spec.name()
        );
        log::info!("🏷  Node name: {}", service_config.network.node_name);
        log::info!("👤 Role: {:?}", self.role);

        service_config
    }
}
