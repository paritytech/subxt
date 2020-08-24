// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Client for embedding substrate nodes.

#![deny(missing_docs)]

use async_std::task;
use futures::{
    channel::mpsc,
    compat::{
        Compat01As03,
        Sink01CompatExt,
        Stream01CompatExt,
    },
    future::{
        select,
        FutureExt,
    },
    sink::SinkExt,
    stream::StreamExt,
};
use futures01::sync::mpsc as mpsc01;
use jsonrpsee::{
    common::{
        Request,
        Response,
    },
    transport::TransportClient,
};
use sc_network::config::TransportConfig;
pub use sc_service::{
    config::{
        DatabaseConfig,
        KeystoreConfig,
    },
    Error as ServiceError,
};
use sc_service::{
    config::{
        NetworkConfiguration,
        TaskType,
        TelemetryEndpoints,
    },
    ChainSpec,
    Configuration,
    RpcHandlers,
    RpcSession,
    TaskManager,
};
use std::{
    future::Future,
    pin::Pin,
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

/// Client for an embedded substrate node.
pub struct SubxtClient {
    to_back: mpsc::Sender<String>,
    from_back: Compat01As03<mpsc01::Receiver<String>>,
}

impl SubxtClient {
    /// Create a new client.
    pub fn new(mut task_manager: TaskManager, rpc: RpcHandlers) -> Self {
        let (to_back, from_front) = mpsc::channel(4);
        let (to_front, from_back) = mpsc01::channel(4);

        let session = RpcSession::new(to_front.clone());
        task::spawn(
            select(
                Box::pin(from_front.for_each(move |message: String| {
                    let rpc = rpc.clone();
                    let session = session.clone();
                    let mut to_front = to_front.clone().sink_compat();
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
            to_back,
            from_back: from_back.compat(),
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

impl TransportClient for SubxtClient {
    type Error = SubxtClientError;

    fn send_request<'a>(
        &'a mut self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        Box::pin(async move {
            let request = serde_json::to_string(&request)?;
            self.to_back.send(request).await?;
            Ok(())
        })
    }

    fn next_response<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<Response, Self::Error>> + Send + 'a>> {
        Box::pin(async move {
            let response = self
                .from_back
                .next()
                .await
                .expect("channel shouldn't close")
                .unwrap();
            Ok(serde_json::from_str(&response)?)
        })
    }
}

impl From<SubxtClient> for jsonrpsee::Client {
    fn from(client: SubxtClient) -> Self {
        let client = jsonrpsee::raw::RawClient::new(client);
        jsonrpsee::Client::new(client)
    }
}

/// Role of the node.
#[derive(Clone, Copy, Debug)]
pub enum Role {
    /// Light client.
    Light,
    /// A full node (maninly used for testing purposes).
    Authority(sp_keyring::AccountKeyring),
}

impl From<Role> for sc_service::Role {
    fn from(role: Role) -> Self {
        match role {
            Role::Light => Self::Light,
            Role::Authority(_) => {
                Self::Authority {
                    sentry_nodes: Default::default(),
                }
            }
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
    pub db: DatabaseConfig,
    /// Keystore configuration.
    pub keystore: KeystoreConfig,
    /// Chain specification.
    pub chain_spec: C,
    /// Role of the node.
    pub role: Role,
    /// Enable telemetry on the given port.
    pub telemetry: Option<u16>,
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
            wasm_external_transport: None,
            use_yamux_flow_control: true,
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
            task_executor: (move |fut, ty| {
                match ty {
                    TaskType::Async => task::spawn(fut),
                    TaskType::Blocking => task::spawn_blocking(|| task::block_on(fut)),
                }
            })
            .into(),
            database: self.db,
            keystore: self.keystore,
            max_runtime_instances: 8,
            announce_block: true,
            dev_key_seed: self.role.into(),
            telemetry_endpoints,

            telemetry_external_transport: Default::default(),
            default_heap_pages: Default::default(),
            disable_grandpa: Default::default(),
            execution_strategies: Default::default(),
            force_authoring: Default::default(),
            offchain_worker: Default::default(),
            prometheus_config: Default::default(),
            pruning: Default::default(),
            rpc_cors: Default::default(),
            rpc_http: Default::default(),
            rpc_ipc: Default::default(),
            rpc_ws: Default::default(),
            rpc_ws_max_connections: Default::default(),
            rpc_methods: Default::default(),
            state_cache_child_ratio: Default::default(),
            state_cache_size: Default::default(),
            tracing_receiver: Default::default(),
            tracing_targets: Default::default(),
            transaction_pool: Default::default(),
            wasm_method: Default::default(),
            base_path: Default::default(),
            informant_output_format: Default::default(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::path::Path;
    use sp_keyring::AccountKeyring;
    use substrate_subxt::{
        balances::TransferCallExt,
        ClientBuilder,
        KusamaRuntime as NodeTemplateRuntime,
        PairSigner,
    };
    use tempdir::TempDir;

    #[async_std::test]
    #[ignore]
    async fn test_client() {
        env_logger::try_init().ok();
        let client = ClientBuilder::<NodeTemplateRuntime>::new()
            .build()
            .await
            .unwrap();
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        let to = AccountKeyring::Bob.to_account_id().into();
        client
            .transfer_and_watch(&signer, &to, 10_000)
            .await
            .unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_light_client() {
        env_logger::try_init().ok();
        let chain_spec_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("dev-chain.json");
        let bytes = async_std::fs::read(chain_spec_path).await.unwrap();
        let chain_spec =
            test_node::chain_spec::ChainSpec::from_json_bytes(bytes).unwrap();
        let tmp = TempDir::new("subxt-").expect("failed to create tempdir");
        let config = SubxtClientConfig {
            // base_path:
            impl_name: "substrate-subxt-light-client",
            impl_version: "0.0.1",
            author: "David Craven",
            copyright_start_year: 2020,
            db: DatabaseConfig::RocksDb {
                path: tmp.path().into(),
                cache_size: 64,
            },
            keystore: KeystoreConfig::InMemory,
            chain_spec,
            role: Role::Light,
            telemetry: None,
        };
        let client = ClientBuilder::<NodeTemplateRuntime>::new()
            .set_client(
                SubxtClient::from_config(config, test_node::service::new_light).unwrap(),
            )
            .build()
            .await
            .unwrap();
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        let to = AccountKeyring::Bob.to_account_id().into();
        client
            .transfer_and_watch(&signer, &to, 10_000)
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn test_full_client() {
        env_logger::try_init().ok();
        let tmp = TempDir::new("subxt-").expect("failed to create tempdir");
        let config = SubxtClientConfig {
            impl_name: "substrate-subxt-full-client",
            impl_version: "0.0.1",
            author: "David Craven",
            copyright_start_year: 2020,
            db: DatabaseConfig::RocksDb {
                path: tmp.path().into(),
                cache_size: 128,
            },
            keystore: KeystoreConfig::InMemory,
            chain_spec: test_node::chain_spec::development_config().unwrap(),
            role: Role::Authority(AccountKeyring::Alice),
            telemetry: None,
        };
        let client = ClientBuilder::<NodeTemplateRuntime>::new()
            .set_client(
                SubxtClient::from_config(config, test_node::service::new_full).unwrap(),
            )
            .build()
            .await
            .unwrap();
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        let to = AccountKeyring::Bob.to_account_id().into();
        client
            .transfer_and_watch(&signer, &to, 10_000)
            .await
            .unwrap();
    }
}
