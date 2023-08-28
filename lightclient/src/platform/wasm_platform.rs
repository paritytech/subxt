// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use core::time::Duration;
use futures::{prelude::*, task::Poll};

use smoldot::libp2p::multiaddr::Multiaddr;
use smoldot_light::platform::{
    ConnectError, PlatformConnection, PlatformRef, PlatformSubstreamDirection, ReadBuffer,
};
use std::{io::IoSlice, pin::Pin};

use super::wasm_helpers::{StreamReadBuffer, StreamWriteBuffer};

/// Subxt plaform implementation for wasm.
///
/// This implementation is a conversion of the implementation from the smoldot:
/// https://github.com/smol-dot/smoldot/blob/f49ce4ea6a325c444ab6ad37d3ab5558edf0d541/light-base/src/platform/default.rs#L52.
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
    type Yield = future::Ready<()>;
    type Instant = super::wasm_helpers::Instant;
    type Connection = std::convert::Infallible;
    type Stream = super::wasm_helpers::Stream;
    type ConnectFuture = future::BoxFuture<
        'static,
        Result<PlatformConnection<Self::Stream, Self::Connection>, ConnectError>,
    >;
    type StreamUpdateFuture<'a> = future::BoxFuture<'a, ()>;
    type NextSubstreamFuture<'a> =
        future::Pending<Option<(Self::Stream, PlatformSubstreamDirection)>>;

    fn now_from_unix_epoch(&self) -> Duration {
        super::wasm_helpers::now_from_unix_epoch()
    }

    fn now(&self) -> Self::Instant {
        super::wasm_helpers::now()
    }

    fn sleep(&self, duration: Duration) -> Self::Delay {
        super::wasm_helpers::sleep(duration)
    }

    fn sleep_until(&self, when: Self::Instant) -> Self::Delay {
        self.sleep(when.saturating_duration_since(self.now()))
    }

    fn yield_after_cpu_intensive(&self) -> Self::Yield {
        // No-op.
        future::ready(())
    }

    fn connect(&self, multiaddr: &str) -> Self::ConnectFuture {
        // We simply copy the address to own it. We could be more zero-cost here, but doing so
        // would considerably complicate the implementation.
        let multiaddr = multiaddr.to_owned();

        tracing::debug!("Connecting to multiaddress={:?}", multiaddr);

        Box::pin(async move {
            let addr = multiaddr.parse::<Multiaddr>().map_err(|_| ConnectError {
                is_bad_addr: true,
                message: "Failed to parse address".to_string(),
            })?;

            let mut iter = addr.iter().fuse();
            let proto1 = iter.next().ok_or(ConnectError {
                is_bad_addr: true,
                message: "Unknown protocols combination".to_string(),
            })?;
            let proto2 = iter.next().ok_or(ConnectError {
                is_bad_addr: true,
                message: "Unknown protocols combination".to_string(),
            })?;
            let proto3 = iter.next();

            if iter.next().is_some() {
                return Err(ConnectError {
                    is_bad_addr: true,
                    message: "Unknown protocols combination".to_string(),
                });
            }

            super::wasm_helpers::connect(proto1, proto2, proto3).await
        })
    }

    fn open_out_substream(&self, c: &mut Self::Connection) {
        // This function can only be called with so-called "multi-stream" connections. We never
        // open such connection.
        match *c {}
    }

    fn next_substream<'a>(&self, c: &'a mut Self::Connection) -> Self::NextSubstreamFuture<'a> {
        // This function can only be called with so-called "multi-stream" connections. We never
        // open such connection.
        match *c {}
    }

    fn update_stream<'a>(&self, stream: &'a mut Self::Stream) -> Self::StreamUpdateFuture<'a> {
        Box::pin(future::poll_fn(|cx| {
            // The `connect` is expected to be called before this method and would populate
            // the buffers properly. When the buffers are empty, this future is shortly dropped.
            let Some((read_buffer, write_buffer)) = stream.buffers.as_mut() else {
                return Poll::Pending;
            };

            // Whether the future returned by `update_stream` should return `Ready` or `Pending`.
            let mut update_stream_future_ready = false;

            if let StreamReadBuffer::Open {
                buffer: ref mut buf,
                ref mut cursor,
            } = read_buffer
            {
                // When reading data from the socket, `poll_read` might return "EOF". In that
                // situation, we transition to the `Closed` state, which would discard the data
                // currently in the buffer. For this reason, we only try to read if there is no
                // data left in the buffer.
                if cursor.start == cursor.end {
                    if let Poll::Ready(result) = Pin::new(&mut stream.socket).poll_read(cx, buf) {
                        update_stream_future_ready = true;
                        match result {
                            Err(_) => {
                                // End the stream.
                                stream.buffers = None;
                                return Poll::Ready(());
                            }
                            Ok(0) => {
                                // EOF.
                                *read_buffer = StreamReadBuffer::Closed;
                            }
                            Ok(bytes) => {
                                *cursor = 0..bytes;
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
                    if let Poll::Ready(result) = Pin::new(&mut stream.socket).poll_write_vectored(
                        cx,
                        &[
                            IoSlice::new(write_queue_slices.0),
                            IoSlice::new(write_queue_slices.1),
                        ],
                    ) {
                        if !*must_close {
                            // In the situation where the API user wants to close the writing
                            // side, simply sending the buffered data isn't enough to justify
                            // making the future ready.
                            update_stream_future_ready = true;
                        }

                        match result {
                            Err(_) => {
                                // End the stream.
                                stream.buffers = None;
                                return Poll::Ready(());
                            }
                            Ok(bytes) => {
                                *must_flush = true;
                                for _ in 0..bytes {
                                    buf.pop_front();
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }

                if buf.is_empty() && *must_close {
                    if let Poll::Ready(result) = Pin::new(&mut stream.socket).poll_close(cx) {
                        update_stream_future_ready = true;
                        match result {
                            Err(_) => {
                                // End the stream.
                                stream.buffers = None;
                                return Poll::Ready(());
                            }
                            Ok(()) => {
                                *write_buffer = StreamWriteBuffer::Closed;
                            }
                        }
                    }
                } else if *must_flush {
                    if let Poll::Ready(result) = Pin::new(&mut stream.socket).poll_flush(cx) {
                        update_stream_future_ready = true;
                        match result {
                            Err(_) => {
                                // End the stream.
                                stream.buffers = None;
                                return Poll::Ready(());
                            }
                            Ok(()) => {
                                *must_flush = false;
                            }
                        }
                    }
                }
            }

            if update_stream_future_ready {
                Poll::Ready(())
            } else {
                // Progress cannot be made since poll_read, poll_write, poll_close, poll_flush
                // are not ready yet. Smoldot drops this future and calls it again with the
                // next processing iteration.
                Poll::Pending
            }
        }))
    }

    fn read_buffer<'a>(&self, stream: &'a mut Self::Stream) -> ReadBuffer<'a> {
        match stream.buffers.as_ref().map(|(r, _)| r) {
            None => ReadBuffer::Reset,
            Some(StreamReadBuffer::Closed) => ReadBuffer::Closed,
            Some(StreamReadBuffer::Open { buffer, cursor }) => {
                ReadBuffer::Open(&buffer[cursor.clone()])
            }
        }
    }

    fn advance_read_cursor(&self, stream: &mut Self::Stream, extra_bytes: usize) {
        let Some(StreamReadBuffer::Open { ref mut cursor, .. }) =
            stream.buffers.as_mut().map(|(r, _)| r)
        else {
            assert_eq!(extra_bytes, 0);
            return;
        };

        assert!(cursor.start + extra_bytes <= cursor.end);
        cursor.start += extra_bytes;
    }

    fn writable_bytes(&self, stream: &mut Self::Stream) -> usize {
        let Some(StreamWriteBuffer::Open {
            ref mut buffer,
            must_close: false,
            ..
        }) = stream.buffers.as_mut().map(|(_, w)| w)
        else {
            return 0;
        };
        buffer.capacity() - buffer.len()
    }

    fn send(&self, stream: &mut Self::Stream, data: &[u8]) {
        debug_assert!(!data.is_empty());

        // Because `writable_bytes` returns 0 if the writing side is closed, and because `data`
        // must always have a size inferior or equal to `writable_bytes`, we know for sure that
        // the writing side isn't closed.
        let Some(StreamWriteBuffer::Open { ref mut buffer, .. }) =
            stream.buffers.as_mut().map(|(_, w)| w)
        else {
            panic!()
        };
        buffer.reserve(data.len());
        buffer.extend(data.iter().copied());
    }

    fn close_send(&self, stream: &mut Self::Stream) {
        // It is not illegal to call this on an already-reset stream.
        let Some((_, write_buffer)) = stream.buffers.as_mut() else {
            return;
        };

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

    fn spawn_task(&self, _: std::borrow::Cow<str>, task: future::BoxFuture<'static, ()>) {
        super::wasm_helpers::spawn(task);
    }

    fn client_name(&self) -> std::borrow::Cow<str> {
        "subxt-light-client".into()
    }

    fn client_version(&self) -> std::borrow::Cow<str> {
        env!("CARGO_PKG_VERSION").into()
    }
}
