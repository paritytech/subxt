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

use crate::{
    error::Error,
    metadata::Metadata,
    srml::system::System,
};
use futures::future::{
    self,
    Future,
};
use jsonrpc_core_client::RpcChannel;
use log;
use num_traits::bounds::Bounded;
use parity_scale_codec::{
    Codec,
    Decode,
    Encode,
};

use runtime_metadata::RuntimeMetadataPrefixed;
use runtime_primitives::{
    generic::UncheckedExtrinsic,
    traits::StaticLookup,
};
use serde::{
    self,
    de::Error as DeError,
    Deserialize,
};
use std::convert::TryInto;
use substrate_primitives::{
    blake2_256,
    storage::StorageKey,
    Pair,
};
use substrate_rpc::{
    author::AuthorClient,
    chain::{
        number::NumberOrHex,
        ChainClient,
    },
    state::StateClient,
};

/// Copy of runtime_primitives::OpaqueExtrinsic to allow a local Deserialize impl
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
pub struct OpaqueExtrinsic(pub Vec<u8>);

impl std::fmt::Debug for OpaqueExtrinsic {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            substrate_primitives::hexdisplay::HexDisplay::from(&self.0)
        )
    }
}

impl<'a> serde::Deserialize<'a> for OpaqueExtrinsic {
    fn deserialize<D>(de: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let r = substrate_primitives::bytes::deserialize(de)?;
        Decode::decode(&mut &r[..])
            .map_err(|e| DeError::custom(format!("Decode error: {}", e)))
    }
}

/// Copy of runtime_primitives::generic::Block with Deserialize implemented
#[derive(PartialEq, Eq, Clone, Encode, Decode, Debug, Deserialize)]
pub struct Block {
    // not included: pub header: Header,
    /// The accompanying extrinsics.
    pub extrinsics: Vec<OpaqueExtrinsic>,
}

/// Copy of runtime_primitives::generic::SignedBlock with Deserialize implemented
#[derive(PartialEq, Eq, Clone, Encode, Decode, Debug, Deserialize)]
pub struct SignedBlock {
    /// Full block.
    pub block: Block,
}

/// Client for substrate rpc interfaces
pub struct Rpc<T: System> {
    state: StateClient<T::Hash>,
    chain: ChainClient<T::BlockNumber, T::Hash, (), SignedBlock>,
    author: AuthorClient<T::Hash, T::Hash>,
}

/// Allows connecting to all inner interfaces on the same RpcChannel
impl<T: System> From<RpcChannel> for Rpc<T> {
    fn from(channel: RpcChannel) -> Self {
        Self {
            state: channel.clone().into(),
            chain: channel.clone().into(),
            author: channel.into(),
        }
    }
}

impl<T: System> Rpc<T> {
    /// Fetch a storage key
    pub fn storage<V: Decode>(
        &self,
        key: StorageKey,
    ) -> impl Future<Item = Option<V>, Error = Error> {
        self.state
            .storage(key, None)
            .map(|data| {
                data.map(|d| Decode::decode(&mut &d.0[..]).expect("Valid storage key"))
            })
            .map_err(Into::into)
    }

    /// Fetch the genesis hash
    pub fn genesis_hash(&self) -> impl Future<Item = T::Hash, Error = Error> {
        let block_zero = T::BlockNumber::min_value();
        self.chain
            .block_hash(Some(NumberOrHex::Number(block_zero)))
            .map_err(Into::into)
            .and_then(|genesis_hash| {
                future::result(genesis_hash.ok_or("Genesis hash not found".into()))
            })
    }

    /// Fetch the metadata
    pub fn metadata(&self) -> impl Future<Item = Metadata, Error = Error> {
        self.state
            .metadata(None)
            .map(|bytes| Decode::decode(&mut &bytes[..]).unwrap())
            .map_err(Into::into)
            .and_then(|meta: RuntimeMetadataPrefixed| {
                future::result(meta.try_into().map_err(|err| format!("{:?}", err).into()))
            })
    }
}

impl<T: System> Rpc<T> {
    /// Create and submit an extrinsic and return corresponding Hash if successful
    pub fn create_and_submit_extrinsic<C, P>(
        self,
        signer: P,
        call: C,
        account_nonce: T::Index,
        genesis_hash: T::Hash,
    ) -> impl Future<Item = T::Hash, Error = Error>
    where
        C: Encode + Send,
        P: Pair,
        P::Public: Into<<T::Lookup as StaticLookup>::Source>,
        P::Signature: Codec,
    {
        let extrinsic =
            Self::create_and_sign_extrinsic(&signer, call, account_nonce, genesis_hash);

        self.author
            .submit_extrinsic(extrinsic.encode().into())
            .map_err(Into::into)
    }

    /// Creates and signs an Extrinsic for the supplied `Call`
    fn create_and_sign_extrinsic<C, P>(
        signer: &P,
        call: C,
        account_nonce: T::Index,
        genesis_hash: T::Hash,
    ) -> UncheckedExtrinsic<
        <T::Lookup as StaticLookup>::Source,
        C,
        P::Signature,
        T::SignedExtra,
    >
    where
        C: Encode + Send,
        P: Pair,
        P::Public: Into<<T::Lookup as StaticLookup>::Source>,
        P::Signature: Codec,
    {
        log::info!(
            "Creating Extrinsic with genesis hash {:?} and account nonce {:?}",
            genesis_hash,
            account_nonce
        );

        let extra = T::extra(account_nonce);
        let raw_payload = (call, extra.clone(), (&genesis_hash, &genesis_hash));
        let signature = raw_payload.using_encoded(|payload| {
            if payload.len() > 256 {
                signer.sign(&blake2_256(payload)[..])
            } else {
                signer.sign(payload)
            }
        });

        UncheckedExtrinsic::new_signed(
            raw_payload.0,
            signer.public().into(),
            signature.into(),
            extra,
        )
    }
}

use crate::ExtrinsicSuccess;
use futures::{
    future::IntoFuture,
    stream::Stream,
};
use jsonrpc_core_client::TypedSubscriptionStream;
use runtime_primitives::traits::Hash;
use srml_system::EventRecord;
use substrate_primitives::{
    storage::StorageChangeSet,
    twox_128,
};
use transaction_pool::txpool::watcher::Status;

impl<T: System> Rpc<T> {
    /// Subscribe to substrate System Events
    fn subscribe_events(
        &self,
    ) -> impl Future<Item = TypedSubscriptionStream<StorageChangeSet<T::Hash>>, Error = Error>
    {
        let events_key = b"System Events";
        let storage_key = twox_128(events_key);
        log::debug!("Events storage key {:?}", storage_key);

        self.state
            .subscribe_storage(Some(vec![StorageKey(storage_key.to_vec())]))
            .map_err(Into::into)
    }

    /// Submit an extrinsic, waiting for it to be finalized.
    /// If successful, returns the block hash.
    fn submit_and_watch<C, P>(
        self,
        extrinsic: UncheckedExtrinsic<
            <T::Lookup as StaticLookup>::Source,
            C,
            P::Signature,
            T::SignedExtra,
        >,
    ) -> impl Future<Item = T::Hash, Error = Error>
    where
        C: Encode + Send,
        P: Pair,
        P::Public: Into<<T::Lookup as StaticLookup>::Source>,
        P::Signature: Codec,
    {
        self.author
            .watch_extrinsic(extrinsic.encode().into())
            .map_err(Into::into)
            .and_then(|stream| {
                stream
                    .filter_map(|status| {
                        match status {
                            // ignore in progress extrinsic for now
                            Status::Future | Status::Ready | Status::Broadcast(_) => None,
                            Status::Finalized(block_hash) => Some(Ok(block_hash)),
                            Status::Usurped(_) => Some(Err("Extrinsic Usurped".into())),
                            Status::Dropped => Some(Err("Extrinsic Dropped".into())),
                            Status::Invalid => Some(Err("Extrinsic Invalid".into())),
                        }
                    })
                    .into_future()
                    .map_err(|(e, _)| e.into())
                    .and_then(|(result, _)| {
                        result
                            .ok_or(Error::from("Stream terminated"))
                            .and_then(|r| r)
                            .into_future()
                    })
            })
    }

    /// Create and submit an extrinsic and return corresponding Event if successful
    #[allow(unused)]
    pub fn create_and_watch_extrinsic<C, P>(
        self,
        signer: P,
        call: C,
        account_nonce: T::Index,
        genesis_hash: T::Hash,
    ) -> impl Future<Item = ExtrinsicSuccess<T>, Error = Error>
    where
        C: Encode + Send,
        P: Pair,
        P::Public: Into<<T::Lookup as StaticLookup>::Source>,
        P::Signature: Codec,
    {
        let events = self.subscribe_events().map_err(Into::into);
        events.and_then(move |events| {
            let extrinsic = Self::create_and_sign_extrinsic(
                &signer,
                call,
                account_nonce,
                genesis_hash,
            );
            let ext_hash = T::Hashing::hash_of(&extrinsic);
            log::info!("Submitting Extrinsic `{:?}`", ext_hash);

            let chain = self.chain.clone();
            self.submit_and_watch::<C, P>(extrinsic)
                .and_then(move |bh| {
                    log::info!("Fetching block {:?}", bh);
                    chain
                        .block(Some(bh))
                        .map(move |b| (bh, b))
                        .map_err(Into::into)
                })
                .and_then(|(h, b)| {
                    b.ok_or(format!("Failed to find block {:?}", h).into())
                        .map(|b| (h, b))
                        .into_future()
                })
                .and_then(move |(bh, sb)| {
                    log::info!(
                        "Found block {:?}, with {} extrinsics",
                        bh,
                        sb.block.extrinsics.len()
                    );
                    wait_for_block_events::<T>(ext_hash, &sb, bh, events)
                })
        })
    }
}

/// Waits for events for the block triggered by the extrinsic
fn wait_for_block_events<T: System>(
    ext_hash: T::Hash,
    signed_block: &SignedBlock,
    block_hash: T::Hash,
    events_stream: TypedSubscriptionStream<StorageChangeSet<T::Hash>>,
) -> impl Future<Item = ExtrinsicSuccess<T>, Error = Error> {
    let ext_index = signed_block
        .block
        .extrinsics
        .iter()
        .position(|ext| {
            let hash = T::Hashing::hash_of(ext);
            hash == ext_hash
        })
        .ok_or(format!("Failed to find Extrinsic with hash {:?}", ext_hash).into())
        .into_future();

    let block_hash = block_hash.clone();
    let block_events = events_stream
        .filter(move |event| event.block == block_hash)
        .into_future()
        .map_err(|(e, _)| e.into())
        .and_then(|(change_set, _)| {
            match change_set {
                None => future::ok(Vec::new()),
                Some(change_set) => {
                    let events = change_set
                        .changes
                        .iter()
                        .filter_map(|(_key, data)| {
                            data.as_ref().map(|data| Decode::decode(&mut &data.0[..]))
                        })
                        .collect::<Result<Vec<Vec<EventRecord<T::Event, T::Hash>>>, _>>()
                        .map(|events| events.into_iter().flat_map(|es| es).collect())
                        .map_err(Into::into);
                    future::result(events)
                }
            }
        });

    block_events
        .join(ext_index)
        .map(move |(events, ext_index)| {
            let events: Vec<T::Event> = events
                .iter()
                .filter_map(|e| {
                    if let srml_system::Phase::ApplyExtrinsic(i) = e.phase {
                        if i as usize == ext_index {
                            Some(e.event.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<T::Event>>();
            ExtrinsicSuccess {
                block: block_hash,
                extrinsic: ext_hash,
                events,
            }
        })
}
