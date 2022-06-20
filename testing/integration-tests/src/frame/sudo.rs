// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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
