// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{
        runtime_types::pallet_staking::{
            RewardDestination,
            ValidatorPrefs,
        },
        staking,
        DispatchError,
    },
    pair_signer,
    test_context,
};
use assert_matches::assert_matches;
use sp_core::{
    sr25519,
    Pair,
};
use sp_keyring::AccountKeyring;
use subxt::Error;

/// Helper function to generate a crypto pair from seed
fn get_from_seed(seed: &str) -> sr25519::Pair {
    sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
}

fn default_validator_prefs() -> ValidatorPrefs {
    ValidatorPrefs {
        commission: sp_runtime::Perbill::default(),
        blocked: false,
    }
}

#[tokio::test]
async fn validate_with_controller_account() {
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let ctx = test_context().await;
    ctx.api
        .tx()
        .staking()
        .validate(default_validator_prefs())
        .unwrap()
        .sign_and_submit_then_watch_default(&alice)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .expect("should be successful");
}

#[tokio::test]
async fn validate_not_possible_for_stash_account() -> Result<(), Error<DispatchError>> {
    let alice_stash = pair_signer(get_from_seed("Alice//stash"));
    let ctx = test_context().await;
    let announce_validator = ctx
        .api
        .tx()
        .staking()
        .validate(default_validator_prefs())?
        .sign_and_submit_then_watch_default(&alice_stash)
        .await?
        .wait_for_finalized_success()
        .await;
    assert_matches!(announce_validator, Err(Error::Module(err)) => {
        assert_eq!(err.pallet, "Staking");
        assert_eq!(err.error, "NotController");
    });
    Ok(())
}

#[tokio::test]
async fn nominate_with_controller_account() {
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let bob = pair_signer(AccountKeyring::Bob.pair());
    let ctx = test_context().await;

    ctx.api
        .tx()
        .staking()
        .nominate(vec![bob.account_id().clone().into()])
        .unwrap()
        .sign_and_submit_then_watch_default(&alice)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .expect("should be successful");
}

#[tokio::test]
async fn nominate_not_possible_for_stash_account() -> Result<(), Error<DispatchError>> {
    let alice_stash = pair_signer(get_from_seed("Alice//stash"));
    let bob = pair_signer(AccountKeyring::Bob.pair());
    let ctx = test_context().await;

    let nomination = ctx
        .api
        .tx()
        .staking()
        .nominate(vec![bob.account_id().clone().into()])?
        .sign_and_submit_then_watch_default(&alice_stash)
        .await?
        .wait_for_finalized_success()
        .await;

    assert_matches!(nomination, Err(Error::Module(err)) => {
        assert_eq!(err.pallet, "Staking");
        assert_eq!(err.error, "NotController");
    });
    Ok(())
}

#[tokio::test]
async fn chill_works_for_controller_only() -> Result<(), Error<DispatchError>> {
    let alice_stash = pair_signer(get_from_seed("Alice//stash"));
    let bob_stash = pair_signer(get_from_seed("Bob//stash"));
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let ctx = test_context().await;

    // this will fail the second time, which is why this is one test, not two
    ctx.api
        .tx()
        .staking()
        .nominate(vec![bob_stash.account_id().clone().into()])?
        .sign_and_submit_then_watch_default(&alice)
        .await?
        .wait_for_finalized_success()
        .await?;

    let ledger = ctx
        .api
        .storage()
        .staking()
        .ledger(alice.account_id(), None)
        .await?
        .unwrap();
    assert_eq!(alice_stash.account_id(), &ledger.stash);

    let chill = ctx
        .api
        .tx()
        .staking()
        .chill()?
        .sign_and_submit_then_watch_default(&alice_stash)
        .await?
        .wait_for_finalized_success()
        .await;

    assert_matches!(chill, Err(Error::Module(err)) => {
        assert_eq!(err.pallet, "Staking");
        assert_eq!(err.error, "NotController");
    });

    let is_chilled = ctx
        .api
        .tx()
        .staking()
        .chill()?
        .sign_and_submit_then_watch_default(&alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<staking::events::Chilled>()?;
    assert!(is_chilled);

    Ok(())
}

#[tokio::test]
async fn tx_bond() -> Result<(), Error<DispatchError>> {
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let ctx = test_context().await;

    let bond = ctx
        .api
        .tx()
        .staking()
        .bond(
            AccountKeyring::Bob.to_account_id().into(),
            100_000_000_000_000,
            RewardDestination::Stash,
        )
        .unwrap()
        .sign_and_submit_then_watch_default(&alice)
        .await?
        .wait_for_finalized_success()
        .await;

    assert!(bond.is_ok());

    let bond_again = ctx
        .api
        .tx()
        .staking()
        .bond(
            AccountKeyring::Bob.to_account_id().into(),
            100_000_000_000_000,
            RewardDestination::Stash,
        )
        .unwrap()
        .sign_and_submit_then_watch_default(&alice)
        .await?
        .wait_for_finalized_success()
        .await;

    assert_matches!(bond_again, Err(Error::Module(err)) => {
        assert_eq!(err.pallet, "Staking");
        assert_eq!(err.error, "AlreadyBonded");
    });
    Ok(())
}

#[tokio::test]
async fn storage_history_depth() -> Result<(), Error<DispatchError>> {
    let ctx = test_context().await;
    let history_depth = ctx.api.storage().staking().history_depth(None).await?;
    assert_eq!(history_depth, 84);
    Ok(())
}

#[tokio::test]
async fn storage_current_era() -> Result<(), Error<DispatchError>> {
    let ctx = test_context().await;
    let _current_era = ctx
        .api
        .storage()
        .staking()
        .current_era(None)
        .await?
        .expect("current era always exists");
    Ok(())
}

#[tokio::test]
async fn storage_era_reward_points() -> Result<(), Error<DispatchError>> {
    let cxt = test_context().await;
    let current_era_result = cxt
        .api
        .storage()
        .staking()
        .eras_reward_points(&0, None)
        .await;
    assert!(current_era_result.is_ok());

    Ok(())
}
