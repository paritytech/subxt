// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use futures::{io, prelude::*};
use send_wrapper::SendWrapper;
use wasm_bindgen::{prelude::*, JsCast};

use std::{
    collections::VecDeque,
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
    task::{Context, Waker},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to connect {0}")]
    ConnectionError(String),
}

/// Websocket for WASM environments.
///
/// This is a rust-based wrapper around browser's WebSocket API.
pub struct WasmSocket {
    inner: Arc<Mutex<InnerWasmSocket>>,
}

/// The state of the [`WasmSocket`].
#[derive(PartialEq, Eq, Clone, Copy)]
enum ConnectionState {
    /// Initial state of the socket.
    Connecting,
    /// Socket is fully opened.
    Opened,
    /// Socket is closed.
    Closed,
    /// Error reported by callbacks.
    Error,
}

struct InnerWasmSocket {
    /// The state of the connection.
    state: ConnectionState,
    /// This implements `Send` and panics if the value is accessed
    /// or dropped from another thread.
    ///
    /// This is safe in wasm environments.
    socket: SendWrapper<web_sys::WebSocket>,
    /// Data buffer for the socket.
    data: VecDeque<u8>,
    /// Waker from `poll_read` / `poll_write`.
    waker: Option<Waker>,
    /// In memory callbacks to handle messages from the browser socket.
    callbacks: Option<SendWrapper<Callbacks>>,
}

/// Registered callbacks of the [`WasmSocket`].
///
/// These need to be kept around until the socket is dropped.
type Callbacks = (
    Closure<dyn FnMut()>,
    Closure<dyn FnMut(web_sys::MessageEvent)>,
    Closure<dyn FnMut(web_sys::Event)>,
    Closure<dyn FnMut(web_sys::CloseEvent)>,
);

impl WasmSocket {
    /// Establish a WebSocket connection.
    ///
    /// The error is a string representing the browser error.
    /// Visit [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket#exceptions_thrown)
    /// for more info.
    pub fn new(addr: &str) -> Result<Self, Error> {
        let socket = match web_sys::WebSocket::new(addr) {
            Ok(socket) => socket,
            Err(err) => return Err(Error::ConnectionError(format!("{:?}", err))),
        };

        socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let inner = Arc::new(Mutex::new(InnerWasmSocket {
            state: ConnectionState::Connecting,
            socket: SendWrapper::new(socket.clone()),
            data: VecDeque::with_capacity(16384),
            waker: None,
            callbacks: None,
        }));

        let open_callback = Closure::<dyn FnMut()>::new({
            let inner = inner.clone();
            move || {
                let mut inner = inner.lock().expect("Mutex is poised; qed");
                inner.state = ConnectionState::Opened;

                if let Some(waker) = inner.waker.take() {
                    waker.wake();
                }
            }
        });
        socket.set_onopen(Some(open_callback.as_ref().unchecked_ref()));

        let message_callback = Closure::<dyn FnMut(_)>::new({
            let inner = inner.clone();
            move |event: web_sys::MessageEvent| {
                let Ok(buffer) = event.data().dyn_into::<js_sys::ArrayBuffer>() else {
                    panic!("Unexpected data format {:?}", event.data());
                };

                let mut inner = inner.lock().expect("Mutex is poised; qed");
                let bytes = js_sys::Uint8Array::new(&buffer).to_vec();
                inner.data.extend(bytes.into_iter());

                if let Some(waker) = inner.waker.take() {
                    waker.wake();
                }
            }
        });
        socket.set_onmessage(Some(message_callback.as_ref().unchecked_ref()));

        let error_callback = Closure::<dyn FnMut(_)>::new({
            let inner = inner.clone();
            move |_| {
                // Callback does not provide useful information, signal it back to the stream.
                let mut inner = inner.lock().expect("Mutex is poised; qed");
                inner.state = ConnectionState::Error;

                if let Some(waker) = inner.waker.take() {
                    waker.wake();
                }
            }
        });
        socket.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        let close_callback = Closure::<dyn FnMut(_)>::new({
            let inner = inner.clone();
            move |_| {
                let mut inner = inner.lock().expect("Mutex is poised; qed");
                inner.state = ConnectionState::Closed;

                if let Some(waker) = inner.waker.take() {
                    waker.wake();
                }
            }
        });
        socket.set_onclose(Some(close_callback.as_ref().unchecked_ref()));

        let callbacks = SendWrapper::new((
            open_callback,
            message_callback,
            error_callback,
            close_callback,
        ));
        inner.lock().expect("Mutex poised; qed").callbacks = Some(callbacks);

        Ok(Self { inner })
    }
}

impl AsyncRead for WasmSocket {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        let mut inner = self.inner.lock().expect("Mutex is poised; qed");
        inner.waker = Some(cx.waker().clone());

        match inner.state {
            ConnectionState::Error => {
                Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, "Socket error")))
            }
            ConnectionState::Closed => Poll::Ready(Err(io::ErrorKind::BrokenPipe.into())),
            ConnectionState::Connecting => Poll::Pending,
            ConnectionState::Opened => {
                if inner.data.is_empty() {
                    return Poll::Pending;
                }

                let n = inner.data.len().min(buf.len());
                for k in buf.iter_mut().take(n) {
                    *k = inner.data.pop_front().expect("Buffer non empty; qed");
                }
                Poll::Ready(Ok(n))
            }
        }
    }
}

impl AsyncWrite for WasmSocket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let mut inner = self.inner.lock().expect("Mutex is poised; qed");
        inner.waker = Some(cx.waker().clone());

        match inner.state {
            ConnectionState::Error => {
                Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, "Socket error")))
            }
            ConnectionState::Closed => Poll::Ready(Err(io::ErrorKind::BrokenPipe.into())),
            ConnectionState::Connecting => Poll::Pending,
            ConnectionState::Opened => match inner.socket.send_with_u8_array(buf) {
                Ok(()) => Poll::Ready(Ok(buf.len())),
                Err(err) => Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Write error: {err:?}"),
                ))),
            },
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Drop for WasmSocket {
    fn drop(&mut self) {
        let inner = self.inner.lock().expect("Mutex is poised; qed");

        if inner.state == ConnectionState::Opened {
            let _ = inner.socket.close();
        }
    }
}
