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

//! Implements support for the pallet_contracts module.

use codec::{
    Decode,
    Encode,
};

use sp_keyring::AccountKeyring;

use crate::{
    node_runtime::contracts::{
        calls::TransactionApi,
        events,
    },
    test_context,
    Runtime,
    TestContext,
    TestNodeProcess,
    TestRuntime,
};
use sp_core::sr25519::Pair;
use subxt::{
    Client,
    Error,
    ExtrinsicSuccess,
    PairSigner,
};

struct ContractsTestContext {
    cxt: TestContext,
    signer: PairSigner<TestRuntime, Pair>,
}

type Hash = <TestRuntime as Runtime>::Hash;
type Address = <TestRuntime as Runtime>::Address;
type AccountId = <TestRuntime as Runtime>::AccountId;

impl ContractsTestContext {
    async fn init() -> Self {
        env_logger::try_init().ok();

        let cxt = test_context().await;
        let signer = PairSigner::new(AccountKeyring::Alice.pair());

        Self { cxt, signer }
    }

    fn contracts_tx(&self) -> &TransactionApi<TestRuntime> {
        &self.cxt.api.tx.contracts
    }

    async fn instantiate_with_code(
        &self,
    ) -> Result<(Hash, AccountId), Error> {
        const CONTRACT: &str = r#"
                (module
                    (func (export "call"))
                    (func (export "deploy"))
                )
            "#;
        let code = wabt::wat2wasm(CONTRACT).expect("invalid wabt");

        let extrinsic = self.contracts_tx().instantiate_with_code(
            100_000_000_000_000_000, // endowment
            500_000_000_000,         // gas_limit
            code,
            vec![], // data
            vec![], // salt
        );
        let result = extrinsic.sign_and_submit_then_watch(&self.signer).await?;
        let code_stored = result
            .find_event::<events::CodeStored>()?
            .ok_or_else(|| Error::Other("Failed to find a CodeStored event".into()))?;
        let instantiated = result
            .find_event::<events::Instantiated>()?
            .ok_or_else(|| Error::Other("Failed to find a Instantiated event".into()))?;

        log::info!("Code hash: {:?}, Contract address: {:?}", code_stored.0, instantiated.0);
        Ok((code_stored.0, instantiated.0))
    }

    async fn instantiate(
        &self,
        code_hash: Hash,
        data: Vec<u8>,
        salt: Vec<u8>,
    ) -> Result<AccountId, Error> {
        // call instantiate extrinsic
        let extrinsic = self.contracts_tx().instantiate(
            100_000_000_000_000_000, // endowment
            500_000_000_000,         // gas_limit
            code_hash,
            data,
            salt,
        );
        let result = extrinsic.sign_and_submit_then_watch(&self.signer).await?;

        log::info!("Instantiate result: {:?}", result);
        let instantiated = result
            .find_event::<events::Instantiated>()?
            .ok_or_else(|| Error::Other("Failed to find a Instantiated event".into()))?;

        Ok(instantiated.0)
    }

    async fn call(
        &self,
        contract: AccountId,
        input_data: Vec<u8>,
    ) -> Result<ExtrinsicSuccess<TestRuntime>, Error> {
        let extrinsic = self.contracts_tx().call(
            contract.into(),
            0,           // value
            500_000_000, // gas_limit
            input_data,
        );
        let result = extrinsic.sign_and_submit_then_watch(&self.signer).await?;
        log::info!("Call result: {:?}", result);
        Ok(result)
    }
}

#[async_std::test]
async fn tx_instantiate_with_code() {
    let ctx = ContractsTestContext::init().await;
    let result = ctx.instantiate_with_code().await;

    assert!(
        result.is_ok(),
        "Error calling instantiate_with_code and receiving CodeStored and Instantiated Events: {:?}",
        result
    );
}

#[async_std::test]
async fn tx_instantiate() {
    let ctx = ContractsTestContext::init().await;
    let (code_hash, _) = ctx.instantiate_with_code().await.unwrap();

    let instantiated = ctx.instantiate(code_hash.into(), vec![], vec![1u8]).await;

    assert!(
        instantiated.is_ok(),
        "Error instantiating contract: {:?}",
        instantiated
    );
}

#[async_std::test]
async fn tx_call() {
    let ctx = ContractsTestContext::init().await;
    let (_, contract) = ctx.instantiate_with_code().await.unwrap();

    let executed = ctx.call(contract.into(), vec![]).await;

    assert!(executed.is_ok(), "Error calling contract: {:?}", executed);
}
