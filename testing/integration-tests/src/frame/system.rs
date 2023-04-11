// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{self, system},
    pair_signer, test_context,
};
use assert_matches::assert_matches;
use sp_keyring::AccountKeyring;

#[tokio::test]
async fn storage_account() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = pair_signer(AccountKeyring::Alice.pair());

    let account_info_addr = node_runtime::storage().system().account(alice.account_id());

    let account_info = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&account_info_addr)
        .await;

    assert_matches!(account_info, Ok(_));
    Ok(())
}

#[tokio::test]
async fn tx_remark_with_event() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = pair_signer(AccountKeyring::Alice.pair());

    let tx = node_runtime::tx()
        .system()
        .remark_with_event(b"remarkable".to_vec());

    let found_event = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<system::events::Remarked>()?;

    assert!(found_event);
    Ok(())
}
