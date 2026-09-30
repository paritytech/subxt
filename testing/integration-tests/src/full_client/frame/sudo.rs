// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{
        self,
        runtime_types::{self, sp_weights::weight_v2::Weight},
        sudo,
    },
    subxt_test, test_context,
};
use subxt_signer::sr25519::dev;

type Call = runtime_types::kitchensink_runtime::RuntimeCall;
type BalancesCall = runtime_types::pallet_balances::pallet::Call;

#[subxt_test]
async fn test_sudo_reports_wrapped_call_failure() -> Result<(), subxt::Error> {
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
        .await?
        .create_signed(&tx, &alice, Default::default())
        .await?;

    // The sudo extrinsic itself succeeds, but the wrapped call still needs to
    // be inspected via the `Sudid` event. `transfer_allow_death` expects a
    // signed origin, so under `Root` we should see the inner call fail.
    let sudo_event = signed_extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?
        .find_first::<sudo::events::Sudid>()?
        .expect("Expected sudo::events::Sudid");

    assert!(matches!(
        sudo_event.sudo_result,
        Err(runtime_types::sp_runtime::DispatchError::BadOrigin)
    ));
    Ok(())
}

#[subxt_test]
async fn test_sudo_reports_wrapped_call_success() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob().public_key().into();

    let call = Call::Balances(BalancesCall::force_set_balance {
        who: bob,
        new_free: 10_000,
    });
    let tx = node_runtime::tx().sudo().sudo(call);

    let signed_extrinsic = api
        .tx()
        .await?
        .create_signed(&tx, &alice, Default::default())
        .await?;

    let sudo_event = signed_extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?
        .find_first::<sudo::events::Sudid>()?
        .expect("Expected sudo::events::Sudid");

    assert!(sudo_event.sudo_result.is_ok());
    Ok(())
}

#[subxt_test]
async fn test_sudo_unchecked_weight() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob().public_key().into();

    let call = Call::Balances(BalancesCall::force_set_balance {
        who: bob,
        new_free: 10_000,
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
        .await?
        .create_signed(&tx, &alice, Default::default())
        .await?;

    let sudo_event = signed_extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?
        .find_first::<sudo::events::Sudid>()?
        .expect("Expected sudo::events::Sudid");

    assert!(sudo_event.sudo_result.is_ok());
    Ok(())
}
