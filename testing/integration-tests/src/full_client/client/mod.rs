// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::collections::HashSet;

use crate::{
    subxt_test, test_context, test_context_reconnecting_rpc_client,
    utils::{node_runtime, wait_for_blocks},
};
use codec::{Decode, Encode};

#[cfg(fullclient)]
use futures::StreamExt;

use subxt::{
    backend::BackendExt,
    error::{DispatchError, Error},
    tx::{TransactionInvalid, ValidationResult},
};
use subxt_signer::sr25519::dev;

#[cfg(fullclient)]
mod legacy_rpcs;

mod unstable_rpcs;

#[cfg(fullclient)]
#[subxt_test]
async fn storage_fetch_raw_keys() {
    let ctx = test_context().await;
    let api = ctx.client();

    let addr = node_runtime::storage().system().account_iter();
    let len = api
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch_raw_keys(addr.to_root_bytes())
        .await
        .unwrap()
        .filter_map(|r| async move { r.ok() })
        .count()
        .await;

    assert_eq!(len, 16)
}

#[cfg(fullclient)]
#[subxt_test]
async fn storage_iter() {
    let ctx = test_context().await;
    let api = ctx.client();

    let addr = node_runtime::storage().system().account_iter();
    let addr_bytes = api.storage().address_bytes(&addr).unwrap();
    assert_eq!(addr_bytes, addr.to_root_bytes());

    let len = api
        .storage()
        .at_latest()
        .await
        .unwrap()
        .iter(addr)
        .await
        .unwrap()
        .filter_map(|r| async move { r.ok() })
        .count()
        .await;

    assert_eq!(len, 16);
}

#[cfg(fullclient)]
#[subxt_test]
async fn storage_child_values_same_across_backends() {
    let ctx = test_context().await;

    let chainhead_client = ctx.chainhead_backend().await;
    let legacy_client = ctx.legacy_backend().await;

    let addr = node_runtime::storage().system().account_iter();
    let block_ref = legacy_client
        .blocks()
        .at_latest()
        .await
        .unwrap()
        .reference();

    let a: Vec<_> = chainhead_client
        .storage()
        .at(block_ref.clone())
        .iter(addr.clone())
        .await
        .unwrap()
        .collect()
        .await;
    let b: Vec<_> = legacy_client
        .storage()
        .at(block_ref.clone())
        .iter(addr)
        .await
        .unwrap()
        .collect()
        .await;

    for (a, b) in a.into_iter().zip(b.into_iter()) {
        let a = a.unwrap();
        let b = b.unwrap();

        assert_eq!(a, b);
    }
}

#[subxt_test]
async fn transaction_validation() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob();

    wait_for_blocks(&api).await;

    let tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(bob.public_key().into(), 10_000);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await
        .unwrap();

    signed_extrinsic
        .validate()
        .await
        .expect("validation failed");

    signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
}

#[subxt_test]
async fn validation_fails() {
    use std::str::FromStr;
    use subxt_signer::{sr25519::Keypair, SecretUri};

    let ctx = test_context().await;
    let api = ctx.client();

    wait_for_blocks(&api).await;

    let from = Keypair::from_uri(&SecretUri::from_str("//AccountWithNoFunds").unwrap()).unwrap();
    let to = dev::bob();

    // The actual TX is not important; the account has no funds to pay for it.
    let tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(to.public_key().into(), 1);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &from, Default::default())
        .await
        .unwrap();

    let validation_res = signed_extrinsic
        .validate()
        .await
        .expect("dryrunning failed");
    assert_eq!(
        validation_res,
        ValidationResult::Invalid(TransactionInvalid::Payment)
    );
}

#[subxt_test]
async fn external_signing() {
    let ctx = test_context().await;
    let api = ctx.client();
    let alice = dev::alice();

    // Create a partial extrinsic. We can get the signer payload at this point, to be
    // signed externally.
    let tx = node_runtime::tx().preimage().note_preimage(vec![0u8]);
    let partial_extrinsic = api
        .tx()
        .create_partial_signed(&tx, &alice.public_key().into(), Default::default())
        .await
        .unwrap();

    // Get the signer payload.
    let signer_payload = partial_extrinsic.signer_payload();
    // Sign it (possibly externally).
    let signature = alice.sign(&signer_payload);
    // Use this to build a signed extrinsic.
    let extrinsic = partial_extrinsic
        .sign_with_address_and_signature(&alice.public_key().into(), &signature.into());

    // And now submit it.
    extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
}

#[cfg(fullclient)]
// TODO: Investigate and fix this test failure when using the ChainHeadBackend.
// (https://github.com/paritytech/subxt/issues/1308)
#[cfg(legacy_backend)]
#[subxt_test]
async fn submit_large_extrinsic() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();

    // 2 MiB blob of data.
    let bytes = vec![0_u8; 2 * 1024 * 1024];
    // The preimage pallet allows storing and managing large byte-blobs.
    let tx = node_runtime::tx().preimage().note_preimage(bytes);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await
        .unwrap();

    signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
}

#[subxt_test]
async fn decode_a_module_error() {
    use node_runtime::runtime_types::pallet_assets::pallet as assets;

    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let alice_addr = alice.public_key().into();

    // Trying to work with an asset ID 1 which doesn't exist should return an
    // "unknown" module error from the assets pallet.
    let freeze_unknown_asset = node_runtime::tx().assets().freeze(1, alice_addr);

    let signed_extrinsic = api
        .tx()
        .create_signed(&freeze_unknown_asset, &alice, Default::default())
        .await
        .unwrap();

    let err = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .expect_err("an 'unknown asset' error");

    let Error::Runtime(DispatchError::Module(module_err)) = err else {
        panic!("Expected a ModuleError, got {err:?}");
    };

    // Decode the error into our generated Error type.
    let decoded_err = module_err.as_root_error::<node_runtime::Error>().unwrap();

    // Decoding should result in an Assets.Unknown error:
    assert_eq!(
        decoded_err,
        node_runtime::Error::Assets(assets::Error::Unknown)
    );
}

#[subxt_test]
async fn unsigned_extrinsic_is_same_shape_as_polkadotjs() {
    let ctx = test_context().await;
    let api = ctx.client();

    let tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::alice().public_key().into(), 12345000000000000);

    let actual_tx = api.tx().create_unsigned(&tx).unwrap();

    let actual_tx_bytes = actual_tx.encoded();

    // How these were obtained:
    // - start local substrate node.
    // - open polkadot.js UI in browser and point at local node.
    // - open dev console (may need to refresh page now) and find the WS connection.
    // - create a balances.transferAllowDeath to ALICE (doesn't matter who from) with 12345 and "submit unsigned".
    // - find the submitAndWatchExtrinsic call in the WS connection to get these bytes:
    let expected_tx_bytes = hex::decode(
        "b004060000d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0f0090c04bb6db2b"
    )
        .unwrap();

    // Make sure our encoding is the same as the encoding polkadot UI created.
    assert_eq!(actual_tx_bytes, expected_tx_bytes);
}

#[subxt_test]
async fn extrinsic_hash_is_same_as_returned() {
    let ctx = test_context().await;
    let api = ctx.client();
    let rpc = ctx.legacy_rpc_methods().await;

    let payload = node_runtime::tx()
        .balances()
        .transfer_allow_death(dev::alice().public_key().into(), 12345000000000000);

    let tx = api
        .tx()
        .create_signed(&payload, &dev::bob(), Default::default())
        .await
        .unwrap();

    // 1. Calculate the hash locally:
    let local_hash = tx.hash();

    // 2. Submit and get the hash back from the node:
    let external_hash = rpc.author_submit_extrinsic(tx.encoded()).await.unwrap();

    assert_eq!(local_hash, external_hash);
}

/// taken from original type <https://docs.rs/pallet-transaction-payment/latest/pallet_transaction_payment/struct.FeeDetails.html>
#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq)]
pub struct FeeDetails {
    /// The minimum fee for a transaction to be included in a block.
    pub inclusion_fee: Option<InclusionFee>,
    /// tip
    pub tip: u128,
}

/// taken from original type <https://docs.rs/pallet-transaction-payment/latest/pallet_transaction_payment/struct.InclusionFee.html>
/// The base fee and adjusted weight and length fees constitute the _inclusion fee_.
#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq)]
pub struct InclusionFee {
    /// minimum amount a user pays for a transaction.
    pub base_fee: u128,
    /// amount paid for the encoded length (in bytes) of the transaction.
    pub len_fee: u128,
    ///
    /// - `targeted_fee_adjustment`: This is a multiplier that can tune the final fee based on the
    ///   congestion of the network.
    /// - `weight_fee`: This amount is computed based on the weight of the transaction. Weight
    ///   accounts for the execution time of a transaction.
    ///
    /// adjusted_weight_fee = targeted_fee_adjustment * weight_fee
    pub adjusted_weight_fee: u128,
}

#[subxt_test]
async fn partial_fee_estimate_correct() {
    let ctx = test_context().await;
    let api = ctx.client();

    let alice = dev::alice();
    let bob = dev::bob();
    let tx = node_runtime::tx()
        .balances()
        .transfer_allow_death(bob.public_key().into(), 1_000_000_000_000);

    let signed_extrinsic = api
        .tx()
        .create_signed(&tx, &alice, Default::default())
        .await
        .unwrap();

    // Method I: TransactionPaymentApi_query_info
    let partial_fee_1 = signed_extrinsic.partial_fee_estimate().await.unwrap();

    // Method II: TransactionPaymentApi_query_fee_details + calculations
    let latest_block_ref = api.backend().latest_finalized_block_ref().await.unwrap();
    let len_bytes: [u8; 4] = (signed_extrinsic.encoded().len() as u32).to_le_bytes();
    let encoded_with_len = [signed_extrinsic.encoded(), &len_bytes[..]].concat();
    let InclusionFee {
        base_fee,
        len_fee,
        adjusted_weight_fee,
    } = api
        .backend()
        .call_decoding::<FeeDetails>(
            "TransactionPaymentApi_query_fee_details",
            Some(&encoded_with_len),
            latest_block_ref.hash(),
        )
        .await
        .unwrap()
        .inclusion_fee
        .unwrap();
    let partial_fee_2 = base_fee + len_fee + adjusted_weight_fee;

    // Both methods should yield the same fee
    assert_eq!(partial_fee_1, partial_fee_2);
}

#[subxt_test]
async fn legacy_and_unstable_block_subscription_reconnect() {
    let ctx = test_context_reconnecting_rpc_client().await;
    let api = ctx.chainhead_backend().await;
    let chainhead_client_blocks = move |num: usize| {
        let api = api.clone();
        async move {
            let mut missed_blocks = false;

            let blocks =
            // Ignore `disconnected events`.
            // This will be emitted by the legacy backend for every reconnection.
            api.blocks().subscribe_finalized().await.unwrap().filter(|item| {
                let disconnected = match item {
                    Ok(_) => false,
                    Err(e) => {
                        if matches!(e, Error::Rpc(subxt::error::RpcError::DisconnectedWillReconnect(e)) if e.contains("Missed at least one block when the connection was lost")) {
                            missed_blocks = true;
                        }
                        e.is_disconnected_will_reconnect()
                    }
                };

                futures::future::ready(!disconnected)
            })
            .take(num)
            .map(|x| x.unwrap().hash().to_string())
            .collect::<Vec<String>>().await;

            (blocks, missed_blocks)
        }
    };

    let (blocks, _) = chainhead_client_blocks(3).await;
    let blocks: HashSet<String> = HashSet::from_iter(blocks.into_iter());

    assert!(blocks.len() == 3);

    let ctx = ctx.restart().await;

    // Make client aware that connection was dropped and force them to reconnect
    let _ = ctx.chainhead_backend().await.backend().genesis_hash().await;

    let (unstable_blocks, blocks_missed) = chainhead_client_blocks(6).await;

    if !blocks_missed {
        let unstable_blocks: HashSet<String> = HashSet::from_iter(unstable_blocks.into_iter());
        let intersection = unstable_blocks.intersection(&blocks).count();
        assert!(intersection >= 3, "intersections size is {}", intersection);
    }
}
