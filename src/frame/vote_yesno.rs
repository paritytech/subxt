//! Implements support for the vote_yesno module

use crate::frame::{system::System, Call};
use codec::Codec;
use frame_support::Parameter;
use sp_runtime::{
    traits::{AtLeast32Bit, MaybeSerializeDeserialize, Member, Zero},
    Permill,
};
use std::fmt::Debug;
use util::{
    traits::{GroupMembership, LockableProfile, ReservableProfile, ShareBank, ShareRegistration},
    voteyesno::VoterYesNoView,
};

/// The subset of the `vote_yesno::Trait` that a client must implement.
pub trait VoteYesNo: System {
    /// The identifier for each vote; ProposalId => Vec<VoteId> s.t. sum(VoteId::Outcomes) => ProposalId::Outcome
    type VoteId: Parameter
        + Member
        + AtLeast32Bit
        + Codec
        + Default
        + Copy
        + MaybeSerializeDeserialize
        + Debug;

    /// The native type for vote strength
    type Signal: Parameter
        + Member
        + AtLeast32Bit
        + Codec
        + Default
        + Copy
        + MaybeSerializeDeserialize
        + Debug
        + Zero;

    /// An instance of the shares module
    type ShareData: GroupMembership<Self::AccountId>
        + ShareRegistration<Self::AccountId>
        + ReservableProfile<Self::AccountId>
        + LockableProfile<Self::AccountId>
        + ShareBank<Self::AccountId>;
}

/// The share identifier type
pub type ShareId<T> =
    <<T as VoteYesNo>::ShareData as ShareRegistration<<T as System>::AccountId>>::ShareId;

/// The organization identifier type
pub type OrgId<T> =
    <<T as VoteYesNo>::ShareData as ShareRegistration<<T as System>::AccountId>>::OrgId;

const MODULE: &str = "VoteYesNo";
const CREATE_SHARE_WEIGHTED_PERCENTAGE_VOTE_NO_EXPIRY: &str =
    "create_share_weighted_percentage_threshold_vote";
const CREATE_SHARE_WEIGHTED_COUNT_VOTE_NO_EXPIRY: &str =
    "create_share_weighted_count_threshold_vote";
const CREATE_1P1V_COUNT_VOTE_NO_EXPIRY: &str = "create_1p1v_count_threshold_vote";
const SUBMIT_VOTE: &str = "submit_vote";

/// Arguments for creating a share weighted vote with thresholds based on percents
#[derive(codec::Encode)]
pub struct CreateShareWeightedPercentageVoteArgs<T: VoteYesNo> {
    organization: OrgId<T>,
    share_id: ShareId<T>,
    passage_threshold_pct: Permill,
    turnout_threshold_pct: Permill,
}

/// Arguments for creating a share weighted vote with thresholds based on signal amounts
#[derive(codec::Encode)]
pub struct CreateShareWeightedCountVoteArgs<T: VoteYesNo> {
    organization: OrgId<T>,
    share_id: ShareId<T>,
    support_requirement: T::Signal,
    turnout_requirement: T::Signal,
}

/// Arguments for creating a 1p1v vote with thresholds based on signal amounts
#[derive(codec::Encode)]
pub struct Create1P1VCountVoteArgs<T: VoteYesNo> {
    organization: OrgId<T>,
    share_id: ShareId<T>,
    support_requirement: T::Signal,
    turnout_requirement: T::Signal,
}

/// Arguments for submitting a vote
#[derive(codec::Encode)]
pub struct SubmitVoteArgs<T: VoteYesNo> {
    organization: OrgId<T>,
    share_id: ShareId<T>,
    vote_id: T::Signal,
    voter: <T as System>::AccountId,
    direction: VoterYesNoView,
    magnitude: Option<T::Signal>,
}

/// Create share weighted percentage threshold vote in the context of an organizational share group
pub fn create_share_weighted_percentage_vote<T: VoteYesNo>(
    organization: OrgId<T>,
    share_id: ShareId<T>,
    passage_threshold_pct: Permill,
    turnout_threshold_pct: Permill,
) -> Call<CreateShareWeightedPercentageVoteArgs<T>> {
    Call::new(
        MODULE,
        CREATE_SHARE_WEIGHTED_PERCENTAGE_VOTE_NO_EXPIRY,
        CreateShareWeightedPercentageVoteArgs {
            organization,
            share_id,
            passage_threshold_pct,
            turnout_threshold_pct,
        },
    )
}

/// Create share weighted count threshold vote in the context of an organizational share group
pub fn create_share_weighted_count_vote<T: VoteYesNo>(
    organization: OrgId<T>,
    share_id: ShareId<T>,
    support_requirement: T::Signal,
    turnout_requirement: T::Signal,
) -> Call<CreateShareWeightedCountVoteArgs<T>> {
    Call::new(
        MODULE,
        CREATE_SHARE_WEIGHTED_COUNT_VOTE_NO_EXPIRY,
        CreateShareWeightedCountVoteArgs {
            organization,
            share_id,
            support_requirement,
            turnout_requirement,
        },
    )
}

/// Create 1 account 1 vote count threshold vote in the context of an organizational share group
pub fn create_1p1v_count_vote<T: VoteYesNo>(
    organization: OrgId<T>,
    share_id: ShareId<T>,
    support_requirement: T::Signal,
    turnout_requirement: T::Signal,
) -> Call<Create1P1VCountVoteArgs<T>> {
    Call::new(
        MODULE,
        CREATE_1P1V_COUNT_VOTE_NO_EXPIRY,
        Create1P1VCountVoteArgs {
            organization,
            share_id,
            support_requirement,
            turnout_requirement,
        },
    )
}

/// Submits a vote
pub fn submit_vote<T: VoteYesNo>(
    organization: OrgId<T>,
    share_id: ShareId<T>,
    vote_id: T::Signal,
    voter: <T as System>::AccountId,
    direction: VoterYesNoView,
    magnitude: Option<T::Signal>,
) -> Call<SubmitVoteArgs<T>> {
    Call::new(
        MODULE,
        SUBMIT_VOTE,
        SubmitVoteArgs {
            organization,
            share_id,
            vote_id,
            voter,
            direction,
            magnitude,
        },
    )
}
