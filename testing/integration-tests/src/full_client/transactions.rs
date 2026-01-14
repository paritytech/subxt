// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::utils::node_runtime;
use crate::{subxt_test, test_context};
use frame_decode::extrinsics::ExtrinsicType;
use subxt_signer::sr25519::dev;

// TODO: When VerifySignature exists on the substrate kitchensink runtime,
// let's try actuallty submitting v4 and v5 signed extrinsics to verify that
// they are actually accepted by the node.

#[subxt_test]
async fn v4_unsigned_encode_decode() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let at_block = api.at_current_block().await?;
    let md = at_block.metadata_ref();

    let call = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::bob().public_key().into(), 1000);

    let tx_bytes = at_block
        .tx()
        .create_v4_unsigned(&call)
        .unwrap()
        .into_encoded();
    let tx_bytes_cursor = &mut &*tx_bytes;

    let decoded =
        frame_decode::extrinsics::decode_extrinsic(tx_bytes_cursor, md, md.types()).unwrap();

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
    let at_block = api.at_current_block().await?;
    let md = at_block.metadata_ref();

    let call = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::bob().public_key().into(), 1000);

    let tx_bytes = at_block
        .tx()
        .create_v5_unsigned(&call)
        .unwrap()
        .into_encoded();
    let tx_bytes_cursor = &mut &*tx_bytes;

    let decoded =
        frame_decode::extrinsics::decode_extrinsic(tx_bytes_cursor, md, md.types()).unwrap();

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
    let at_block = api.at_current_block().await?;
    let md = at_block.metadata_ref();

    let call = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::bob().public_key().into(), 1000);

    let tx_bytes = at_block
        .tx()
        .create_v4_signable(&call, &dev::alice().public_key().into(), Default::default())
        .await
        .unwrap()
        .sign(&dev::alice())
        .into_encoded();
    let tx_bytes_cursor = &mut &*tx_bytes;

    let decoded =
        frame_decode::extrinsics::decode_extrinsic(tx_bytes_cursor, md, md.types()).unwrap();

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
    let at_block = api.at_current_block().await?;
    let md = at_block.metadata_ref();

    let dummy_signer = dev::alice();

    let call = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::bob().public_key().into(), 1000);

    let tx_bytes = at_block
        .tx()
        .create_v5_signable(&call, &dev::alice().public_key().into(), Default::default())
        .await
        .unwrap()
        .sign(&dummy_signer) // No signature payload is added, but may be inserted into tx extensions.
        .into_encoded();
    let tx_bytes_cursor = &mut &*tx_bytes;

    let decoded =
        frame_decode::extrinsics::decode_extrinsic(tx_bytes_cursor, md, md.types()).unwrap();

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
