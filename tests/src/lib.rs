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

mod node_proc;

pub use node_proc::TestNodeProcess;

use sp_keyring::AccountKeyring;
use sp_runtime::traits::BlakeTwo256;
use subxt::{
    PairSigner,
    Runtime,
    subxt,
};

#[subxt(runtime_metadata_path = "node_runtime.scale")]
mod node_runtime {
    #[subxt(substitute_type = "sp_runtime::multiaddress::MultiAddress")]
    use sp_runtime::MultiAddress;
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestRuntime;

impl Runtime for TestRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = sp_runtime::AccountId32;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Header = sp_runtime::generic::Header<Self::BlockNumber, BlakeTwo256>;
    type Extra = subxt::extrinsic::DefaultExtra<Self>;
    type Signature = sp_runtime::MultiSignature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}

/// substrate node should be installed on the $PATH
const SUBSTRATE_NODE_PATH: &str = "substrate";

pub(crate) async fn test_node_process_with(
    key: AccountKeyring,
) -> TestNodeProcess<TestRuntime> {
    let path = std::env::var("SUBSTRATE_NODE_PATH").unwrap_or_else(|_| {
        if which::which(SUBSTRATE_NODE_PATH).is_err() {
            panic!("A substrate binary should be installed on your path for integration tests. \
            See https://github.com/paritytech/substrate-subxt/tree/master#integration-testing")
        }
        SUBSTRATE_NODE_PATH.to_string()
    });

    let proc = TestNodeProcess::<TestRuntime>::build(path.as_str())
        .with_authority(key)
        .scan_for_open_ports()
        .spawn::<TestRuntime>()
        .await;
    proc.unwrap()
}

pub(crate) async fn test_node_process() -> TestNodeProcess<TestRuntime> {
    test_node_process_with(AccountKeyring::Alice).await
}

// #[async_std::test]
// async fn test_insert_key() {
//     let test_node_process = test_node_process_with(AccountKeyring::Bob).await;
//     let client = test_node_process.client();
//     let public = AccountKeyring::Alice.public().as_array_ref().to_vec();
//     client
//         .insert_key(
//             "aura".to_string(),
//             "//Alice".to_string(),
//             public.clone().into(),
//         )
//         .await
//         .unwrap();
//     assert!(client
//         .has_key(public.clone().into(), "aura".to_string())
//         .await
//         .unwrap());
// }

#[async_std::test]
async fn test_tx_transfer_balance() {
    use crate::node_runtime::balances::calls::Transfer;

    let mut signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let node_process = test_node_process().await;
    let client = node_process.client();
    client
        .submit(
            Transfer {
                dest: &dest,
                value: 10_000,
            },
            &signer,
        )
        .await
        .unwrap();

    // check that nonce is handled correctly
    signer.increment_nonce();
    client
        .submit(
            Transfer {
                dest: &dest,
                value: 10_000,
            },
            &signer,
        )
        .await
        .unwrap();
}

// #[async_std::test]
// async fn test_getting_hash() {
//     let node_process = test_node_process().await;
//     node_process.client().block_hash(None).await.unwrap();
// }
//
// #[async_std::test]
// async fn test_getting_block() {
//     let node_process = test_node_process().await;
//     let client = node_process.client();
//     let block_hash = client.block_hash(None).await.unwrap();
//     client.block(block_hash).await.unwrap();
// }
//
// #[async_std::test]
// async fn test_getting_read_proof() {
//     let node_process = test_node_process().await;
//     let client = node_process.client();
//     let block_hash = client.block_hash(None).await.unwrap();
//     client
//         .read_proof(
//             vec![
//                 StorageKey(well_known_keys::HEAP_PAGES.to_vec()),
//                 StorageKey(well_known_keys::EXTRINSIC_INDEX.to_vec()),
//             ],
//             block_hash,
//         )
//         .await
//         .unwrap();
// }
//
// #[async_std::test]
// async fn test_chain_subscribe_blocks() {
//     let node_process = test_node_process().await;
//     let client = node_process.client();
//     let mut blocks = client.subscribe_blocks().await.unwrap();
//     blocks.next().await.unwrap();
// }
//
// #[async_std::test]
// async fn test_chain_subscribe_finalized_blocks() {
//     let node_process = test_node_process().await;
//     let client = node_process.client();
//     let mut blocks = client.subscribe_finalized_blocks().await.unwrap();
//     blocks.next().await.unwrap();
// }
//
// #[async_std::test]
// async fn test_fetch_keys() {
//     let node_process = test_node_process().await;
//     let client = node_process.client();
//     let keys = client
//         .fetch_keys::<system::AccountStore<_>>(4, None, None)
//         .await
//         .unwrap();
//     assert_eq!(keys.len(), 4)
// }
//
// #[async_std::test]
// async fn test_iter() {
//     let node_process = test_node_process().await;
//     let client = node_process.client();
//     let mut iter = client.iter::<system::AccountStore<_>>(None).await.unwrap();
//     let mut i = 0;
//     while let Some(_) = iter.next().await.unwrap() {
//         i += 1;
//     }
//     assert_eq!(i, 13);
// }
