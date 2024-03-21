// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub use reconnecting_jsonrpsee_ws_client::{CallRetryPolicy, RetryPolicy};

use super::{RawRpcFuture, RawRpcSubscription, RpcClientT};
use crate::error::RpcError;
use futures::{FutureExt, StreamExt, TryStreamExt};
use reconnecting_jsonrpsee_ws_client::{Client as InnerClient, SubscriptionId};
use serde_json::value::RawValue;
use std::collections::HashMap;
use std::time::Duration;

const NO_RETRY_ON_RECONNECT: [&str; 12] = [
    // Subscription with side-effects.
    "author_submitAndWatchExtrinsic",
    // chainHead doesn't work without the correct followID.
    "chainHead_unstable_body",
    "chainHead_unstable_call",
    "chainHead_unstable_continue",
    "chainHead_unstable_follow",
    "chainHead_unstable_header",
    "chainHead_unstable_stopOperation",
    "chainHead_unstable_storage",
    "chainHead_unstable_unfollow",
    "chainHead_unstable_unpin",
    // Subscription with side-effects.
    "transactionWatch_unstable_submitAndWatch",
    "transactionWatch_unstable_unwatch",
];

const RETRY_ON_RECONNECT: [&str; 111] = [
    "account_nextIndex",
    "author_hasKey",
    "author_hasSessionKeys",
    "author_insertKey",
    "author_pendingExtrinsics",
    "author_removeExtrinsic",
    "author_rotateKeys",
    "author_submitExtrinsic",
    "author_unwatchExtrinsic",
    "babe_epochAuthorship",
    "beefy_getFinalizedHead",
    "beefy_subscribeJustifications",
    "beefy_unsubscribeJustifications",
    "chainSpec_v1_chainName",
    "chainSpec_v1_genesisHash",
    "chainSpec_v1_properties",
    "chain_getBlock",
    "chain_getBlockHash",
    "chain_getFinalisedHead",
    "chain_getFinalizedHead",
    "chain_getHead",
    "chain_getHeader",
    "chain_getRuntimeVersion",
    "chain_subscribeAllHeads",
    "chain_subscribeFinalisedHeads",
    "chain_subscribeFinalizedHeads",
    "chain_subscribeNewHead",
    "chain_subscribeNewHeads",
    "chain_subscribeRuntimeVersion",
    "chain_unsubscribeAllHeads",
    "chain_unsubscribeFinalisedHeads",
    "chain_unsubscribeFinalizedHeads",
    "chain_unsubscribeNewHead",
    "chain_unsubscribeNewHeads",
    "chain_unsubscribeRuntimeVersion",
    "childstate_getKeys",
    "childstate_getKeysPaged",
    "childstate_getKeysPagedAt",
    "childstate_getStorage",
    "childstate_getStorageEntries",
    "childstate_getStorageHash",
    "childstate_getStorageSize",
    "dev_getBlockStats",
    "grandpa_proveFinality",
    "grandpa_roundState",
    "grandpa_subscribeJustifications",
    "grandpa_unsubscribeJustifications",
    "mmr_generateProof",
    "mmr_root",
    "mmr_verifyProof",
    "mmr_verifyProofStateless",
    "offchain_localStorageGet",
    "offchain_localStorageSet",
    "payment_queryFeeDetails",
    "payment_queryInfo",
    "rpc_methods",
    "state_call",
    "state_callAt",
    "state_getChildReadProof",
    "state_getKeys",
    "state_getKeysPaged",
    "state_getKeysPagedAt",
    "state_getMetadata",
    "state_getPairs",
    "state_getReadProof",
    "state_getRuntimeVersion",
    "state_getStorage",
    "state_getStorageAt",
    "state_getStorageHash",
    "state_getStorageHashAt",
    "state_getStorageSize",
    "state_getStorageSizeAt",
    "state_queryStorage",
    "state_queryStorageAt",
    "state_subscribeRuntimeVersion",
    "state_subscribeStorage",
    "state_traceBlock",
    "state_trieMigrationStatus",
    "state_unsubscribeRuntimeVersion",
    "state_unsubscribeStorage",
    "statement_broadcasts",
    "statement_dump",
    "statement_posted",
    "statement_postedClear",
    "statement_remove",
    "statement_submit",
    "subscribe_newHead",
    "sync_state_genSyncSpec",
    "system_accountNextIndex",
    "system_addLogFilter",
    "system_addReservedPeer",
    "system_chain",
    "system_chainType",
    "system_dryRun",
    "system_dryRunAt",
    "system_health",
    "system_localListenAddresses",
    "system_localPeerId",
    "system_name",
    "system_nodeRoles",
    "system_peers",
    "system_properties",
    "system_removeReservedPeer",
    "system_reservedPeers",
    "system_resetLogFilter",
    "system_syncState",
    "system_unstable_networkState",
    "system_version",
    "transaction_v1_broadcast",
    "transaction_v1_stop",
    "unsubscribe_newHead",
];

/// Reconnecting rpc client builder.
pub struct Builder {
    retry_policy: RetryPolicy,
    methods: HashMap<&'static str, CallRetryPolicy>,
}

impl Builder {
    /// Create a new builder.
    pub fn new() -> Self {
        let mut methods = HashMap::new();

        for method in NO_RETRY_ON_RECONNECT.into_iter() {
            methods.insert(method, CallRetryPolicy::Drop);
        }

        for method in RETRY_ON_RECONNECT.into_iter() {
            methods.insert(method, CallRetryPolicy::Retry);
        }

        Self {
            retry_policy: RetryPolicy::exponential(Duration::from_millis(10))
                .with_max_delay(Duration::from_secs(30)),
            methods,
        }
    }

    /// Configure custom retry policy for a specific rpc call/subscription.
    pub fn retry_policy_for_method(
        mut self,
        method: &'static str,
        policy: CallRetryPolicy,
    ) -> Self {
        self.methods.insert(method, policy);

        self
    }

    /// Set retry policy when reconnecting.
    pub fn retry_policy_for_reconnect(self, retry_policy: RetryPolicy) -> Self {
        Self {
            retry_policy,
            methods: self.methods,
        }
    }

    /// Build a new rpc client i.e, connect.
    pub async fn build(self, url: String) -> Result<Client, RpcError> {
        let client = InnerClient::builder()
            .retry_policy(self.retry_policy)
            .build(url)
            .await
            .map_err(|e| RpcError::ClientError(Box::new(e)))?;

        Ok(Client {
            inner: client,
            methods: self.methods,
        })
    }
}

/// Reconnecting rpc client.
pub struct Client {
    inner: InnerClient,
    methods: HashMap<&'static str, CallRetryPolicy>,
}

impl Client {
    /// Create a builder.
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Future that returns when the reconnection has started.
    pub async fn on_reconnect(&self) {
        self.inner.on_reconnect().await
    }

    /// Counter to determine how many times the client has reconnected.
    pub fn reconnect_count(&self) -> usize {
        self.inner.reconnect_count()
    }

    fn get_method_retry_policy(&self, method: &str) -> CallRetryPolicy {
        if let Some(policy) = self.methods.get(method) {
            *policy
        } else {
            tracing::warn!("unknown method `{method}`; setting retry policy to drop on reconnect");
            CallRetryPolicy::Drop
        }
    }
}

impl RpcClientT for Client {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        async {
            let retry_policy = self.get_method_retry_policy(method);
            self.inner
                .request_raw_with_policy(method.to_string(), params, retry_policy)
                .await
                .map_err(|e| RpcError::ClientError(Box::new(e)))
        }
        .boxed()
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        async {
            let retry_policy = self.get_method_retry_policy(sub);
            let sub = self
                .inner
                .subscribe_raw_with_policy(sub.to_string(), params, unsub.to_string(), retry_policy)
                .await
                .map_err(|e| RpcError::ClientError(Box::new(e)))?;

            let id = match sub.id() {
                SubscriptionId::Num(n) => n.to_string(),
                SubscriptionId::Str(s) => s.to_string(),
            };
            let stream = sub
                .map_err(|e| RpcError::DisconnectedWillReconnect(e.to_string()))
                .boxed();

            Ok(RawRpcSubscription {
                stream,
                id: Some(id),
            })
        }
        .boxed()
    }
}
