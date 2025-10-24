// Copyright 2019-2025 Parity Technologies (UK) Ltd.
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
    subxt_test, test_context,
};
use subxt::error::{
    DispatchError, Error, TransactionEventsError, TransactionFinalizedSuccessError,
};
use subxt_signer::{
    SecretUri,
    sr25519::{self, dev},
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

#[subxt_test]
async fn validate_with_stash_account() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice_stash = get_from_seed("Alice//stash");

    let tx = node_runtime::tx()
        .staking()
        .validate(default_validator_prefs());

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice_stash, Default::default())
        .await
        .unwrap();

    signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .expect("should be successful");
}

#[subxt_test]
async fn validate_not_possible_for_controller_account() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();

    let tx = node_runtime::tx()
        .staking()
        .validate(default_validator_prefs());

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await?;

    let announce_validator = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await;

    if let Err(TransactionFinalizedSuccessError::SuccessError(
        TransactionEventsError::ExtrinsicFailed(DispatchError::Module(err)),
    )) = announce_validator
    {
        let details = err.details().unwrap();
        assert_eq!(details.pallet.name(), "Staking");
        assert_eq!(&details.variant.name, "NotController");
    } else {
        panic!("Expected an error");
    }
    Ok(())
}

#[subxt_test]
async fn nominate_with_stash_account() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice_stash = get_from_seed("Alice//stash");
    let bob = dev::bob();

    let tx = node_runtime::tx()
        .staking()
        .nominate(vec![bob.public_key().to_address()]);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice_stash, Default::default())
        .await
        .unwrap();

    signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .expect("should be successful");
}

#[subxt_test]
async fn nominate_not_possible_for_controller_account() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob();

    let tx = node_runtime::tx()
        .staking()
        .nominate(vec![bob.public_key().to_address()]);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await
        .unwrap();
    let nomination = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await;

    if let Err(TransactionFinalizedSuccessError::SuccessError(
        TransactionEventsError::ExtrinsicFailed(DispatchError::Module(err)),
    )) = nomination
    {
        let details = err.details().unwrap();
        assert_eq!(details.pallet.name(), "Staking");
        assert_eq!(&details.variant.name, "NotController");
    } else {
        panic!("Expected an error");
    }
    Ok(())
}

#[subxt_test]
async fn chill_works_for_stash_only() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice_stash = get_from_seed("Alice//stash");
    let bob_stash = get_from_seed("Bob//stash");
    let alice = dev::alice();

    // this will fail the second time, which is why this is one test, not two
    let nominate_tx = node_runtime::tx()
        .staking()
        .nominate(vec![bob_stash.public_key().to_address()]);

    let signed_extrinsic = api
        .tx()
        .create_signed(&nominate_tx, &alice_stash, Default::default())
        .await?;
    signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await?;

    let ledger_addr = node_runtime::storage().staking().ledger();
    let ledger = api
        .storage()
        .at_latest()
        .await?
        .fetch(ledger_addr, (alice_stash.public_key().to_account_id(),))
        .await?
        .decode()?;
    assert_eq!(alice_stash.public_key().to_account_id(), ledger.stash);

    let chill_tx = node_runtime::tx().staking().chill();

    let signed_extrinsic = api
        .tx()
        .create_signed(&chill_tx, &alice, Default::default())
        .await?;

    let chill = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await;

    if let Err(TransactionFinalizedSuccessError::SuccessError(
        TransactionEventsError::ExtrinsicFailed(DispatchError::Module(err)),
    )) = chill
    {
        let details = err.details().unwrap();
        assert_eq!(details.pallet.name(), "Staking");
        assert_eq!(&details.variant.name, "NotController");
    } else {
        panic!("Expected an error");
    }

    let signed_extrinsic = api
        .tx()
        .create_signed(&chill_tx, &alice_stash, Default::default())
        .await?;
    let is_chilled = signed_extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<staking::events::Chilled>()?;

    assert!(is_chilled);

    Ok(())
}

#[subxt_test]
async fn tx_bond() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();

    let bond_tx = node_runtime::tx()
        .staking()
        .bond(100_000_000_000_000, RewardDestination::Stash);

    let signed_extrinsic = api
        .tx()
        .create_signed(&bond_tx, &alice, Default::default())
        .await?;
    let bond = signed_extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await;
    assert!(bond.is_ok());

    let signed_extrinsic = api
        .tx()
        .create_signed(&bond_tx, &alice, Default::default())
        .await?;

    let bond_again = signed_extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await;

    if let Err(TransactionFinalizedSuccessError::SuccessError(
        TransactionEventsError::ExtrinsicFailed(DispatchError::Module(err)),
    )) = bond_again
    {
        let details = err.details().unwrap();
        assert_eq!(details.pallet.name(), "Staking");
        assert_eq!(&details.variant.name, "AlreadyBonded");
    } else {
        panic!("Expected an error");
    }
    Ok(())
}

#[subxt_test]
async fn storage_history_depth() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let history_depth_addr = node_runtime::constants().staking().history_depth();
    let history_depth = api.constants().at(&history_depth_addr)?;
    assert_eq!(history_depth, 84);
    Ok(())
}

#[subxt_test]
async fn storage_current_era() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let current_era_addr = node_runtime::storage().staking().current_era();
    let _current_era = api
        .storage()
        .at_latest()
        .await?
        .fetch(current_era_addr, ())
        .await?
        .decode()?;
    Ok(())
}

#[subxt_test]
async fn storage_era_reward_points() -> Result<(), Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let reward_points_addr = node_runtime::storage().staking().eras_reward_points();
    let current_era_result = api
        .storage()
        .at_latest()
        .await?
        .fetch(reward_points_addr, (0,))
        .await?
        .decode();
    assert!(current_era_result.is_ok());

    Ok(())
}
