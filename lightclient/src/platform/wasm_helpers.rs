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
pub struct Stream(
    #[pin]
    pub  smoldot::libp2p::with_buffers::WithBuffers<
        future::BoxFuture<'static, Result<WasmSocket, std::io::Error>>,
        WasmSocket,
        Instant,
    >,
);
