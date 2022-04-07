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
    node_runtime::{
        self,
        DispatchError,
    },
    pair_signer,
    test_context,
};
use sp_keyring::AccountKeyring;

#[tokio::test]
async fn storage_plain_lookup() -> Result<(), subxt::Error<DispatchError>> {
    let ctx = test_context().await;

    // Look up a plain value. Wait long enough that we don't get the genesis block data,
    // because it may have no storage associated with it.
    tokio::time::sleep(std::time::Duration::from_secs(6)).await;
    let entry = ctx.api.storage().timestamp().now(None).await?;
    assert!(entry > 0);

    Ok(())
}

#[tokio::test]
async fn storage_map_lookup() -> Result<(), subxt::Error<DispatchError>> {
    let ctx = test_context().await;

    let signer = pair_signer(AccountKeyring::Alice.pair());
    let alice = AccountKeyring::Alice.to_account_id();

    // Do some transaction to bump the Alice nonce to 1:
    ctx.api
        .tx()
        .system()
        .remark(vec![1, 2, 3, 4, 5])?
        .sign_and_submit_then_watch_default(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Look up the nonce for the user (we expect it to be 1).
    let entry = ctx.api.storage().system().account(&alice, None).await?;
    assert_eq!(entry.nonce, 1);

    Ok(())
}

// This fails until the fix in https://github.com/paritytech/subxt/pull/458 is introduced.
// Here we create a key that looks a bit like a StorageNMap key, but should in fact be
// treated as a StorageKey (ie we should hash both values together with one hasher, rather
// than hash both values separately, or ignore the second value).
#[tokio::test]
async fn storage_n_mapish_key_is_properly_created(
) -> Result<(), subxt::Error<DispatchError>> {
    use codec::Encode;
    use node_runtime::{
        runtime_types::sp_core::crypto::KeyTypeId,
        session::storage::KeyOwner,
    };
    use subxt::{
        storage::StorageKeyPrefix,
        StorageEntry,
    };

    // This is what the generated code hashes a `session().key_owner(..)` key into:
    let actual_key_bytes = KeyOwner(&KeyTypeId([1, 2, 3, 4]), &[5u8, 6, 7, 8])
        .key()
        .final_key(StorageKeyPrefix::new::<KeyOwner>())
        .0;

    // Let's manually hash to what we assume it should be and compare:
    let expected_key_bytes = {
        // Hash the prefix to the storage entry:
        let mut bytes = sp_core::twox_128("Session".as_bytes()).to_vec();
        bytes.extend(&sp_core::twox_128("KeyOwner".as_bytes())[..]);
        // twox64_concat a *tuple* of the args expected:
        let suffix = (KeyTypeId([1, 2, 3, 4]), vec![5u8, 6, 7, 8]).encode();
        bytes.extend(&sp_core::twox_64(&suffix));
        bytes.extend(&suffix);
        bytes
    };

    assert_eq!(actual_key_bytes, expected_key_bytes);
    Ok(())
}

#[tokio::test]
async fn storage_n_map_storage_lookup() -> Result<(), subxt::Error<DispatchError>> {
    let ctx = test_context().await;

    // Boilerplate; we create a new asset class with ID 99, and then
    // we "approveTransfer" of some of this asset class. This gives us an
    // entry in the `Approvals` StorageNMap that we can try to look up.
    let signer = pair_signer(AccountKeyring::Alice.pair());
    let alice = AccountKeyring::Alice.to_account_id();
    let bob = AccountKeyring::Bob.to_account_id();
    ctx.api
        .tx()
        .assets()
        .create(99, alice.clone().into(), 1)?
        .sign_and_submit_then_watch_default(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;
    ctx.api
        .tx()
        .assets()
        .approve_transfer(99, bob.clone().into(), 123)?
        .sign_and_submit_then_watch_default(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    // The actual test; look up this approval in storage:
    let entry = ctx
        .api
        .storage()
        .assets()
        .approvals(&99, &alice, &bob, None)
        .await?;
    assert_eq!(entry.map(|a| a.amount), Some(123));
    Ok(())
}
