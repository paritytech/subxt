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
