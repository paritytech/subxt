// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of subxt.
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
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
    node_runtime::{
        runtime_types,
        sudo,
        DefaultConfig,
    },
    test_context,
};
use assert_matches::assert_matches;
use sp_keyring::AccountKeyring;
use subxt::extrinsic::PairSigner;

// todo: [AJ] supply alias for top level call types? runtime_types::node_runtime::Call
type Call = runtime_types::node_runtime::Call;
type BalancesCall = runtime_types::pallet_balances::pallet::Call;

#[async_std::test]
async fn test_sudo() {
    let alice = PairSigner::<DefaultConfig, _>::new(AccountKeyring::Alice.pair());
    let bob = AccountKeyring::Bob.to_account_id().clone().into();
    let cxt = test_context().await;

    // todo: [AJ] allow encoded call to be constructed dynamically
    let call = Call::Balances(BalancesCall::transfer {
        dest: bob,
        value: 10_000,
    });

    let res = cxt
        .api
        .tx()
        .sudo()
        .sudo(call)
        .sign_and_submit_then_watch(&alice)
        .await
        .unwrap();
    let sudid = res.find_event::<sudo::events::Sudid>();
    assert_matches!(sudid, Ok(Some(_)))
}

#[async_std::test]
async fn test_sudo_unchecked_weight() {
    let alice = PairSigner::<DefaultConfig, _>::new(AccountKeyring::Alice.pair());
    let bob = AccountKeyring::Bob.to_account_id().into();
    let cxt = test_context().await;

    let call = Call::Balances(BalancesCall::transfer {
        dest: bob,
        value: 10_000,
    });

    let res = cxt
        .api
        .tx()
        .sudo()
        .sudo_unchecked_weight(call, 0)
        .sign_and_submit_then_watch(&alice)
        .await
        .unwrap();

    let sudid = res.find_event::<sudo::events::Sudid>();
    assert_matches!(sudid, Ok(Some(_)))
}
