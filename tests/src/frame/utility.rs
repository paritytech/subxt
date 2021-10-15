// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

use crate::{
    node_runtime::runtime_types,
    // node_runtime::runtime_types::frame_system::pallet::Call,
    // node_runtime::runtime_types::node_runtime::Call::System, node_runtime::system,
    node_runtime::utility, test_context, TestRuntime,
};
use assert_matches::assert_matches;
use sp_keyring::AccountKeyring;
use subxt::extrinsic::{PairSigner, Signer};

type Call = runtime_types::node_runtime::Call;
type SystemCall = runtime_types::frame_system::pallet::Call;
type BalancesCall = runtime_types::pallet_balances::pallet::Call;

#[async_std::test]
async fn tx_batch_remarks() {
    let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
    let cxt = test_context().await;

    let call_a = Call::System(SystemCall::remark {
        remark: b"cool remark".to_vec(),
    });

    let call_b = Call::System(SystemCall::remark {
        remark: b"awesome remark".to_vec(),
    });

    let result = cxt
        .api
        .tx()
        .utility()
        .batch(vec![call_a, call_b])
        .sign_and_submit_then_watch(&alice)
        .await
        .unwrap();

    let batch_completed = result.find_event::<utility::events::BatchCompleted>();
    assert_matches!(batch_completed, Ok(Some(_)));
}

#[async_std::test]
async fn tx_batch_transfers() {
    let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
    let bob = PairSigner::<TestRuntime, _>::new(AccountKeyring::Bob.pair());
    let bob_address = bob.account_id().clone().into();
    let cxt = test_context().await;

    let call = Call::Balances(BalancesCall::transfer {
        dest: bob_address,
        value: 10_000,
    });

    let result = cxt
        .api
        .tx()
        .utility()
        .batch(vec![call])
        .sign_and_submit_then_watch(&alice)
        .await
        .unwrap();
    
    // todo: test fails with error Rpc(RestartNeeded("Custom error: Unparsable response"))

    let batch_completed = result.find_event::<utility::events::BatchCompleted>();
    assert_matches!(batch_completed, Ok(Some(_)));

    
}