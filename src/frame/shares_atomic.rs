use crate::{
    error::Error,
    frame::{balances::Balances, Call},
    Client,
};
use codec::{Codec, Decode, Encode};
use frame_support::Parameter;
use sp_runtime::{
    traits::{AtLeast32Bit, MaybeSerializeDeserialize, Member, Zero},
    Permill,
};
use std::fmt::Debug;
use util::traits::{
    GroupMembership, LockableProfile, ReservableProfile, ShareBank, ShareRegistration,
};

/// The subset of the `shares_atomic::Trait` that a client must implement.
pub trait SharesAtomic: System {
    type OrgId: Parameter
        + Member
        + AtLeast32Bit
        + Codec
        + Default
        + Copy
        + MaybeSerializeDeserialize
        + Debug
        + Zero;

    type ShareId: Parameter
        + Member
        + AtLeast32Bit
        + Codec
        + Default
        + Copy
        + MaybeSerializeDeserialize
        + Debug;
}

const MODULE: &str = "SharesAtomic";
const RESERVE: &str = "reserve";

/// Arguments for requesting a share reservation
#[derive(codec::Encode)]
pub struct ReserveArgs<T: SharesAtomic> {
    org: T::OrgId,
    share: T::ShareId,
    account: <T as System>::AccountId,
}

/// Request the share reservation
pub fn reserve<T: SharesAtomic>(
    org: T::OrgId,
    share: T::ShareId,
    account: <T as System>::AccountId,
) -> Call<ReserveArgs<T>> {
    Call::new(
        MODULE,
        RESERVE,
        ReserveArgs {
            org,
            share,
            account,
        },
    )
}
