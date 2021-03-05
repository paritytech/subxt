// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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
    channel::{
        mpsc,
        oneshot,
    },
    compat::Stream01CompatExt,
    future::{
        select,
        FutureExt,
    },
    sink::SinkExt,
    stream::StreamExt,
};
use futures01::sync::mpsc as mpsc01;
use jsonrpsee_types::{
    client::{
        FrontToBack,
        Subscription,
    },
    error::Error as JsonRpseeError,
    jsonrpc,
    jsonrpc::{
        Call,
        DeserializeOwned,
        Id,
        MethodCall,
        Notification,
        Output,
        Request,
        SubscriptionId,
        SubscriptionNotif,
        Version,
    },
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
        WasmExecutionMethod,
    },
    ChainSpec,
    Configuration,
    KeepBlocks,
    RpcHandlers,
    RpcSession,
    TaskManager,
};
use std::marker::PhantomData;
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
#[derive(Clone)]
pub struct SubxtClient {
    to_back: mpsc::Sender<FrontToBack>,
}

impl SubxtClient {
    /// Create a new client.
    pub fn new(mut task_manager: TaskManager, rpc: RpcHandlers) -> Self {
        let (to_back, from_front) = mpsc::channel(4);

        task::spawn(
            select(
                Box::pin(from_front.for_each(move |message: FrontToBack| {
                    let rpc = rpc.clone();
                    let (to_front, from_back) = mpsc01::channel(4);
                    let session = RpcSession::new(to_front.clone());
                    async move {
                        match message {
                            FrontToBack::Notification { method, params } => {
                                let request =
                                    Request::Single(Call::Notification(Notification {
                                        jsonrpc: Version::V2,
                                        method,
                                        params,
                                    }));
                                if let Ok(message) = serde_json::to_string(&request) {
                                    rpc.rpc_query(&session, &message).await;
                                }
                            }

                            FrontToBack::StartRequest {
                                method,
                                params,
                                send_back,
                            } => {
                                let request =
                                    Request::Single(Call::MethodCall(MethodCall {
                                        jsonrpc: Version::V2,
                                        method: method.into(),
                                        params: params.into(),
                                        id: Id::Num(0),
                                    }));
                                if let Ok(message) = serde_json::to_string(&request) {
                                    if let Some(response) =
                                        rpc.rpc_query(&session, &message).await
                                    {
                                        let result = match serde_json::from_str::<Output>(
                                            &response,
                                        )
                                        .expect("failed to decode request response")
                                        {
                                            Output::Success(success) => {
                                                Ok(success.result)
                                            }
                                            Output::Failure(failure) => {
                                                Err(JsonRpseeError::Request(
                                                    failure.error,
                                                ))
                                            }
                                        };

                                        send_back
                                            .send(result)
                                            .expect("failed to send request response");
                                    }
                                }
                            }

                            FrontToBack::Subscribe {
                                subscribe_method,
                                params,
                                unsubscribe_method: _,
                                send_back,
                            } => {
                                let request =
                                    Request::Single(Call::MethodCall(MethodCall {
                                        jsonrpc: Version::V2,
                                        method: subscribe_method,
                                        params,
                                        id: Id::Num(0),
                                    }));

                                let (mut send_front_sub, send_back_sub) =
                                    mpsc::channel(4);
                                if let Ok(message) = serde_json::to_string(&request) {
                                    if let Some(response) =
                                        rpc.rpc_query(&session, &message).await
                                    {
                                        let result = match serde_json::from_str::<Output>(
                                            &response,
                                        )
                                        .expect("failed to decode subscription response")
                                        {
                                            Output::Success(_) => {
                                                Ok((
                                                    send_back_sub,
                                                    SubscriptionId::Num(0),
                                                ))
                                            }
                                            Output::Failure(failure) => {
                                                Err(JsonRpseeError::Request(
                                                    failure.error,
                                                ))
                                            }
                                        };

                                        send_back.send(result).expect(
                                            "failed to send subscription response",
                                        );
                                    }
                                }

                                task::spawn(async move {
                                    let mut from_back = from_back.compat();
                                    let _session = session.clone();

                                    while let Some(Ok(response)) = from_back.next().await
                                    {
                                        let notif = serde_json::from_str::<
                                            SubscriptionNotif,
                                        >(
                                            &response
                                        )
                                        .expect("failed to decode subscription notif");
                                        send_front_sub
                                            .send(notif.params.result)
                                            .await
                                            .expect("failed to send subscription notif")
                                    }
                                });
                            }

                            FrontToBack::SubscriptionClosed(_) => {
                                // TODO: implement
                            }
                        }
                    }
                })),
                Box::pin(async move {
                    task_manager.future().await.ok();
                }),
            )
            .map(drop),
        );

        Self { to_back }
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

    /// Send a JSONRPC notification.
    pub async fn notification<M, P>(
        &self,
        method: M,
        params: P,
    ) -> Result<(), JsonRpseeError>
    where
        M: Into<String> + Send,
        P: Into<jsonrpc::Params> + Send,
    {
        self.to_back
            .clone()
            .send(FrontToBack::Notification {
                method: method.into(),
                params: params.into(),
            })
            .await
            .map_err(|e| JsonRpseeError::TransportError(Box::new(e)))
    }

    /// Send a JSONRPC request.
    pub async fn request<T, M, P>(
        &self,
        method: M,
        params: P,
    ) -> Result<T, JsonRpseeError>
    where
        T: DeserializeOwned,
        M: Into<String> + Send,
        P: Into<jsonrpc::Params> + Send,
    {
        let (send_back_tx, send_back_rx) = oneshot::channel();

        self.to_back
            .clone()
            .send(FrontToBack::StartRequest {
                method: method.into(),
                params: params.into(),
                send_back: send_back_tx,
            })
            .await
            .map_err(|e| JsonRpseeError::TransportError(Box::new(e)))?;

        let json_value = match send_back_rx.await {
            Ok(Ok(v)) => v,
            Ok(Err(err)) => return Err(err),
            Err(err) => return Err(JsonRpseeError::TransportError(Box::new(err))),
        };
        jsonrpc::from_value(json_value).map_err(JsonRpseeError::ParseError)
    }

    /// Send a subscription request to the server.
    pub async fn subscribe<SM, UM, P, N>(
        &self,
        subscribe_method: SM,
        params: P,
        unsubscribe_method: UM,
    ) -> Result<Subscription<N>, JsonRpseeError>
    where
        SM: Into<String> + Send,
        UM: Into<String> + Send,
        P: Into<jsonrpc::Params> + Send,
        N: DeserializeOwned,
    {
        let subscribe_method = subscribe_method.into();
        let unsubscribe_method = unsubscribe_method.into();
        let params = params.into();

        let (send_back_tx, send_back_rx) = oneshot::channel();
        self.to_back
            .clone()
            .send(FrontToBack::Subscribe {
                subscribe_method,
                unsubscribe_method,
                params,
                send_back: send_back_tx,
            })
            .await
            .map_err(JsonRpseeError::Internal)?;

        let (notifs_rx, id) = match send_back_rx.await {
            Ok(Ok(val)) => val,
            Ok(Err(err)) => return Err(err),
            Err(err) => return Err(JsonRpseeError::TransportError(Box::new(err))),
        };
        Ok(Subscription {
            to_back: self.to_back.clone(),
            notifs_rx,
            marker: PhantomData,
            id,
        })
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
    /// Wasm execution method
    pub wasm_method: WasmExecutionMethod,
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
            telemetry_handle: Default::default(),
            telemetry_span: Default::default(),
            default_heap_pages: Default::default(),
            disable_grandpa: Default::default(),
            disable_log_reloading: Default::default(),
            execution_strategies: Default::default(),
            force_authoring: Default::default(),
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
        NodeTemplateRuntime,
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
            wasm_method: Default::default(),
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
            wasm_method: Default::default(),
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
