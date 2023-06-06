use futures_timer::Delay;
use futures_util::{future, FutureExt};

use gloo_net::websocket::{futures::WebSocket, Message, WebSocketError};
use smoldot::libp2p::multiaddr::{Multiaddr, ProtocolRef};

use futures::SinkExt;
use futures_util::stream::{SplitSink, SplitStream, StreamExt};

use smoldot_light::platform::ConnectError;
use smoldot_light::platform::PlatformConnection;
use smoldot_light::platform::PlatformSubstreamDirection;

use std::sync::Arc;
use std::sync::Mutex;
use std::{
    io::IoSlice,
    net::{IpAddr, SocketAddr},
};

use core::{mem, pin, str, task, time::Duration};
use std::{
    borrow::Cow,
    collections::{BTreeMap, VecDeque},
    sync::atomic::{AtomicU64, Ordering},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct Platform {}

impl Platform {
    pub const fn new() -> Self {
        Self {}
    }
}

impl smoldot_light::platform::PlatformRef for Platform {
    type Delay = future::BoxFuture<'static, ()>;

    // No-op yielding.
    type Yield = future::Ready<()>;

    type Instant = instant::Instant;

    type Connection = std::convert::Infallible;

    type Stream = ConnectionStream;

    type ConnectFuture = future::BoxFuture<
        'static,
        Result<PlatformConnection<Self::Stream, Self::Connection>, ConnectError>,
    >;

    type StreamUpdateFuture<'a> = future::BoxFuture<'a, ()>;

    type NextSubstreamFuture<'a> =
        future::Pending<Option<(Self::Stream, PlatformSubstreamDirection)>>;

    fn now_from_unix_epoch(&self) -> instant::Duration {
        tracing::trace!("[call] now_from_unix_epoch");

        // The documentation of `now_from_unix_epoch()` mentions that it's ok to panic if we're
        // before the UNIX epoch.

        let res = instant::SystemTime::now()
            .duration_since(instant::SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| panic!());

        // let res = std::time::UNIX_EPOCH
        //     .elapsed()
        //     .expect("Invalid systime cannot be configured earlier than `UNIX_EPOCH`");
        tracing::trace!("[response] now_from_unix_epoch={:?}", res);

        res
    }

    fn now(&self) -> Self::Instant {
        // tracing::trace!("[call] now");
        let now = instant::Instant::now();
        // tracing::trace!("[res] now ={:?}", now);
        now
    }

    fn sleep(&self, duration: Duration) -> Self::Delay {
        tracing::trace!("[call] sleep");
        let future = futures_timer::Delay::new(duration).boxed();
        tracing::trace!("[res] sleep");
        future
    }

    fn sleep_until(&self, when: Self::Instant) -> Self::Delay {
        tracing::trace!("[call] sleep_until");
        let res = self.sleep(when.saturating_duration_since(self.now()));
        tracing::trace!("[res] sleep_until");
        res
    }

    fn spawn_task(
        &self,
        task_name: std::borrow::Cow<str>,
        task: futures_util::future::BoxFuture<'static, ()>,
    ) {
        tracing::trace!("[call] spawn_task task_name={:?}", task_name);

        wasm_bindgen_futures::spawn_local(task)
    }

    fn client_name(&self) -> std::borrow::Cow<str> {
        "subxt".into()
    }

    fn client_version(&self) -> std::borrow::Cow<str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn yield_after_cpu_intensive(&self) -> Self::Yield {
        future::ready(())
    }

    fn connect(&self, url: &str) -> Self::ConnectFuture {
        tracing::trace!("[call] connect url={:?}", url);

        let url = url.to_string();

        Box::pin(async move {
            // let url = url.to_string();

            let multiaddr = url.parse::<Multiaddr>().map_err(|_| ConnectError {
                message: format!("Address {url} is not a valid multiaddress"),
                is_bad_addr: true,
            })?;

            // First two protocals must be valid, the third one is optional.
            let mut proto_iter = multiaddr.iter().fuse();

            let addr = match (
                proto_iter.next().ok_or(ConnectError {
                    message: format!("Unknown protocol combination"),
                    is_bad_addr: true,
                })?,
                proto_iter.next().ok_or(ConnectError {
                    message: format!("Unknown protocol combination"),
                    is_bad_addr: true,
                })?,
                proto_iter.next(),
            ) {
                (ProtocolRef::Ip4(ip), ProtocolRef::Tcp(port), None) => {
                    SocketAddr::new(IpAddr::V4((ip).into()), port)
                }
                (ProtocolRef::Ip6(ip), ProtocolRef::Tcp(port), None) => {
                    SocketAddr::new(IpAddr::V6((ip).into()), port)
                }
                (ProtocolRef::Ip4(ip), ProtocolRef::Tcp(port), Some(ProtocolRef::Ws)) => {
                    SocketAddr::new(IpAddr::V4((ip).into()), port)
                }
                (ProtocolRef::Ip6(ip), ProtocolRef::Tcp(port), Some(ProtocolRef::Ws)) => {
                    SocketAddr::new(IpAddr::V6((ip).into()), port)
                }
                // TODO: Minimal protocol, check that basic connection is working.
                // TODO: we don't care about the differences between Dns, Dns4, and Dns6
                // (
                //     ProtocolRef::Dns(addr) | ProtocolRef::Dns4(addr) | ProtocolRef::Dns6(addr),
                //     ProtocolRef::Tcp(port),
                //     None,
                // ) => (either::Right((addr.to_string(), *port)), None),
                // (
                //     ProtocolRef::Dns(addr) | ProtocolRef::Dns4(addr) | ProtocolRef::Dns6(addr),
                //     ProtocolRef::Tcp(port),
                //     Some(ProtocolRef::Ws),
                // ) => (
                //     either::Right((addr.to_string(), *port)),
                //     Some(format!("{}:{}", addr, *port)),
                // ),
                _ => {
                    return Err(ConnectError {
                        is_bad_addr: true,
                        message: "Unknown protocols combination".to_string(),
                    })
                }
            };

            // TODO: use `addr` instead.
            let websocket = WebSocket::open(url.as_ref()).map_err(|e| ConnectError {
                is_bad_addr: false,
                message: "Cannot stablish WebSocket connection".to_string(),
            })?;

            // let (to_sender, from_sender) = mpsc::channel(1024);
            // let (to_receiver, from_receiver) = mpsc::channel(1024);

            //TODO: Spawn a task:
            //    enum Protocol {
            //        Send(String),
            //        Recv(String),
            //    }
            //
            //    bg_task {
            //        while ..
            //        Future:
            //            A: if Send() .. self.send();
            //            B: self.next() -> recv_end.send("Msg");
            //
            //    }
            //
            //    from fn send(&self, stream: &mut Self::Stream, data: &[u8]) {
            //        let str: String = data.into();
            //        sender.send(str);
            //    }
            //
            //    from fn read_buffer<'a> {
            let (sender, receiver) = websocket.split();
            let conn = ConnectionStream {
                inner: Arc::new(Mutex::new(ConnectionInner { sender, receiver })),
            };

            Ok(PlatformConnection::SingleStreamMultistreamSelectNoiseYamux(
                conn,
            ))
        })
    }

    fn open_out_substream(&self, _connection: &mut Self::Connection) {
        // Called from MultiStream connections that are never opened for this implementation.
    }

    fn next_substream<'a>(
        &self,
        connection: &'a mut Self::Connection,
    ) -> Self::NextSubstreamFuture<'a> {
        // Called from MultiStream connections that are never opened for this implementation.
        // futures::future::pending::<Option<(ConnectionStream, PlatformSubstreamDirection)>>()
        futures::future::pending()
    }

    fn update_stream<'a>(&self, stream: &'a mut Self::Stream) -> Self::StreamUpdateFuture<'a> {
        tracing::trace!("[call] update_stream");

        Box::pin(async move {})
    }

    fn read_buffer<'a>(
        &self,
        stream: &'a mut Self::Stream,
    ) -> smoldot_light::platform::ReadBuffer<'a> {
        tracing::trace!("[call] read_buffer");

        let mut locked = stream
            .inner
            .lock()
            .expect("Mutex should not be poised; qed");

        // let msg = futures_executor::block_on(async {
        let msg = futures::executor::block_on(async {
            match locked.receiver.next().await {
                Some(Ok(msg)) => Some(msg),
                _ => None,
            }
        });

        match msg {
            Some(msg) => {
                let msg = Box::leak(Box::new(msg));

                match msg {
                    Message::Text(text) => {
                        smoldot_light::platform::ReadBuffer::Open(text.as_bytes())
                    }
                    Message::Bytes(bytes) => smoldot_light::platform::ReadBuffer::Open(bytes),
                }
            }
            None => smoldot_light::platform::ReadBuffer::Closed,
        }
    }

    fn advance_read_cursor(&self, stream: &mut Self::Stream, bytes: usize) {
        tracing::trace!("[call] advance_read_cursor");
    }

    fn writable_bytes(&self, stream: &mut Self::Stream) -> usize {
        tracing::trace!("[call] writable_bytes");
        1024
    }

    fn send(&self, stream: &mut Self::Stream, data: &[u8]) {
        tracing::trace!("[call] send");

        let mut locked = stream
            .inner
            .lock()
            .expect("Mutex should not be poised; qed");

        if let Ok(message) = String::from_utf8(data.into()) {
            let _ = locked.sender.send(Message::Text(message));
        }
    }

    fn close_send(&self, stream: &mut Self::Stream) {
        tracing::trace!("[call] close_send");
    }
}

pub struct ConnectionInner {
    sender: SplitSink<WebSocket, Message>,
    receiver: SplitStream<WebSocket>,
}

unsafe impl Send for ConnectionInner {}

pub struct ConnectionStream {
    inner: Arc<Mutex<ConnectionInner>>,
}

unsafe impl Send for ConnectionStream {}
