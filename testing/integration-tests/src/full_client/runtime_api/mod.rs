// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{node_runtime, submit_tx_wait_for_finalized_success, subxt_test, test_context};
use codec::Encode;
use subxt::utils::AccountId32;
use subxt_signer::sr25519::dev;

#[subxt_test]
async fn account_nonce() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let alice_account_id: AccountId32 = alice.public_key().into();

    // Check Alice nonce is starting from 0.
    let runtime_api_call = node_runtime::apis()
        .account_nonce_api()
        .account_nonce(alice_account_id.clone());
    let nonce = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await?;
    assert_eq!(nonce, 0);

    // Do some transaction to bump the Alice nonce to 1:
    let remark_tx = node_runtime::tx().system().remark(vec![1, 2, 3, 4, 5]);
    let signed_extrinsic = api
        .tx()
        .create_signed(&remark_tx, &alice, Default::default())
        .await?;
    submit_tx_wait_for_finalized_success(&signed_extrinsic).await?;

    let runtime_api_call = node_runtime::apis()
        .account_nonce_api()
        .account_nonce(alice_account_id);
    let nonce = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await?;
    assert_eq!(nonce, 1);

    Ok(())
}

#[subxt_test]
async fn unchecked_extrinsic_encoding() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob();
    let bob_address = bob.public_key().to_address();

    // Construct a tx from Alice to Bob.
    let tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(bob_address, 10_000);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await
        .unwrap();

    let tx_bytes = signed_extrinsic.into_encoded();
    let len = tx_bytes.len() as u32;

    // Manually encode the runtime API call arguments to make a raw call.
    let mut encoded = tx_bytes.clone();
    encoded.extend(len.encode());

    let expected_result: node_runtime::runtime_types::pallet_transaction_payment::types::FeeDetails<
        ::core::primitive::u128,
    > = api
        .runtime_api()
        .at_latest()
        .await?
        .call_raw(
            "TransactionPaymentApi_query_fee_details",
            Some(encoded.as_ref()),
        )
        .await?;

    // Use the generated API to confirm the result with the raw call.
    let runtime_api_call = node_runtime::apis()
        .transaction_payment_api()
        .query_fee_details(tx_bytes.into(), len);

    let result = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await?;

    assert_eq!(expected_result, result);

    Ok(())
}
