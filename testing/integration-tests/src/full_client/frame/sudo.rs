// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{
        self,
        runtime_types::{self, sp_weights::weight_v2::Weight},
        sudo,
    },
    submit_tx_wait_for_finalized_success, subxt_test, test_context,
};
use subxt_signer::sr25519::dev;

type Call = runtime_types::kitchensink_runtime::RuntimeCall;
type BalancesCall = runtime_types::pallet_balances::pallet::Call;

#[subxt_test(timeout = 800)]
async fn test_sudo() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob().public_key().into();

    let call = Call::Balances(BalancesCall::transfer_allow_death {
        dest: bob,
        value: 10_000,
    });
    let tx = node_runtime::tx().sudo().sudo(call);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await?;

    let found_event = submit_tx_wait_for_finalized_success(&signed_extrinsic)
        .await?
        .has::<sudo::events::Sudid>()?;

    assert!(found_event);
    Ok(())
}

#[subxt_test(timeout = 800)]
async fn test_sudo_unchecked_weight() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob().public_key().into();

    let call = Call::Balances(BalancesCall::transfer_allow_death {
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

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await?;

    let found_event = submit_tx_wait_for_finalized_success(&signed_extrinsic)
        .await?
        .has::<sudo::events::Sudid>()?;

    assert!(found_event);
    Ok(())
}
