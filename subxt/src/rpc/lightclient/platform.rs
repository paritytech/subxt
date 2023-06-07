use futures_timer::Delay;
use futures_util::AsyncWriteExt;
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
use std::task::Context;
use std::{
    io::IoSlice,
    net::{IpAddr, SocketAddr},
};

use core::task::Poll;

use core::ops;
use core::pin::Pin;
use core::{mem, pin, str, task, time::Duration};

use std::{
    borrow::Cow,
    collections::{BTreeMap, VecDeque},
    sync::atomic::{AtomicU64, Ordering},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use smoldot_light::platform::ReadBuffer;

use tokio::sync::{mpsc, oneshot};

/// Wasm compatible light-client platform for executing low-level operations.
#[derive(Clone)]
pub struct Platform {}

impl Platform {
    /// Constructs a new [`Platform`].
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
        tracing::trace!("[now_from_unix_epoch]");

        // The documentation of `now_from_unix_epoch()` mentions that it's ok to panic if we're
        // before the UNIX epoch.
        let res = instant::SystemTime::now()
            .duration_since(instant::SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| {
                panic!("Invalid systime cannot be configured earlier than `UNIX_EPOCH`")
            });
        tracing::trace!("[now_from_unix_epoch] result={:?}", res);

        res
    }

    fn now(&self) -> Self::Instant {
        // tracing::trace!("[now]");

        instant::Instant::now()
    }

    fn sleep(&self, duration: Duration) -> Self::Delay {
        tracing::trace!("[sleep] duration={:?}", duration);

        futures_timer::Delay::new(duration).boxed()
    }

    fn sleep_until(&self, when: Self::Instant) -> Self::Delay {
        tracing::trace!("[sleep_until] when={:?}", when);

        self.sleep(when.saturating_duration_since(self.now()))
    }

    fn spawn_task(
        &self,
        task_name: std::borrow::Cow<str>,
        task: futures_util::future::BoxFuture<'static, ()>,
    ) {
        tracing::trace!("[spawn_task] task_name={:?}", task_name);

        wasm_bindgen_futures::spawn_local(task)
    }

    fn client_name(&self) -> std::borrow::Cow<str> {
        "subxt".into()
    }

    fn client_version(&self) -> std::borrow::Cow<str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn yield_after_cpu_intensive(&self) -> Self::Yield {
        tracing::trace!("[yield_after_cpu_intensive]");
        future::ready(())
    }

    fn connect(&self, url: &str) -> Self::ConnectFuture {
        tracing::trace!("[connect] url={:?}", url);

        let url = url.to_string();
        Box::pin(async move {
            let multiaddr = url.parse::<Multiaddr>().map_err(|err| {
                tracing::trace!("[connect] Address provided {} is invalid {:?}", url, err);
                ConnectError {
                    message: format!("Address {url} is not a valid multiaddress"),
                    is_bad_addr: true,
                }
            })?;

            // First two protocals must be valid, the third one is optional.
            let mut proto_iter = multiaddr.iter().fuse();

            let proto1 = proto_iter.next().ok_or_else(|| {
                tracing::trace!("[connect] Cannot find first protocol");
                ConnectError {
                    message: format!("Unknown protocol combination"),
                    is_bad_addr: true,
                }
            })?;

            let proto2 = proto_iter.next().ok_or_else(|| {
                tracing::trace!("[connect] Cannot find second protocol");
                ConnectError {
                    message: format!("Unknown protocol combination"),
                    is_bad_addr: true,
                }
            })?;

            let proto3 = proto_iter.next();

            let addr = match (proto1, proto2, proto3) {
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
                _ => {
                    tracing::warn!("[connect] Unknown protocol combination");

                    return Err(ConnectError {
                        is_bad_addr: true,
                        message: "Unknown protocols combination".to_string(),
                    });
                }
            };

            let addr = format!("ws://{}", addr.to_string());
            tracing::trace!("[connect] Connecting to addr={:?}", addr);

            // TODO: use `addr` instead.
            let websocket = WebSocket::open(addr.as_ref()).map_err(|err| {
                tracing::trace!("[connect] Cannot connect to add {:?}", err);
                ConnectError {
                    is_bad_addr: false,
                    message: "Cannot stablish WebSocket connection".to_string(),
                }
            })?;
            tracing::trace!("[connect] Connection established");

            let (sender, receiver) = websocket.split();
            let conn = ConnectionStream {
                inner: Arc::new(Mutex::new(ConnectionInner { sender, receiver })),
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
            };

            Ok(PlatformConnection::SingleStreamMultistreamSelectNoiseYamux(
                conn,
            ))
        })
    }

    fn open_out_substream(&self, _connection: &mut Self::Connection) {
        tracing::trace!("[call] open_out_substream");

        // Called from MultiStream connections that are never opened for this implementation.
    }

    fn next_substream<'a>(
        &self,
        connection: &'a mut Self::Connection,
    ) -> Self::NextSubstreamFuture<'a> {
        tracing::trace!("[call] next_substream");
        // Called from MultiStream connections that are never opened for this implementation.
        // futures::future::pending::<Option<(ConnectionStream, PlatformSubstreamDirection)>>()
        futures::future::pending()
    }

    fn update_stream<'a>(&self, stream: &'a mut Self::Stream) -> Self::StreamUpdateFuture<'a> {
        tracing::trace!("[update_stream]");

        use futures::Future;

        Box::pin(future::poll_fn(|cx| {
            let Some((read_buffer, write_buffer)) = stream.buffers.as_mut() else {
                tracing::trace!("[update_stream] Buffers are empty");
                return Poll::Pending
            };

            let mut locked = stream.inner.lock().unwrap();

            // Whether the future returned by `update_stream` should return `Ready` or `Pending`.
            let mut update_stream_future_ready = false;

            if let StreamReadBuffer::Open {
                buffer: ref mut buf,
                ref mut cursor,
            } = read_buffer
            {
                tracing::trace!("[update_stream] StreamReadBuffer is open");

                // When reading data from the socket, `poll_read` might return "EOF". In that
                // situation, we transition to the `Closed` state, which would discard the data
                // currently in the buffer. For this reason, we only try to read if there is no
                // data left in the buffer.
                if cursor.start == cursor.end {
                    let mut stream_recv = locked.receiver.next();
                    if let Poll::Ready(result) = Pin::new(&mut stream_recv).poll(cx) {
                        tracing::trace!("[update_stream] Received from socket");
                        update_stream_future_ready = true;
                        match result {
                            Some(Ok(message)) => {
                                tracing::trace!(
                                    "[update_stream] Received from socket message={:?}",
                                    message
                                );
                                // These bytes must end-up in the read buffer.
                                let bytes = match message {
                                    Message::Text(text) => text.into_bytes(),
                                    Message::Bytes(bytes) => bytes,
                                };

                                for (index, byte) in bytes.iter().enumerate() {
                                    buf[index] = *byte;
                                }

                                *cursor = 0..bytes.len();
                            }
                            Some(Err(err)) => {
                                tracing::warn!(
                                    "[update_stream] Reached Websocket error: {:?}",
                                    err
                                );

                                stream.buffers = None;
                                return Poll::Ready(());
                            }
                            None => {
                                tracing::warn!("[update_stream] Reached EOF");
                                // EOF.
                                *read_buffer = StreamReadBuffer::Closed;
                            }
                        }
                    }
                }
            }

            if let StreamWriteBuffer::Open {
                buffer: ref mut buf,
                must_flush,
                must_close,
            } = write_buffer
            {
                while !buf.is_empty() {
                    let write_queue_slices = buf.as_slices();
                    let len = write_queue_slices.0.len() + write_queue_slices.1.len();

                    let slices = &[
                        IoSlice::new(write_queue_slices.0),
                        IoSlice::new(write_queue_slices.1),
                    ];

                    tracing::trace!(
                        "[update_stream] Prepare to send first={:?}",
                        write_queue_slices.0
                    );

                    tracing::trace!(
                        "[update_stream] Prepare to send second={:?}",
                        write_queue_slices.1
                    );

                    let len = write_queue_slices.1.len();
                    let message = Message::Bytes(write_queue_slices.1.to_owned());

                    tracing::trace!("[update_stream] Sending={:?} len={}", message, len);

                    let mut stream_send = locked.sender.send(message);

                    if let Poll::Ready(result) = Pin::new(&mut stream_send).poll(cx) {
                        if !*must_close {
                            // In the situation where the API user wants to close the writing
                            // side, simply sending the buffered data isn't enough to justify
                            // making the future ready.
                            update_stream_future_ready = true;
                        }

                        match result {
                            Err(err) => {
                                tracing::trace!("[update_stream] Sending Error {:?}", err);

                                // End the stream.
                                stream.buffers = None;
                                return Poll::Ready(());
                            }
                            Ok(_) => {
                                tracing::trace!("[update_stream] Sending ok");

                                *must_flush = true;
                                for _ in 0..len {
                                    buf.pop_front();
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }

                // if buf.is_empty() && *must_close {
                //     if let Poll::Ready(result) = Pin::new(&mut stream.socket).poll_close(cx) {
                //         update_stream_future_ready = true;
                //         match result {
                //             Err(_) => {
                //                 // End the stream.
                //                 stream.buffers = None;
                //                 return Poll::Ready(());
                //             }
                //             Ok(()) => {
                //                 *write_buffer = StreamWriteBuffer::Closed;
                //             }
                //         }
                //     }
                // } else if *must_flush {
                //     if let Poll::Ready(result) = Pin::new(&mut stream.socket).poll_flush(cx) {
                //         update_stream_future_ready = true;
                //         match result {
                //             Err(_) => {
                //                 // End the stream.
                //                 stream.buffers = None;
                //                 return Poll::Ready(());
                //             }
                //             Ok(()) => {
                //                 *must_flush = false;
                //             }
                //         }
                //     }
                // }
            }

            if update_stream_future_ready {
                tracing::trace!("[update_stream] Future ready");

                Poll::Ready(())
            } else {
                tracing::trace!("[update_stream] Future pending");

                Poll::Pending
            }
        }))
        // Box::pin(async move {

        // })
    }

    fn read_buffer<'a>(
        &self,
        stream: &'a mut Self::Stream,
    ) -> smoldot_light::platform::ReadBuffer<'a> {
        tracing::trace!("[read_buffer]");

        match stream.buffers.as_ref().map(|(r, _)| r) {
            None => ReadBuffer::Reset,
            Some(StreamReadBuffer::Closed) => ReadBuffer::Closed,
            Some(StreamReadBuffer::Open { buffer, cursor }) => {
                ReadBuffer::Open(&buffer[cursor.clone()])
            }
        }

        // let mut locked = stream
        //     .inner
        //     .lock()
        //     .expect("Mutex should not be poised; qed");

        // // let recv_future = Box::pin(locked.receiver.next());

        // let mut future = locked.receiver.next();

        // match future.poll_unpin(&mut Context::from_waker(
        //     futures_util::task::noop_waker_ref(),
        // )) {
        //     task::Poll::Ready(result) => {
        //         tracing::warn!("Got result {:?}", result);

        //         panic!("OPS with result {:?}", result);
        //     }
        //     task::Poll::Pending => {
        //         // panic!("OPS pending");

        //         smoldot_light::platform::ReadBuffer::Closed
        //         // tracing::warn!("Got pending...");
        //     }
        // }

        // match future::Future::poll(
        //     locked.receiver.next().fuse(),
        //     &mut Context::from_waker(futures_util::task::noop_waker_ref()),
        // ) {
        //     task::Poll::Ready(result) => {
        //         tracing::warn!("Got result {:?}", result);
        //     }
        //     task::Poll::Pending => {
        //         tracing::warn!("Got pending...");
        //     }
        // };

        // panic!("OPS - from reading");

        // // let msg = futures_executor::block_on(async {
        // let msg = futures::executor::block_on(async {
        //     match locked.receiver.next().await {
        //         Some(Ok(msg)) => Some(msg),
        //         _ => None,
        //     }
        // });

        // match msg {
        //     Some(msg) => {
        //         let msg = Box::leak(Box::new(msg));

        //         match msg {
        //             Message::Text(text) => {
        //                 smoldot_light::platform::ReadBuffer::Open(text.as_bytes())
        //             }
        //             Message::Bytes(bytes) => smoldot_light::platform::ReadBuffer::Open(bytes),
        //         }
        //     }
        //     None => smoldot_light::platform::ReadBuffer::Closed,
        // }
    }

    fn advance_read_cursor(&self, stream: &mut Self::Stream, bytes: usize) {
        tracing::trace!("[advance_read_cursor]");

        let Some(StreamReadBuffer::Open { ref mut cursor, .. }) =
            stream.buffers.as_mut().map(|(r, _)| r)
        else {
            assert_eq!(bytes, 0);
            return
        };

        assert!(cursor.start + bytes <= cursor.end);
        cursor.start += bytes;
    }

    fn writable_bytes(&self, stream: &mut Self::Stream) -> usize {
        tracing::trace!("[writable_bytes]");

        let Some(StreamWriteBuffer::Open { ref mut buffer, must_close: false, ..}) =
        stream.buffers.as_mut().map(|(_, w)| w) else { return 0 };
        buffer.capacity() - buffer.len()
    }

    fn send(&self, stream: &mut Self::Stream, data: &[u8]) {
        tracing::trace!("[send] data={:?}", data);

        let Some(StreamWriteBuffer::Open { ref mut buffer, .. } )=
        stream.buffers.as_mut().map(|(_, w)| w) else { panic!() };
        buffer.reserve(data.len());
        buffer.extend(data.iter().copied());

        // let mut locked = stream
        //     .inner
        //     .lock()
        //     .expect("Mutex should not be poised; qed");

        // if let Ok(message) = String::from_utf8(data.into()) {
        //     let _ = locked.sender.send(Message::Text(message));
        // }
    }

    fn close_send(&self, stream: &mut Self::Stream) {
        tracing::trace!("[close_send]");

        // It is not illegal to call this on an already-reset stream.
        let Some((_, write_buffer)) = stream.buffers.as_mut() else { return };

        match write_buffer {
            StreamWriteBuffer::Open {
                must_close: must_close @ false,
                ..
            } => *must_close = true,
            _ => {
                // However, it is illegal to call this on a stream that was already close
                // attempted.
                panic!()
            }
        }
    }
}

/// Connection stream of the light-client.
pub struct ConnectionStream {
    inner: Arc<Mutex<ConnectionInner>>,

    /// Read and write buffers of the connection, or `None` if the socket has been reset.
    buffers: Option<(StreamReadBuffer, StreamWriteBuffer)>,
}

/// Safe to implement `Send` in single threaded environments (WASM).
unsafe impl Send for ConnectionStream {}

/// Inner details of a `ConnectionStream` that represents the web socket.
struct ConnectionInner {
    sender: SplitSink<WebSocket, Message>,
    receiver: SplitStream<WebSocket>,
}

/// Safe to implement `Send` in single threaded environments (WASM).
unsafe impl Send for ConnectionInner {}

enum StreamReadBuffer {
    Open {
        buffer: Vec<u8>,
        cursor: ops::Range<usize>,
    },
    Closed,
}

enum StreamWriteBuffer {
    Open {
        buffer: VecDeque<u8>,
        must_flush: bool,
        must_close: bool,
    },
    Closed,
}
