// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Miscellaneous utility helpers.

mod account_id20;
mod era;
#[cfg(feature = "jsonrpsee")]
mod fetch_chain_spec;
mod multi_address;
mod multi_signature;
mod range_map;
mod static_type;
mod unchecked_extrinsic;
mod wrapper_opaque;
mod yesnomaybe;

pub mod bits;

use codec::{Compact, Decode, Encode};
use derive_where::derive_where;

pub use account_id20::AccountId20;
pub use era::Era;
pub use multi_address::MultiAddress;
pub use multi_signature::MultiSignature;
pub use primitive_types::{H160, H256, H512};
pub use range_map::{RangeMap, RangeMapBuilder, RangeMapError};
pub use static_type::Static;
pub use unchecked_extrinsic::UncheckedExtrinsic;
pub use wrapper_opaque::WrapperKeepOpaque;
pub use yesnomaybe::{Maybe, No, NoMaybe, Yes, YesMaybe, YesNo};

pub use subxt_utils_accountid32::AccountId32;

// Lightclient helper to fetch chain spec from a running node.
#[cfg(feature = "jsonrpsee")]
pub use fetch_chain_spec::{FetchChainspecError, fetch_chainspec_from_rpc_node};

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

/// Decodes a compact encoded value from the beginning of the provided bytes,
/// returning the value and any remaining bytes.
pub fn strip_compact_prefix(bytes: &[u8]) -> Result<(u64, &[u8]), codec::Error> {
    let cursor = &mut &*bytes;
    let val = <Compact<u64>>::decode(cursor)?;
    Ok((val.0, *cursor))
}

/// A version of [`core::marker::PhantomData`] that is also Send and Sync (which is fine
/// because regardless of the generic param, it is always possible to Send + Sync this
/// 0 size type).
#[derive(Encode, Decode, scale_info::TypeInfo)]
#[derive_where(Clone, PartialEq, Debug, Eq, Default, Hash)]
#[scale_info(skip_type_params(T))]
#[doc(hidden)]
pub struct PhantomDataSendSync<T>(core::marker::PhantomData<T>);

impl<T> PhantomDataSendSync<T> {
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

unsafe impl<T> Send for PhantomDataSendSync<T> {}
unsafe impl<T> Sync for PhantomDataSendSync<T> {}

/// This represents a key-value collection and is SCALE compatible
/// with collections like BTreeMap. This has the same type params
/// as `BTreeMap` which allows us to easily swap the two during codegen.
pub type KeyedVec<K, V> = Vec<(K, V)>;

/// A quick helper to encode some bytes to hex.
pub fn to_hex(bytes: impl AsRef<[u8]>) -> String {
    format!("0x{}", hex::encode(bytes.as_ref()))
}
