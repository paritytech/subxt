// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{
        self,
        runtime_types::{self, sp_weights::weight_v2::Weight},
        sudo,
    },
    pair_signer, test_context,
};
use sp_keyring::AccountKeyring;

type Call = runtime_types::kitchensink_runtime::RuntimeCall;
type BalancesCall = runtime_types::pallet_balances::pallet::Call;

#[tokio::test]
async fn test_sudo() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = pair_signer(AccountKeyring::Alice.pair());
    let bob = AccountKeyring::Bob.to_account_id().into();

    let call = Call::Balances(BalancesCall::transfer {
        dest: bob,
        value: 10_000,
    });
    let tx = node_runtime::tx().sudo().sudo(call);

    let found_event = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<sudo::events::Sudid>()?;

    assert!(found_event);
    Ok(())
}

#[tokio::test]
async fn test_sudo_unchecked_weight() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = pair_signer(AccountKeyring::Alice.pair());
    let bob = AccountKeyring::Bob.to_account_id().into();

    let call = Call::Balances(BalancesCall::transfer {
        dest: bob,
        value: 10_000,
    });
    let tx = node_runtime::tx().sudo().sudo_unchecked_weight(
        call,
        Weight {
            ref_time: 0,
            proof_size: 0,
        },
    );

    let found_event = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &alice)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<sudo::events::Sudid>()?;

    assert!(found_event);
    Ok(())
}
