// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Create and submit extrinsics.
//!
//! An extrinsic is submitted with an "signed extra" and "additional" parameters, which can be
//! different for each chain. The trait [`crate::config::ExtrinsicParams`] determines exactly which
//! additional and signed extra parameters are used when constructing an extrinsic, and is a part
//! of the chain configuration (see [`crate::config::Config`]).

mod tx_client;
mod tx_progress;

pub use subxt_core::tx::payload::{dynamic, DefaultPayload, DynamicPayload, Payload};
pub use subxt_core::tx::signer::{self, Signer};
pub use tx_client::{
    PartialExtrinsic, SubmittableExtrinsic, TransactionInvalid, TransactionUnknown, TxClient,
    ValidationResult,
};
pub use tx_progress::{TxInBlock, TxProgress, TxStatus};
