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
        runtime_types,
        sudo,
        DispatchError,
    },
    pair_signer,
    test_context,
};
use sp_keyring::AccountKeyring;

type Call = runtime_types::node_runtime::Call;
type BalancesCall = runtime_types::pallet_balances::pallet::Call;

#[tokio::test]
async fn test_sudo() -> Result<(), subxt::Error<DispatchError>> {
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let bob = AccountKeyring::Bob.to_account_id().into();
    let cxt = test_context().await;

    let call = Call::Balances(BalancesCall::transfer {
        dest: bob,
        value: 10_000,
    });

    let found_event = cxt
        .api
        .tx()
        .sudo()
        .sudo(call)?
        .sign_and_submit_then_watch_default(&alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<sudo::events::Sudid>()?;

    assert!(found_event);
    Ok(())
}

#[tokio::test]
async fn test_sudo_unchecked_weight() -> Result<(), subxt::Error<DispatchError>> {
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let bob = AccountKeyring::Bob.to_account_id().into();
    let cxt = test_context().await;

    let call = Call::Balances(BalancesCall::transfer {
        dest: bob,
        value: 10_000,
    });

    let found_event = cxt
        .api
        .tx()
        .sudo()
        .sudo_unchecked_weight(call, 0)?
        .sign_and_submit_then_watch_default(&alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<sudo::events::Sudid>()?;

    assert!(found_event);
    Ok(())
}
