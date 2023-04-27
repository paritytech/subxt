// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{node_runtime, pair_signer, test_context};
use sp_keyring::AccountKeyring;
use subxt::utils::AccountId32;

#[tokio::test]
async fn account_nonce() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let signer = pair_signer(AccountKeyring::Alice.pair());
    let alice: AccountId32 = AccountKeyring::Alice.to_account_id().into();

    // Check Alice nonce is starting from 0.
    let runtime_api_call = node_runtime::apis()
        .account_nonce_api()
        .account_nonce(alice);
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
        .sign_and_submit_then_watch_default(&remark_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    let runtime_api_call = node_runtime::apis()
        .account_nonce_api()
        .account_nonce(alice);
    let nonce = api
        .runtime_api()
        .at_latest()
        .await?
        .call(runtime_api_call)
        .await?;
    assert_eq!(nonce, 1);

    Ok(())
}
