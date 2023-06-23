// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Wasm implementation for the light client's platform using
//! custom websockets.

use core::time::Duration;
use futures_util::{future, FutureExt};
use smoldot::libp2p::multiaddr::ProtocolRef;
use smoldot_light::platform::{ConnectError, PlatformConnection};
use std::{
    collections::VecDeque,
    net::{IpAddr, SocketAddr},
};

use super::wasm_socket::WasmSocket;

pub fn spawn(task: future::BoxFuture<'static, ()>) {
    wasm_bindgen_futures::spawn_local(task);
}

pub fn now_from_unix_epoch() -> Duration {
    instant::SystemTime::now()
        .duration_since(instant::SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|_| {
            panic!("Invalid systime cannot be configured earlier than `UNIX_EPOCH`")
        })
}

pub type Instant = instant::Instant;

pub fn now() -> Instant {
    instant::Instant::now()
}

pub type Delay = future::BoxFuture<'static, ()>;

pub fn sleep(duration: Duration) -> Delay {
    futures_timer::Delay::new(duration).boxed()
}

pub struct Stream {
    pub socket: WasmSocket,
    /// Read and write buffers of the connection, or `None` if the socket has been reset.
    pub buffers: Option<(StreamReadBuffer, StreamWriteBuffer)>,
}

pub enum StreamReadBuffer {
    Open {
        buffer: Vec<u8>,
        cursor: std::ops::Range<usize>,
    },
    Closed,
}

pub enum StreamWriteBuffer {
    Open {
        buffer: VecDeque<u8>,
        must_flush: bool,
        must_close: bool,
    },
    Closed,
}

pub async fn connect<'a>(
    proto1: ProtocolRef<'a>,
    proto2: ProtocolRef<'a>,
    proto3: Option<ProtocolRef<'a>>,
) -> Result<PlatformConnection<Stream, std::convert::Infallible>, ConnectError> {
    // Ensure ahead of time that the multiaddress is supported.
    let addr = match (&proto1, &proto2, &proto3) {
        (ProtocolRef::Ip4(ip), ProtocolRef::Tcp(port), Some(ProtocolRef::Ws)) => {
            SocketAddr::new(IpAddr::V4((*ip).into()), *port)
        }
        (ProtocolRef::Ip6(ip), ProtocolRef::Tcp(port), Some(ProtocolRef::Ws)) => {
            SocketAddr::new(IpAddr::V6((*ip).into()), *port)
        }
        _ => {
            return Err(ConnectError {
                is_bad_addr: true,
                message: "Unknown protocols combination".to_string(),
            })
        }
    };

    let addr = format!("ws://{}", addr.to_string());
    tracing::debug!("Connecting to addr={addr}");

    let socket = WasmSocket::new(addr.as_str()).map_err(|err| ConnectError {
        is_bad_addr: false,
        message: format!("Failed to reach peer: {err}"),
    })?;

    Ok(PlatformConnection::SingleStreamMultistreamSelectNoiseYamux(
        Stream {
            socket,
            buffers: Some((
                StreamReadBuffer::Open {
                    buffer: vec![0; 16384],
                    cursor: 0..0,
                },
                StreamWriteBuffer::Open {
                    buffer: VecDeque::with_capacity(16384),
                    must_close: false,
                    must_flush: false,
                },
            )),
        },
    ))
}
