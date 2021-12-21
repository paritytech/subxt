// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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
        runtime_types::pallet_staking::{
            RewardDestination,
            ValidatorPrefs,
        },
        staking,
        DefaultConfig,
    },
    test_context,
};
use assert_matches::assert_matches;
use sp_core::{
    sr25519,
    Pair,
};
use sp_keyring::AccountKeyring;
use subxt::{
    extrinsic::{
        PairSigner,
        Signer,
    },
    Error,
    RuntimeError,
};

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

#[async_std::test]
async fn validate_with_controller_account() {
    let alice = PairSigner::<DefaultConfig, _>::new(AccountKeyring::Alice.pair());
    let cxt = test_context().await;
    cxt.api
        .tx()
        .staking()
        .validate(default_validator_prefs())
        .sign_and_submit_then_watch(&alice)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .expect("should be successful");
}

#[async_std::test]
async fn validate_not_possible_for_stash_account() -> Result<(), Error> {
    let alice_stash = PairSigner::<DefaultConfig, _>::new(get_from_seed("Alice//stash"));
    let cxt = test_context().await;
    let announce_validator = cxt
        .api
        .tx()
        .staking()
        .validate(default_validator_prefs())
        .sign_and_submit_then_watch(&alice_stash)
        .await?
        .wait_for_finalized_success()
        .await;
    assert_matches!(announce_validator, Err(Error::Runtime(RuntimeError::Module(module_err))) => {
        assert_eq!(module_err.pallet, "Staking");
        assert_eq!(module_err.error, "NotController");
    });
    Ok(())
}

#[async_std::test]
async fn nominate_with_controller_account() {
    let alice = PairSigner::<DefaultConfig, _>::new(AccountKeyring::Alice.pair());
    let bob = PairSigner::<DefaultConfig, _>::new(AccountKeyring::Bob.pair());
    let cxt = test_context().await;

    cxt.api
        .tx()
        .staking()
        .nominate(vec![bob.account_id().clone().into()])
        .sign_and_submit_then_watch(&alice)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .expect("should be successful");
}

#[async_std::test]
async fn nominate_not_possible_for_stash_account() -> Result<(), Error> {
    let alice_stash =
        PairSigner::<DefaultConfig, sr25519::Pair>::new(get_from_seed("Alice//stash"));
    let bob = PairSigner::<DefaultConfig, _>::new(AccountKeyring::Bob.pair());
    let cxt = test_context().await;

    let nomination = cxt
        .api
        .tx()
        .staking()
        .nominate(vec![bob.account_id().clone().into()])
        .sign_and_submit_then_watch(&alice_stash)
        .await?
        .wait_for_finalized_success()
        .await;

    assert_matches!(nomination, Err(Error::Runtime(RuntimeError::Module(module_err))) => {
        assert_eq!(module_err.pallet, "Staking");
        assert_eq!(module_err.error, "NotController");
    });
    Ok(())
}

#[async_std::test]
async fn chill_works_for_controller_only() -> Result<(), Error> {
    let alice_stash =
        PairSigner::<DefaultConfig, sr25519::Pair>::new(get_from_seed("Alice//stash"));
    let bob_stash =
        PairSigner::<DefaultConfig, sr25519::Pair>::new(get_from_seed("Bob//stash"));
    let alice = PairSigner::<DefaultConfig, _>::new(AccountKeyring::Alice.pair());
    let cxt = test_context().await;

    // this will fail the second time, which is why this is one test, not two
    cxt.api
        .tx()
        .staking()
        .nominate(vec![bob_stash.account_id().clone().into()])
        .sign_and_submit_then_watch(&alice)
        .await?
        .wait_for_finalized_success()
        .await?;

    let ledger = cxt
        .api
        .storage()
        .staking()
        .ledger(alice.account_id().clone(), None)
        .await?
        .unwrap();
    assert_eq!(alice_stash.account_id(), &ledger.stash);

    let chill = cxt
        .api
        .tx()
        .staking()
        .chill()
        .sign_and_submit_then_watch(&alice_stash)
        .await?
        .wait_for_finalized_success()
        .await;

    assert_matches!(chill, Err(Error::Runtime(RuntimeError::Module(module_err))) => {
        assert_eq!(module_err.pallet, "Staking");
        assert_eq!(module_err.error, "NotController");
    });

    let is_chilled = cxt
        .api
        .tx()
        .staking()
        .chill()
        .sign_and_submit_then_watch(&alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has_event::<staking::events::Chilled>()?;
    assert!(is_chilled);

    Ok(())
}

#[async_std::test]
async fn tx_bond() -> Result<(), Error> {
    let alice = PairSigner::<DefaultConfig, _>::new(AccountKeyring::Alice.pair());
    let cxt = test_context().await;

    let bond = cxt
        .api
        .tx()
        .staking()
        .bond(
            AccountKeyring::Bob.to_account_id().into(),
            100_000_000_000_000,
            RewardDestination::Stash,
        )
        .sign_and_submit_then_watch(&alice)
        .await?
        .wait_for_finalized_success()
        .await;

    assert!(bond.is_ok());

    let bond_again = cxt
        .api
        .tx()
        .staking()
        .bond(
            AccountKeyring::Bob.to_account_id().into(),
            100_000_000_000_000,
            RewardDestination::Stash,
        )
        .sign_and_submit_then_watch(&alice)
        .await?
        .wait_for_finalized_success()
        .await;

    assert_matches!(bond_again, Err(Error::Runtime(RuntimeError::Module(module_err))) => {
        assert_eq!(module_err.pallet, "Staking");
        assert_eq!(module_err.error, "AlreadyBonded");
    });

    Ok(())
}

#[async_std::test]
async fn storage_history_depth() -> Result<(), Error> {
    let cxt = test_context().await;
    let history_depth = cxt.api.storage().staking().history_depth(None).await?;
    assert_eq!(history_depth, 84);
    Ok(())
}

#[async_std::test]
async fn storage_current_era() -> Result<(), Error> {
    let cxt = test_context().await;
    let _current_era = cxt
        .api
        .storage()
        .staking()
        .current_era(None)
        .await?
        .expect("current era always exists");
    Ok(())
}

#[async_std::test]
async fn storage_era_reward_points() -> Result<(), Error> {
    let cxt = test_context().await;
    let current_era_result = cxt
        .api
        .storage()
        .staking()
        .eras_reward_points(0, None)
        .await;
    assert!(current_era_result.is_ok());

    Ok(())
}
