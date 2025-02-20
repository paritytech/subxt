// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{self, balances, runtime_types, system},
    subxt_test, test_context,
};
use codec::Decode;
use subxt::{
    error::{DispatchError, Error, TokenError},
    utils::{AccountId32, MultiAddress},
};
use subxt_signer::sr25519::dev;

#[subxt_test]
async fn tx_basic_transfer() -> Result<(), subxt::Error> {
    let alice = dev::alice();
    let bob = dev::bob();
    let bob_address = bob.public_key().to_address();
    let ctx = test_context().await;
    let api = ctx.client();

    let alice_account_addr = node_runtime::storage()
        .system()
        .account(alice.public_key().to_account_id());
    let bob_account_addr = node_runtime::storage()
        .system()
        .account(bob.public_key().to_account_id());

    let alice_pre = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&alice_account_addr)
        .await?;
    let bob_pre = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&bob_account_addr)
        .await?;

    let tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(bob_address, 10_000);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await?;

    let events = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await?;

    let event = events
        .find_first::<balances::events::Transfer>()
        .expect("Failed to decode balances::events::Transfer")
        .expect("Failed to find balances::events::Transfer");
    let _extrinsic_success = events
        .find_first::<system::events::ExtrinsicSuccess>()
        .expect("Failed to decode ExtrinisicSuccess")
        .expect("Failed to find ExtrinisicSuccess");

    let expected_event = balances::events::Transfer {
        from: alice.public_key().to_account_id(),
        to: bob.public_key().to_account_id(),
        amount: 10_000,
    };
    assert_eq!(event, expected_event);

    let alice_post = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&alice_account_addr)
        .await?;
    let bob_post = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&bob_account_addr)
        .await?;

    assert!(alice_pre.data.free - 10_000 >= alice_post.data.free);
    assert_eq!(bob_pre.data.free + 10_000, bob_post.data.free);
    Ok(())
}

#[cfg(fullclient)]
#[subxt_test]
async fn tx_dynamic_transfer() -> Result<(), subxt::Error> {
    use subxt::ext::scale_value::{At, Composite, Value};

    let alice = dev::alice();
    let bob = dev::bob();
    let ctx = test_context().await;
    let api = ctx.client();

    let alice_account_addr = subxt::dynamic::storage(
        "System",
        "Account",
        vec![Value::from_bytes(alice.public_key().to_account_id())],
    );
    let bob_account_addr = subxt::dynamic::storage(
        "System",
        "Account",
        vec![Value::from_bytes(bob.public_key().to_account_id())],
    );

    let alice_pre = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&alice_account_addr)
        .await?;
    let bob_pre = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&bob_account_addr)
        .await?;

    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer_allow_death",
        vec![
            Value::unnamed_variant(
                "Id",
                vec![Value::from_bytes(bob.public_key().to_account_id())],
            ),
            Value::u128(10_000u128),
        ],
    );

    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &alice)
        .await?
        .wait_for_finalized_success()
        .await?;

    let event_fields = events
        .iter()
        .filter_map(|ev| ev.ok())
        .find(|ev| ev.pallet_name() == "Balances" && ev.variant_name() == "Transfer")
        .expect("Failed to find Transfer event")
        .field_values()?
        .map_context(|_| ());

    let expected_fields = Composite::Named(vec![
        (
            "from".into(),
            Value::unnamed_composite(vec![Value::from_bytes(alice.public_key().to_account_id())]),
        ),
        (
            "to".into(),
            Value::unnamed_composite(vec![Value::from_bytes(bob.public_key().to_account_id())]),
        ),
        ("amount".into(), Value::u128(10_000)),
    ]);
    assert_eq!(event_fields, expected_fields);

    let alice_post = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&alice_account_addr)
        .await?;
    let bob_post = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&bob_account_addr)
        .await?;

    let alice_pre_free = alice_pre
        .to_value()?
        .at("data")
        .at("free")
        .unwrap()
        .as_u128()
        .unwrap();
    let alice_post_free = alice_post
        .to_value()?
        .at("data")
        .at("free")
        .unwrap()
        .as_u128()
        .unwrap();

    let bob_pre_free = bob_pre
        .to_value()?
        .at("data")
        .at("free")
        .unwrap()
        .as_u128()
        .unwrap();
    let bob_post_free = bob_post
        .to_value()?
        .at("data")
        .at("free")
        .unwrap()
        .as_u128()
        .unwrap();

    assert!(alice_pre_free - 10_000 >= alice_post_free);
    assert_eq!(bob_pre_free + 10_000, bob_post_free);

    Ok(())
}

#[subxt_test]
async fn multiple_sequential_transfers_work() -> Result<(), subxt::Error> {
    let alice = dev::alice();
    let bob = dev::bob();
    let bob_address: MultiAddress<AccountId32, u32> = bob.public_key().into();
    let ctx = test_context().await;
    let api = ctx.client();

    let bob_account_info_addr = node_runtime::storage()
        .system()
        .account(bob.public_key().to_account_id());

    let bob_pre = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&bob_account_info_addr)
        .await?;

    // Do a transfer several times. If this works, it indicates that the
    // nonce is properly incremented each time.
    let tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(bob_address.clone(), 10_000);
    for _ in 0..3 {
        let signed_extrinsic = api
            .tx()
            .create_signed(&tx, &alice, Default::default())
            .await?;

        signed_extrinsic
            .submit_and_watch()
            .await
            .unwrap()
            .wait_for_finalized_success()
            .await?;
    }

    let bob_post = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&bob_account_info_addr)
        .await?;

    assert_eq!(bob_pre.data.free + 30_000, bob_post.data.free);
    Ok(())
}

#[subxt_test]
async fn storage_total_issuance() {
    let ctx = test_context().await;
    let api = ctx.client();

    let addr = node_runtime::storage().balances().total_issuance();
    let total_issuance = api
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch_or_default(&addr)
        .await
        .unwrap();
    assert_ne!(total_issuance, 0);
}

#[subxt_test]
async fn storage_balance_lock() -> Result<(), subxt::Error> {
    let bob_signer = dev::bob();
    let bob: AccountId32 = dev::bob().public_key().into();
    let ctx = test_context().await;
    let api = ctx.client();

    let tx = node_runtime::tx().staking().bond(
        100_000_000_000_000,
        runtime_types::pallet_staking::RewardDestination::Stash,
    );

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &bob_signer, Default::default())
        .await?;

    signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await?
        .find_first::<system::events::ExtrinsicSuccess>()?
        .expect("No ExtrinsicSuccess Event found");

    let holds_addr = node_runtime::storage().balances().holds(bob);

    let holds = api
        .storage()
        .at_latest()
        .await?
        .fetch_or_default(&holds_addr)
        .await?
        .0;

    // There is now a hold on the balance being staked
    assert_eq!(holds.len(), 1);
    assert_eq!(holds[0].amount, 100_000_000_000_000);

    Ok(())
}

#[subxt_test]
async fn transfer_error() {
    let alice = dev::alice();
    let alice_addr = alice.public_key().into();
    let bob = dev::one(); // some dev account with no funds.
    let bob_address = bob.public_key().into();
    let ctx = test_context().await;
    let api = ctx.client();

    let to_bob_tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(bob_address, 100_000_000_000_000_000);
    let to_alice_tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(alice_addr, 100_000_000_000_000_000);

    let signed_extrinsic = api
        .tx()
        .create_signed(&to_bob_tx, &alice, Default::default())
        .await
        .unwrap();
    signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();

    // When we try giving all of the funds back, Bob doesn't have
    // anything left to pay transfer fees, so we hit an error.
    let signed_extrinsic = api
        .tx()
        .create_signed(&to_alice_tx, &bob, Default::default())
        .await
        .unwrap();

    let res = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await;

    assert!(
        matches!(
            res,
            Err(Error::Runtime(DispatchError::Token(
                TokenError::FundsUnavailable
            )))
        ),
        "Expected an insufficient balance, got {res:?}"
    );
}

#[subxt_test]
async fn transfer_implicit_subscription() {
    let alice = dev::alice();
    let bob: AccountId32 = dev::bob().public_key().into();
    let ctx = test_context().await;
    let api = ctx.client();

    let to_bob_tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(bob.clone().into(), 10_000);

    let signed_extrinsic = api
        .tx()
        .create_signed(&to_bob_tx, &alice, Default::default())
        .await
        .unwrap();

    let event = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap()
        .find_first::<balances::events::Transfer>()
        .expect("Can decode events")
        .expect("Can find balance transfer event");

    assert_eq!(
        event,
        balances::events::Transfer {
            from: alice.public_key().to_account_id(),
            to: bob,
            amount: 10_000
        }
    );
}

#[subxt_test]
async fn constant_existential_deposit() {
    let ctx = test_context().await;
    let api = ctx.client();

    // get and decode constant manually via metadata:
    let metadata = api.metadata();
    let balances_metadata = metadata.pallet_by_name("Balances").unwrap();
    let constant_metadata = balances_metadata
        .constant_by_name("ExistentialDeposit")
        .unwrap();
    let existential_deposit = u128::decode(&mut constant_metadata.value()).unwrap();
    assert_eq!(existential_deposit, 100_000_000_000_000);

    // constant address for API access:
    let addr = node_runtime::constants().balances().existential_deposit();

    // Make sure thetwo are identical:
    assert_eq!(existential_deposit, api.constants().at(&addr).unwrap());
}
