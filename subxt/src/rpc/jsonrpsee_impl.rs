// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{
    RpcClientT,
    RpcResponse,
    RpcSubscription
};
use jsonrpsee::core::client::{
    Client
};

impl RpcClientT for Client {
    fn request<P, I, R>(&self, method: &str, params: P) -> RpcResponse<R>
    where
        P: IntoIterator<Item = I>,
        I: serde::Serialize,
        R: serde::de::DeserializeOwned {
        Box::pin(self.request(method, params))
    }

    fn subscribe<P, I, R>(&self, sub: &str, params: P, unsub: &str) -> RpcSubscription<R>
        where
            P: IntoIterator<Item = I>,
            I: serde::Serialize,
            R: serde::de::DeserializeOwned {
        Box::pin(self.subscribe(sub, params, unsub))
    }
}