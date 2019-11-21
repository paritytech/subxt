//! Implements support for the pallet_balances module.
use crate::{
    error::Error,
    palette::{
        Call,
        system::System,
    },
    Client,
};
use futures::future::{
    self,
    Future,
};
use parity_scale_codec::{Encode, Codec};
use runtime_primitives::traits::{
    MaybeSerialize,
    Member,
    SimpleArithmetic,
};
use runtime_support::Parameter;
use std::fmt::Debug;

/// The subset of the `pallet_balances::Trait` that a client must implement.
pub trait Balances: System {
    /// The balance of an account.
    type Balance: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + Debug
        + From<<Self as System>::BlockNumber>;
}

/// Blanket impl for using existing runtime types
impl<T: palette_system::Trait + pallet_balances::Trait + Debug> Balances for T
where
    <T as palette_system::Trait>::Header: serde::de::DeserializeOwned,
{
    type Balance = T::Balance;
}

/// The Balances extension trait for the Client.
pub trait BalancesStore {
    /// Balances type.
    type Balances: Balances;

    /// The 'free' balance of a given account.
    ///
    /// This is the only balance that matters in terms of most operations on
    /// tokens. It alone is used to determine the balance when in the contract
    ///  execution environment. When this balance falls below the value of
    ///  `ExistentialDeposit`, then the 'current account' is deleted:
    ///  specifically `FreeBalance`. Further, the `OnFreeBalanceZero` callback
    /// is invoked, giving a chance to external modules to clean up data
    /// associated with the deleted account.
    ///
    /// `system::AccountNonce` is also deleted if `ReservedBalance` is also
    /// zero. It also gets collapsed to zero if it ever becomes less than
    /// `ExistentialDeposit`.
    fn free_balance(
        &self,
        account_id: <Self::Balances as System>::AccountId,
    ) -> Box<dyn Future<Item = <Self::Balances as Balances>::Balance, Error = Error> + Send>;
}

impl<T: Balances + 'static, S: 'static> BalancesStore for Client<T, S> {
    type Balances = T;

    fn free_balance(
        &self,
        account_id: <Self::Balances as System>::AccountId,
    ) -> Box<dyn Future<Item = <Self::Balances as Balances>::Balance, Error = Error> + Send>
    {
        let free_balance_map = || {
            Ok(self
                .metadata()
                .module("Balances")?
                .storage("FreeBalance")?
                .get_map::<
                <Self::Balances as System>::AccountId,
                <Self::Balances as Balances>::Balance>()?)
        };
        let map = match free_balance_map() {
            Ok(map) => map,
            Err(err) => return Box::new(future::err(err)),
        };
        Box::new(self.fetch_or(map.key(account_id), map.default()))
    }
}

const MODULE: &str = "Balances";
const TRANSFER: &str = "transfer";

#[derive(Encode)]
pub struct TransferArgs<T: Balances> {
    to: <T as System>::Address,
    #[codec(compact)]
    amount: <T as Balances>::Balance
}

/// Transfer some liquid free balance to another account.
///
/// `transfer` will set the `FreeBalance` of the sender and receiver.
/// It will decrease the total issuance of the system by the `TransferFee`.
/// If the sender's account is below the existential deposit as a result
/// of the transfer, the account will be reaped.
pub fn transfer<T: Balances>(
    to: <T as System>::Address,
    amount: <T as Balances>::Balance,
) -> Call<TransferArgs<T>> {
    Call::new(MODULE, TRANSFER, TransferArgs { to, amount })
}
