// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utils.

use crate::backend::rpc::reconnecting_rpc_client::RpcError;
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};
use tokio::sync::Notify;

#[derive(Clone, Debug)]
pub(crate) struct ReconnectCounter(Arc<AtomicUsize>);

impl Default for ReconnectCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl ReconnectCounter {
    pub fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }

    pub fn get(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }

    pub fn inc(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

pub(crate) fn reconnect_channel() -> (ReconnectTx, ReconnectRx) {
    let count = ReconnectCounter::new();
    let reconn_init = Arc::new(Notify::new());
    let reconn_compl = Arc::new(Notify::new());
    (
        ReconnectTx {
            reconn_init: reconn_init.clone(),
            reconn_compl: reconn_compl.clone(),
            count: count.clone(),
        },
        ReconnectRx {
            reconn_init,
            reconn_compl,
            count,
        },
    )
}

#[derive(Debug, Clone)]
pub(crate) struct ReconnectTx {
    reconn_init: Arc<Notify>,
    reconn_compl: Arc<Notify>,
    count: ReconnectCounter,
}

impl ReconnectTx {
    pub fn reconnect_initiated(&self) {
        self.reconn_init.notify_one();
    }

    pub fn reconnected(&self) {
        self.reconn_compl.notify_one();
        self.count.inc();
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ReconnectRx {
    reconn_init: Arc<Notify>,
    reconn_compl: Arc<Notify>,
    count: ReconnectCounter,
}

impl ReconnectRx {
    pub async fn reconnect_started(&self) {
        self.reconn_init.notified().await;
    }

    pub async fn reconnected(&self) {
        self.reconn_compl.notified().await;
    }

    pub fn count(&self) -> usize {
        self.count.get()
    }
}

pub fn display_close_reason(err: &RpcError) -> String {
    match err {
        RpcError::RestartNeeded(e) => e.to_string(),
        other => other.to_string(),
    }
}
