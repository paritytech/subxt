// Copyright 2020 Parity Technologies (UK) Ltd.
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

use codec::{Codec, Decode, Encode, HasCompact};
use frame_support::Parameter;
use serde::de::DeserializeOwned;
use sp_core::storage::StorageKey;
use sp_runtime::{
    traits::{
        AtLeast32Bit, Bounded, CheckEqual, Extrinsic, Hash, Header, MaybeDisplay,
        MaybeMallocSizeOf, MaybeSerialize, MaybeSerializeDeserialize, Member,
        SimpleBitOps,
    },
    RuntimeDebug,
};
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{
    frame::{Call, Store},
    metadata::{Metadata, MetadataError},
};

/// Data type used to index nominators in the compact type
pub type NominatorIndex = u32;

/// Data type used to index validators in the compact type.
pub type ValidatorIndex = u16;

/// Maximum number of stakers that can be stored in a snapshot.
pub(crate) const MAX_VALIDATORS: usize = ValidatorIndex::max_value() as usize;
pub(crate) const MAX_NOMINATORS: usize = NominatorIndex::max_value() as usize;

/// Counter for the number of eras that have passed.
pub type EraIndex = u32;

/// Counter for the number of "reward" points earned by a given validator.
pub type RewardPoint = u32;


/// The subset of the `frame::Trait` that a client must implement.
pub trait Staking: super::system::System {
	/*
	type UnixTime;
	type CurrencyToVote;
	type RewardRemainder;
	type Event;
	type Slash;
	type Reward;
	type SessionsPerEra;
	type BondingDuration;
	type SlashDeferDuration;
	type SlashCancelOrigin;
	type SessionInterface;
	type RewardCurve;
	type NextNewSession;
	type ElectionLookahead;
	type Call;
	type MaxIterations;
	type MaxNominatorRewardPerValidator;
	type UnsignedPriority; */
}

/// Just a Balance/BlockNumber tuple to encode when a chunk of funds will be unlocked.
#[derive(PartialEq, Eq, Clone, Encode, Decode)]
pub struct UnlockChunk<Balance: HasCompact> {
	/// Amount of funds to be unlocked.
	#[codec(compact)]
	value: Balance,
	/// Era number at which point it'll be unlocked.
	#[codec(compact)]
	era: EraIndex,
}

/// The ledger of a (bonded) stash.
#[derive(PartialEq, Eq, Clone, Encode, Decode)]
pub struct StakingLedger<AccountId, Balance: HasCompact> {
	/// The stash account whose balance is actually locked and at stake.
	pub stash: AccountId,
	/// The total amount of the stash's balance that we are currently accounting for.
	/// It's just `active` plus all the `unlocking` balances.
	#[codec(compact)]
	pub total: Balance,
	/// The total amount of the stash's balance that will be at stake in any forthcoming
	/// rounds.
	#[codec(compact)]
	pub active: Balance,
	/// Any balance that is becoming free, which may eventually be transferred out
	/// of the stash (assuming it doesn't get slashed first).
	pub unlocking: Vec<UnlockChunk<Balance>>,
	/// List of eras for which the stakers behind a validator have claimed rewards. Only updated
	/// for validators.
	pub claimed_rewards: Vec<EraIndex>,
}

const MODULE: &str = "Staking";

/// Number of eras to keep in history.
///
/// Information is kept for eras in `[current_era - history_depth; current_era]`.
///
/// Must be more than the number of eras delayed by session otherwise.
/// I.e. active era must always be in history.
/// I.e. `active_era > current_era - history_depth` must be guaranteed.
#[derive(Encode, Decode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HistoryDepth<T: Staking>(PhantomData<T>);

impl<T: Staking> Store<T> for HistoryDepth<T> {
    const MODULE: &'static str = MODULE;
    const FIELD: &'static str = "HistoryDepth";
    type Returns = u32;

    fn key(&self, metadata: &Metadata) -> Result<StorageKey, MetadataError> {
        Ok(metadata.module(Self::MODULE)?.storage(Self::FIELD)?.plain()?.key())
    }
}

/// The ideal number of staking participants.
#[derive(Encode, Decode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ValidatorCount<T: Staking>(PhantomData<T>);

impl<T: Staking> Store<T> for ValidatorCount<T> {
    const MODULE: &'static str = MODULE;
    const FIELD: &'static str = "ValidatorCount";
    type Returns = u32;

    fn key(&self, metadata: &Metadata) -> Result<StorageKey, MetadataError> {
        Ok(metadata.module(Self::MODULE)?.storage(Self::FIELD)?.plain()?.key())
    }
}

/// Minimum number of staking participants before emergency conditions are imposed.
#[derive(Encode, Decode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MinimumValidatorCount<T: Staking>(PhantomData<T>);

impl<T: Staking> Store<T> for MinimumValidatorCount<T> {
    const MODULE: &'static str = MODULE;
    const FIELD: &'static str = "MinimumValidatorCount";
    type Returns = u32;

    fn key(&self, metadata: &Metadata) -> Result<StorageKey, MetadataError> {
        Ok(metadata.module(Self::MODULE)?.storage(Self::FIELD)?.plain()?.key())
    }
}

/// Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
/// easy to initialize and the performance hit is minimal (we expect no more than four
/// invulnerables) and restricted to testnets.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Invulnerables<T: Staking>(pub core::marker::PhantomData<T>);

impl<T: Staking> Store<T> for Invulnerables<T> {
    const MODULE: &'static str = MODULE;
    const FIELD: &'static str = "Invulnerables";
    type Returns = Vec<T::AccountId>;

    fn key(&self, metadata: &Metadata) -> Result<StorageKey, MetadataError> {
        Ok(metadata.module(Self::MODULE)?.storage(Self::FIELD)?.plain()?.key())
    }
}

/// Map from all locked "stash" accounts to the controller account.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Bonded<T: Staking>(pub PhantomData<T>);

impl<T: Staking> Store<T> for Bonded<T> {
    const MODULE: &'static str = MODULE;
    const FIELD: &'static str = "Bonded";
    type Returns = Vec<T::AccountId>;

    fn key(&self, metadata: &Metadata) -> Result<StorageKey, MetadataError> {
        Ok(metadata.module(Self::MODULE)?.storage(Self::FIELD)?.map()?.key(&self.0))
    }
}

/// Map from all (unlocked) "controller" accounts to the info regarding the staking.
#[derive(Encode, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Ledger<T: Staking>(pub T::AccountId);

impl<T: Staking> Store<T> for Ledger<T> {
    const MODULE: &'static str = MODULE;
    const FIELD: &'static str = "Ledger";
    type Returns = Option<StakingLedger<T::AccountId, ()>>;

    fn key(&self, metadata: &Metadata) -> Result<StorageKey, MetadataError> {
        Ok(metadata.module(Self::MODULE)?.storage(Self::FIELD)?.map()?.key(&self.0))
    }
}
