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

//! Implements support for the frame_staking module.

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
    fmt::Debug,
    marker::PhantomData,
};

pub use pallet_staking::{
    ActiveEraInfo,
    EraIndex,
    EraRewardPoints,
    Exposure,
    Nominations,
    RewardDestination,
    RewardPoint,
    StakingLedger,
    ValidatorPrefs,
};

/// Similar to `ErasStakers`, this holds the preferences of validators.
///
/// This is keyed first by the era index to allow bulk deletion and then the stash account.
///
/// Is it removed after `HISTORY_DEPTH` eras.
#[derive(Encode, Decode, Debug, Store)]
pub struct ErasValidatorPrefsStore<T: Staking> {
    #[store(returns = ValidatorPrefs)]
    /// Era index
    index: EraIndex,
    /// Account ID
    account_id: T::AccountId,
}

/// Rewards for the last `HISTORY_DEPTH` eras.
/// If reward hasn't been set or has been removed then 0 reward is returned.
#[derive(Clone, Encode, Decode, Debug, Store)]
pub struct ErasRewardPoints<T: Staking> {
    #[store(returns = EraRewardPoints<T::AccountId>)]
    index: EraIndex,
    _phantom: PhantomData<T>,
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

    /// Maximum number of validators that can be stored in a snapshot.
    const MAX_VALIDATORS: usize;

    /// Maximum number of nominators that can be stored in a snapshot.
    const MAX_NOMINATORS: usize;
}

/// Just a Balance/BlockNumber tuple to encode when a chunk of funds will be unlocked.
#[derive(Clone, Encode, Decode, Debug)]
pub struct UnlockChunk<T: Staking> {
    /// Amount of funds to be unlocked.
    #[codec(compact)]
    pub value: T::Balance,
    /// Era number at which point it'll be unlocked.
    #[codec(compact)]
    pub era: EraIndex,
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

/// The ideal number of staking participants.
#[derive(Encode, Decode, Copy, Clone, Debug, Store)]
pub struct ValidatorCountStore<T: Staking> {
    #[store(returns = u32)]
    /// Marker for the runtime
    pub _runtime: PhantomData<T>,
}

/// Minimum number of staking participants before emergency conditions are imposed.
#[derive(
    Encode, Decode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Store,
)]
pub struct MinimumValidatorCountStore<T: Staking> {
    #[store(returns = u32)]
    /// Marker for the runtime
    pub _runtime: PhantomData<T>,
}

/// Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
/// easy to initialize and the performance hit is minimal (we expect no more than four
/// invulnerables) and restricted to testnets.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Store)]
pub struct InvulnerablesStore<T: Staking> {
    #[store(returns = Vec<T::AccountId>)]
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

/// Clipped Exposure of validator at era.
///
/// This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the
/// `T::MaxNominatorRewardedPerValidator` biggest stakers.
/// (Note: the field `total` and `own` of the exposure remains unchanged).
/// This is used to limit the i/o cost for the nominator payout.
///
/// This is keyed fist by the era index to allow bulk deletion and then the stash account.
///
/// Is it removed after `HISTORY_DEPTH` eras.
/// If stakers hasn't been set or has been removed then empty exposure is returned.
#[derive(Encode, Copy, Clone, Debug, Store)]
pub struct ErasStakersClippedStore<T: Staking> {
    #[store(returns = Exposure<T::AccountId, T::Balance>)]
    era: EraIndex,
    validator_stash: T::AccountId,
}

/// The active era information, it holds index and start.
///
/// The active era is the era currently rewarded.
/// Validator set of this era must be equal to `SessionInterface::validators`.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Store)]
pub struct ActiveEraStore<T: Staking> {
    #[store(returns = Option<ActiveEraInfo>)]
    /// Marker for the runtime
    pub _runtime: PhantomData<T>,
}

/// Declare the desire to validate for the origin controller.
///
/// Effects will be felt at the beginning of the next era.
///
/// The dispatch origin for this call must be _Signed_ by the controller, not the stash.
/// And, it can be only called when [`EraElectionStatus`] is `Closed`.
///
/// # <weight>
/// - Independent of the arguments. Insignificant complexity.
/// - Contains a limited number of reads.
/// - Writes are limited to the `origin` account key.
/// -----------
/// Base Weight: 17.13 µs
/// DB Weight:
/// - Read: Era Election Status, Ledger
/// - Write: Nominators, Validators
/// # </weight>
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ValidateCall<T: Staking> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Validation preferences.
    pub prefs: ValidatorPrefs,
}

/// Declare the desire to nominate `targets` for the origin controller.
///
/// Effects will be felt at the beginning of the next era. This can only be called when
/// [`EraElectionStatus`] is `Closed`.
///
/// The dispatch origin for this call must be _Signed_ by the controller, not the stash.
/// And, it can be only called when [`EraElectionStatus`] is `Closed`.
///
/// # <weight>
/// - The transaction's complexity is proportional to the size of `targets` (N)
/// which is capped at CompactAssignments::LIMIT (MAX_NOMINATIONS).
/// - Both the reads and writes follow a similar pattern.
/// ---------
/// Base Weight: 22.34 + .36 * N µs
/// where N is the number of targets
/// DB Weight:
/// - Reads: Era Election Status, Ledger, Current Era
/// - Writes: Validators, Nominators
/// # </weight>
#[derive(Call, Encode, Debug)]
pub struct NominateCall<T: Staking> {
    /// The targets that are being nominated
    pub targets: Vec<T::Address>,
}

/// Claim a payout.
#[derive(PartialEq, Eq, Clone, Call, Encode, Decode, Debug)]
struct PayoutStakersCall<'a, T: Staking> {
    pub validator_stash: &'a T::AccountId,
    pub era: EraIndex,
}
