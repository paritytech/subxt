// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Implements support for the pallet_staking module.

use super::balances::{
    Balances,
    BalancesEventsDecoder as _,
};
use codec::{
    Decode,
    Encode,
};

use std::{
    collections::BTreeMap,
    fmt::Debug,
    marker::PhantomData,
};

pub use pallet_staking::{
    ActiveEraInfo,
    EraIndex,
    Exposure,
    Nominations,
    RewardDestination,
    RewardPoint,
    StakingLedger,
    ValidatorPrefs,
};

/// Rewards for the last `HISTORY_DEPTH` eras.
/// If reward hasn't been set or has been removed then 0 reward is returned.
#[derive(Clone, Encode, Decode, Debug, Store)]
pub struct ErasRewardPointsStore<T: Staking> {
    #[store(returns = EraRewardPoints<T::AccountId>)]
    /// Era index
    pub index: EraIndex,
    /// Marker for the runtime
    pub _phantom: PhantomData<T>,
}

/// Preference of what happens regarding validation.
#[derive(Clone, Encode, Decode, Debug, Call)]
pub struct SetPayeeCall<T: Staking> {
    /// The payee
    pub payee: RewardDestination<T::AccountId>,
    /// Marker for the runtime
    pub _runtime: PhantomData<T>,
}

/// The subset of the `frame::Trait` that a client must implement.
#[module]
pub trait Staking: Balances {}

/// Number of eras to keep in history.
///
/// Information is kept for eras in `[current_era - history_depth; current_era]`.
///
/// Must be more than the number of eras delayed by session otherwise.
/// I.e. active era must always be in history.
/// I.e. `active_era > current_era - history_depth` must be guaranteed.
#[derive(Encode, Decode, Copy, Clone, Debug, Default, Store)]
pub struct HistoryDepthStore<T: Staking> {
    #[store(returns = u32)]
    /// Marker for the runtime
    pub _runtime: PhantomData<T>,
}

/// Map from all locked "stash" accounts to the controller account.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Store)]
pub struct BondedStore<T: Staking> {
    #[store(returns = Option<T::AccountId>)]
    /// Tٗhe stash account
    pub stash: T::AccountId,
}

/// Map from all (unlocked) "controller" accounts to the info regarding the staking.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Store)]
pub struct LedgerStore<T: Staking> {
    #[store(returns = Option<StakingLedger<T::AccountId, T::Balance>>)]
    /// The controller account
    pub controller: T::AccountId,
}

/// Where the reward payment should be made. Keyed by stash.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Store)]
pub struct PayeeStore<T: Staking> {
    #[store(returns = RewardDestination<T::AccountId>)]
    /// Tٗhe stash account
    pub stash: T::AccountId,
}

/// The map from (wannabe) validator stash key to the preferences of that validator.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Store)]
pub struct ValidatorsStore<T: Staking> {
    #[store(returns = ValidatorPrefs)]
    /// Tٗhe stash account
    pub stash: T::AccountId,
}

/// The map from nominator stash key to the set of stash keys of all validators to nominate.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Store)]
pub struct NominatorsStore<T: Staking> {
    #[store(returns = Option<Nominations<T::AccountId>>)]
    /// Tٗhe stash account
    pub stash: T::AccountId,
}

/// The current era index.
///
/// This is the latest planned era, depending on how the Session pallet queues the validator
/// set, it might be active or not.
#[derive(Encode, Copy, Clone, Debug, Store)]
pub struct CurrentEraStore<T: Staking> {
    #[store(returns = Option<EraIndex>)]
    /// Marker for the runtime
    pub _runtime: PhantomData<T>,
}

/// Reward points of an era. Used to split era total payout between validators.
///
/// This points will be used to reward validators and their respective nominators.
#[derive(PartialEq, Encode, Decode, Default, Debug)]
pub struct EraRewardPoints<AccountId: Ord> {
    /// Total number of points. Equals the sum of reward points for each validator.
    pub total: RewardPoint,
    /// The reward points earned by a given validator.
    pub individual: BTreeMap<AccountId, RewardPoint>,
}

/// Declare no desire to either validate or nominate.
///
/// Effective at the beginning of the next era.
///
/// The dispatch origin for this call must be _Signed_ by the controller, not the stash.
/// Can only be called when [`EraElectionStatus`] is `Closed`.
#[derive(Debug, Call, Encode)]
pub struct ChillCall<T: Staking> {
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

impl<T: Staking> Default for ChillCall<T> {
    fn default() -> Self {
        Self {
            _runtime: PhantomData,
        }
    }
}
impl<T: Staking> Clone for ChillCall<T> {
    fn clone(&self) -> Self {
        Self {
            _runtime: self._runtime,
        }
    }
}
impl<T: Staking> Copy for ChillCall<T> {}

/// Declare the desire to validate for the origin controller.
///
/// Effective at the beginning of the next era.
///
/// The dispatch origin for this call must be _Signed_ by the controller, not the stash.
/// Can only be called when [`EraElectionStatus`] is `Closed`.
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ValidateCall<T: Staking> {
    /// Runtime marker
    pub _runtime: PhantomData<T>,
    /// Validation preferences
    pub prefs: ValidatorPrefs,
}

/// Declare the desire to nominate `targets` for the origin controller.
///
/// Effective at the beginning of the next era.
///
/// The dispatch origin for this call must be _Signed_ by the controller, not the stash.
/// Can only be called when [`EraElectionStatus`] is `Closed`.
#[derive(Call, Encode, Debug)]
pub struct NominateCall<T: Staking> {
    /// The targets that are being nominated
    pub targets: Vec<T::Address>,
}

#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod tests {
    use super::*;
    use crate::{
        error::RuntimeError,
        extrinsic::{
            PairSigner,
            Signer,
        },
        frame::balances::*,
        runtimes::KusamaRuntime as RT,
        ClientBuilder,
        Error,
        ExtrinsicSuccess,
    };
    use assert_matches::assert_matches;
    use sp_core::{
        sr25519,
        Pair,
    };
    use sp_keyring::AccountKeyring;

    /// Helper function to generate a crypto pair from seed
    fn get_from_seed(seed: &str) -> sr25519::Pair {
        sr25519::Pair::from_string(&format!("//{}", seed), None)
            .expect("static values are valid; qed")
    }

    #[async_std::test]
    async fn test_validate_with_controller_account() -> Result<(), Error> {
        env_logger::try_init().ok();
        let alice = PairSigner::<RT, _>::new(AccountKeyring::Alice.pair());
        let client = ClientBuilder::<RT>::new().build().await?;
        let announce_validator = client
            .validate_and_watch(&alice, ValidatorPrefs::default())
            .await;
        assert_matches!(announce_validator, Ok(ExtrinsicSuccess {block: _, extrinsic: _, events}) => {
            // TOOD: this is unsatisfying – can we do better?
            assert_eq!(events.len(), 3);
        });

        Ok(())
    }

    #[async_std::test]
    async fn test_validate_not_possible_for_stash_account() -> Result<(), Error> {
        env_logger::try_init().ok();
        let alice_stash = PairSigner::<RT, _>::new(get_from_seed("Alice//stash"));
        let client = ClientBuilder::<RT>::new().build().await?;
        let announce_validator = client
            .validate_and_watch(&alice_stash, ValidatorPrefs::default())
            .await;
        assert_matches!(announce_validator, Err(Error::Runtime(RuntimeError::Module(module_err))) => {
            assert_eq!(module_err.module, "Staking");
            assert_eq!(module_err.error, "NotController");
        });
        Ok(())
    }

    #[async_std::test]
    async fn test_nominate_with_controller_account() -> Result<(), Error> {
        env_logger::try_init().ok();
        let alice = PairSigner::<RT, _>::new(AccountKeyring::Alice.pair());
        let bob = PairSigner::<RT, _>::new(AccountKeyring::Bob.pair());
        let client = ClientBuilder::<RT>::new().build().await?;

        let nomination = client
            .nominate_and_watch(&alice, vec![bob.account_id().clone()])
            .await;
        assert_matches!(nomination, Ok(ExtrinsicSuccess {block: _, extrinsic: _, events}) => {
            // TOOD: this is unsatisfying – can we do better?
            assert_eq!(events.len(), 3);
        });
        Ok(())
    }

    #[async_std::test]
    async fn test_nominate_not_possible_for_stash_account() -> Result<(), Error> {
        env_logger::try_init().ok();
        let alice_stash =
            PairSigner::<RT, sr25519::Pair>::new(get_from_seed("Alice//stash"));
        let bob = PairSigner::<RT, _>::new(AccountKeyring::Bob.pair());
        let client = ClientBuilder::<RT>::new().build().await?;

        let nomination = client
            .nominate_and_watch(&alice_stash, vec![bob.account_id().clone()])
            .await;
        assert_matches!(nomination, Err(Error::Runtime(RuntimeError::Module(module_err))) => {
            assert_eq!(module_err.module, "Staking");
            assert_eq!(module_err.error, "NotController");
        });
        Ok(())
    }

    #[async_std::test]
    async fn test_chill_works_for_controller_only() -> Result<(), Error> {
        env_logger::try_init().ok();
        let alice_stash =
            PairSigner::<RT, sr25519::Pair>::new(get_from_seed("Alice//stash"));
        let bob_stash = PairSigner::<RT, sr25519::Pair>::new(get_from_seed("Bob//stash"));
        let alice = PairSigner::<RT, _>::new(AccountKeyring::Alice.pair());
        let client = ClientBuilder::<RT>::new().build().await?;

        // this will fail the second time, which is why this is one test, not two
        client
            .nominate_and_watch(&alice, vec![bob_stash.account_id().clone()])
            .await?;
        let store = LedgerStore {
            controller: alice.account_id().clone(),
        };
        let StakingLedger { stash, .. } = client.fetch(&store, None).await?.unwrap();
        assert_eq!(alice_stash.account_id(), &stash);
        let chill = client.chill_and_watch(&alice_stash).await;

        assert_matches!(chill, Err(Error::Runtime(RuntimeError::Module(module_err))) => {
            assert_eq!(module_err.module, "Staking");
            assert_eq!(module_err.error, "NotController");
        });

        let chill = client.chill_and_watch(&alice).await;
        assert_matches!(chill, Ok(ExtrinsicSuccess {block: _, extrinsic: _, events}) => {
            // TOOD: this is unsatisfying – can we do better?
            assert_eq!(events.len(), 3);
        });
        Ok(())
    }

    #[async_std::test]
    async fn test_total_issuance_is_okay() -> Result<(), Error> {
        env_logger::try_init().ok();
        let client = ClientBuilder::<RT>::new().build().await?;
        let total_issuance = client.total_issuance(None).await?;
        assert!(total_issuance > 1u128 << 32);
        Ok(())
    }

    #[async_std::test]
    async fn test_history_depth_is_okay() -> Result<(), Error> {
        env_logger::try_init().ok();
        let client = ClientBuilder::<RT>::new().build().await?;
        let history_depth = client.history_depth(None).await?;
        assert_eq!(history_depth, 84);
        Ok(())
    }

    #[async_std::test]
    async fn test_current_era_is_okay() -> Result<(), Error> {
        env_logger::try_init().ok();
        let client = ClientBuilder::<RT>::new().build().await?;
        let _current_era = client
            .current_era(None)
            .await?
            .expect("current era always exists");
        Ok(())
    }

    #[async_std::test]
    async fn test_era_reward_points_is_okay() -> Result<(), Error> {
        env_logger::try_init().ok();
        let client = ClientBuilder::<RT>::new().build().await?;
        let store = ErasRewardPointsStore {
            _phantom: PhantomData,
            index: 0,
        };

        let _current_era = client
            .fetch(&store, None)
            .await?
            .expect("current era always exists");
        Ok(())
    }
}
