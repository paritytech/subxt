use crate::frame::{system::System, Call};
use codec::Codec;
use frame_support::Parameter;
use sp_runtime::traits::{AtLeast32Bit, MaybeSerializeDeserialize, Member, Zero};
use std::fmt::Debug;

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
const RESERVE: &str = "reserve_shares";

/// Arguments for requesting a share reservation
#[derive(codec::Encode)]
pub struct ReserveArgs<T: SharesAtomic> {
    org: T::OrgId,
    share: T::ShareId,
    account: <T as System>::AccountId,
}

/// Request the share reservation
pub fn reserve_shares<T: SharesAtomic>(
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
