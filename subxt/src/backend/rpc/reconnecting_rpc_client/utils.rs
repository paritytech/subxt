// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utils.

use crate::backend::rpc::reconnecting_rpc_client::RpcError;

pub fn display_close_reason(err: &RpcError) -> String {
    match err {
        RpcError::RestartNeeded(e) => e.to_string(),
        other => other.to_string(),
    }
}
