// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
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
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use sp_keyring::AccountKeyring;

use crate::{
    node_runtime::{
        self,
        contracts::{
            calls::TransactionApi,
            events,
            storage,
        },
        system,
        DispatchError,
    },
    test_context,
    NodeRuntimeParams,
    TestContext,
};
use sp_core::sr25519::Pair;
use sp_runtime::MultiAddress;
use subxt::{
    Client,
    Config,
    DefaultConfig,
    Error,
    PairSigner,
    TransactionProgress,
};

struct ContractsTestContext {
    cxt: TestContext,
    signer: PairSigner<DefaultConfig, Pair>,
}

type Hash = <DefaultConfig as Config>::Hash;
type AccountId = <DefaultConfig as Config>::AccountId;

impl ContractsTestContext {
    async fn init() -> Self {
        tracing_subscriber::fmt::try_init().ok();
        let cxt = test_context().await;
        let signer = PairSigner::new(AccountKeyring::Alice.pair());

        Self { cxt, signer }
    }

    fn client(&self) -> &Client<DefaultConfig> {
        self.cxt.client()
    }

    fn contracts_tx(&self) -> TransactionApi<DefaultConfig, NodeRuntimeParams> {
        self.cxt.api.tx().contracts()
    }

    async fn instantiate_with_code(
        &self,
    ) -> Result<(Hash, AccountId), Error<DispatchError>> {
        tracing::info!("instantiate_with_code:");
        const CONTRACT: &str = r#"
                (module
                    (func (export "call"))
                    (func (export "deploy"))
                )
            "#;
        let code = wabt::wat2wasm(CONTRACT).expect("invalid wabt");

        let events = self
            .cxt
            .api
            .tx()
            .contracts()
            .instantiate_with_code(
                100_000_000_000_000_000, // endowment
                500_000_000_000,         // gas_limit
                None,                    // storage_deposit_limit
                code,
                vec![], // data
                vec![], // salt
            )?
            .sign_and_submit_then_watch_default(&self.signer)
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
    ) -> Result<AccountId, Error<DispatchError>> {
        // call instantiate extrinsic
        let result = self
            .contracts_tx()
            .instantiate(
                100_000_000_000_000_000, // endowment
                500_000_000_000,         // gas_limit
                None,                    // storage_deposit_limit
                code_hash,
                data,
                salt,
            )?
            .sign_and_submit_then_watch_default(&self.signer)
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
    ) -> Result<
        TransactionProgress<'_, DefaultConfig, DispatchError, node_runtime::Event>,
        Error<DispatchError>,
    > {
        tracing::info!("call: {:?}", contract);
        let result = self
            .contracts_tx()
            .call(
                MultiAddress::Id(contract),
                0,           // value
                500_000_000, // gas_limit
                None,        // storage_deposit_limit
                input_data,
            )?
            .sign_and_submit_then_watch_default(&self.signer)
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

    let contract_info = cxt
        .cxt
        .api
        .storage()
        .contracts()
        .contract_info_of(&contract, None)
        .await;
    assert!(contract_info.is_ok());

    let keys = cxt
        .client()
        .storage()
        .fetch_keys::<storage::ContractInfoOf>(5, None, None)
        .await
        .unwrap()
        .iter()
        .map(|key| hex::encode(&key.0))
        .collect::<Vec<_>>();
    println!("keys post: {:?}", keys);

    let executed = cxt.call(contract, vec![]).await;

    assert!(executed.is_ok(), "Error calling contract: {:?}", executed);
}
