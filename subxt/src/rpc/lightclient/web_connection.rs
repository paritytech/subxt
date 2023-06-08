// Copyright (C) 2022  Vince Vasta
// SPDX-License-Identifier: MIT
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Libp2p transports built on [Websys](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html).
#![warn(clippy::all, rust_2018_idioms)]

use futures::{future::Ready, io, prelude::*};
use send_wrapper::SendWrapper;
use smoldot::libp2p::multiaddr::{Multiaddr, ProtocolRef};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{MessageEvent, WebSocket};

use std::{
    collections::VecDeque,
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
    task::{Context, Waker},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("js function error {0}")]
    JsError(String),
    #[error("operation not supported")]
    NotSupported,
}

/// A Websocket connection created by the [`WebsocketTransport`].
pub struct Connection {
    /// We need to use Mutex as libp2p requires this to be Send.
    shared: Arc<Mutex<Shared>>,
}

struct Shared {
    opened: bool,
    closed: bool,
    error: bool,
    data: VecDeque<u8>,
    waker: Option<Waker>,
    socket: SendWrapper<WebSocket>,
    closures: Option<SendWrapper<Closures>>,
}

type Closures = (
    Closure<dyn FnMut()>,
    Closure<dyn FnMut(MessageEvent)>,
    Closure<dyn FnMut(web_sys::Event)>,
    Closure<dyn FnMut(web_sys::CloseEvent)>,
);

impl Connection {
    /// Returns (open, closed)
    pub fn state(&self) -> (bool, bool) {
        let shared = self.shared.lock().expect("Poised; qed");
        (shared.opened, shared.closed)
    }

    pub fn new(socket: WebSocket) -> Self {
        socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let shared = Arc::new(Mutex::new(Shared {
            opened: false,
            closed: false,
            error: false,
            data: VecDeque::with_capacity(1 << 16),
            waker: None,
            socket: SendWrapper::new(socket.clone()),
            closures: None,
        }));

        let open_callback = Closure::<dyn FnMut()>::new({
            let shared = shared.clone();
            move || {
                let mut locked = shared.lock().expect("Mutex poised; qed");
                locked.opened = true;
                if let Some(waker) = &locked.waker {
                    waker.wake_by_ref();
                }
            }
        });
        socket.set_onopen(Some(open_callback.as_ref().unchecked_ref()));

        let message_callback = Closure::<dyn FnMut(_)>::new({
            let shared = shared.clone();
            move |e: MessageEvent| {
                if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                    let mut locked = shared.lock().expect("Mutex poised; qed");
                    let bytes = js_sys::Uint8Array::new(&abuf).to_vec();
                    locked.data.extend(bytes.into_iter());
                    if let Some(waker) = &locked.waker {
                        waker.wake_by_ref();
                    }
                } else {
                    panic!("Unexpected data format {:?}", e.data());
                }
            }
        });
        socket.set_onmessage(Some(message_callback.as_ref().unchecked_ref()));

        let error_callback = Closure::<dyn FnMut(_)>::new({
            let shared = shared.clone();
            move |_| {
                // The error event for error callback doesn't give any information and
                // generates error on the browser console we just signal it to the
                // stream.
                shared.lock().expect("Mutex poised; qed").error = true;
            }
        });
        socket.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        let close_callback = Closure::<dyn FnMut(_)>::new({
            let shared = shared.clone();
            move |_| {
                let mut locked = shared.lock().expect("Mutex poised; qed");
                locked.closed = true;
                if let Some(waker) = &locked.waker {
                    waker.wake_by_ref();
                }
            }
        });
        socket.set_onclose(Some(close_callback.as_ref().unchecked_ref()));

        // Manage closures memory.
        let closures = SendWrapper::new((
            open_callback,
            message_callback,
            error_callback,
            close_callback,
        ));

        shared.lock().expect("Mutex poised; qed").closures = Some(closures);

        Self { shared }
    }
}

impl AsyncRead for Connection {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        let mut shared = self.shared.lock().expect("Mutex poised; qed");
        shared.waker = Some(cx.waker().clone());

        if shared.error {
            Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, "Socket error")))
        } else if shared.closed {
            Poll::Ready(Err(io::ErrorKind::BrokenPipe.into()))
        } else if shared.data.is_empty() {
            Poll::Pending
        } else {
            let n = shared.data.len().min(buf.len());
            for k in buf.iter_mut().take(n) {
                *k = shared.data.pop_front().unwrap();
            }
            Poll::Ready(Ok(n))
        }
    }
}

impl AsyncWrite for Connection {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let mut shared = self.shared.lock().expect("Mutex poised; qed");
        shared.waker = Some(cx.waker().clone());

        if shared.error {
            Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, "Socket error")))
        } else if shared.closed {
            Poll::Ready(Err(io::ErrorKind::BrokenPipe.into()))
        } else if !shared.opened {
            Poll::Pending
        } else {
            match shared.socket.send_with_u8_array(buf) {
                Ok(()) => Poll::Ready(Ok(buf.len())),
                Err(err) => Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Write error: {err:?}"),
                ))),
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Pending
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let shared = self.shared.lock().expect("Mutex poised; qed");
        if shared.opened {
            let _ = shared.socket.close();
        }
    }
}
