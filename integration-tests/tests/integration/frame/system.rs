// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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
        system,
        DispatchError,
    },
    pair_signer,
    test_context,
};
use assert_matches::assert_matches;
use sp_keyring::AccountKeyring;

#[tokio::test]
async fn storage_account() -> Result<(), subxt::Error<DispatchError>> {
    let alice = pair_signer(AccountKeyring::Alice.pair());

    let cxt = test_context().await;
    let account_info = cxt
        .api
        .storage()
        .system()
        .account(alice.account_id(), None)
        .await;

    assert_matches!(account_info, Ok(_));
    Ok(())
}

#[tokio::test]
async fn tx_remark_with_event() -> Result<(), subxt::Error<DispatchError>> {
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let cxt = test_context().await;

    let found_event = cxt
        .api
        .tx()
        .system()
        .remark_with_event(b"remarkable".to_vec())?
        .sign_and_submit_then_watch_default(&alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<system::events::Remarked>()?;

    assert!(found_event);
    Ok(())
}
