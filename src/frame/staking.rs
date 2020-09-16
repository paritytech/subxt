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
    pub payee: RewardDestination,
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
    #[store(returns = RewardDestination)]
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

#[cfg(all(test, feature = "integration-tests"))]
mod tests {
    use super::*;
    use crate::{
        extrinsic::PairSigner,
        runtimes::KusamaRuntime as RT,
        ClientBuilder,
    };
    // use sp_core::{sr25519::Pair, Pair as _};
    use crate::{
        error::Error,
        extrinsic::Signer,
        frame::{
            balances::*,
            system::*,
        },
    };
    use sp_keyring::AccountKeyring;

    #[async_std::test]
    async fn test_nominate() -> Result<(), Error> {
        env_logger::try_init().ok();
        let alice = PairSigner::<RT, _>::new(AccountKeyring::Alice.pair());
        let bob = PairSigner::<RT, _>::new(AccountKeyring::Bob.pair());

        let client = ClientBuilder::<RT>::new().build().await?;
        let current_era = client.current_era(None).await?;
        println!("Current era: {:?}", current_era);
        let hd = client.history_depth(None).await?;
        println!("History depth: {:?}", hd);
        let total_issuance = client.total_issuance(None).await?;
        println!("total issuance: {:?}", total_issuance);
        let alice_account = client.account(&alice.account_id(), None).await?;
        println!("Alice's account info: {:?}", alice_account);
        let o = client
            .nominate(&alice, vec![bob.account_id().clone()])
            .await?;
        println!("Nom nom: {:?}", o);
        let o = client
            .validate(&bob, ValidatorPrefs::default())
            .await?;
        println!("Validator result: {:?}", o);
        for &i in &[RewardDestination::Controller] {
            for &j in &[&bob, &alice] {
                println!(
                    "Transaction result: {:?}",
                    client.set_payee(j, i).await?
                );
            }
        }
        assert_eq!(
            client
                .fetch(
                    &ValidatorsStore {
                        stash: bob.account_id().clone()
                    },
                    None
                )
                .await?,
            None
        );
        Ok(())
    }
}
