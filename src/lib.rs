// Copyright 2019 Parity Technologies (UK) Ltd.
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

use futures::future::Future;
use jsonrpc_core_client::transports::ws;
use metadata::Metadata;
use parity_scale_codec::{
    Codec,
    Decode,
};
use std::convert::TryFrom;
use substrate_primitives::Pair;
use url::Url;

pub use error::Error;

mod error;
mod metadata;
mod rpc;

pub use rpc::RpcTypes;

/// Captures data for when an extrinsic is successfully included in a block
#[derive(Debug)]
pub struct ExtrinsicSuccess<T: RpcTypes> {
    pub block: T::Hash,
    pub extrinsic: T::Hash,
    pub events: Vec<T::Event>,
}

/// Creates, signs and submits an Extrinsic with the given `Call` to a substrate node.
pub fn submit<T: rpc::RpcTypes, C: Codec + Send>(
    url: &Url,
    signer: T::Pair,
    call: C,
    extra: T::Extra,
) -> impl Future<Item = ExtrinsicSuccess<T>, Error = error::Error>
where
    <T::Pair as Pair>::Public: Into<T::AccountId>,
    <T::Pair as Pair>::Signature: Codec,
{
    ws::connect(url.as_str())
        .expect("Url is a valid url; qed")
        .map_err(Into::into)
        .and_then(|rpc: rpc::Rpc<T>| rpc.create_and_submit_extrinsic(signer, call, extra))
}

/// Fetches a storage key from a substrate node.
pub fn fetch<T: RpcTypes, V: Decode>(
    url: &Url,
    key: Vec<u8>,
) -> impl Future<Item = Option<V>, Error = error::Error> {
    ws::connect(url.as_str())
        .expect("Url is a valid url; qed")
        .map_err(Into::into)
        .and_then(|rpc: rpc::Rpc<T>| rpc.fetch::<V>(key))
        .map_err(Into::into)
}

/// Fetches a storage key from a substrate node
pub fn fetch_or<T: RpcTypes, V: Decode>(
    url: &Url,
    key: Vec<u8>,
    default: V,
) -> impl Future<Item = V, Error = error::Error> {
    fetch::<T, V>(url, key).map(|value| value.unwrap_or(default))
}

/// Fetches a storage key from a substrate node.
pub fn fetch_or_default<T: RpcTypes, V: Decode + Default>(
    url: &Url,
    key: Vec<u8>,
) -> impl Future<Item = V, Error = error::Error> {
    fetch::<T, V>(url, key).map(|value| value.unwrap_or_default())
}

/// Fetches the metadata from a substrate node.
pub fn metadata<T: RpcTypes>(
    url: &Url,
) -> impl Future<Item = Metadata, Error = error::Error> {
    ws::connect(url.as_str())
        .expect("Url is a valid url; qed")
        .map_err(Into::into)
        .and_then(|rpc: rpc::Rpc<T>| {
            rpc.fetch_metadata().map_err(Into::into).and_then(|meta| {
                let result =
                    TryFrom::try_from(meta).map_err(|err| format!("{:?}", err).into());
                futures::future::result(result)
            })
        })
}

#[cfg(test)]
pub mod tests {
    use node_runtime::Runtime;
    use parity_codec::Encode;
    use runtime_primitives::generic::Era;
    use runtime_support::StorageMap;
    use substrate_primitives::crypto::Pair as _;

    fn run<F>(f: F) -> Result<F::Item, F::Error>
    where
        F: futures::Future + Send + 'static,
        F::Item: Send + 'static,
        F::Error: Send + 'static,
    {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(f)
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn node_runtime_balance_transfer() {
        env_logger::try_init().ok();
        let url = url::Url::parse("ws://127.0.0.1:9944").unwrap();
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
        let future = super::submit::<Runtime, _, _, _, _>(&url, signer, call, extra);
        run(future).unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn node_runtime_fetch_account_balance() {
        env_logger::try_init().ok();
        let url = url::Url::parse("ws://127.0.0.1:9944").unwrap();
        let account = substrate_keyring::AccountKeyring::Alice.pair().public();
        let key = <srml_balances::FreeBalance<Runtime>>::key_for(&account);
        type Balance = <Runtime as srml_balances::Trait>::Balance;
        let future = super::fetch::<Runtime, (), (), (), (), Balance>(&url, key);
        run(future).unwrap();
    }

    #[test]
    #[ignore] // requires locally running substrate node
    fn node_runtime_fetch_metadata() {
        env_logger::try_init().ok();
        let url = url::Url::parse("ws://127.0.0.1:9944").unwrap();
        let future = super::metadata::<Runtime, (), (), (), ()>(&url);
        let metadata = run(future).unwrap();

        let dest = substrate_keyring::AccountKeyring::Bob.pair().public();
        let transfer = srml_balances::Call::transfer(dest.into(), 10_000);
        let call = node_runtime::Call::Balances(transfer.clone())
            .encode()
            .to_vec();
        let call2 = metadata.call("Balances", transfer).unwrap();
        assert_eq!(call, call2);
    }
}
