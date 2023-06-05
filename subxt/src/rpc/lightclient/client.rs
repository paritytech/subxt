// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.
use super::{
    background::{BackendMessage, BackgroundTask},
    LightClientError,
};
use crate::{
    error::{Error, RpcError},
    rpc::{RpcClientT, RpcFuture, RpcSubscription},
};
use core::time::Duration;
use futures::{lock::Mutex as AsyncMutex, stream::StreamExt, Stream};

#[cfg(feature = "jsonrpsee-ws")]
use jsonrpsee::{
    async_client::ClientBuilder,
    client_transport::ws::{Uri, WsTransportClientBuilder},
    core::client::ClientT,
    rpc_params,
};
use serde_json::value::RawValue;
// use smoldot_light::{platform::async_std::AsyncStdTcpWebSocket, ChainId};
use smoldot_light::ChainId;
use std::{
    iter,
    num::NonZeroU32,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::ReceiverStream;

// use smoldot_light_wasm::platform::Platform as WasmPlatform;

use super::platform::Platform as MyPlatform;

const LOG_TARGET: &str = "light-client";

/// Inner structure to work with light clients.
struct LightClientInner {
    /// Smoldot light client implementation that leverages the `AsyncStdTcpWebSocket`.
    ///
    /// Note: `AsyncStdTcpWebSocket` is not wasm compatible.
    client: smoldot_light::Client<MyPlatform>,
    /// The ID of the chain used to identify the chain protocol (ie. substrate).
    ///
    /// Note: A single chain is supported for a client. This aligns with the subxt's
    /// vision of the Client.
    chain_id: ChainId,
    /// Communicate with the backend task.
    to_backend: mpsc::Sender<BackendMessage>,
    /// Atomic used to generate unique IDs.
    id_provider: AtomicU64,
}

impl LightClientInner {
    /// Generate the next unique ID used to populate the Json RPC request.
    ///
    /// This is unique to identify the sender of the request.
    fn next_id(&mut self) -> String {
        let id = self.id_provider.fetch_add(1, Ordering::AcqRel);
        id.to_string()
    }

    /// Register a RPC method request.
    ///
    /// Returns a channel that produces only one item, which is the result of the method.
    ///
    /// The result is a raw jsonrpc string similar to:
    ///
    /// ```bash
    /// {"jsonrpc":"2.0","id":"1","result":"my result object"}
    /// ```
    ///
    /// # Note
    ///
    /// Registering the request must happen before submitting the request in order
    /// for the background task to provide a response.
    async fn register_request(
        &self,
        id: String,
    ) -> Result<oneshot::Receiver<Box<RawValue>>, LightClientError> {
        let (sender, receiver) = oneshot::channel();

        self.to_backend
            .send(BackendMessage::Request { id, sender })
            .await
            .map_err(|_| LightClientError::BackgroundClosed)?;

        Ok(receiver)
    }

    /// Register a RPC subscription request.
    ///
    /// Returns a channel that produces the items of the subscription.
    ///
    /// The JsonRPC subscription is generated as follows:
    /// - Make a plain RPC method request which returns the subscription ID, in the result field:
    ///
    /// ```bash
    /// {"jsonrpc":"2.0","id":"1","result":"0"}
    /// ```
    ///
    /// - Register with the provided ID to the notifications of the subscription. Notifications look like:
    ///
    /// ```bash
    /// {"jsonrpc":"2.0","method":"author_extrinsicUpdate","params":{"subscription":"0","result":"Dropped"}}
    /// ```
    ///
    /// # Note
    ///
    /// The notification messages are buffered internally to ensure that users will receive all
    /// messages in the following case:
    ///
    /// * T0. [`Self::register_request()`].
    /// * T1. submit a plain RPC method request.
    /// * T2. the subscription produces a notification. (T2 happens before the user calls this method)
    /// * T3. user parses the subscription ID from (T1) and calls [`Self::register_subscription`].
    async fn register_subscription(
        &self,
        id: String,
    ) -> Result<mpsc::Receiver<Box<RawValue>>, LightClientError> {
        let (sender, receiver) = mpsc::channel(128);

        self.to_backend
            .send(BackendMessage::Subscription { id, sender })
            .await
            .map_err(|_| LightClientError::BackgroundClosed)?;

        Ok(receiver)
    }
}

/// Must stop the execution immediately. The message is a UTF-8 string found in the memory of
/// the WebAssembly at offset `message_ptr` and with length `message_len`.
///
/// > **Note**: This function is typically implemented using `throw`.
///
/// After this function has been called, no further Wasm functions must be called again on
/// this Wasm virtual machine. Explanation below.
///
/// # About throwing and safety
///
/// Rust programs can be configured in two panicking modes: `abort`, or `unwind`. Safe or
/// unsafe Rust code must be written by keeping in mind that the execution of a function can
/// be suddenly interrupted by a panic, but can rely on the fact that this panic will either
/// completely abort the program, or unwind the stack. In the latter case, they can rely on
/// the fact that `std::panic::catch_unwind` will catch this unwinding and let them perform
/// some additional clean-ups.
///
/// This function is typically implemented using `throw`. However, "just" throwing a JavaScript
/// exception from within the implementation of this function is neither `abort`, because the
/// JavaScript could call into the Wasm again later, nor `unwind`, because it isn't caught by
/// `std::panic::catch_unwind`. By being neither of the two, it breaks the assumptions that
/// some Rust codes might rely on for either correctness or safety.
/// In order to solve this problem, we enforce that `panic` must behave like `abort`, and
/// forbid calling into the Wasm virtual machine again.
///
/// Beyond the `panic` function itself, any other FFI function that throws must similarly
/// behave like `abort` and prevent any further execution.
#[no_mangle]
pub extern "C" fn panic(message_ptr: u32, message_len: u32) {
    let slice =
        unsafe { std::slice::from_raw_parts(message_ptr as *const u8, message_len as usize) };
    if let Ok(message) = std::str::from_utf8(slice) {
        panic!("{message}");
    }
}

/// Copies the entire content of the buffer with the given index to the memory of the
/// WebAssembly at offset `target_pointer`.
///
/// In situations where a buffer must be provided from the JavaScript to the Rust code, the
/// JavaScript must (prior to calling the Rust function that requires the buffer) assign a
/// "buffer index" to the buffer it wants to provide. The Rust code then calls the
/// [`buffer_size`] and [`buffer_copy`] functions in order to obtain the length and content
/// of the buffer.
#[no_mangle]
pub extern "C" fn buffer_copy(buffer_index: u32, target_pointer: u32) {}

/// Returns the size (in bytes) of the buffer with the given index.
///
/// See the documentation of [`buffer_copy`] for context.
#[no_mangle]

pub extern "C" fn buffer_size(buffer_index: u32) -> u32 {
    0
}

/// The queue of JSON-RPC responses of the given chain is no longer empty.
///
/// This function is only ever called after [`json_rpc_responses_peek`] has returned a `len`
/// of 0.
///
/// This function might be called spuriously, however this behavior must not be relied upon.
#[no_mangle]
pub extern "C" fn json_rpc_responses_non_empty(chain_id: u32) {}

/// Client is emitting a log entry.
///
/// Each log entry is made of a log level (`1 = Error, 2 = Warn, 3 = Info, 4 = Debug,
/// 5 = Trace`), a log target (e.g. "network"), and a log message.
///
/// The log target and message is a UTF-8 string found in the memory of the WebAssembly
/// virtual machine at offset `ptr` and with length `len`.
#[no_mangle]
pub extern "C" fn log(
    level: u32,
    target_ptr: u32,
    target_len: u32,
    message_ptr: u32,
    message_len: u32,
) {
    let target_slice =
        unsafe { std::slice::from_raw_parts(target_ptr as *const u8, target_len as usize) };
    let target = std::str::from_utf8(target_slice).unwrap_or_else(|_| "cannot decode target");

    let msg_slice =
        unsafe { std::slice::from_raw_parts(message_ptr as *const u8, message_len as usize) };
    let message = std::str::from_utf8(msg_slice).unwrap_or_else(|_| "cannot decode message");

    println!("log level={level} target={target} message={message}");
}

/// Called when [`advance_execution`] should be executed again.
///
/// This function might be called from within [`advance_execution`], in which case
/// [`advance_execution`] should be called again immediately after it returns.
#[no_mangle]
pub extern "C" fn advance_execution_ready() {}

/// After at least `milliseconds` milliseconds have passed, [`timer_finished`] must be called.
///
/// It is not a logic error to call [`timer_finished`] *before* `milliseconds` milliseconds
/// have passed, and this will likely cause smoldot to restart a new timer for the remainder
/// of the duration.
///
/// When [`timer_finished`] is called, the value of the monotonic clock (in the WASI bindings)
/// must have increased by at least the given number of `milliseconds`.
///
/// If `milliseconds` is 0, [`timer_finished`] should be called as soon as possible.
///
/// `milliseconds` never contains a negative number, `NaN` or infinite.
#[no_mangle]
pub extern "C" fn start_timer(milliseconds: f64) {}

/// Must initialize a new connection that tries to connect to the given multiaddress.
///
/// The multiaddress is a UTF-8 string found in the WebAssembly memory at offset `addr_ptr`
/// and with `addr_len` bytes. The string is a multiaddress such as `/ip4/1.2.3.4/tcp/5/ws`.
///
/// The `id` parameter is an identifier for this connection, as chosen by the Rust code. It
/// must be passed on every interaction with this connection.
///
/// Returns 0 to indicate success, or 1 to indicate that an error happened. If an error is
/// returned, the `id` doesn't correspond to anything.
///
/// > **Note**: If you implement this function using for example `new WebSocket()`, please
/// >           keep in mind that exceptions should be caught and turned into an error code.
///
/// If an error happened, assign a so-called "buffer index" (a `u32`) representing the buffer
/// containing the UTF-8 error message, then write this buffer index as little-endian to the
/// memory of the WebAssembly indicated by `error_buffer_index_ptr`. The Rust code will call
/// [`buffer_size`] and [`buffer_copy`] in order to obtain the content of this buffer. The
/// buffer index should remain assigned and buffer alive until the next time the JavaScript
/// code retains control. Then, write at location `error_buffer_index_ptr + 4` a `1` if the
/// error is caused by the address being forbidden or unsupported, and `0` otherwise. If no
/// error happens, nothing should be written to `error_buffer_index_ptr`.
///
/// At any time, a connection can be in one of the three following states:
///
/// - `Opening` (initial state)
/// - `Open`
/// - `Reset`
///
/// When in the `Opening` or `Open` state, the connection can transition to the `Reset` state
/// if the remote closes the connection or refuses the connection altogether. When that
/// happens, [`connection_reset`] must be called. Once in the `Reset` state, the connection
/// cannot transition back to another state.
///
/// Initially in the `Opening` state, the connection can transition to the `Open` state if the
/// remote accepts the connection. When that happens, [`connection_open_single_stream`] or
/// [`connection_open_multi_stream`] must be called.
///
/// There exists two kind of connections: single-stream and multi-stream. Single-stream
/// connections are assumed to have a single stream open at all time and the encryption and
/// multiplexing are handled internally by smoldot. Multi-stream connections open and close
/// streams over time using [`connection_stream_opened`] and [`stream_reset`], and the
/// encryption and multiplexing are handled by the user of these bindings.
#[no_mangle]
pub extern "C" fn connection_new(
    id: u32,
    addr_ptr: u32,
    addr_len: u32,
    error_buffer_index_ptr: u32,
) -> u32 {
    0
}

/// Abruptly close a connection previously initialized with [`connection_new`].
///
/// This destroys the identifier passed as parameter. This identifier must never be passed
/// through the FFI boundary, unless the same identifier is later allocated again with
/// [`connection_new`].
///
/// Must never be called if [`connection_reset`] has been called on that object in the past.
///
/// The connection must be closed in the background. The Rust code isn't interested in incoming
/// messages from this connection anymore.
///
/// > **Note**: In JavaScript, remember to unregister event handlers before calling for
/// >           example `WebSocket.close()`.
#[no_mangle]
pub extern "C" fn reset_connection(id: u32) {}

/// Queues a new outbound substream opening. The [`connection_stream_opened`] function must
/// later be called when the substream has been successfully opened.
///
/// This function will only be called for multi-stream connections. The connection must
/// currently be in the `Open` state. See the documentation of [`connection_new`] for details.
///
/// > **Note**: No mechanism exists in this API to handle the situation where a substream fails
/// >           to open, as this is not supposed to happen. If you need to handle such a
/// >           situation, either try again opening a substream again or reset the entire
/// >           connection.
#[no_mangle]
pub extern "C" fn connection_stream_open(connection_id: u32) {}

/// Abruptly closes an existing substream of a multi-stream connection. The substream must
/// currently be in the `Open` state.
///
/// Must never be called if [`stream_reset`] has been called on that object in the past.
///
/// This function will only be called for multi-stream connections. The connection must
/// currently be in the `Open` state. See the documentation of [`connection_new`] for details.
#[no_mangle]
pub extern "C" fn connection_stream_reset(connection_id: u32, stream_id: u32) {}

/// Queues data on the given stream. The data is found in the memory of the WebAssembly
/// virtual machine, at the given pointer.
///
/// If `connection_id` is a single-stream connection, then the value of `stream_id` should
/// be ignored. If `connection_id` is a multi-stream connection, then the value of `stream_id`
/// contains the identifier of the stream on which to send the data, as was provided to
/// [`connection_stream_opened`].
///
/// The connection associated with that stream (and, in the case of a multi-stream connection,
/// the stream itself must currently be in the `Open` state. See the documentation of
/// [`connection_new`] for details.
///
/// The size of the buffer must not exceed the number of writable bytes of the given stream.
/// Use [`stream_writable_bytes`] to notify that more data can be sent on the stream.
#[no_mangle]
pub extern "C" fn stream_send(connection_id: u32, stream_id: u32, ptr: u32, len: u32) {}

/// Close the sending side of the given stream of the given connection.
///
/// Never called for connection types where this isn't possible to implement (i.e. WebSocket
/// and WebRTC at the moment).
///
/// If `connection_id` is a single-stream connection, then the value of `stream_id` should
/// be ignored. If `connection_id` is a multi-stream connection, then the value of `stream_id`
/// contains the identifier of the stream whose sending side should be closed, as was provided
/// to [`connection_stream_opened`].
///
/// The connection associated with that stream (and, in the case of a multi-stream connection,
/// the stream itself must currently be in the `Open` state. See the documentation of
/// [`connection_new`] for details.
#[no_mangle]
pub extern "C" fn stream_send_close(connection_id: u32, stream_id: u32) {}

/// Called when the Wasm execution enters the context of a certain task. This is useful for
/// debugging purposes.
///
/// Only one task can be currently executing at any time.
///
/// The name of the task is a UTF-8 string found in the memory of the WebAssembly virtual
/// machine at offset `ptr` and with length `len`.
#[no_mangle]
pub extern "C" fn current_task_entered(ptr: u32, len: u32) {}

/// Called when the Wasm execution leave the context of a certain task. This is useful for
/// debugging purposes.
///
/// Only one task can be currently executing at any time.
#[no_mangle]
pub extern "C" fn current_task_exit() {}

/// The LightClient RPC offers a slightly different RPC methods than the
/// substrate based chains. This is because the light client only exposes
/// a small subset of the RPCs needed for basic functionality.
pub struct LightClient {
    // Note: Used for interior mutability as subxt's RpcClientT trait
    // passes the RPC client as immutable reference and the smoldot_light crate
    // needed a mutable reference to the smoldot_light::Client.
    inner: Arc<AsyncMutex<LightClientInner>>,
}

impl LightClient {
    /// Construct a new [`LightClient`], providing a URL to connect to.
    ///
    /// The URL is utilized to fetch the chain specification.
    #[cfg(feature = "jsonrpsee-ws")]
    pub async fn from_url(url: impl AsRef<str>) -> Result<LightClient, Error> {
        let url = url
            .as_ref()
            .parse::<Uri>()
            .map_err(|_| Error::LightClient(LightClientError::InvalidUrl))?;

        if url.scheme_str() != Some("ws") && url.scheme_str() != Some("wss") {
            return Err(Error::LightClient(LightClientError::InvalidScheme));
        }

        let (sender, receiver) = WsTransportClientBuilder::default()
            .build(url)
            .await
            .map_err(|_| LightClientError::Handshake)?;

        let client = ClientBuilder::default()
            .request_timeout(Duration::from_secs(180))
            .max_notifs_per_subscription(4096)
            .build_with_tokio(sender, receiver);

        let result: serde_json::Value = client
            .request("sync_state_genSyncSpec", rpc_params![true])
            .await
            .map_err(|err| Error::Rpc(RpcError::ClientError(Box::new(err))))?;

        LightClient::new(&result.to_string())
    }

    /// Constructs a new [`LightClient`], providing the chain specification.
    ///
    /// The chain specification can be downloaded from a trusted network via
    /// the `sync_state_genSyncSpec` RPC method. This parameter expects the
    /// chain spec in text format (ie not in hex-encoded scale-encoded as RPC methods
    /// will provide).
    pub fn new(chain_spec: &str) -> Result<LightClient, Error> {
        tracing::trace!(target: LOG_TARGET, "Create light client");

        // let platform = AsyncStdTcpWebSocket::new(
        //     env!("CARGO_PKG_NAME").into(),
        //     env!("CARGO_PKG_VERSION").into(),
        // );

        let platform = MyPlatform::new();

        let mut client = smoldot_light::Client::new(platform);

        let smoldot_light::AddChainSuccess {
            chain_id,
            json_rpc_responses,
        } = client
            .add_chain(smoldot_light::AddChainConfig {
                // The most important field of the configuration is the chain specification. This is a
                // JSON document containing all the information necessary for the client to connect to said
                // chain.
                specification: chain_spec,

                // Configures some constants about the JSON-RPC endpoints.
                // It is also possible to pass `Disabled`, in which case the chain will not be able to
                // handle JSON-RPC requests. This can be used to save up some resources.
                json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                    // Maximum number of JSON-RPC in the queue of requests waiting to be processed.
                    // This parameter is necessary for situations where the JSON-RPC clients aren't
                    // trusted. If you control all the requests that are sent out and don't want them
                    // to fail, feel free to pass `u32::max_value()`.
                    max_pending_requests: NonZeroU32::new(128)
                        .expect("Valid number is greater than zero; qed"),
                    // Maximum number of active subscriptions before new ones are automatically
                    // rejected. Any JSON-RPC request that causes the server to generate notifications
                    // counts as a subscription.
                    // While a typical reasonable value would be for example 64, existing UIs tend to
                    // start a lot of subscriptions, and a value such as 1024 is recommended.
                    // Similarly, if you don't want any limit, feel free to pass `u32::max_value()`.
                    max_subscriptions: 1024,
                },
                // This field is necessary only if adding a parachain.
                potential_relay_chains: iter::empty(),
                // After a chain has been added, it is possible to extract a "database" (in the form of a
                // simple string). This database can later be passed back the next time the same chain is
                // added again.
                // A database with an invalid format is simply ignored by the client.
                // In this example, we don't use this feature, and as such we simply pass an empty string,
                // which is intentionally an invalid database content.
                database_content: "",
                // The client gives the possibility to insert an opaque "user data" alongside each chain.
                // This avoids having to create a separate `HashMap<ChainId, ...>` in parallel of the
                // client.
                user_data: (),
            })
            .map_err(|err| LightClientError::AddChainError(err.to_string()))?;

        let (to_backend, backend) = mpsc::channel(128);

        // `json_rpc_responses` can only be `None` if we had passed `json_rpc: Disabled`.
        let rpc_responses = json_rpc_responses.expect("Light client RPC configured; qed");

        wasm_bindgen_futures::spawn_local(async move {
            // tokio::spawn(async move {
            let mut task = BackgroundTask::new();
            task.start_task(backend, rpc_responses).await;
        });

        Ok(LightClient {
            inner: Arc::new(AsyncMutex::new(LightClientInner {
                client,
                chain_id,
                to_backend,
                id_provider: AtomicU64::new(1),
            })),
        })
    }
}

impl RpcClientT for LightClient {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RpcFuture<'a, Box<RawValue>> {
        let inner = self.inner.clone();

        Box::pin(async move {
            let mut data = inner.lock().await;

            let params = match params {
                Some(params) => serde_json::to_string(&params).map_err(|_| {
                    RpcError::ClientError(Box::new(LightClientError::InvalidParams))
                })?,
                None => "[]".into(),
            };

            // Obtain an unique ID.
            let id = data.next_id();
            // Register the ID for responses.
            let rx = data
                .register_request(id.clone())
                .await
                .map_err(|err| RpcError::ClientError(Box::new(err)))?;

            // Submit the RPC request with the provided ID.
            // Note: The ID is necessary otherwise smoldot reaches an 'unreachable!()' macro.
            let request = format!(
                r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":{}}}"#,
                id, method, params
            );
            tracing::trace!(target: LOG_TARGET, "Submit request {:?}", request);
            let chain_id = data.chain_id;

            data.client
                .json_rpc_request(request, chain_id)
                .map_err(|err| {
                    RpcError::ClientError(Box::new(LightClientError::Request(err.to_string())))
                })?;

            let response = rx
                .await
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?;

            tracing::trace!(target: LOG_TARGET, "RPC response {:?}", response);

            Ok(response)
        })
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        _unsub: &'a str,
    ) -> RpcFuture<'a, RpcSubscription> {
        let inner = self.inner.clone();

        Box::pin(async move {
            let mut data = inner.lock().await;

            tracing::trace!(
                target: LOG_TARGET,
                "Subscribe to {:?} with params {:?}",
                sub,
                params
            );

            let params = match params {
                Some(params) => serde_json::to_string(&params).map_err(|_| {
                    RpcError::ClientError(Box::new(LightClientError::InvalidParams))
                })?,
                None => "[]".into(),
            };

            // For subscriptions we need to make a plain RPC request to the subscription method.
            // The server will return as a result the subscription ID.
            // Then, the subscription ID is registered in the backend and will receive notifications from the chain.
            let id = data.next_id();
            let rx = data
                .register_request(id.clone())
                .await
                .map_err(|err| RpcError::ClientError(Box::new(err)))?;
            let request = format!(
                r#"{{"jsonrpc":"2.0","id":"{}", "method":"{}","params":{}}}"#,
                id, sub, params
            );

            let chain_id = data.chain_id;
            data.client
                .json_rpc_request(request, chain_id)
                .map_err(|err| {
                    RpcError::ClientError(Box::new(LightClientError::Request(err.to_string())))
                })?;

            // The subscription ID.
            let sub_id = rx
                .await
                .map_err(|_| RpcError::ClientError(Box::new(LightClientError::BackgroundClosed)))?;

            let sub_id_str = sub_id.get();
            // Try removing the first and last chars that are extra quotes.
            let sub_id_str = if sub_id_str.len() > 2 {
                &sub_id_str[1..sub_id_str.len() - 1]
            } else {
                sub_id_str
            };
            let sub_id = sub_id_str.to_string();
            tracing::trace!(target: LOG_TARGET, "Subscription ID {:?}", sub_id);

            let rx = data
                .register_subscription(sub_id.clone())
                .await
                .map_err(|err| RpcError::ClientError(Box::new(err)))?;
            let stream = ReceiverStream::new(rx);

            let rpc_substription_stream: Pin<
                Box<dyn Stream<Item = Result<Box<RawValue>, RpcError>> + Send + 'static>,
            > = Box::pin(stream.map(Ok));

            let rpc_subscription: RpcSubscription = RpcSubscription {
                stream: rpc_substription_stream,
                id: Some(sub_id),
            };

            Ok(rpc_subscription)
        })
    }
}
