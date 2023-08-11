// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module exposes a legacy backend implementation, which relies
//! on the legacy RPC API methods.

pub mod methods;

use crate::config::Config;
use super::{ Backend, rpc::{RpcClient, RpcClientT} };
use std::sync::Arc;

/// The legacy backend.
pub struct LegacyBackend<T: Config> {
    client: RpcClient,
    _marker: std::marker::PhantomData<T>
}

impl <T: Config> LegacyBackend<T> {
    pub fn new<R: RpcClientT>(client: R) -> Self {
        Self {
            client: RpcClient::new(Arc::new(client)),
            _marker: std::marker::PhantomData
        }
    }
}

// impl <T: Config> Backend<T> for LegacyBackend<T> {

// }