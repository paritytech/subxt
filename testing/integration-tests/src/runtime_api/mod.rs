// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{node_runtime, test_context};
use subxt::utils::AccountId32;
use subxt_signer::sr25519::dev;

#[tokio::test]
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
    api.tx()
        .sign_and_submit_then_watch_default(&remark_tx, &alice)
        .await?
        .wait_for_finalized_success()
        .await?;

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
