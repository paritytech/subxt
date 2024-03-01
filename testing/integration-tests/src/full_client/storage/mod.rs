// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{node_runtime, test_context, utils::wait_for_blocks};

#[cfg(fullclient)]
use subxt::utils::AccountId32;
#[cfg(fullclient)]
use subxt_signer::sr25519::dev;

#[subxt_test(timeout = 800)]
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

#[cfg(fullclient)]
#[subxt_test(timeout = 800)]
async fn storage_map_lookup() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let signer = dev::alice();
    let alice: AccountId32 = dev::alice().public_key().into();

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
#[cfg(fullclient)]
#[subxt_test(timeout = 800)]
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

#[cfg(fullclient)]
#[subxt_test(timeout = 800)]
async fn storage_n_map_storage_lookup() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    // Boilerplate; we create a new asset class with ID 99, and then
    // we "approveTransfer" of some of this asset class. This gives us an
    // entry in the `Approvals` StorageNMap that we can try to look up.
    let signer = dev::alice();
    let alice: AccountId32 = dev::alice().public_key().into();
    let bob: AccountId32 = dev::bob().public_key().into();

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

#[cfg(fullclient)]
#[subxt_test(timeout = 800)]
async fn storage_partial_lookup() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    // Boilerplate; we create a new asset class with ID 99, and then
    // we "approveTransfer" of some of this asset class. This gives us an
    // entry in the `Approvals` StorageNMap that we can try to look up.
    let signer = dev::alice();
    let alice: AccountId32 = dev::alice().public_key().into();
    let bob: AccountId32 = dev::bob().public_key().into();

    // Create two assets; one with ID 99 and one with ID 100.
    let assets = [
        (99, alice.clone(), bob.clone(), 123),
        (100, bob.clone(), alice.clone(), 124),
    ];
    for (asset_id, admin, delegate, amount) in assets.clone() {
        let tx1 = node_runtime::tx()
            .assets()
            .create(asset_id, admin.into(), 1);
        let tx2 = node_runtime::tx()
            .assets()
            .approve_transfer(asset_id, delegate.into(), amount);
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
    }

    // Check all approvals.
    let addr = node_runtime::storage().assets().approvals_iter();
    let addr_bytes = api.storage().address_bytes(&addr)?;
    let mut results = api.storage().at_latest().await?.iter(addr).await?;
    let mut approvals = Vec::new();
    while let Some(Ok((key, value))) = results.next().await {
        assert!(key.starts_with(&addr_bytes));
        approvals.push(value);
    }
    assert_eq!(approvals.len(), assets.len());
    let mut amounts = approvals.iter().map(|a| a.amount).collect::<Vec<_>>();
    amounts.sort();
    let mut expected = assets.iter().map(|a| a.3).collect::<Vec<_>>();
    expected.sort();
    assert_eq!(amounts, expected);

    // Check all assets starting with ID 99.
    for (asset_id, _, _, amount) in assets.clone() {
        let addr = node_runtime::storage().assets().approvals_iter1(asset_id);
        let second_addr_bytes = api.storage().address_bytes(&addr)?;
        // Keys must be different, since we are adding to the root key.
        assert_ne!(addr_bytes, second_addr_bytes);

        let mut results = api.storage().at_latest().await?.iter(addr).await?;

        let mut approvals = Vec::new();
        while let Some(Ok((key, value))) = results.next().await {
            assert!(key.starts_with(&addr_bytes));
            approvals.push(value);
        }
        assert_eq!(approvals.len(), 1);
        assert_eq!(approvals[0].amount, amount);
    }

    Ok(())
}

#[subxt_test(timeout = 800)]
async fn storage_runtime_wasm_code() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let wasm_blob = api.storage().at_latest().await?.runtime_wasm_code().await?;
    assert!(wasm_blob.len() > 1000); // the wasm should be super big
    Ok(())
}

#[subxt_test(timeout = 800)]
async fn storage_pallet_storage_version() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    // cannot assume anything about version number, but should work to fetch it
    let _version = api
        .storage()
        .at_latest()
        .await?
        .storage_version("System")
        .await?;
    let _version = api
        .storage()
        .at_latest()
        .await?
        .storage_version("Balances")
        .await?;
    Ok(())
}
