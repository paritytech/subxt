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
    pub block: T::Hash,
    pub extrinsic: T::Hash,
    pub events: Vec<T::Event>,
}

/// Creates, signs and submits an Extrinsic with the given `Call` to a substrate node.
pub fn submit<T, P, C, E, SE>(
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

#[cfg(test)]
pub mod tests {
    use node_runtime::Runtime;
    use runtime_primitives::generic::Era;
    use substrate_primitives::crypto::Pair as _;

    #[test] #[ignore] // requires locally running substrate node
    fn node_runtime_balance_transfer() {
        let url = url::Url::parse("ws://localhost:9944").unwrap();
        let signer = substrate_keyring::AccountKeyring::Alice.pair();

        let dest = substrate_keyring::AccountKeyring::Bob.pair().public();
        let transfer = srml_balances::Call::transfer(dest.into(), 10_000);
        let call = node_runtime::Call::Balances(transfer);

        let extra = |nonce| {
            (
                srml_system::CheckGenesis::<Runtime>::new(),
                srml_system::CheckEra::<Runtime>::from(Era::Immortal),
                srml_system::CheckNonce::<Runtime>::from(nonce),
                srml_system::CheckWeight::<Runtime>::new(),
                srml_balances::TakeFees::<Runtime>::from(0),
            )
        };
        let result = super::submit::<Runtime, _, _, _, _>(&url, signer, call, extra);
        assert!(result.is_ok())
    }
}
