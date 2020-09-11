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
use frame_support::Parameter;
use sp_runtime::traits::{
    MaybeSerialize,
    Member,
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
pub trait Staking: Balances {
    /// Data type used to index nominators in the compact type
    type NominatorIndex: Parameter
        + codec::Codec
        + Member
        + Default
        + Copy
        + MaybeSerialize
        + Debug;

    /// Data type used to index validators in the compact type.
    type ValidatorIndex: Parameter
        + codec::Codec
        + Send
        + Sync
        + Default
        + Member
        + Copy
        + MaybeSerialize
        + Debug;
}

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

/// Clipped Exposure of validator at era.
///
/// This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the
/// `T::MaxNominatorRewardedPerValidator` biggest stakers.
/// (Note: the field `total` and `own` of the exposure remains unchanged).
/// This is used to limit the i/o cost for the nominator payout.
///
/// This is keyed fist by the era index to allow bulk deletion and then the stash account.
///
/// It is removed after `HISTORY_DEPTH` eras.
/// If stakers hasn't been set or has been removed then empty exposure is returned.
#[derive(Encode, Copy, Clone, Debug, Store)]
pub struct ErasStakersClippedStore<T: Staking> {
    #[store(returns = Exposure<T::AccountId, T::Balance>)]
    /// Era index
    pub era: EraIndex,
    /// Stash account of the validator
    pub validator_stash: T::AccountId,
}

/// The active era information, holds index and start.
///
/// The active era is the era currently rewarded.
/// Validator set of this era must be equal to `SessionInterface::validators`.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Store)]
pub struct ActiveEraStore<T: Staking> {
    #[store(returns = Option<ActiveEraInfo>)]
    /// Marker for the runtime
    pub _runtime: PhantomData<T>,
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

/// Claim a payout for a validator’s stakers
#[derive(PartialEq, Eq, Clone, Call, Encode, Decode, Debug)]
pub struct PayoutStakersCall<'a, T: Staking> {
    /// Stash account of the validator
    pub validator_stash: &'a T::AccountId,
    /// Era index
    pub era: EraIndex,
}
