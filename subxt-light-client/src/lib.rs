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
///
use wasm_bindgen::prelude::*;

use std::vec::Vec;
use std::{collections::BinaryHeap, sync::Mutex};

pub struct Buffers {
    indices: Vec<(u32, u32)>,
    buffers: Vec<Vec<u8>>,
}

impl Buffers {
    /// Translate the provided light-client buffer ID to the internal
    /// mapping.
    ///
    /// In case the provided buffer ID is not registered, allocate a new buffer
    /// for it.
    fn translate_id(&mut self, id: u32) -> u32 {
        let search = self.indices.iter().find(|(current, _)| current == &id);

        match search {
            Some((_, result)) => *result,
            None => {
                // Note: limit the space to u32, it should not overflow in WASM builds.
                let insert_id = self.buffers.len() as u32;
                self.indices.push((id, insert_id));
                self.buffers.push(Vec::with_capacity(1024));
                insert_id
            }
        }
    }
}

lazy_static::lazy_static! {
    static ref BUFFERS: Mutex<Buffers> = Mutex::new(Buffers {
        indices: vec![],
        buffers: vec![],
    });
}

#[wasm_bindgen]
pub fn greet() {
    println!("Hello world");
}

#[no_mangle]
pub extern "C" fn panic(message_ptr: u32, message_len: u32) {
    tracing::trace!(
        "panic message_ptr={:?} message_len={:?}",
        message_ptr,
        message_len
    );

    let message = unsafe {
        let ptr = std::slice::from_raw_parts(message_ptr as *const u8, message_len as usize);
        std::str::from_utf8_unchecked(ptr)
    };

    panic!("{message}");
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

pub extern "C" fn buffer_copy(buffer_index: u32, target_pointer: u32) {
    tracing::trace!(
        "buffer_copy index={:?} target={:?}",
        buffer_index,
        target_pointer
    );

    let mut buffers = BUFFERS
        .try_lock()
        .expect("Cannot get buffers in single-threaded environments; qed");

    let source_id = buffers.translate_id(buffer_index);
    let source_len = buffers.buffers[source_id as usize].len();

    let dst_ptr = target_pointer as *mut u8;
    let src_ptr = buffers.buffers[source_id as usize].as_ptr() as *const u8;

    unsafe {
        std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, source_len);
    }
}

/// Returns the size (in bytes) of the buffer with the given index.
///
/// See the documentation of [`buffer_copy`] for context.
#[no_mangle]

pub extern "C" fn buffer_size(buffer_index: u32) -> u32 {
    tracing::trace!("buffer_size index={:?}", buffer_index);

    let mut buffers = BUFFERS
        .try_lock()
        .expect("Cannot get buffers in single-threaded environments; qed");

    let source_id = buffers.translate_id(buffer_index);
    buffers.buffers[source_id as usize].len() as u32
}

/// The queue of JSON-RPC responses of the given chain is no longer empty.
///
/// This function is only ever called after [`json_rpc_responses_peek`] has returned a `len`
/// of 0.
///
/// This function might be called spuriously, however this behavior must not be relied upon.
#[no_mangle]
pub extern "C" fn json_rpc_responses_non_empty(chain_id: u32) {
    tracing::trace!("json_rpc_responses_non_empty chain_id={:?}", chain_id);
    // TODO: must notify something...
}

/// Client is emitting a log entry.
///
/// Each log entry is made of a log level (`1 = Error, 2 = Warn, 3 = Info, 4 = Debug,
/// 5 = Trace`), a log target (e.g. "network"), and a log message.
///
/// The log target and message is a UTF-8 string found in the memory of the WebAssembly
/// virtual machine at offset `ptr` and with length `len`.
#[no_mangle]

pub extern "C" fn log_binding(
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

    tracing::warn!(
        "log level={:?} target={:?} message={:?}",
        level,
        target,
        message
    );
}

/// Called when [`advance_execution`] should be executed again.
///
/// This function might be called from within [`advance_execution`], in which case
/// [`advance_execution`] should be called again immediately after it returns.
#[no_mangle]

pub extern "C" fn advance_execution_ready() {
    // Note: No-op function.
    tracing::trace!("advance_execution_ready");
}

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

pub extern "C" fn start_timer(milliseconds: f64) {
    tracing::trace!("start_timer milliseconds={:?}", milliseconds);

    let timeout = gloo_timers::callback::Timeout::new(milliseconds as u32, move || {
        tracing::trace!("Timout expired");

        unsafe {
            timer_finished();
        }
    });

    timeout.forget();
}

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
    tracing::trace!(
        "connection_new id={:?} addr_ptr={:?} addr_len={:?} error_buffer_index_ptr={:?}",
        id,
        addr_ptr,
        addr_len,
        error_buffer_index_ptr
    );
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

pub extern "C" fn reset_connection(id: u32) {
    tracing::trace!("reset_connection id={:?}", id,);
}

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

pub extern "C" fn connection_stream_open(connection_id: u32) {
    tracing::trace!("connection_stream_open id={:?}", connection_id);
}

/// Abruptly closes an existing substream of a multi-stream connection. The substream must
/// currently be in the `Open` state.
///
/// Must never be called if [`stream_reset`] has been called on that object in the past.
///
/// This function will only be called for multi-stream connections. The connection must
/// currently be in the `Open` state. See the documentation of [`connection_new`] for details.
#[no_mangle]

pub extern "C" fn connection_stream_reset(connection_id: u32, stream_id: u32) {
    tracing::trace!(
        "connection_stream_reset id={:?} stream_id={:?}",
        connection_id,
        stream_id
    );
}

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

pub extern "C" fn stream_send(connection_id: u32, stream_id: u32, ptr: u32, len: u32) {
    tracing::trace!(
        "stream_send id={:?} stream_id={:?} ptr={:?} len={:?}",
        connection_id,
        stream_id,
        ptr,
        len,
    );
}

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

pub extern "C" fn stream_send_close(connection_id: u32, stream_id: u32) {
    tracing::trace!(
        "stream_send_close id={:?} stream_id={:?}",
        connection_id,
        stream_id,
    );
}

/// Called when the Wasm execution enters the context of a certain task. This is useful for
/// debugging purposes.
///
/// Only one task can be currently executing at any time.
///
/// The name of the task is a UTF-8 string found in the memory of the WebAssembly virtual
/// machine at offset `ptr` and with length `len`.
#[no_mangle]

pub extern "C" fn current_task_entered(ptr: u32, len: u32) {
    tracing::trace!("current_task_entered ptr={:?} len={:?}", ptr, len);
}

/// Called when the Wasm execution leave the context of a certain task. This is useful for
/// debugging purposes.
///
/// Only one task can be currently executing at any time.
#[no_mangle]

pub extern "C" fn current_task_exit() {
    tracing::trace!("current_task_exit");
}

/// See [`json_rpc_responses_peek`].
#[repr(C)]
pub struct JsonRpcResponseInfo {
    /// Pointer in memory where the JSON-RPC response can be found.
    pub ptr: u32,
    /// Length of the JSON-RPC response in bytes. If 0, indicates that the queue is empty.
    pub len: u32,
}

extern "C" {
    /// Initializes the client.
    ///
    /// This is the first function that must be called. Failure to do so before calling another
    /// method will lead to a Rust panic. Calling this function multiple times will also lead to a
    /// panic.
    ///
    /// The client will emit log messages by calling the [`log()`] function, provided the log level is
    /// inferior or equal to the value of `max_log_level` passed here.
    #[no_mangle]
    pub fn init(max_log_level: u32);

    /// Advances the execution of the client, performing CPU-heavy tasks.
    ///
    /// This function **must** be called regularly, otherwise nothing will happen.
    ///
    /// After this function is called or during a call to this function, [`advance_execution_ready`]
    /// might be called, indicating that [`advance_execution`] should be called again.
    #[no_mangle]
    pub fn advance_execution();

    /// Adds a chain to the client. The client will try to stay connected and synchronize this chain.
    ///
    /// Assign a so-called "buffer index" (a `u32`) representing the chain specification, database
    /// content, and list of potential relay chains, then provide these buffer indices to the function.
    /// The Rust code will call [`buffer_size`] and [`buffer_copy`] in order to obtain the content of
    /// these buffers. The buffer indices can be de-assigned and buffers destroyed once this function
    /// returns.
    ///
    /// The content of the chain specification and database content must be in UTF-8.
    ///
    /// > **Note**: The database content is an opaque string that can be obtained by calling
    /// >           the `chainHead_unstable_finalizedDatabase` JSON-RPC function.
    ///
    /// The list of potential relay chains is a buffer containing a list of 32-bits-little-endian chain
    /// ids. If the chain specification refer to a parachain, these chain ids are the ones that will be
    /// looked up to find the corresponding relay chain.
    ///
    /// If `json_rpc_running` is 0, then no JSON-RPC service will be started and it is forbidden to
    /// send JSON-RPC requests targeting this chain. This can be used to save up resources.
    ///
    /// If an error happens during the creation of the chain, a chain id will be allocated
    /// nonetheless, and must later be de-allocated by calling [`remove_chain`]. This allocated chain,
    /// however, will be in an erroneous state. Use [`chain_is_ok`] to determine whether this function
    /// was successful. If not, use [`chain_error_len`] and [`chain_error_ptr`] to obtain the error
    /// message.
    #[no_mangle]
    pub fn add_chain(
        chain_spec_buffer_index: u32,
        database_content_buffer_index: u32,
        json_rpc_running: u32,
        potential_relay_chains_buffer_index: u32,
    ) -> u32;
    /// Removes a chain previously added using [`add_chain`]. Instantly unsubscribes all the JSON-RPC
    /// subscriptions and cancels all in-progress requests corresponding to that chain.
    ///
    /// If the removed chain was an erroneous chain, calling this function will invalidate the pointer
    /// returned by [`chain_error_ptr`].
    #[no_mangle]
    pub fn remove_chain(chain_id: u32);
    /// Returns `1` if creating this chain was successful. Otherwise, returns `0`.
    ///
    /// If `0` is returned, use [`chain_error_len`] and [`chain_error_ptr`] to obtain an error
    /// message.
    #[no_mangle]
    pub fn chain_is_ok(chain_id: u32) -> u32;

    /// Returns the length of the error message stored for this chain.
    ///
    /// Must only be called on an erroneous chain. Use [`chain_is_ok`] to determine whether a chain is
    /// in an erroneous state. Returns `0` if the chain isn't erroneous.
    #[no_mangle]
    pub fn chain_error_len(chain_id: u32) -> u32;

    /// Returns a pointer to the error message stored for this chain. The error message is a UTF-8
    /// string starting at the memory offset returned by this function, and whose length can be
    /// determined by calling [`chain_error_len`].
    ///
    /// Must only be called on an erroneous chain. Use [`chain_is_ok`] to determine whether a chain is
    /// in an erroneous state. Returns `0` if the chain isn't erroneous.
    #[no_mangle]
    pub fn chain_error_ptr(chain_id: u32) -> u32;
    /// Emit a JSON-RPC request or notification towards the given chain previously added using
    /// [`add_chain`].
    ///
    /// A buffer containing a UTF-8 JSON-RPC request or notification must be passed as parameter. The
    /// format of the JSON-RPC requests and notifications is described in
    /// [the standard JSON-RPC 2.0 specification](https://www.jsonrpc.org/specification).
    ///
    /// Assign a so-called "buffer index" (a `u32`) representing the buffer containing the UTF-8
    /// request, then provide this buffer index to the function. The Rust code will call
    /// [`buffer_size`] and [`buffer_copy`] in order to obtain the content of this buffer. The buffer
    /// index can be de-assigned and buffer destroyed once this function returns.
    ///
    /// Responses and notifications are notified using [`json_rpc_responses_non_empty`], and can
    /// be read with [`json_rpc_responses_peek`].
    ///
    /// It is forbidden to call this function on an erroneous chain or a chain that was created with
    /// `json_rpc_running` equal to 0.
    ///
    /// This function returns:
    /// - 0 on success.
    /// - 1 if the request couldn't be parsed as a valid JSON-RPC request.
    /// - 2 if the chain is currently overloaded with JSON-RPC requests and refuses to queue another
    /// one.
    ///
    #[no_mangle]
    pub fn json_rpc_send(text_buffer_index: u32, chain_id: u32) -> u32;

    /// Obtains information about the first response in the queue of JSON-RPC responses.
    ///
    /// This function returns a pointer within the memory of the WebAssembly virtual machine where is
    /// stored a struct of type [`JsonRpcResponseInfo`]. This pointer remains valid until
    /// [`json_rpc_responses_pop`] or [`remove_chain`] is called with the same `chain_id`.
    ///
    /// The response or notification is a UTF-8 string found in the memory of the WebAssembly
    /// virtual machine at offset `ptr` and with length `len`, where `ptr` and `len` are found in the
    /// [`JsonRpcResponseInfo`].
    ///
    /// If `len` is equal to 0, this indicates that the queue of JSON-RPC responses is empty.
    /// When a `len` of 0 is returned, [`json_rpc_responses_non_empty`] will later be called to
    /// indicate that it is no longer empty.
    ///
    /// After having read the response or notification, use [`json_rpc_responses_pop`] to remove it
    /// from the queue. You can then call [`json_rpc_responses_peek`] again to read the next response.
    #[no_mangle]
    pub fn json_rpc_responses_peek(chain_id: u32) -> u32;

    /// Removes the first response from the queue of JSON-RPC responses. This is the response whose
    /// information can be retrieved using [`json_rpc_responses_peek`].
    ///
    /// Calling this function invalidates the pointer previously returned by a call to
    /// [`json_rpc_responses_peek`] with the same `chain_id`.
    ///
    /// It is forbidden to call this function on an erroneous chain or a chain that was created with
    /// `json_rpc_running` equal to 0.
    #[no_mangle]
    pub fn json_rpc_responses_pop(chain_id: u32);

    /// Must be called in response to [`start_timer`] after the given duration has passed.
    #[no_mangle]
    pub fn timer_finished();

    /// Called by the JavaScript code if the connection switches to the `Open` state. The connection
    /// must be in the `Opening` state.
    ///
    /// Must be called at most once per connection object.
    ///
    /// See also [`connection_new`].
    ///
    /// When in the `Open` state, the connection can receive messages. Use [`stream_message`] in order
    /// to provide to the Rust code the messages received by the connection.
    ///
    /// The `handshake_ty` parameter indicates the type of handshake. It must always be 0 at the
    /// moment, indicating a multistream-select+Noise+Yamux handshake.
    ///
    /// `write_closable` must be non-zero if and only if it makes sense to call [`stream_send_close`]
    /// on this connection. If zero, then [`stream_send_close`] will never be called.
    #[no_mangle]
    pub fn connection_open_single_stream(
        connection_id: u32,
        handshake_ty: u32,
        initial_writable_bytes: u32,
        write_closable: u32,
    );

    /// Called by the JavaScript code if the connection switches to the `Open` state. The connection
    /// must be in the `Opening` state.
    ///
    /// Must be called at most once per connection object.
    ///
    /// See also [`connection_new`].
    ///
    /// Assign a so-called "buffer index" (a `u32`) representing the buffer containing the handshake
    /// type, then provide this buffer index to the function. The Rust code will call [`buffer_size`]
    /// and [`buffer_copy`] in order to obtain the content of this buffer. The buffer index can be
    /// de-assigned and buffer destroyed once this function returns.
    ///
    /// The buffer must contain a single 0 byte (indicating WebRTC), followed with the multihash
    /// representation of the hash of the local node's TLS certificate, followed with the multihash
    /// representation of the hash of the remote node's TLS certificate.
    #[no_mangle]
    pub fn connection_open_multi_stream(connection_id: u32, handshake_ty_buffer_index: u32);

    /// Notify of a message being received on the stream. The connection associated with that stream
    /// (and, in the case of a multi-stream connection, the stream itself) must be in the `Open` state.
    ///
    /// Assign a so-called "buffer index" (a `u32`) representing the buffer containing the message,
    /// then provide this buffer index to the function. The Rust code will call [`buffer_size`] and
    /// [`buffer_copy`] in order to obtain the content of this buffer. The buffer index can be
    /// de-assigned and buffer destroyed once this function returns.
    ///
    /// If `connection_id` is a single-stream connection, then the value of `stream_id` is ignored.
    /// If `connection_id` is a multi-stream connection, then `stream_id` corresponds to the stream
    /// on which the data was received, as was provided to [`connection_stream_opened`].
    ///
    /// See also [`connection_open_single_stream`] and [`connection_open_multi_stream`].
    #[no_mangle]
    pub fn stream_message(connection_id: u32, stream_id: u32, buffer_index: u32);

    /// Notify that extra bytes can be written onto the stream. The connection associated with that
    /// stream (and, in the case of a multi-stream connection, the stream itself) must be in the
    /// `Open` state.
    ///
    /// `total_sent - total_reported_writable_bytes` must always be `>= 0`, where `total_sent` is the
    /// total number of bytes sent on the stream using [`stream_send`] and
    /// `total_reported_writable_bytes` is the total number of bytes reported using
    /// [`stream_writable_bytes`].
    /// In other words, this function is meant to notify that data sent using [`stream_send`̀] has
    /// effectively been sent out. It is not possible to exceed the `initial_writable_bytes` provided
    /// when the stream was created.
    ///
    /// If `connection_id` is a single-stream connection, then the value of `stream_id` is ignored.
    /// If `connection_id` is a multi-stream connection, then `stream_id` corresponds to the stream
    /// on which the data was received, as was provided to [`connection_stream_opened`].
    #[no_mangle]
    pub fn stream_writable_bytes(connection_id: u32, stream_id: u32, num_bytes: u32);

    /// Called by the JavaScript code when the given multi-stream connection has a new substream.
    ///
    /// `connection_id` *must* be a multi-stream connection.
    ///
    /// The value of `stream_id` is chosen at the discretion of the caller. It is illegal to use the
    /// same `stream_id` as an existing stream on that same connection that is still open.
    ///
    /// For the `outbound` parameter, pass `0` if the substream has been opened by the remote, and any
    /// value other than `0` if the substream has been opened in response to a call to
    /// [`connection_stream_open`].
    #[no_mangle]
    pub fn connection_stream_opened(
        connection_id: u32,
        stream_id: u32,
        outbound: u32,
        initial_writable_bytes: u32,
    );

    /// Can be called at any point by the JavaScript code if the connection switches to the `Reset`
    /// state.
    ///
    /// Must only be called once per connection object.
    /// Must never be called if [`reset_connection`] has been called on that object in the past.
    ///
    /// Assign a so-called "buffer index" (a `u32`) representing the buffer containing the UTF-8
    /// reason for closing, then provide this buffer index to the function. The Rust code will call
    /// [`buffer_size`] and [`buffer_copy`] in order to obtain the content of this buffer. The buffer
    /// index can be de-assigned and buffer destroyed once this function returns.
    ///
    /// See also [`connection_new`].
    #[no_mangle]
    pub fn connection_reset(connection_id: u32, buffer_index: u32);

    /// Can be called at any point by the JavaScript code if the stream switches to the `Reset`
    /// state.
    ///
    /// Must only be called once per stream.
    /// Must never be called if [`connection_stream_reset`] has been called on that object in the past.
    ///
    /// The `stream_id` becomes dead and can be re-used for another stream on the same connection.
    ///
    /// It is illegal to call this function on a single-stream connections.
    ///
    /// See also [`connection_open_multi_stream`].
    #[no_mangle]
    pub fn stream_reset(connection_id: u32, stream_id: u32);

}
