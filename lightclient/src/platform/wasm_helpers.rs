// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Wasm implementation for the light client's platform using
//! custom websockets.

use super::wasm_socket::WasmSocket;

use core::time::Duration;
use futures_util::{future, FutureExt};

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

/// Implementation detail of a stream from the `SubxtPlatform`.
#[pin_project::pin_project]
pub struct Stream(#[pin] pub with_buffers::WithBuffers<WasmSocket>);

pub mod with_buffers {
    use smoldot::libp2p::read_write;

    use core::{
        fmt, future, mem, ops,
        pin::{self, Pin},
        task::Poll,
    };

    use crate::platform::wasm_helpers::Instant;
    use futures_util::{AsyncRead, AsyncWrite};
    use std::io;
    /// Holds an implementation of `AsyncRead` and `AsyncWrite`, alongside with a read buffer and a
    /// write buffer.
    #[pin_project::pin_project]
    pub struct WithBuffers<T> {
        /// Actual socket to read from/write to.
        #[pin]
        socket: T,
        /// Error that has happened on the socket, if any.
        error: Option<io::Error>,
        /// Storage for data read from the socket. The first [`WithBuffers::read_buffer_valid`] bytes
        /// contain actual socket data, while the rest contains garbage data.
        /// The capacity of this buffer is at least equal to the amount of bytes requested by the
        /// inner data consumer.
        read_buffer: Vec<u8>,
        /// Number of bytes of data in [`WithBuffers::read_buffer`] that contain actual data.
        read_buffer_valid: usize,
        read_buffer_reasonable_capacity: usize,
        /// True if reading from the socket has returned `Ok(0)` earlier, in other words "end of
        /// file".
        read_closed: bool,
        /// Storage for data to write to the socket.
        write_buffers: Vec<Vec<u8>>,
        /// True if the consumer has closed the writing side earlier.
        write_closed: bool,
        /// True if the consumer has closed the writing side earlier, and the socket still has to
        /// be closed.
        close_pending: bool,
        /// True if data has been written on the socket and the socket needs to be flushed.
        flush_pending: bool,

        /// Value of [`read_write::ReadWrite::now`] that was fed by the latest call to
        /// [`WithBuffers::read_write_access`].
        read_write_now: Option<Instant>,
        /// Value of [`read_write::ReadWrite::wake_up_after`] produced by the latest call
        /// to [`WithBuffers::read_write_access`].
        read_write_wake_up_after: Option<Instant>,
    }

    impl<T> WithBuffers<T> {
        /// Initializes a new [`WithBuffers`] with the given socket.
        ///
        /// The socket must still be open in both directions.
        pub fn new(socket: T) -> Self {
            let read_buffer_reasonable_capacity = 65536; // TODO: make configurable?

            WithBuffers {
                socket,
                error: None,
                read_buffer: Vec::with_capacity(read_buffer_reasonable_capacity),
                read_buffer_valid: 0,
                read_buffer_reasonable_capacity,
                read_closed: false,
                write_buffers: Vec::with_capacity(64),
                write_closed: false,
                close_pending: false,
                flush_pending: false,
                read_write_now: None,
                read_write_wake_up_after: None,
            }
        }

        /// Returns an object that implements `Deref<Target = ReadWrite>`. This object can be used
        /// to push or pull data to/from the socket.
        ///
        /// > **Note**: The parameter requires `Self` to be pinned for consistency with
        /// >           [`WithBuffers::wait_read_write_again`].
        pub fn read_write_access(
            self: Pin<&mut Self>,
            now: Instant,
        ) -> Result<ReadWriteAccess, &io::Error> {
            let this = self.project();

            debug_assert!(this
                .read_write_now
                .as_ref()
                .map_or(true, |old_now| *old_now <= now));
            *this.read_write_wake_up_after = None;
            *this.read_write_now = Some(now);

            if let Some(error) = this.error.as_ref() {
                return Err(error);
            }

            this.read_buffer.truncate(*this.read_buffer_valid);

            let write_bytes_queued = this.write_buffers.iter().map(Vec::len).sum();

            Ok(ReadWriteAccess {
                read_buffer_len_before: this.read_buffer.len(),
                write_buffers_len_before: this.write_buffers.len(),
                read_write: read_write::ReadWrite {
                    now,
                    incoming_buffer: mem::take(this.read_buffer),
                    expected_incoming_bytes: if !*this.read_closed { Some(0) } else { None },
                    read_bytes: 0,
                    write_bytes_queued,
                    write_buffers: mem::take(this.write_buffers),
                    write_bytes_queueable: if !*this.write_closed {
                        // Limit outgoing buffer size to 128kiB.
                        // TODO: make configurable?
                        Some((128 * 1024usize).saturating_sub(write_bytes_queued))
                    } else {
                        None
                    },
                    wake_up_after: this.read_write_wake_up_after.take(),
                },
                read_buffer: this.read_buffer,
                read_buffer_valid: this.read_buffer_valid,
                read_buffer_reasonable_capacity: *this.read_buffer_reasonable_capacity,
                write_buffers: this.write_buffers,
                write_closed: this.write_closed,
                close_pending: this.close_pending,
                read_write_wake_up_after: this.read_write_wake_up_after,
            })
        }
    }

    impl<T> WithBuffers<T>
    where
        T: AsyncRead + AsyncWrite,
    {
        /// Waits until [`WithBuffers::read_write_access`] should be called again.
        ///
        /// Returns if an error happens on the socket. If an error happened in the past on the socket,
        /// the future never yields.
        pub async fn wait_read_write_again<F>(
            self: Pin<&mut Self>,
            timer_builder: impl FnOnce(Instant) -> F,
        ) where
            F: future::Future<Output = ()>,
        {
            let mut this = self.project();

            // Return immediately if `wake_up_after <= now`.
            match (&*this.read_write_wake_up_after, &*this.read_write_now) {
                (Some(when_wake_up), Some(now)) if *when_wake_up <= *now => {
                    return;
                }
                _ => {}
            }

            let mut timer = pin::pin!({
                let fut = this
                    .read_write_wake_up_after
                    .as_ref()
                    .map(|when| timer_builder(*when));
                async {
                    if let Some(fut) = fut {
                        fut.await;
                    } else {
                        future::pending::<()>().await;
                    }
                }
            });

            // Grow the read buffer in order to make space for potentially more data.
            this.read_buffer.resize(this.read_buffer.capacity(), 0);

            future::poll_fn(move |cx| {
                if this.error.is_some() {
                    // Never return.
                    return Poll::Pending;
                }

                // If still `true` at the end of the function, `Poll::Pending` is returned.
                let mut pending = true;

                match future::Future::poll(Pin::new(&mut timer), cx) {
                    Poll::Pending => {}
                    Poll::Ready(()) => {
                        pending = false;
                    }
                }

                if !*this.read_closed {
                    let read_result = AsyncRead::poll_read(
                        this.socket.as_mut(),
                        cx,
                        &mut this.read_buffer[*this.read_buffer_valid..],
                    );

                    match read_result {
                        Poll::Pending => {}
                        Poll::Ready(Ok(0)) => {
                            *this.read_closed = true;
                            pending = false;
                        }
                        Poll::Ready(Ok(n)) => {
                            *this.read_buffer_valid += n;
                            // TODO: consider waking up only if the expected bytes of the consumer are exceeded
                            pending = false;
                        }
                        Poll::Ready(Err(err)) => {
                            *this.error = Some(err);
                            return Poll::Ready(());
                        }
                    };
                }

                loop {
                    if this.write_buffers.iter().any(|b| !b.is_empty()) {
                        let write_result = {
                            let buffers = this
                                .write_buffers
                                .iter()
                                .map(|buf| io::IoSlice::new(buf))
                                .collect::<Vec<_>>();
                            AsyncWrite::poll_write_vectored(this.socket.as_mut(), cx, &buffers)
                        };

                        match write_result {
                            Poll::Ready(Ok(0)) => {
                                // It is not legal for `poll_write` to return 0 bytes written.
                                unreachable!();
                            }
                            Poll::Ready(Ok(mut n)) => {
                                *this.flush_pending = true;
                                while n > 0 {
                                    let first_buf = this.write_buffers.first_mut().unwrap();
                                    if first_buf.len() <= n {
                                        n -= first_buf.len();
                                        this.write_buffers.remove(0);
                                    } else {
                                        // TODO: consider keeping the buffer as is but starting the next write at a later offset
                                        first_buf.copy_within(n.., 0);
                                        first_buf.truncate(first_buf.len() - n);
                                        break;
                                    }
                                }
                                // Wake up if the write buffers switch from non-empty to empty.
                                if this.write_buffers.is_empty() {
                                    pending = false;
                                }
                            }
                            Poll::Ready(Err(err)) => {
                                *this.error = Some(err);
                                return Poll::Ready(());
                            }
                            Poll::Pending => break,
                        };
                    } else if *this.flush_pending {
                        match AsyncWrite::poll_flush(this.socket.as_mut(), cx) {
                            Poll::Ready(Ok(())) => {
                                *this.flush_pending = false;
                            }
                            Poll::Ready(Err(err)) => {
                                *this.error = Some(err);
                                return Poll::Ready(());
                            }
                            Poll::Pending => break,
                        }
                    } else if *this.close_pending {
                        match AsyncWrite::poll_close(this.socket.as_mut(), cx) {
                            Poll::Ready(Ok(())) => {
                                *this.close_pending = false;
                                pending = false;
                                break;
                            }
                            Poll::Ready(Err(err)) => {
                                *this.error = Some(err);
                                return Poll::Ready(());
                            }
                            Poll::Pending => break,
                        }
                    } else {
                        break;
                    }
                }

                if !pending {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })
            .await;
        }
    }

    impl<T: fmt::Debug> fmt::Debug for WithBuffers<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("WithBuffers").field(&self.socket).finish()
        }
    }

    /// See [`WithBuffers::read_write_access`].
    pub struct ReadWriteAccess<'a> {
        read_write: read_write::ReadWrite<Instant>,

        read_buffer_len_before: usize,
        write_buffers_len_before: usize,

        // Fields below as references from the content of the `WithBuffers`.
        read_buffer: &'a mut Vec<u8>,
        read_buffer_valid: &'a mut usize,
        read_buffer_reasonable_capacity: usize,
        write_buffers: &'a mut Vec<Vec<u8>>,
        write_closed: &'a mut bool,
        close_pending: &'a mut bool,
        read_write_wake_up_after: &'a mut Option<Instant>,
    }

    impl<'a> ops::Deref for ReadWriteAccess<'a> {
        type Target = read_write::ReadWrite<Instant>;

        fn deref(&self) -> &Self::Target {
            &self.read_write
        }
    }

    impl<'a> ops::DerefMut for ReadWriteAccess<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.read_write
        }
    }

    impl<'a> Drop for ReadWriteAccess<'a> {
        fn drop(&mut self) {
            *self.read_buffer = mem::take(&mut self.read_write.incoming_buffer);
            *self.read_buffer_valid = self.read_buffer.len();

            // Adjust `read_buffer` to the number of bytes requested by the consumer.
            if let Some(expected_incoming_bytes) = self.read_write.expected_incoming_bytes {
                if expected_incoming_bytes < self.read_buffer_reasonable_capacity
                    && self.read_buffer.is_empty()
                {
                    // We use `shrink_to(0)` then `reserve(cap)` rather than just `shrink_to(cap)`
                    // so that the `Vec` doesn't try to preserve the data in the read buffer.
                    self.read_buffer.shrink_to(0);
                    self.read_buffer
                        .reserve(self.read_buffer_reasonable_capacity);
                } else if expected_incoming_bytes > self.read_buffer.len() {
                    self.read_buffer
                        .reserve(expected_incoming_bytes - self.read_buffer.len());
                }
                debug_assert!(self.read_buffer.capacity() >= expected_incoming_bytes);
            }

            *self.write_buffers = mem::take(&mut self.read_write.write_buffers);

            if self.read_write.write_bytes_queueable.is_none() && !*self.write_closed {
                *self.write_closed = true;
                *self.close_pending = true;
            }

            *self.read_write_wake_up_after = self.read_write.wake_up_after.take();

            // If the consumer has advanced its reading or writing sides, we make the next call to
            // `read_write_access` return immediately by setting `wake_up_after`.
            if (self.read_buffer_len_before != self.read_buffer.len()
                && self
                    .read_write
                    .expected_incoming_bytes
                    .map_or(false, |b| b <= self.read_buffer.len()))
                || (self.write_buffers_len_before != self.write_buffers.len()
                    && !*self.write_closed)
            {
                *self.read_write_wake_up_after = Some(self.read_write.now);
            }
        }
    }
}
