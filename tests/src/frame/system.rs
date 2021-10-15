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
    node_runtime::runtime_types::frame_system::pallet::Call,
    node_runtime::runtime_types::node_runtime::Call::System, node_runtime::system,
    node_runtime::utility, test_context, TestRuntime,
};
use assert_matches::assert_matches;
use sp_keyring::AccountKeyring;
use subxt::extrinsic::{PairSigner, Signer};

#[async_std::test]
async fn storage_account() {
    let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());

    let cxt = test_context().await;
    let account_info = cxt
        .api
        .storage()
        .system()
        .account(alice.account_id().clone().into(), None)
        .await;
    assert_matches!(account_info, Ok(_))
}

#[async_std::test]
async fn tx_remark_with_event() {
    let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
    let cxt = test_context().await;

    let result = cxt
        .api
        .tx()
        .system()
        .remark_with_event(b"remarkable".to_vec())
        .sign_and_submit_then_watch(&alice)
        .await
        .unwrap();

    let remarked = result.find_event::<system::events::Remarked>();
    assert_matches!(remarked, Ok(Some(_)));
}
