// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{
    node_runtime::{
        self,
        contracts::events,
        runtime_types::{pallet_contracts::wasm::Determinism, sp_weights::weight_v2::Weight},
        system,
    },
    submit_tx_wait_for_finalized_success, test_context, TestClient, TestConfig, TestContext,
};
use subxt::ext::futures::StreamExt;
use subxt::{tx::TxProgress, utils::MultiAddress, Config, Error};
use subxt_signer::sr25519::{self, dev};

struct ContractsTestContext {
    cxt: TestContext,
    signer: sr25519::Keypair,
}

type Hash = <TestConfig as Config>::Hash;
type AccountId = <TestConfig as Config>::AccountId;

/// A dummy contract which does nothing at all.
const CONTRACT: &str = r#"
    (module
        (import "env" "memory" (memory 1 1))
        (func (export "deploy"))
        (func (export "call"))
    )
"#;

const PROOF_SIZE: u64 = u64::MAX / 2;

impl ContractsTestContext {
    async fn init() -> Self {
        let cxt = test_context().await;
        let signer = dev::alice();

        Self { cxt, signer }
    }

    fn client(&self) -> TestClient {
        self.cxt.client()
    }

    async fn upload_code(&self) -> Result<Hash, Error> {
        let code = wabt::wat2wasm(CONTRACT).expect("invalid wabt");

        let upload_tx =
            node_runtime::tx()
                .contracts()
                .upload_code(code, None, Determinism::Enforced);

        let signed_extrinsic = self
            .client()
            .tx()
            .create_signed(&upload_tx, &self.signer, Default::default())
            .await?;
        let events = submit_tx_wait_for_finalized_success(&signed_extrinsic).await?;

        let code_stored = events
            .find_first::<events::CodeStored>()?
            .ok_or_else(|| Error::Other("Failed to find a CodeStored event".into()))?;
        Ok(code_stored.code_hash)
    }

    async fn instantiate_with_code(&self) -> Result<(Hash, AccountId), Error> {
        tracing::info!("instantiate_with_code:");
        let code = wabt::wat2wasm(CONTRACT).expect("invalid wabt");

        let instantiate_tx = node_runtime::tx().contracts().instantiate_with_code(
            100_000_000_000_000_000, // endowment
            Weight {
                ref_time: 500_000_000_000,
                proof_size: PROOF_SIZE,
            }, // gas_limit
            None,                    // storage_deposit_limit
            code,
            vec![], // data
            vec![], // salt
        );

        let signed_extrinsic = self
            .client()
            .tx()
            .create_signed(&instantiate_tx, &self.signer, Default::default())
            .await?;
        let events = submit_tx_wait_for_finalized_success(&signed_extrinsic).await?;

        let code_stored = events
            .find_first::<events::CodeStored>()?
            .ok_or_else(|| Error::Other("Failed to find a CodeStored event".into()))?;
        let instantiated = events
            .find_first::<events::Instantiated>()?
            .ok_or_else(|| Error::Other("Failed to find a Instantiated event".into()))?;
        let _extrinsic_success = events
            .find_first::<system::events::ExtrinsicSuccess>()?
            .ok_or_else(|| Error::Other("Failed to find a ExtrinsicSuccess event".into()))?;

        tracing::info!("  Block hash: {:?}", events.block_hash());
        tracing::info!("  Code hash: {:?}", code_stored.code_hash);
        tracing::info!("  Contract address: {:?}", instantiated.contract);
        Ok((code_stored.code_hash, instantiated.contract))
    }

    async fn instantiate(
        &self,
        code_hash: Hash,
        data: Vec<u8>,
        salt: Vec<u8>,
    ) -> Result<AccountId, Error> {
        // call instantiate extrinsic
        let instantiate_tx = node_runtime::tx().contracts().instantiate(
            100_000_000_000_000_000, // endowment
            Weight {
                ref_time: 500_000_000_000,
                proof_size: PROOF_SIZE,
            }, // gas_limit
            None,                    // storage_deposit_limit
            code_hash,
            data,
            salt,
        );

        let signed_extrinsic = self
            .client()
            .tx()
            .create_signed(&instantiate_tx, &self.signer, Default::default())
            .await?;
        let result = submit_tx_wait_for_finalized_success(&signed_extrinsic).await?;

        tracing::info!("Instantiate result: {:?}", result);
        let instantiated = result
            .find_first::<events::Instantiated>()?
            .ok_or_else(|| Error::Other("Failed to find a Instantiated event".into()))?;

        Ok(instantiated.contract)
    }

    async fn call(
        &self,
        contract: AccountId,
        input_data: Vec<u8>,
    ) -> Result<TxProgress<TestConfig, TestClient>, Error> {
        tracing::info!("call: {:?}", contract);
        let call_tx = node_runtime::tx().contracts().call(
            MultiAddress::Id(contract),
            0, // value
            Weight {
                ref_time: 500_000_000,
                proof_size: PROOF_SIZE,
            }, // gas_limit
            None, // storage_deposit_limit
            input_data,
        );

        let result = self
            .client()
            .tx()
            .sign_and_submit_then_watch_default(&call_tx, &self.signer)
            .await?;

        tracing::info!("Call result: {:?}", result);
        Ok(result)
    }
}

#[subxt_test(timeout = 800)]
async fn tx_instantiate_with_code() {
    let ctx = ContractsTestContext::init().await;
    let result = ctx.instantiate_with_code().await;

    assert!(
        result.is_ok(),
        "Error calling instantiate_with_code and receiving CodeStored and Instantiated Events: {result:?}"
    );
}

#[subxt_test(timeout = 800)]
async fn tx_instantiate() {
    let ctx = ContractsTestContext::init().await;
    let code_hash = ctx.upload_code().await.unwrap();

    let instantiated = ctx.instantiate(code_hash, vec![], vec![]).await;

    assert!(
        instantiated.is_ok(),
        "Error instantiating contract: {instantiated:?}"
    );
}

#[subxt_test(timeout = 800)]
async fn tx_call() {
    let cxt = ContractsTestContext::init().await;
    let (_, contract) = cxt.instantiate_with_code().await.unwrap();

    let info_addr = node_runtime::storage()
        .contracts()
        .contract_info_of(&contract);

    let contract_info = cxt
        .client()
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&info_addr)
        .await;

    assert!(
        contract_info.is_ok(),
        "Contract info is not ok, is: {contract_info:?}"
    );

    let info_addr_iter = node_runtime::storage().contracts().contract_info_of_iter();

    let keys_and_values = cxt
        .client()
        .storage()
        .at_latest()
        .await
        .unwrap()
        .iter(info_addr_iter)
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;

    assert_eq!(keys_and_values.len(), 1);
    println!("keys+values post: {keys_and_values:?}");

    let executed = cxt.call(contract, vec![]).await;

    assert!(executed.is_ok(), "Error calling contract: {executed:?}");
}
