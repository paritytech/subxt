// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// ink! is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ink! is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::error::Result;
use futures::future::Future;
use jsonrpc_core_client::transports::ws;
use parity_codec::Codec;

use runtime_primitives::traits::SignedExtension;
use substrate_primitives::Pair;
use url::Url;

mod error;
mod rpc;

/// Captures data for when an extrinsic is successfully included in a block
#[derive(Debug)]
pub struct ExtrinsicSuccess<T: srml_system::Trait> {
    pub block: <T as srml_system::Trait>::Hash,
    pub extrinsic: <T as srml_system::Trait>::Hash,
    pub events: Vec<<T as srml_system::Trait>::Event>,
}

/// Creates, signs and submits an Extrinsic with the given `Call` to a substrate node.
pub fn submit<T, P, C, I, E, SE>(
    url: &Url,
    signer: P,
    call: C,
    extra: E,
) -> Result<ExtrinsicSuccess<T>>
where
    T: srml_system::Trait,
    P: Pair,
    P::Signature: Codec,
    P::Public: Into<T::AccountId>,
    C: Codec + Send + 'static,
    E: Fn(T::Index) -> SE + Send + 'static,
    SE: SignedExtension + 'static,
{
    let submit = ws::connect(url.as_str())
        .expect("Url is a valid url; qed")
        .map_err(Into::into)
        .and_then(|rpc: rpc::Rpc<T, C, P, E, SE>| {
            rpc.create_and_submit_extrinsic(signer, call, extra)
        });

    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(submit)
}
