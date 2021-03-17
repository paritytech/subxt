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

use super::*;
use sp_core::storage::{
    well_known_keys,
    StorageKey,
};
use sp_keyring::AccountKeyring;
use substrate_subxt_client::{
    DatabaseConfig,
    KeystoreConfig,
    Role,
    SubxtClient,
    SubxtClientConfig,
};
use tempdir::TempDir;

pub(crate) type TestRuntime = crate::NodeTemplateRuntime;

pub(crate) async fn test_client_with(
    key: AccountKeyring,
) -> (Client<TestRuntime>, TempDir) {
    env_logger::try_init().ok();
    let tmp = TempDir::new("subxt-").expect("failed to create tempdir");
    let config = SubxtClientConfig {
        impl_name: "substrate-subxt-full-client",
        impl_version: "0.0.1",
        author: "substrate subxt",
        copyright_start_year: 2020,
        db: DatabaseConfig::RocksDb {
            path: tmp.path().join("db"),
            cache_size: 128,
        },
        keystore: KeystoreConfig::Path {
            path: tmp.path().join("keystore"),
            password: None,
        },
        chain_spec: test_node::chain_spec::development_config().unwrap(),
        role: Role::Authority(key),
        telemetry: None,
        wasm_method: Default::default(),
    };
    let client = ClientBuilder::new()
        .set_client(
            SubxtClient::from_config(config, test_node::service::new_full)
                .expect("Error creating subxt client"),
        )
        .set_page_size(3)
        .build()
        .await
        .expect("Error creating client");
    (client, tmp)
}
pub(crate) async fn test_client() -> (Client<TestRuntime>, TempDir) {
    test_client_with(AccountKeyring::Alice).await
}

#[async_std::test]
async fn test_insert_key() {
    // Bob is not an authority, so block production should be disabled.
    let (client, _tmp) = test_client_with(AccountKeyring::Bob).await;
    let mut blocks = client.subscribe_blocks().await.unwrap();
    // get the genesis block.
    assert_eq!(blocks.next().await.unwrap().number, 0);
    let public = AccountKeyring::Alice.public().as_array_ref().to_vec();
    client
        .insert_key(
            "aura".to_string(),
            "//Alice".to_string(),
            public.clone().into(),
        )
        .await
        .unwrap();
    assert!(client
        .has_key(public.clone().into(), "aura".to_string())
        .await
        .unwrap());
    // Alice is an authority, so blocks should be produced.
    assert_eq!(blocks.next().await.unwrap().number, 1);
}

#[async_std::test]
async fn test_tx_transfer_balance() {
    let mut signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let (client, _) = test_client().await;
    let nonce = client
        .account(&AccountKeyring::Alice.to_account_id(), None)
        .await
        .unwrap()
        .nonce;
    signer.set_nonce(nonce);
    client
        .submit(
            balances::TransferCall {
                to: &dest,
                amount: 10_000,
            },
            &signer,
        )
        .await
        .unwrap();

    // check that nonce is handled correctly
    signer.increment_nonce();
    client
        .submit(
            balances::TransferCall {
                to: &dest,
                amount: 10_000,
            },
            &signer,
        )
        .await
        .unwrap();
}

#[async_std::test]
async fn test_getting_hash() {
    let (client, _) = test_client().await;
    client.block_hash(None).await.unwrap();
}

#[async_std::test]
async fn test_getting_block() {
    let (client, _) = test_client().await;
    let block_hash = client.block_hash(None).await.unwrap();
    client.block(block_hash).await.unwrap();
}

#[async_std::test]
async fn test_getting_read_proof() {
    let (client, _) = test_client().await;
    let block_hash = client.block_hash(None).await.unwrap();
    client
        .read_proof(
            vec![
                StorageKey(well_known_keys::HEAP_PAGES.to_vec()),
                StorageKey(well_known_keys::EXTRINSIC_INDEX.to_vec()),
            ],
            block_hash,
        )
        .await
        .unwrap();
}

#[async_std::test]
async fn test_chain_subscribe_blocks() {
    let (client, _) = test_client().await;
    let mut blocks = client.subscribe_blocks().await.unwrap();
    blocks.next().await;
}

#[async_std::test]
async fn test_chain_subscribe_finalized_blocks() {
    let (client, _) = test_client().await;
    let mut blocks = client.subscribe_finalized_blocks().await.unwrap();
    blocks.next().await;
}

#[async_std::test]
async fn test_fetch_keys() {
    let (client, _) = test_client().await;
    let keys = client
        .fetch_keys::<system::AccountStore<_>>(4, None, None)
        .await
        .unwrap();
    assert_eq!(keys.len(), 4)
}

#[async_std::test]
async fn test_iter() {
    let (client, _) = test_client().await;
    let mut iter = client.iter::<system::AccountStore<_>>(None).await.unwrap();
    let mut i = 0;
    while let Some(_) = iter.next().await.unwrap() {
        i += 1;
    }
    assert_eq!(i, 4);
}
