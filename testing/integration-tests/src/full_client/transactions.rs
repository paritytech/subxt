use crate::utils::node_runtime;
use crate::{subxt_test, test_context};
use core::ops::Deref;
use frame_decode::extrinsics::ExtrinsicType;
use subxt_signer::sr25519::dev;

#[subxt_test]
async fn v4_unsigned_encode_decode() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let md = api.metadata();

    let call = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::bob().public_key().into(), 1000);

    let tx_bytes = api.tx().create_v4_unsigned(&call).unwrap().into_encoded();
    let tx_bytes_cursor = &mut &*tx_bytes;

    let decoded = frame_decode::extrinsics::decode_extrinsic(
        tx_bytes_cursor,
        md.deref(),
        api.metadata().types(),
    )
    .unwrap();

    assert_eq!(tx_bytes_cursor.len(), 0);
    assert_eq!(decoded.version(), 4);
    assert_eq!(decoded.ty(), ExtrinsicType::Bare);
    assert_eq!(decoded.pallet_name(), "Balances");
    assert_eq!(decoded.call_name(), "transfer_allow_death");
    assert!(decoded.signature_payload().is_none());

    Ok(())
}

#[subxt_test]
async fn v5_bare_encode_decode() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let md = api.metadata();

    let call = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::bob().public_key().into(), 1000);

    let tx_bytes = api.tx().create_v5_bare(&call).unwrap().into_encoded();
    let tx_bytes_cursor = &mut &*tx_bytes;

    let decoded = frame_decode::extrinsics::decode_extrinsic(
        tx_bytes_cursor,
        md.deref(),
        api.metadata().types(),
    )
    .unwrap();

    assert_eq!(tx_bytes_cursor.len(), 0);
    assert_eq!(decoded.version(), 5);
    assert_eq!(decoded.ty(), ExtrinsicType::Bare);
    assert_eq!(decoded.pallet_name(), "Balances");
    assert_eq!(decoded.call_name(), "transfer_allow_death");
    assert!(decoded.transaction_extension_payload().is_none());
    assert!(decoded.signature_payload().is_none());

    Ok(())
}

#[subxt_test]
async fn v4_signed_encode_decode() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let md = api.metadata();

    let call = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::bob().public_key().into(), 1000);

    let tx_bytes = api
        .tx()
        .create_partial(&call, &dev::alice().public_key().into(), Default::default())
        .await
        .unwrap()
        .to_v4_signed(&dev::alice())
        .into_encoded();
    let tx_bytes_cursor = &mut &*tx_bytes;

    let decoded = frame_decode::extrinsics::decode_extrinsic(
        tx_bytes_cursor,
        md.deref(),
        api.metadata().types(),
    )
    .unwrap();

    assert_eq!(tx_bytes_cursor.len(), 0);
    assert_eq!(decoded.version(), 4);
    assert_eq!(decoded.ty(), ExtrinsicType::Signed);
    assert_eq!(decoded.pallet_name(), "Balances");
    assert_eq!(decoded.call_name(), "transfer_allow_death");
    assert!(decoded.signature_payload().is_some());

    Ok(())
}

#[subxt_test]
async fn v5_general_encode_decode() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let md = api.metadata();

    let call = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::bob().public_key().into(), 1000);

    let tx_bytes = api
        .tx()
        .create_partial(&call, &dev::alice().public_key().into(), Default::default())
        .await
        .unwrap()
        .to_v5_general() // No signature added in the transaction extensions
        .into_encoded();
    let tx_bytes_cursor = &mut &*tx_bytes;

    let decoded = frame_decode::extrinsics::decode_extrinsic(
        tx_bytes_cursor,
        md.deref(),
        api.metadata().types(),
    )
    .unwrap();

    assert_eq!(tx_bytes_cursor.len(), 0);
    assert_eq!(decoded.version(), 5);
    assert_eq!(decoded.ty(), ExtrinsicType::General);
    assert_eq!(decoded.pallet_name(), "Balances");
    assert_eq!(decoded.call_name(), "transfer_allow_death");
    assert!(decoded.transaction_extension_payload().is_some());
    // v5 general extrinsics have no signature payload; signature in tx extensions:
    assert!(decoded.signature_payload().is_none());

    Ok(())
}
