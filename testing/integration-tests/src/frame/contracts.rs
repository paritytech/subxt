// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use sp_keyring::AccountKeyring;

use crate::{
    node_runtime::{
        self,
        contracts::events,
        system,
    },
    test_context,
    TestContext,
};
use sp_core::sr25519::Pair;
use sp_runtime::MultiAddress;
use subxt::{
    tx::{
        PairSigner,
        TxProgress,
    },
    Config,
    Error,
    OnlineClient,
    SubstrateConfig,
};

struct ContractsTestContext {
    cxt: TestContext,
    signer: PairSigner<SubstrateConfig, Pair>,
}

type Hash = <SubstrateConfig as Config>::Hash;
type AccountId = <SubstrateConfig as Config>::AccountId;

impl ContractsTestContext {
    async fn init() -> Self {
        let cxt = test_context().await;
        let signer = PairSigner::new(AccountKeyring::Alice.pair());

        Self { cxt, signer }
    }

    fn client(&self) -> OnlineClient<SubstrateConfig> {
        self.cxt.client()
    }

    async fn instantiate_with_code(&self) -> Result<(Hash, AccountId), Error> {
        tracing::info!("instantiate_with_code:");
        const CONTRACT: &str = r#"
                (module
                    (func (export "call"))
                    (func (export "deploy"))
                )
            "#;
        let code = wabt::wat2wasm(CONTRACT).expect("invalid wabt");

        let instantiate_tx = node_runtime::tx().contracts().instantiate_with_code(
            100_000_000_000_000_000, // endowment
            500_000_000_000,         // gas_limit
            None,                    // storage_deposit_limit
            code,
            vec![], // data
            vec![], // salt
        );

        let events = self
            .client()
            .tx()
            .sign_and_submit_then_watch_default(&instantiate_tx, &self.signer)
            .await?
            .wait_for_finalized_success()
            .await?;

        let code_stored = events
            .find_first::<events::CodeStored>()?
            .ok_or_else(|| Error::Other("Failed to find a CodeStored event".into()))?;
        let instantiated = events
            .find_first::<events::Instantiated>()?
            .ok_or_else(|| Error::Other("Failed to find a Instantiated event".into()))?;
        let _extrinsic_success = events
            .find_first::<system::events::ExtrinsicSuccess>()?
            .ok_or_else(|| {
                Error::Other("Failed to find a ExtrinsicSuccess event".into())
            })?;

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
            500_000_000_000,         // gas_limit
            None,                    // storage_deposit_limit
            code_hash,
            data,
            salt,
        );

        let result = self
            .client()
            .tx()
            .sign_and_submit_then_watch_default(&instantiate_tx, &self.signer)
            .await?
            .wait_for_finalized_success()
            .await?;

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
    ) -> Result<TxProgress<SubstrateConfig, OnlineClient<SubstrateConfig>>, Error> {
        tracing::info!("call: {:?}", contract);
        let call_tx = node_runtime::tx().contracts().call(
            MultiAddress::Id(contract),
            0,           // value
            500_000_000, // gas_limit
            None,        // storage_deposit_limit
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

#[tokio::test]
async fn tx_instantiate_with_code() {
    let ctx = ContractsTestContext::init().await;
    let result = ctx.instantiate_with_code().await;

    assert!(
        result.is_ok(),
        "Error calling instantiate_with_code and receiving CodeStored and Instantiated Events: {:?}",
        result
    );
}

#[tokio::test]
async fn tx_instantiate() {
    let ctx = ContractsTestContext::init().await;
    let (code_hash, _) = ctx.instantiate_with_code().await.unwrap();

    let instantiated = ctx.instantiate(code_hash, vec![], vec![1u8]).await;

    assert!(
        instantiated.is_ok(),
        "Error instantiating contract: {:?}",
        instantiated
    );
}

#[tokio::test]
async fn tx_call() {
    let cxt = ContractsTestContext::init().await;
    let (_, contract) = cxt.instantiate_with_code().await.unwrap();

    let info_addr = node_runtime::storage()
        .contracts()
        .contract_info_of(&contract);

    let contract_info = cxt.client().storage().fetch(&info_addr, None).await;
    assert!(contract_info.is_ok());

    let keys = cxt
        .client()
        .storage()
        .fetch_keys(&info_addr.to_bytes(), 10, None, None)
        .await
        .unwrap()
        .iter()
        .map(|key| hex::encode(&key.0))
        .collect::<Vec<_>>();
    println!("keys post: {:?}", keys);

    let executed = cxt.call(contract, vec![]).await;

    assert!(executed.is_ok(), "Error calling contract: {:?}", executed);
}
