// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
    node_runtime::DispatchError,
    pair_signer,
    test_context,
};
use sp_keyring::AccountKeyring;

#[async_std::test]
async fn storage_plain_lookup() -> Result<(), subxt::Error<DispatchError>> {
    let ctx = test_context().await;

    // Look up a plain value
    let entry = ctx.api.storage().timestamp().now(None).await?;
    assert!(entry > 0);

    Ok(())
}

#[async_std::test]
async fn storage_map_lookup() -> Result<(), subxt::Error<DispatchError>> {
    let ctx = test_context().await;

    let signer = pair_signer(AccountKeyring::Alice.pair());
    let alice = AccountKeyring::Alice.to_account_id();

    // Do some transaction to bump the Alice nonce to 1:
    ctx.api
        .tx()
        .system()
        .remark(vec![1, 2, 3, 4, 5])
        .sign_and_submit_then_watch(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Look up the nonce for the user (we expect it to be 1).
    let entry = ctx
        .api
        .storage()
        .system()
        .account(alice, None)
        .await?;
    assert_eq!(entry.nonce, 1);

    Ok(())
}

#[async_std::test]
async fn storage_n_map_storage_lookup() -> Result<(), subxt::Error<DispatchError>> {
    let ctx = test_context().await;

    // Boilerplate; we create a new asset class with ID 99, and then
    // we "approveTransfer" of some of this asset class. This Gives us an
    // entry in the `Approvals` StorageNMap that we can try to look up.
    let signer = pair_signer(AccountKeyring::Alice.pair());
    let alice = AccountKeyring::Alice.to_account_id();
    let bob = AccountKeyring::Bob.to_account_id();
    ctx.api
        .tx()
        .assets()
        .create(99, alice.clone().into(), 1)
        .sign_and_submit_then_watch(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;
    ctx.api
        .tx()
        .assets()
        .approve_transfer(99, bob.clone().into(), 123)
        .sign_and_submit_then_watch(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    // The actual test; look up this approval in storage:
    let entry = ctx
        .api
        .storage()
        .assets()
        .approvals(99, alice, bob, None)
        .await?;
    assert_eq!(entry.map(|a| a.amount), Some(123));
    Ok(())
}
