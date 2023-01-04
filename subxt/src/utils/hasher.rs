// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A basic `Hasher` trait and impl which is used in `Config`.

use codec::Encode;

/// Output from BlakeTwo256 hashing.
pub use primitive_types::H256;

/// Represents a type that can hash some value to some output.
/// Adapted from `sp_runtime::traits::Hash` and `sp_runtime::traits::Hasher`.
pub trait Hasher {
    /// The type given back from the hash operation
    type Output;

    /// Hash some bytes to the given output type.
    fn hash(s: &[u8]) -> Self::Output;

    /// Hash some SCALE encodable type to the given output type.
    fn hash_of<S: Encode>(s: &S) -> Self::Output {
        let out = s.encode();
        Self::hash(&out)
    }
}

/// A type that can hash values using the blaks2_256 algorithm.
pub struct BlakeTwo256;
impl Hasher for BlakeTwo256 {
    type Output = primitive_types::H256;
    fn hash(s: &[u8]) -> Self::Output {
        sp_core_hashing::blake2_256(s).into()
    }
}
