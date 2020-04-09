//! Implements support for the vote_yesno module

use crate::frame::{system::System, Call};
use codec::Codec;
use frame_support::Parameter;
use sp_runtime::{
    traits::{AtLeast32Bit, MaybeSerializeDeserialize, Member},
    Permill,
};
use std::fmt::Debug;
use util::traits::{
    GroupMembership, LockableProfile, ReservableProfile, ShareBank, ShareRegistration,
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

const MODULE: &str = "Balances";
const CREATEVOTE: &str = "create_vote";

/// Arguments for creating a vote
#[derive(codec::Encode)]
pub struct CreateVoteArgs<T: VoteYesNo> {
    org: OrgId<T>,
    share: ShareId<T>,
    support_required: Permill,
    turnout_required: Permill,
}

/// Create some vote in the context of an organizational share group
pub fn create_vote<T: VoteYesNo>(
    org: OrgId<T>,
    share: ShareId<T>,
    support_required: Permill,
    turnout_required: Permill,
) -> Call<CreateVoteArgs<T>> {
    Call::new(
        MODULE,
        CREATEVOTE,
        CreateVoteArgs {
            org,
            share,
            support_required,
            turnout_required,
        },
    )
}
