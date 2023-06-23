// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Native implementation for the light client's platform using
//! `tokio::net::TcpStream` for connections.

use core::time::Duration;
use futures_util::{future, FutureExt};
use smoldot::libp2p::{multiaddr::ProtocolRef, websocket};
use smoldot_light::platform::{ConnectError, PlatformConnection};
use std::{
    collections::VecDeque,
    net::{IpAddr, SocketAddr},
};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

pub fn spawn(task: future::BoxFuture<'static, ()>) {
    tokio::spawn(task);
}

pub fn now_from_unix_epoch() -> Duration {
    // Intentionally panic if the time is configured earlier than the UNIX EPOCH.
    std::time::UNIX_EPOCH.elapsed().unwrap_or_else(|_| {
        panic!("Invalid systime cannot be configured earlier than `UNIX_EPOCH`")
    })
}

pub type Instant = std::time::Instant;

pub fn now() -> Instant {
    Instant::now()
}

pub type Delay = future::BoxFuture<'static, ()>;

pub fn sleep(duration: Duration) -> Delay {
    tokio::time::sleep(duration).boxed()
}

pub type CompatTcpStream = Compat<TcpStream>;
pub type Socket = future::Either<CompatTcpStream, websocket::Connection<CompatTcpStream>>;

pub struct Stream {
    pub socket: Socket,
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
    let (addr, host_if_websocket) = match (&proto1, &proto2, &proto3) {
        (ProtocolRef::Ip4(ip), ProtocolRef::Tcp(port), None) => (
            either::Left(SocketAddr::new(IpAddr::V4((*ip).into()), *port)),
            None,
        ),
        (ProtocolRef::Ip6(ip), ProtocolRef::Tcp(port), None) => (
            either::Left(SocketAddr::new(IpAddr::V6((*ip).into()), *port)),
            None,
        ),
        (ProtocolRef::Ip4(ip), ProtocolRef::Tcp(port), Some(ProtocolRef::Ws)) => {
            let addr = SocketAddr::new(IpAddr::V4((*ip).into()), *port);
            (either::Left(addr), Some(addr.to_string()))
        }
        (ProtocolRef::Ip6(ip), ProtocolRef::Tcp(port), Some(ProtocolRef::Ws)) => {
            let addr = SocketAddr::new(IpAddr::V6((*ip).into()), *port);
            (either::Left(addr), Some(addr.to_string()))
        }
        (
            ProtocolRef::Dns(addr) | ProtocolRef::Dns4(addr) | ProtocolRef::Dns6(addr),
            ProtocolRef::Tcp(port),
            None,
        ) => (either::Right((addr.to_string(), *port)), None),
        (
            ProtocolRef::Dns(addr) | ProtocolRef::Dns4(addr) | ProtocolRef::Dns6(addr),
            ProtocolRef::Tcp(port),
            Some(ProtocolRef::Ws),
        ) => (
            either::Right((addr.to_string(), *port)),
            Some(format!("{}:{}", addr, *port)),
        ),
        _ => {
            return Err(ConnectError {
                is_bad_addr: true,
                message: "Unknown protocols combination".to_string(),
            })
        }
    };

    tracing::debug!("Connecting to addr={addr:?}");

    let tcp_socket = match addr {
        either::Left(socket_addr) => tokio::net::TcpStream::connect(socket_addr).await,
        either::Right((dns, port)) => tokio::net::TcpStream::connect((&dns[..], port)).await,
    };

    if let Ok(tcp_socket) = &tcp_socket {
        let _ = tcp_socket.set_nodelay(true);
    }

    let socket: Socket = match (tcp_socket, host_if_websocket) {
        (Ok(tcp_socket), Some(host)) => future::Either::Right(
            websocket::websocket_client_handshake(websocket::Config {
                tcp_socket: tcp_socket.compat(),
                host: &host,
                url: "/",
            })
            .await
            .map_err(|err| ConnectError {
                message: format!("Failed to negotiate WebSocket: {err}"),
                is_bad_addr: false,
            })?,
        ),
        (Ok(tcp_socket), None) => future::Either::Left(tcp_socket.compat()),
        (Err(err), _) => {
            return Err(ConnectError {
                is_bad_addr: false,
                message: format!("Failed to reach peer: {err}"),
            })
        }
    };

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
