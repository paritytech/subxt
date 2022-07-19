// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Create signed or unsigned extrinsics.
//!
//! This modules exposes the extrinsic's parameters and the ability to sign an extrinsic.
//!
//!
//! An extrinsic is submitted with an "signed extra" and "additional" parameters, which can be
//! different for each chain. The trait [ExtrinsicParams] determines exactly which
//! additional and signed extra parameters are used when constructing an extrinsic.
//!
//!
//! The structure [BaseExtrinsicParams] is a base implementation of the trait which
//! configures most of the "signed extra" and "additional" parameters as needed for
//! Polkadot and Substrate nodes. Only the shape of the tip payments differs, leading to
//! [SubstrateExtrinsicParams] and [PolkadotExtrinsicParams] structs which pick an
//! appropriate shape for Substrate/Polkadot chains respectively.

mod params;
mod signer;
mod tx_client;
mod tx_payload;
mod tx_progress;

pub use self::{
    params::{
        AssetTip,
        BaseExtrinsicParams,
        BaseExtrinsicParamsBuilder,
        Era,
        ExtrinsicParams,
        PlainTip,
        PolkadotExtrinsicParams,
        PolkadotExtrinsicParamsBuilder,
        SubstrateExtrinsicParams,
        SubstrateExtrinsicParamsBuilder,
    },
    signer::{
        PairSigner,
        Signer,
    },
    tx_client::{
        SignedSubmittableExtrinsic,
        TxClient,
    },
    tx_payload::{
        dynamic,
        DynamicTxPayload,
        StaticTxPayload,
        TxPayload,
    },
    tx_progress::{
        TxEvents,
        TxInBlock,
        TxProgress,
        TxStatus,
    },
};
