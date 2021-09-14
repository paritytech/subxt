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

//! Implements support for the pallet_balances module.

use crate::{
    test_node_process, TestRuntime, node_runtime::{RuntimeApi, balances},
};
use codec::{
    Decode,
    Encode,
};
use core::marker::PhantomData;
use sp_runtime::traits::{
    AtLeast32Bit,
    MaybeSerialize,
    Member,
};
use std::fmt::Debug;

use subxt::{
    Error,
    ModuleError,
    RuntimeError,
    extrinsic::{
        PairSigner,
        Signer,
    },
    EventSubscription,
};
use sp_core::{
    sr25519::Pair,
    Pair as _,
};
use sp_keyring::AccountKeyring;

#[async_std::test]
async fn test_basic_transfer() {
    env_logger::try_init().ok();
    let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
    let bob = PairSigner::<TestRuntime, _>::new(AccountKeyring::Bob.pair());
    let bob_address = bob.account_id().clone().into();
    let test_node_proc = test_node_process().await;
    let client = test_node_proc.client();
    let api = crate::node_runtime::RuntimeApi::<TestRuntime>::new(client.clone());

    let alice_pre = api.storage.system.account(alice.account_id().clone().into(), None).await.unwrap();
    let bob_pre = api.storage.system.account(bob.account_id().clone().into(), None).await.unwrap();

    let extrinsic = api.tx.balances.transfer(&bob_address, 10_000).await.unwrap();
    let result = extrinsic.sign_and_submit_then_watch(&alice).await.unwrap();
    let event = result.find_event::<balances::events::Transfer>().unwrap().unwrap();
    let expected_event = balances::events::Transfer {
        from: alice.account_id().clone(),
        to: bob.account_id().clone(),
        amount: 10_000,
    };
    assert_eq!(event, expected_event);

    let alice_post = api.storage.system.account(alice.account_id().clone().into(), None).await.unwrap();
    let bob_post = api.storage.system.account(bob.account_id().clone().into(), None).await.unwrap();

    assert!(alice_pre.data.free - 10_000 >= alice_post.data.free);
    assert_eq!(bob_pre.data.free + 10_000, bob_post.data.free);
}

// #[async_std::test]
// async fn test_state_total_issuance() {
//     env_logger::try_init().ok();
//     let test_node_proc = test_node_process().await;
//     let client = test_node_proc.client();
//     let total_issuance = client.total_issuance(None).await.unwrap();
//     assert_ne!(total_issuance, 0);
// }
//
// #[async_std::test]
// async fn test_state_read_free_balance() {
//     env_logger::try_init().ok();
//     let test_node_proc = test_node_process().await;
//     let client = test_node_proc.client();
//     let account = AccountKeyring::Alice.to_account_id();
//     let info = client.account(&account, None).await.unwrap();
//     assert_ne!(info.data.free, 0);
// }
//
// #[async_std::test]
// async fn test_state_balance_lock() -> Result<(), crate::Error> {
//
//     env_logger::try_init().ok();
//     let bob = PairSigner::<TestRuntime, _>::new(AccountKeyring::Bob.pair());
//     let test_node_proc = test_node_process().await;
//     let client = test_node_proc.client();
//
//     client
//         .bond_and_watch(
//             &bob,
//             &AccountKeyring::Charlie.to_account_id().clone().into(),
//             100_000_000_000_000,
//             RewardDestination::Stash,
//         )
//         .await?;
//
//     let locks = client
//         .locks(&AccountKeyring::Bob.to_account_id(), None)
//         .await?;
//
//     assert_eq!(
//         locks,
//         vec![BalanceLock {
//             id: *b"staking ",
//             amount: 100_000_000_000_000,
//             reasons: Reasons::All,
//         }]
//     );
//
//     Ok(())
// }
//
// #[async_std::test]
// async fn test_transfer_error() {
//     env_logger::try_init().ok();
//     let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
//     let alice_addr = alice.account_id().clone().into();
//     let hans = PairSigner::<TestRuntime, _>::new(Pair::generate().0);
//     let hans_address = hans.account_id().clone().into();
//     let test_node_proc = test_node_process().await;
//     let client = test_node_proc.client();
//     client
//         .transfer_and_watch(&alice, &hans_address, 100_000_000_000_000_000)
//         .await
//         .unwrap();
//     let res = client
//         .transfer_and_watch(&hans, &alice_addr, 100_000_000_000_000_000)
//         .await;
//
//     if let Err(Error::Runtime(RuntimeError::Module(error))) = res {
//         let error2 = ModuleError {
//             module: "Balances".into(),
//             error: "InsufficientBalance".into(),
//         };
//         assert_eq!(error, error2);
//     } else {
//         panic!("expected an error");
//     }
// }
//
// #[async_std::test]
// async fn test_transfer_subscription() {
//     env_logger::try_init().ok();
//     let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
//     let bob = AccountKeyring::Bob.to_account_id();
//     let bob_addr = bob.clone().into();
//     let test_node_proc = test_node_process().await;
//     let client = test_node_proc.client();
//     let sub = client.subscribe_events().await.unwrap();
//     let decoder = client.events_decoder();
//     let mut sub = EventSubscription::<TestRuntime>::new(sub, &decoder);
//     sub.filter_event::<TransferEvent<_>>();
//     client.transfer(&alice, &bob_addr, 10_000).await.unwrap();
//     let raw = sub.next().await.unwrap().unwrap();
//     let event = TransferEvent::<TestRuntime>::decode(&mut &raw.data[..]).unwrap();
//     assert_eq!(
//         event,
//         TransferEvent {
//             from: alice.account_id().clone(),
//             to: bob.clone(),
//             amount: 10_000,
//         }
//     );
// }
