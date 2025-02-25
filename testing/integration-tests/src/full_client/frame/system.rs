// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{self, system},
    subxt_test, test_context,
};
use assert_matches::assert_matches;
use subxt_signer::sr25519::dev;

#[subxt_test]
async fn storage_account() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();

    let account_info_addr = node_runtime::storage()
        .system()
        .account(alice.public_key().to_account_id());

    let account_info = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&account_info_addr)
        .await;

    assert_matches!(account_info, Ok(_));
    Ok(())
}

#[subxt_test]
async fn tx_remark_with_event() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();

    let tx = node_runtime::tx()
        .system()
        .remark_with_event(b"remarkable".to_vec());

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await?;

    let found_event = signed_extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<system::events::Remarked>()?;

    assert!(found_event);
    Ok(())
}
