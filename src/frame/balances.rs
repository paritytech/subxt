// Copyright 2019 Parity Technologies (UK) Ltd.
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

//! Implements support for the pallet_balances module.

use std::fmt::Debug;

use futures::future::{
    self,
    Future,
};

use frame_support::Parameter;
use sp_runtime::traits::{
    MaybeSerialize,
    Member,
    SimpleArithmetic,
};

use crate::{
    error::Error,
    frame::{
        system::System,
        Call,
    },
    Client,
};

/// The subset of the `pallet_balances::Trait` that a client must implement.
pub trait Balances: System {
    /// The balance of an account.
    type Balance: Parameter
        + Member
        + SimpleArithmetic
        + codec::Codec
        + Default
        + Copy
        + MaybeSerialize
        + Debug
        + From<<Self as System>::BlockNumber>;
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

/// Arguments for transferring a balance
#[derive(codec::Encode)]
pub struct TransferArgs<T: Balances> {
    to: <T as System>::Address,
    #[codec(compact)]
    amount: <T as Balances>::Balance,
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
