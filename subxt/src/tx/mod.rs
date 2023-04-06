// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Create and submit extrinsics.
//!
//! An extrinsic is submitted with an "signed extra" and "additional" parameters, which can be
//! different for each chain. The trait [`crate::config::ExtrinsicParams`] determines exactly which
//! additional and signed extra parameters are used when constructing an extrinsic, and is a part
//! of the chain configuration (see [`crate::config::Config`]).

mod signer;
mod tx_client;
mod tx_payload;
mod tx_progress;

// The PairSigner impl currently relies on Substrate bits and pieces, so make it an optional
// feature if we want to avoid needing sp_core and sp_runtime.
#[cfg(feature = "substrate-compat")]
pub use self::signer::PairSigner;

pub use self::{
    signer::Signer,
    tx_client::{SubmittableExtrinsic, TxClient},
    tx_payload::{dynamic, BoxedPayload, DynamicPayload, Payload, TxPayload},
    tx_progress::{TxInBlock, TxProgress, TxStatus},
};

#[cfg(test)]
mod tests {
    use std::{iter, pin::Pin};

    use futures::Stream;
    use jsonrpsee::core::JsonRawValue;
    use primitive_types::H256;
    use serde::Serialize;
    use tokio::sync::mpsc;

    use crate::{
        client::{OfflineClientT, OnlineClientT},
        config::polkadot::PolkadotConfig,
        error::RpcError,
        rpc::{types::SubstrateTxStatus, RpcSubscription, Subscription},
        tx::TxProgress,
        Error, OnlineClient,
    };

    use serde_json::value::RawValue;

    #[derive(Clone, Debug)]
    struct MockClient;

    impl OfflineClientT<PolkadotConfig> for MockClient {
        fn metadata(&self) -> crate::Metadata {
            panic!("just a mock impl to satisfy trait bounds")
        }

        fn genesis_hash(&self) -> <PolkadotConfig as crate::Config>::Hash {
            panic!("just a mock impl to satisfy trait bounds")
        }

        fn runtime_version(&self) -> crate::rpc::types::RuntimeVersion {
            panic!("just a mock impl to satisfy trait bounds")
        }
    }

    impl OnlineClientT<PolkadotConfig> for MockClient {
        fn rpc(&self) -> &crate::rpc::Rpc<PolkadotConfig> {
            panic!("just a mock impl to satisfy trait bounds")
        }
    }

    #[tokio::test]
    async fn wait_for_finalized_returns_err_when_usurped() {
        let c = MockClient;
        let stream_elements: Vec<SubstrateTxStatus<H256, H256>> = vec![
            SubstrateTxStatus::Ready,
            SubstrateTxStatus::Usurped(Default::default()),
        ];
        let sub = create_substrate_tx_status_subscription(stream_elements);
        let tx_progress: TxProgress<PolkadotConfig, MockClient> =
            TxProgress::new(sub, c, Default::default());
        let finalized_result = tx_progress.wait_for_finalized().await;
        assert!(matches!(
            finalized_result,
            Err(Error::Transaction(crate::error::TransactionError::Usurped))
        ));
    }

    #[tokio::test]
    async fn wait_for_finalized_returns_err_when_dropped() {
        let c = MockClient;
        let stream_elements: Vec<SubstrateTxStatus<H256, H256>> =
            vec![SubstrateTxStatus::Ready, SubstrateTxStatus::Dropped];
        let sub = create_substrate_tx_status_subscription(stream_elements);
        let tx_progress: TxProgress<PolkadotConfig, MockClient> =
            TxProgress::new(sub, c, Default::default());
        let finalized_result = tx_progress.wait_for_finalized().await;
        assert!(matches!(
            finalized_result,
            Err(Error::Transaction(crate::error::TransactionError::Dropped))
        ));
    }

    #[tokio::test]
    async fn wait_for_finalized_returns_err_when_invalid() {
        let c = MockClient;
        let stream_elements: Vec<SubstrateTxStatus<H256, H256>> =
            vec![SubstrateTxStatus::Ready, SubstrateTxStatus::Invalid];
        let sub = create_substrate_tx_status_subscription(stream_elements);
        let tx_progress: TxProgress<PolkadotConfig, MockClient> =
            TxProgress::new(sub, c, Default::default());
        let finalized_result = tx_progress.wait_for_finalized().await;
        assert!(matches!(
            finalized_result,
            Err(Error::Transaction(crate::error::TransactionError::Invalid))
        ));
    }

    fn create_substrate_tx_status_subscription<Hash: Send + 'static + Serialize>(
        elements: Vec<SubstrateTxStatus<Hash, Hash>>,
    ) -> Subscription<SubstrateTxStatus<Hash, Hash>> {
        let rpc_substription_stream: Pin<
            Box<dyn Stream<Item = Result<Box<RawValue>, RpcError>> + Send + 'static>,
        > = Box::pin(futures::stream::iter(elements.into_iter().map(|e| {
            let s = serde_json::to_string(&e).unwrap();
            let r: Box<RawValue> = RawValue::from_string(s).unwrap();
            Ok(r)
        })));

        let subtxstatus: SubstrateTxStatus<Hash, Hash> = SubstrateTxStatus::Dropped;

        let rpc_subscription: RpcSubscription = RpcSubscription {
            stream: rpc_substription_stream,
            id: None,
        };

        let sub: Subscription<SubstrateTxStatus<Hash, Hash>> = Subscription::new(rpc_subscription);
        sub
    }
}
