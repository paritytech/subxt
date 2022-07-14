// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Miscellaneous utility helpers.

use codec::{
    Decode,
    DecodeAll,
    Encode,
};
use derivative::Derivative;

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

/// A wrapper for any type `T` which implement encode/decode in a way compatible with `Vec<u8>`.
///
/// [`WrapperKeepOpaque`] stores the type only in its opaque format, aka as a `Vec<u8>`. To
/// access the real type `T` [`Self::try_decode`] needs to be used.
#[derive(Derivative, Encode, Decode)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Default(bound = ""),
    Hash(bound = "")
)]
pub struct WrapperKeepOpaque<T> {
    data: Vec<u8>,
    _phantom: PhantomDataSendSync<T>,
}

impl<T: Decode> WrapperKeepOpaque<T> {
    /// Try to decode the wrapped type from the inner `data`.
    ///
    /// Returns `None` if the decoding failed.
    pub fn try_decode(&self) -> Option<T> {
        T::decode_all(&mut &self.data[..]).ok()
    }

    /// Returns the length of the encoded `T`.
    pub fn encoded_len(&self) -> usize {
        self.data.len()
    }

    /// Returns the encoded data.
    pub fn encoded(&self) -> &[u8] {
        &self.data
    }

    /// Create from the given encoded `data`.
    pub fn from_encoded(data: Vec<u8>) -> Self {
        Self {
            data,
            _phantom: PhantomDataSendSync::new(),
        }
    }
}

/// A version of [`std::marker::PhantomData`] that is also Send and Sync (which is fine
/// because regardless of the generic param, it is always possible to Send + Sync this
/// 0 size type).
#[derive(Derivative, Encode, Decode, scale_info::TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = ""),
    Default(bound = ""),
    Hash(bound = "")
)]
#[scale_info(skip_type_params(T))]
#[doc(hidden)]
pub struct PhantomDataSendSync<T>(core::marker::PhantomData<T>);

impl<T> PhantomDataSendSync<T> {
    pub(crate) fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

unsafe impl<T> Send for PhantomDataSendSync<T> {}
unsafe impl<T> Sync for PhantomDataSendSync<T> {}

/// This represents a key-value collection and is SCALE compatible
/// with collections like BTreeMap. This has the same type params
/// as `BTreeMap` which allows us to easily swap the two during codegen.
pub type KeyedVec<K, V> = Vec<(K, V)>;
