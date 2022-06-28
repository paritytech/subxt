// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{
        system,
        DispatchError,
    },
    pair_signer,
    test_context,
};
use assert_matches::assert_matches;
use sp_keyring::AccountKeyring;

#[tokio::test]
async fn storage_account() -> Result<(), subxt::Error<DispatchError>> {
    let alice = pair_signer(AccountKeyring::Alice.pair());

    let cxt = test_context().await;
    let account_info = cxt
        .api
        .storage()
        .system()
        .account(alice.account_id(), None)
        .await;

    assert_matches!(account_info, Ok(_));
    Ok(())
}

#[tokio::test]
async fn tx_remark_with_event() -> Result<(), subxt::Error<DispatchError>> {
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let cxt = test_context().await;

    let found_event = cxt
        .api
        .tx()
        .system()
        .remark_with_event(b"remarkable".to_vec())?
        .sign_and_submit_then_watch_default(&alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<system::events::Remarked>()?;

    assert!(found_event);
    Ok(())
}
