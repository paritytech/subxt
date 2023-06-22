// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{
        self,
        runtime_types::{
            pallet_staking::{RewardDestination, ValidatorPrefs},
            sp_arithmetic::per_things::Perbill,
        },
        staking,
    },
    test_context,
};
use assert_matches::assert_matches;
use subxt::error::{DispatchError, Error};
use subxt_signer::{
    sr25519::{self, dev},
    SecretUri,
};

/// Helper function to generate a crypto pair from seed
fn get_from_seed(seed: &str) -> sr25519::Keypair {
    use std::str::FromStr;
    let uri = SecretUri::from_str(&format!("//{seed}")).expect("expected to be valid");
    sr25519::Keypair::from_uri(&uri).expect("expected to be valid")
}

fn default_validator_prefs() -> ValidatorPrefs {
    ValidatorPrefs {
        commission: Perbill(0),
        blocked: false,
    }
}

#[tokio::test]
async fn validate_with_controller_account() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();

    let tx = node_runtime::tx()
        .staking()
        .validate(default_validator_prefs());

    api.tx()
        .sign_and_submit_then_watch_default(&tx, &alice)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .expect("should be successful");
}

#[tokio::test]
async fn validate_not_possible_for_stash_account() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice_stash = get_from_seed("Alice//stash");

    let tx = node_runtime::tx()
        .staking()
        .validate(default_validator_prefs());

    let announce_validator = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &alice_stash)
        .await?
        .wait_for_finalized_success()
        .await;
    assert_matches!(announce_validator, Err(Error::Runtime(DispatchError::Module(err))) => {
        let details = err.details().unwrap();
        assert_eq!(details.pallet.name(), "Staking");
        assert_eq!(&details.variant.name, "NotController");
    });
    Ok(())
}

#[tokio::test]
async fn nominate_with_controller_account() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob();

    let tx = node_runtime::tx()
        .staking()
        .nominate(vec![bob.public_key().to_address()]);

    api.tx()
        .sign_and_submit_then_watch_default(&tx, &alice)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .expect("should be successful");
}

#[tokio::test]
async fn nominate_not_possible_for_stash_account() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice_stash = get_from_seed("Alice//stash");
    let bob = dev::bob();

    let tx = node_runtime::tx()
        .staking()
        .nominate(vec![bob.public_key().to_address()]);

    let nomination = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &alice_stash)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await;

    assert_matches!(nomination, Err(Error::Runtime(DispatchError::Module(err))) => {
        let details = err.details().unwrap();
        assert_eq!(details.pallet.name(), "Staking");
        assert_eq!(&details.variant.name, "NotController");
    });
    Ok(())
}

#[tokio::test]
async fn chill_works_for_controller_only() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice_stash = get_from_seed("Alice//stash");
    let bob_stash = get_from_seed("Bob//stash");
    let alice = dev::alice();

    // this will fail the second time, which is why this is one test, not two
    let nominate_tx = node_runtime::tx()
        .staking()
        .nominate(vec![bob_stash.public_key().to_address()]);
    api.tx()
        .sign_and_submit_then_watch_default(&nominate_tx, &alice)
        .await?
        .wait_for_finalized_success()
        .await?;

    let ledger_addr = node_runtime::storage()
        .staking()
        .ledger(alice.public_key().to_account_id());
    let ledger = api
        .storage()
        .at_latest()
        .await?
        .fetch(&ledger_addr)
        .await?
        .unwrap();
    assert_eq!(alice_stash.public_key().to_account_id(), ledger.stash);

    let chill_tx = node_runtime::tx().staking().chill();

    let chill = api
        .tx()
        .sign_and_submit_then_watch_default(&chill_tx, &alice_stash)
        .await?
        .wait_for_finalized_success()
        .await;

    assert_matches!(chill, Err(Error::Runtime(DispatchError::Module(err))) => {
        let details = err.details().unwrap();
        assert_eq!(details.pallet.name(), "Staking");
        assert_eq!(&details.variant.name, "NotController");
    });

    let is_chilled = api
        .tx()
        .sign_and_submit_then_watch_default(&chill_tx, &alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<staking::events::Chilled>()?;
    assert!(is_chilled);

    Ok(())
}

#[tokio::test]
async fn tx_bond() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();

    let bond_tx = node_runtime::tx().staking().bond(
        dev::bob().public_key().into(),
        100_000_000_000_000,
        RewardDestination::Stash,
    );

    let bond = api
        .tx()
        .sign_and_submit_then_watch_default(&bond_tx, &alice)
        .await?
        .wait_for_finalized_success()
        .await;

    assert!(bond.is_ok());

    let bond_again = api
        .tx()
        .sign_and_submit_then_watch_default(&bond_tx, &alice)
        .await?
        .wait_for_finalized_success()
        .await;

    assert_matches!(bond_again, Err(Error::Runtime(DispatchError::Module(err))) => {
        let details = err.details().unwrap();
        assert_eq!(details.pallet.name(), "Staking");
        assert_eq!(&details.variant.name, "AlreadyBonded");
    });
    Ok(())
}

#[tokio::test]
async fn storage_history_depth() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let history_depth_addr = node_runtime::constants().staking().history_depth();
    let history_depth = api.constants().at(&history_depth_addr)?;
    assert_eq!(history_depth, 84);
    Ok(())
}

#[tokio::test]
async fn storage_current_era() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let current_era_addr = node_runtime::storage().staking().current_era();
    let _current_era = api
        .storage()
        .at_latest()
        .await?
        .fetch(&current_era_addr)
        .await?
        .expect("current era always exists");
    Ok(())
}

#[tokio::test]
async fn storage_era_reward_points() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let reward_points_addr = node_runtime::storage().staking().eras_reward_points(0);
    let current_era_result = api
        .storage()
        .at_latest()
        .await?
        .fetch(&reward_points_addr)
        .await;
    assert!(current_era_result.is_ok());

    Ok(())
}
