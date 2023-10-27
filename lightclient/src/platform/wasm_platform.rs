// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::wasm_socket::WasmSocket;

use core::time::Duration;
use futures::prelude::*;
use smoldot_light::platform::{
    Address, ConnectError, ConnectionType, IpAddr, MultiStreamAddress, MultiStreamWebRtcConnection,
    PlatformRef, SubstreamDirection,
};
use std::{io, net::SocketAddr, pin::Pin};

const LOG_TARGET: &str = "subxt-platform-wasm";

/// Subxt platform implementation for wasm.
///
/// This implementation is a conversion of the implementation from the smoldot:
/// https://github.com/smol-dot/smoldot/blob/6401d4df90569e23073d646b14a8fbf9f7e6bdd3/light-base/src/platform/default.rs#L83.
///
/// This platform will evolve over time and we'll need to keep this code in sync.
#[derive(Clone)]
pub struct SubxtPlatform {}

impl SubxtPlatform {
    pub fn new() -> Self {
        SubxtPlatform {}
    }
}

impl PlatformRef for SubxtPlatform {
    type Delay = super::wasm_helpers::Delay;
    type Instant = super::wasm_helpers::Instant;
    type MultiStream = std::convert::Infallible;
    type Stream = super::wasm_helpers::Stream;
    type StreamConnectFuture = future::BoxFuture<'static, Result<Self::Stream, ConnectError>>;
    type MultiStreamConnectFuture = future::BoxFuture<
        'static,
        Result<MultiStreamWebRtcConnection<Self::MultiStream>, ConnectError>,
    >;
    type ReadWriteAccess<'a> = super::wasm_helpers::with_buffers::ReadWriteAccess<'a>;
    type StreamUpdateFuture<'a> = future::BoxFuture<'a, ()>;
    type StreamErrorRef<'a> = &'a std::io::Error;
    type NextSubstreamFuture<'a> = future::Pending<Option<(Self::Stream, SubstreamDirection)>>;

    fn now_from_unix_epoch(&self) -> Duration {
        super::wasm_helpers::now_from_unix_epoch()
    }

    fn now(&self) -> Self::Instant {
        super::wasm_helpers::now()
    }

    fn fill_random_bytes(&self, buffer: &mut [u8]) {
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), buffer);
    }

    fn sleep(&self, duration: Duration) -> Self::Delay {
        super::wasm_helpers::sleep(duration)
    }

    fn sleep_until(&self, when: Self::Instant) -> Self::Delay {
        self.sleep(when.saturating_duration_since(self.now()))
    }

    fn spawn_task(
        &self,
        _task_name: std::borrow::Cow<str>,
        task: impl future::Future<Output = ()> + Send + 'static,
    ) {
        wasm_bindgen_futures::spawn_local(task);
    }

    fn client_name(&self) -> std::borrow::Cow<str> {
        "subxt-light-client".into()
    }

    fn client_version(&self) -> std::borrow::Cow<str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn supports_connection_type(&self, connection_type: ConnectionType) -> bool {
        let result = matches!(
            connection_type,
            ConnectionType::WebSocketIpv4 { .. }
                | ConnectionType::WebSocketIpv6 { .. }
                | ConnectionType::WebSocketDns { .. }
        );

        tracing::trace!(
            target: LOG_TARGET,
            "Supports connection type={:?} result={}",
            connection_type, result
        );

        result
    }

    fn connect_stream(&self, multiaddr: Address) -> Self::StreamConnectFuture {
        tracing::trace!(target: LOG_TARGET, "Connect stream to multiaddr={:?}", multiaddr);

        // `PlatformRef` trait guarantees that `connect_stream` is only called with addresses
        // stated in `supports_connection_type`.
        let addr = match multiaddr {
            Address::WebSocketDns {
                hostname,
                port,
                secure: true,
            } => {
                format!("wss://{}:{}", hostname, port)
            }
            Address::WebSocketDns {
                hostname,
                port,
                secure: false,
            } => {
                format!("ws://{}:{}", hostname, port)
            }
            Address::WebSocketIp {
                ip: IpAddr::V4(ip),
                port,
            } => {
                let addr = SocketAddr::from((ip, port));
                format!("ws://{}", addr.to_string())
            }
            Address::WebSocketIp {
                ip: IpAddr::V6(ip),
                port,
            } => {
                let addr = SocketAddr::from((ip, port));
                format!("ws://{}", addr.to_string())
            }

            // The API user of the `PlatformRef` trait is never supposed to open connections of
            // a type that isn't supported.
            _ => {
                unreachable!("Connecting to an address not supported. This code path indicates a bug in smoldot. Please raise an issue at https://github.com/smol-dot/smoldot/issues")
            }
        };

        Box::pin(async move {
            tracing::debug!(target: LOG_TARGET, "Connecting to addr={addr}");

            let socket = WasmSocket::new(addr.as_str()).map_err(|err| ConnectError {
                message: format!("Failed to reach peer: {err}"),
            })?;

            Ok(super::wasm_helpers::Stream(
                super::wasm_helpers::with_buffers::WithBuffers::new(socket),
            ))
        })
    }

    fn connect_multistream(&self, _address: MultiStreamAddress) -> Self::MultiStreamConnectFuture {
        panic!("Multistreams are not currently supported. This code path indicates a bug in smoldot. Please raise an issue at https://github.com/smol-dot/smoldot/issues")
    }

    fn open_out_substream(&self, c: &mut Self::MultiStream) {
        // This function can only be called with so-called "multi-stream" connections. We never
        // open such connection.
        match *c {}
    }

    fn next_substream(&self, c: &'_ mut Self::MultiStream) -> Self::NextSubstreamFuture<'_> {
        // This function can only be called with so-called "multi-stream" connections. We never
        // open such connection.
        match *c {}
    }

    fn read_write_access<'a>(
        &self,
        stream: Pin<&'a mut Self::Stream>,
    ) -> Result<Self::ReadWriteAccess<'a>, &'a io::Error> {
        let stream = stream.project();
        stream.0.read_write_access(Self::Instant::now())
    }

    fn wait_read_write_again<'a>(
        &self,
        stream: Pin<&'a mut Self::Stream>,
    ) -> Self::StreamUpdateFuture<'a> {
        let stream = stream.project();
        Box::pin(stream.0.wait_read_write_again(|when| async move {
            let now = super::wasm_helpers::now();
            let duration = when.saturating_duration_since(now);
            super::wasm_helpers::sleep(duration).await;
        }))
    }
}
