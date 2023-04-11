// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{node_runtime, pair_signer, test_context, utils::wait_for_blocks};
use sp_keyring::AccountKeyring;
use subxt::utils::AccountId32;

#[tokio::test]
async fn storage_plain_lookup() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    // Look up a plain value. Wait long enough that we don't get the genesis block data,
    // because it may have no storage associated with it.
    wait_for_blocks(&api).await;

    let addr = node_runtime::storage().timestamp().now();
    let entry = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&addr)
        .await?;
    assert!(entry > 0);

    Ok(())
}

#[tokio::test]
async fn storage_map_lookup() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let signer = pair_signer(AccountKeyring::Alice.pair());
    let alice: AccountId32 = AccountKeyring::Alice.to_account_id().into();

    // Do some transaction to bump the Alice nonce to 1:
    let remark_tx = node_runtime::tx().system().remark(vec![1, 2, 3, 4, 5]);
    api.tx()
        .sign_and_submit_then_watch_default(&remark_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Look up the nonce for the user (we expect it to be 1).
    let nonce_addr = node_runtime::storage().system().account(alice);
    let entry = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&nonce_addr)
        .await?;
    assert_eq!(entry.nonce, 1);

    Ok(())
}

// This fails until the fix in https://github.com/paritytech/subxt/pull/458 is introduced.
// Here we create a key that looks a bit like a StorageNMap key, but should in fact be
// treated as a StorageKey (ie we should hash both values together with one hasher, rather
// than hash both values separately, or ignore the second value).
#[tokio::test]
async fn storage_n_mapish_key_is_properly_created() -> Result<(), subxt::Error> {
    use codec::Encode;
    use node_runtime::runtime_types::sp_core::crypto::KeyTypeId;

    let ctx = test_context().await;
    let api = ctx.client();

    // This is what the generated code hashes a `session().key_owner(..)` key into:
    let actual_key = node_runtime::storage()
        .session()
        .key_owner(KeyTypeId([1, 2, 3, 4]), [5u8, 6, 7, 8]);
    let actual_key_bytes = api.storage().address_bytes(&actual_key)?;

    // Let's manually hash to what we assume it should be and compare:
    let expected_key_bytes = {
        // Hash the prefix to the storage entry:
        let mut bytes = sp_core::twox_128("Session".as_bytes()).to_vec();
        bytes.extend(&sp_core::twox_128("KeyOwner".as_bytes())[..]);
        // twox64_concat a *tuple* of the args expected:
        let suffix = (KeyTypeId([1, 2, 3, 4]), vec![5u8, 6, 7, 8]).encode();
        bytes.extend(sp_core::twox_64(&suffix));
        bytes.extend(&suffix);
        bytes
    };

    assert_eq!(actual_key_bytes, expected_key_bytes);
    Ok(())
}

#[tokio::test]
async fn storage_n_map_storage_lookup() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    // Boilerplate; we create a new asset class with ID 99, and then
    // we "approveTransfer" of some of this asset class. This gives us an
    // entry in the `Approvals` StorageNMap that we can try to look up.
    let signer = pair_signer(AccountKeyring::Alice.pair());
    let alice: AccountId32 = AccountKeyring::Alice.to_account_id().into();
    let bob: AccountId32 = AccountKeyring::Bob.to_account_id().into();

    let tx1 = node_runtime::tx()
        .assets()
        .create(99, alice.clone().into(), 1);
    let tx2 = node_runtime::tx()
        .assets()
        .approve_transfer(99, bob.clone().into(), 123);
    api.tx()
        .sign_and_submit_then_watch_default(&tx1, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;
    api.tx()
        .sign_and_submit_then_watch_default(&tx2, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    // The actual test; look up this approval in storage:
    let addr = node_runtime::storage().assets().approvals(99, alice, bob);
    let entry = api.storage().at_latest().await?.fetch(&addr).await?;
    assert_eq!(entry.map(|a| a.amount), Some(123));
    Ok(())
}
