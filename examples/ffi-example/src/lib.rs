use hex::decode;
use once_cell::sync::OnceCell;
use std::{ffi::CStr, os::raw::c_char};
use subxt::{
    OnlineClient, PolkadotConfig, dynamic::Value, ext::scale_value::Composite,
    tx::dynamic as dynamic_call,
};
use subxt_signer::sr25519::dev;
use tokio::runtime::Runtime;

static TOKIO: OnceCell<Runtime> = OnceCell::new();
fn tokio_rt() -> &'static Runtime {
    TOKIO.get_or_init(|| Runtime::new().expect("failed to start tokio"))
}

/// A simple C‐ABI function to transfer `amount` to a hex‐encoded `dest`.
/// Assumes a running node’s WS endpoint is at ws://127.0.0.1:8000
#[unsafe(no_mangle)]
pub extern "C" fn do_transfer(dest_hex: *const c_char, amount: u64) -> i32 {
    let amount = amount as u128;
    // We need to convert C string to Rust str
    let raw_s = unsafe { CStr::from_ptr(dest_hex).to_str().unwrap_or_default() };

    // Strip optional 0x prefix
    let s = raw_s.strip_prefix("0x").unwrap_or(raw_s);

    // Decode hex, force a 32‐byte AccountId
    let raw = decode(s).expect("hex decode");
    let arr: [u8; 32] = raw.as_slice().try_into().expect("must be 32 bytes");

    // Wrap into a MultiAddress::Id variant for dynamic calls:
    let dst = Value::variant(
        "Id",
        Composite::unnamed(vec![
            // scale encode
            Value::from_bytes(arr.to_vec()),
        ]),
    );

    // Spin up (or reuse) our Tokio runtime and connect:
    let client = tokio_rt().block_on(async {
        OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:8000")
            .await
            .unwrap()
    });
    let signer = dev::alice();

    // Build the dynamic metadata extrinsic:
    let tx = dynamic_call(
        "Balances",
        "transfer_keep_alive",
        vec![
            dst.clone(),
            // primitive numeric value
            Value::u128(amount),
        ],
    );

    // Submit and wait for finalize
    let res: Result<_, subxt::Error> = tokio_rt().block_on(async {
        let progress = client
            .tx()
            .sign_and_submit_then_watch_default(&tx, &signer)
            .await?;
        progress.wait_for_finalized_success().await
    });

    // Return code
    match res {
        Ok(_) => 0,
        Err(e) => {
            // print the Subxt error
            eprintln!("do_transfer failed: {:#?}", e);
            -1
        }
    }
}
