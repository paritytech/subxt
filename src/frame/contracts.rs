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

use crate::frame::{
    balances::Balances,
    system::System,
};
use codec::{
    Decode,
    Encode,
};

/// Gas units are chosen to be represented by u64 so that gas metering
/// instructions can operate on them efficiently.
pub type Gas = u64;

/// The subset of the `pallet_contracts::Trait` that a client must implement.
#[module]
pub trait Contracts: System + Balances {}

/// Instantiates a new contract from the supplied `code` optionally transferring
/// some balance.
///
/// This is the only function that can deploy new code to the chain.
///
/// Instantiation is executed as follows:
///
/// - The supplied `code` is instrumented, deployed, and a `code_hash` is created for that code.
/// - If the `code_hash` already exists on the chain the underlying `code` will be shared.
/// - The destination address is computed based on the sender, code_hash and the salt.
/// - The smart-contract account is created at the computed address.
/// - The `endowment` is transferred to the new account.
/// - The `deploy` function is executed in the context of the newly-created account.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct InstantiateWithCodeCall<'a, T: Contracts> {
    /// The balance to transfer from the `origin` to the newly created contract.
    #[codec(compact)]
    pub endowment: <T as Balances>::Balance,
    /// The gas limit enforced when executing the constructor.
    #[codec(compact)]
    pub gas_limit: Gas,
    /// The contract code to deploy in raw bytes.
    pub code: &'a [u8],
    /// The input data to pass to the contract constructor.
    pub data: &'a [u8],
    /// Used for the address derivation.
    pub salt: &'a [u8],
}

/// Instantiates a contract from a previously deployed wasm binary.
///
/// This function is identical to [`InstantiateWithCodeCall`] but without the
/// code deployment step. Instead, the `code_hash` of an on-chain deployed wasm binary
/// must be supplied.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct InstantiateCall<'a, T: Contracts> {
    /// The balance to transfer from the `origin` to the newly created contract.
    #[codec(compact)]
    pub endowment: <T as Balances>::Balance,
    /// The gas limit enforced when executing the constructor.
    #[codec(compact)]
    pub gas_limit: Gas,
    /// Code hash of the already deployed on-chain deployed wasm binary.
    pub code_hash: &'a <T as System>::Hash,
    /// Data to initialize the contract with.
    pub data: &'a [u8],
    /// Used for the address derivation.
    pub salt: &'a [u8],
}

/// Makes a call to an account, optionally transferring some balance.
///
/// * If the account is a smart-contract account, the associated code will be
/// executed and any value will be transferred.
/// * If the account is a regular account, any value will be transferred.
/// * If no account exists and the call value is not less than `existential_deposit`,
/// a regular account will be created and any value will be transferred.
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct CallCall<'a, T: Contracts> {
    /// Address of the contract.
    pub dest: &'a <T as System>::Address,
    /// Value to transfer to the contract.
    #[codec(compact)]
    pub value: <T as Balances>::Balance,
    /// Gas limit.
    #[codec(compact)]
    pub gas_limit: Gas,
    /// Data to send to the contract.
    pub data: &'a [u8],
}

/// Code stored event.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct CodeStoredEvent<T: Contracts> {
    /// Code hash of the contract.
    pub code_hash: T::Hash,
}

/// Instantiated event.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct InstantiatedEvent<T: Contracts> {
    /// Caller that instantiated the contract.
    pub caller: <T as System>::AccountId,
    /// The address of the contract.
    pub contract: <T as System>::AccountId,
}

/// Contract execution event.
///
/// Emitted upon successful execution of a contract, if any contract events were produced.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ContractExecutionEvent<T: Contracts> {
    /// Caller of the contract.
    pub caller: <T as System>::AccountId,
    /// SCALE encoded contract event data.
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use sp_keyring::AccountKeyring;

    use super::*;
    use crate::{
        tests::{
            test_node_process,
            TestNodeProcess,
            TestRuntime,
        },
        Client,
        Error,
        ExtrinsicSuccess,
        PairSigner,
    };
    use sp_core::sr25519::Pair;

    struct TestContext {
        node_process: TestNodeProcess<TestRuntime>,
        signer: PairSigner<TestRuntime, Pair>,
    }

    impl TestContext {
        async fn init() -> Self {
            env_logger::try_init().ok();

            let node_process = test_node_process().await;
            let signer = PairSigner::new(AccountKeyring::Alice.pair());

            TestContext {
                node_process,
                signer,
            }
        }

        async fn instantiate_with_code(
            &self,
        ) -> Result<CodeStoredEvent<TestRuntime>, Error> {
            const CONTRACT: &str = r#"
                (module
                    (func (export "call"))
                    (func (export "deploy"))
                )
            "#;
            let code = wabt::wat2wasm(CONTRACT).expect("invalid wabt");

            let result = self
                .client()
                .instantiate_with_code_and_watch(
                    &self.signer,
                    100_000_000_000_000_000, // endowment
                    500_000_000_000,         // gas_limit
                    &code,
                    &[], // data
                    &[], // salt
                )
                .await?;
            let event = result.code_stored()?.ok_or_else(|| {
                Error::Other("Failed to find a CodeStored event".into())
            })?;
            log::info!("Code hash: {:?}", event.code_hash);
            Ok(event)
        }

        async fn instantiate(
            &self,
            code_hash: &<TestRuntime as System>::Hash,
            data: &[u8],
            salt: &[u8],
        ) -> Result<InstantiatedEvent<TestRuntime>, Error> {
            // call instantiate extrinsic
            let result = self
                .client()
                .instantiate_and_watch(
                    &self.signer,
                    100_000_000_000_000_000, // endowment
                    500_000_000_000,         // gas_limit
                    code_hash,
                    data,
                    salt,
                )
                .await?;

            log::info!("Instantiate result: {:?}", result);
            let instantiated = result.instantiated()?.ok_or_else(|| {
                Error::Other("Failed to find a Instantiated event".into())
            })?;

            Ok(instantiated)
        }

        async fn call(
            &self,
            contract: &<TestRuntime as System>::Address,
            input_data: &[u8],
        ) -> Result<ExtrinsicSuccess<TestRuntime>, Error> {
            let result = self
                .client()
                .call_and_watch(
                    &self.signer,
                    contract,
                    0,           // value
                    500_000_000, // gas_limit
                    input_data,
                )
                .await?;
            log::info!("Call result: {:?}", result);
            Ok(result)
        }

        fn client(&self) -> &Client<TestRuntime> {
            self.node_process.client()
        }
    }

    #[async_std::test]
    async fn tx_instantiate_with_code() {
        let ctx = TestContext::init().await;
        let code_stored = ctx.instantiate_with_code().await;

        assert!(
            code_stored.is_ok(),
            format!(
                "Error calling instantiate_with_code and receiving CodeStored Event: {:?}",
                code_stored
            )
        );
    }

    #[async_std::test]
    async fn tx_instantiate() {
        let ctx = TestContext::init().await;
        let code_stored = ctx.instantiate_with_code().await.unwrap();

        let instantiated = ctx.instantiate(&code_stored.code_hash, &[], &[1u8]).await;

        assert!(
            instantiated.is_ok(),
            format!("Error instantiating contract: {:?}", instantiated)
        );
    }

    #[async_std::test]
    async fn tx_call() {
        let ctx = TestContext::init().await;
        let code_stored = ctx.instantiate_with_code().await.unwrap();

        let instantiated = ctx
            .instantiate(&code_stored.code_hash.into(), &[], &[1u8])
            .await
            .unwrap();
        let executed = ctx.call(&instantiated.contract.into(), &[]).await;

        assert!(
            executed.is_ok(),
            format!("Error calling contract: {:?}", executed)
        );
    }
}
