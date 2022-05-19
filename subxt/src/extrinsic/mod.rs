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

//! Create signed or unsigned extrinsics.
//!
//! This modules exposes the extrinsic's parameters and the ability to sign an extrinsic.
//!
//!
//! An extrinsic can be submitted with an "signed extra" and "additional" parameters for
//! further customization. The trait [ExtrinsicParams] is at the core of an extrinsic parameter.
//!
//!
//! The structure [BaseExtrinsicParams] is a default implementation of the trait for the
//! Polkadot/Substrate chains.
//! For this implementation:
//! - "signed extra" contains:
//!     - sp_runtime::Era: This is utilized to determine the longevity of a transaction.
//!     - nonce: Account index (aka nonce) that stores the number of previous transactions
//!         associated with a sender account. This is utilized to avoid replay attacks.
//!     - tip: A tip payment for including the transaction in the block.
//! - "additional" contains:
//!     - spec_version: The version of the runtime specification of the node.
//!     - transaction_version: The version of the extrinsic interface. This allows hardware
//!         wallets to know which transactions can be safely signed.
//!     - genesis_hash: The hash of the Genesis block.
//!     - mortality_checkpoint: The block hash after which the transaction becomes valid.

mod params;
mod signer;

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
};
