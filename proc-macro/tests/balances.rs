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

#[macro_use]
extern crate substrate_subxt;

use codec::{
    Codec,
    Decode,
    Encode,
};
use sp_keyring::AccountKeyring;
use std::fmt::Debug;
use substrate_subxt::{
    sp_runtime::traits::{
        AtLeast32Bit,
        MaybeSerialize,
        Member,
    },
    system::{
        System,
        SystemEventsDecoder,
    },
    ClientBuilder,
    KusamaRuntime,
    PairSigner,
};

#[module]
pub trait Balances: System {
    type Balance: Member
        + AtLeast32Bit
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + Debug
        + From<<Self as System>::BlockNumber>;
}

#[derive(Clone, Decode, Default)]
pub struct AccountData<Balance> {
    pub free: Balance,
    pub reserved: Balance,
    pub misc_frozen: Balance,
    pub fee_frozen: Balance,
}

#[derive(Encode, Store)]
pub struct AccountStore<'a, T: Balances> {
    #[store(returns = AccountData<T::Balance>)]
    pub account_id: &'a <T as System>::AccountId,
}

#[derive(Call, Encode)]
pub struct TransferCall<'a, T: Balances> {
    pub to: &'a <T as System>::Address,
    #[codec(compact)]
    pub amount: T::Balance,
}

#[derive(Debug, Decode, Eq, Event, PartialEq)]
pub struct TransferEvent<T: Balances> {
    pub from: <T as System>::AccountId,
    pub to: <T as System>::AccountId,
    pub amount: T::Balance,
}

impl Balances for KusamaRuntime {
    type Balance = u128;
}

subxt_test!({
    name: transfer_test_case,
    runtime: KusamaRuntime,
    account: Alice,
    step: {
        state: {
            alice: &AccountStore { account_id: &alice },
            bob: &AccountStore { account_id: &bob },
        },
        call: TransferCall {
            to: &bob,
            amount: 10_000,
        },
        event: TransferEvent {
            from: alice.clone(),
            to: bob.clone(),
            amount: 10_000,
        },
        assert: {
            assert_eq!(pre.alice.free, post.alice.free - 10_000);
            assert_eq!(pre.bob.free, post.bob.free + 10_000);
        },
    },
});

#[async_std::test]
#[ignore]
async fn transfer_balance_example() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let client = ClientBuilder::<KusamaRuntime>::new().build().await?;
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let alice = AccountKeyring::Alice.to_account_id();
    let bob = AccountKeyring::Bob.to_account_id();

    let alice_account = client.account(&alice, None).await?;
    let bob_account = client.account(&bob, None).await?;
    let pre = (alice_account, bob_account);

    let _hash = client
        .transfer(&signer, &bob.clone().into(), 10_000)
        .await?;

    let result = client
        .transfer_and_watch(&signer, &bob.clone().into(), 10_000)
        .await?;

    assert_eq!(
        result.transfer()?,
        Some(TransferEvent {
            from: alice.clone(),
            to: bob.clone(),
            amount: 10_000,
        })
    );

    let alice_account = client.account(&alice, None).await?;
    let bob_account = client.account(&bob, None).await?;
    let post = (alice_account, bob_account);

    assert_eq!(pre.0.free, post.0.free - 10_000);
    assert_eq!(pre.1.free, post.1.free + 10_000);
    Ok(())
}
